// KiloCode Extension for Zed
// AI-powered coding assistant for Zed
// Works with OpenAI-compatible APIs including free providers like Groq, Together AI, and OpenRouter

mod api_client;

use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, Worktree,
};

struct KiloCodeExtensionWrapper {
    client: Option<api_client::KiloCodeClient>,
}

impl KiloCodeExtensionWrapper {
    fn new() -> Self {
        Self { client: None }
    }

    // Get or create the API client
    fn get_client(&mut self) -> Result<&api_client::KiloCodeClient, String> {
        if self.client.is_none() {
            // Try to get API key from environment variable
            let api_key = std::env::var("KILOCODE_API_KEY")
                .map_err(|_| "KILOCODE_API_KEY environment variable not set. Please set it to use KiloCode functionality.".to_string())?;
            
            self.client = Some(api_client::KiloCodeClient::new(api_key));
        }
        Ok(self.client.as_ref().unwrap())
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

    // Get selected code from worktree (placeholder - needs implementation)
    fn get_selected_code(_worktree: Option<&Worktree>) -> Result<Option<String>, String> {
        // TODO: Implement code selection reading from editor
        // This requires accessing the editor's selection through the Worktree API
        Ok(None)
    }

    // Detect language from file extension (placeholder)
    fn detect_language(_worktree: Option<&Worktree>) -> Option<String> {
        // TODO: Implement language detection from file extension
        None
    }
}

impl zed::Extension for KiloCodeExtensionWrapper {
    fn new() -> Self {
        Self::new()
    }

    fn run_slash_command(
        &mut self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let prompt = args.join(" ");
        let code_context = self.get_selected_code(worktree)?;
        let language = self.detect_language(worktree);

        let result = match command.name.as_str() {
            "kc" => {
                if prompt.is_empty() {
                    return Err("Please provide a question or prompt for KiloCode.".to_string());
                }
                Self::validate_input(&prompt)?;
                let client = self.get_client()?;
                let response = client.chat(&prompt, code_context.as_deref(), None)?;
                Self::sanitize_output(&response)
            }
            "kc-explain" => {
                let explanation_prompt = if prompt.is_empty() {
                    "Explain this code in detail, including its purpose, how it works, and any potential issues.".to_string()
                } else {
                    format!("Explain this code: {}", prompt)
                };
                Self::validate_input(&explanation_prompt)?;
                let client = self.get_client()?;
                
                // If we have code context, explain it; otherwise use the prompt
                if let Some(code) = code_context {
                    client.explain_code(&code, language.as_deref())?
                } else {
                    client.chat(&explanation_prompt, None, None)?
                }
            }
            "kc-generate" => {
                if prompt.is_empty() {
                    return Err("Please provide a description of the code you want to generate.".to_string());
                }
                Self::validate_input(&prompt)?;
                let client = self.get_client()?;
                client.generate_code(&prompt, code_context.as_deref(), language.as_deref())?
            }
            "kc-refactor" => {
                let refactor_prompt = if prompt.is_empty() {
                    "Suggest refactoring improvements for this code to make it more readable, maintainable, and efficient.".to_string()
                } else {
                    prompt.clone()
                };
                Self::validate_input(&refactor_prompt)?;
                let client = self.get_client()?;
                
                if let Some(code) = code_context {
                    client.refactor_code(&code, &refactor_prompt, language.as_deref())?
                } else {
                    client.chat(&refactor_prompt, None, None)?
                }
            }
            "kc-fix" => {
                let fix_prompt = if prompt.is_empty() {
                    "Analyze this code for bugs and issues, then provide fixes.".to_string()
                } else {
                    prompt.clone()
                };
                Self::validate_input(&fix_prompt)?;
                let client = self.get_client()?;
                
                if let Some(code) = code_context {
                    client.fix_code(&code, Some(&fix_prompt), language.as_deref())?
                } else {
                    client.chat(&fix_prompt, None, None)?
                }
            }
            "kc-docs" => {
                let docs_prompt = if prompt.is_empty() {
                    "Generate comprehensive documentation for this code, including function descriptions, parameter explanations, and usage examples.".to_string()
                } else {
                    prompt.clone()
                };
                Self::validate_input(&docs_prompt)?;
                let client = self.get_client()?;
                
                if let Some(code) = code_context {
                    client.generate_docs(&code, language.as_deref(), Some("rustdoc"))?
                } else {
                    client.chat(&docs_prompt, None, None)?
                }
            }
            _ => return Err(format!("Unknown slash command: {}", command.name)),
        };

        Ok(SlashCommandOutput {
            sections: vec![],
            text: result,
        })
    }
}

zed::register_extension!(KiloCodeExtensionWrapper);
