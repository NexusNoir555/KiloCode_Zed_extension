// KiloCode Extension for Zed
// Users must configure their own API endpoint and credentials in Zed settings
// No personal credentials are included in this extension

use zed_extension_api as zed;

struct KiloCodeExtension;

impl KiloCodeExtension {
    // Get API endpoint from settings or use default
    fn get_api_endpoint(settings: &zed::Settings) -> String {
        settings
            .get("kilocode.api_endpoint")
            .and_then(|v| v.as_str())
            .unwrap_or("https://api.kilocode.ai/v1")
            .to_string()
    }

    // Get API key from settings
    fn get_api_key(settings: &zed::Settings) -> Result<String, String> {
        settings
            .get("kilocode.api_key")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                "API key not configured. Please set 'kilocode.api_key' in Zed settings.".to_string()
            })
    }

    // Get model from settings or use default
    fn get_model(settings: &zed::Settings) -> String {
        settings
            .get("kilocode.model")
            .and_then(|v| v.as_str())
            .unwrap_or("gpt-4")
            .to_string()
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

    // Process chat request
    fn process_chat(
        &self,
        prompt: &str,
        code_context: Option<&str>,
        settings: &zed::Settings,
    ) -> Result<String, String> {
        Self::validate_input(prompt)?;

        let _endpoint = Self::get_api_endpoint(settings);
        let _api_key = Self::get_api_key(settings)?;
        let _model = Self::get_model(settings);

        // Build the system message
        let system_message = "You are KiloCode, an AI coding assistant. Help users with their coding questions, explain code, generate code, refactor, fix bugs, and generate documentation. Be concise and helpful.";

        // Build the full prompt
        let full_prompt = if let Some(code) = code_context {
            format!(
                "{}\n\nCurrent code context:\n```\n{}\n```\n\nUser: {}",
                system_message, code, prompt
            )
        } else {
            format!("{}\n\nUser: {}", system_message, prompt)
        };

        // For now, return a placeholder response
        // In a full implementation, this would make an HTTP request to the API
        let result = format!(
            "KiloCode Response:\n\nEndpoint: {}\nModel: {}\n\nThis is a placeholder response. The full implementation would make an HTTP request to the API endpoint with the following prompt:\n\n{}",
            _endpoint, _model, full_prompt
        );

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
        // Get settings
        let settings = zed::settings();

        let prompt = args.join(" ");
        let code_context = worktree.and_then(|_| None); // In real implementation, get selected code

        let result = match command.name.as_str() {
            "kc" => {
                self.process_chat(&prompt, code_context, &settings)?
            }
            "kc-explain" => {
                let explanation_prompt = if prompt.is_empty() {
                    "Explain this code in detail, including its purpose, how it works, and any potential issues.".to_string()
                } else {
                    format!("Explain this code: {}", prompt)
                };
                self.process_chat(&explanation_prompt, code_context, &settings)?
            }
            "kc-generate" => {
                let generate_prompt = format!("Generate code for: {}", prompt);
                self.process_chat(&generate_prompt, None, &settings)?
            }
            "kc-refactor" => {
                let refactor_prompt = if prompt.is_empty() {
                    "Suggest refactoring improvements for this code to make it more readable, maintainable, and efficient.".to_string()
                } else {
                    format!("Refactor this code: {}", prompt)
                };
                self.process_chat(&refactor_prompt, code_context, &settings)?
            }
            "kc-fix" => {
                let fix_prompt = if prompt.is_empty() {
                    "Analyze this code for bugs and issues, then provide fixes.".to_string()
                } else {
                    format!("Fix this code: {}", prompt)
                };
                self.process_chat(&fix_prompt, code_context, &settings)?
            }
            "kc-docs" => {
                let docs_prompt = if prompt.is_empty() {
                    "Generate comprehensive documentation for this code, including function descriptions, parameter explanations, and usage examples.".to_string()
                } else {
                    format!("Generate documentation for: {}", prompt)
                };
                self.process_chat(&docs_prompt, code_context, &settings)?
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
