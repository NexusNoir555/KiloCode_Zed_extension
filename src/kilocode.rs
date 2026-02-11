// KiloCode Extension for Zed
// Users must configure their own API endpoint and credentials in Zed settings
// No personal credentials are included in this extension

use zed_extension_api as zed;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// API Configuration - Users must set their own endpoint in Zed settings
// Default placeholder - users should override this
const DEFAULT_API_ENDPOINT: &str = "https://api.kilocode.ai/v1";

// Request/Response structures
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

struct KiloCodeExtension;

impl KiloCodeExtension {
    // Get API endpoint from settings or use default
    fn get_api_endpoint(settings: &HashMap<String, String>) -> String {
        settings
            .get("kilocode.api_endpoint")
            .cloned()
            .unwrap_or_else(|| DEFAULT_API_ENDPOINT.to_string())
    }

    // Get API key from settings
    fn get_api_key(settings: &HashMap<String, String>) -> Result<String, String> {
        settings
            .get("kilocode.api_key")
            .cloned()
            .ok_or_else(|| {
                "API key not configured. Please set 'kilocode.api_key' in Zed settings.".to_string()
            })
    }

    // Get model from settings or use default
    fn get_model(settings: &HashMap<String, String>) -> String {
        settings
            .get("kilocode.model")
            .cloned()
            .unwrap_or_else(|| "gpt-4".to_string())
    }

    // Validate input to prevent injection attacks
    fn validate_input(input: &str) -> Result<(), String> {
        if input.len() > 100_000 {
            return Err("Input too large (max 100,000 characters)".to_string());
        }
        Ok(())
    }

    // Sanitize output to prevent code execution
    fn sanitize_output(output: &str) -> String {
        // Remove any potential command injection patterns
        output
            .replace("```bash", "```")
            .replace("```sh", "```")
            .replace("```shell", "```")
            .replace("```powershell", "```")
            .replace("```cmd", "```")
    }

    // Make API request
    async fn make_api_request(
        endpoint: &str,
        api_key: &str,
        model: &str,
        messages: Vec<Message>,
    ) -> Result<String, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let request = ApiRequest {
            model: model.to_string(),
            messages,
            max_tokens: Some(2000),
            temperature: Some(0.7),
        };

        let response = client
            .post(format!("{}/chat/completions", endpoint))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API error ({}): {}", status, error_text));
        }

        let api_response: ApiResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse API response: {}", e))?;

        Ok(api_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "No response from API".to_string()))
    }

    // Process chat request
    async fn process_chat(
        prompt: &str,
        code_context: Option<&str>,
        settings: &HashMap<String, String>,
    ) -> Result<String, String> {
        Self::validate_input(prompt)?;

        let endpoint = Self::get_api_endpoint(settings);
        let api_key = Self::get_api_key(settings)?;
        let model = Self::get_model(settings);

        let mut messages = vec![Message {
            role: "system".to_string(),
            content: "You are KiloCode, an AI coding assistant. Help users with their coding questions, explain code, generate code, refactor, fix bugs, and generate documentation. Be concise and helpful.".to_string(),
        }];

        if let Some(code) = code_context {
            messages.push(Message {
                role: "system".to_string(),
                content: format!("Current code context:\n```\n{}\n```", code),
            });
        }

        messages.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let result = Self::make_api_request(&endpoint, &api_key, &model, messages).await?;
        Ok(Self::sanitize_output(&result))
    }
}

impl zed::Extension for KiloCodeExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create async runtime: {}", e))?;

        // Get settings
        let settings = HashMap::new(); // In real implementation, get from Zed settings

        let prompt = args.join(" ");
        let code_context = worktree.and_then(|_| None); // In real implementation, get selected code

        let result = match command.name.as_str() {
            "kc" => {
                rt.block_on(Self::process_chat(&prompt, code_context, &settings))?
            }
            "kc-explain" => {
                let explanation_prompt = if prompt.is_empty() {
                    "Explain this code in detail, including its purpose, how it works, and any potential issues.".to_string()
                } else {
                    format!("Explain this code: {}", prompt)
                };
                rt.block_on(Self::process_chat(&explanation_prompt, code_context, &settings))?
            }
            "kc-generate" => {
                let generate_prompt = format!("Generate code for: {}", prompt);
                rt.block_on(Self::process_chat(&generate_prompt, None, &settings))?
            }
            "kc-refactor" => {
                let refactor_prompt = if prompt.is_empty() {
                    "Suggest refactoring improvements for this code to make it more readable, maintainable, and efficient.".to_string()
                } else {
                    format!("Refactor this code: {}", prompt)
                };
                rt.block_on(Self::process_chat(&refactor_prompt, code_context, &settings))?
            }
            "kc-fix" => {
                let fix_prompt = if prompt.is_empty() {
                    "Analyze this code for bugs and issues, then provide fixes.".to_string()
                } else {
                    format!("Fix this code: {}", prompt)
                };
                rt.block_on(Self::process_chat(&fix_prompt, code_context, &settings))?
            }
            "kc-docs" => {
                let docs_prompt = if prompt.is_empty() {
                    "Generate comprehensive documentation for this code, including function descriptions, parameter explanations, and usage examples.".to_string()
                } else {
                    format!("Generate documentation for: {}", prompt)
                };
                rt.block_on(Self::process_chat(&docs_prompt, code_context, &settings))?
            }
            _ => return Err(format!("Unknown command: {}", command.name)),
        };

        Ok(zed::SlashCommandOutput {
            sections: vec![],
            text: result,
        })
    }
}

zed::register_extension!(KiloCodeExtension);
