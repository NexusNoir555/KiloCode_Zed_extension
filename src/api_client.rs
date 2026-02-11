// api_client.rs - KiloCode API client for Zed extension
// This module handles HTTP communication with OpenAI-compatible APIs
// Supports multiple providers: OpenAI, Groq, Together, OpenRouter, etc.

use zed_extension_api::http_client::*;
use serde::{Deserialize, Serialize};

/// KiloCode API client (OpenAI-compatible)
pub struct KiloCodeClient {
    api_key: String,
    base_url: String,
    model: String,
}

/// OpenAI-compatible chat message
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// OpenAI-compatible chat request
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

/// OpenAI-compatible chat response
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
}

impl KiloCodeClient {
    /// Create a new KiloCode API client with default settings
    /// Reads configuration from environment variables:
    /// - KILOCODE_API_KEY: API key (required)
    /// - KILOCODE_API_URL: Base URL (default: https://api.openai.com/v1)
    /// - KILOCODE_MODEL: Model name (default: gpt-3.5-turbo)
    pub fn new(api_key: String) -> Self {
        let base_url = std::env::var("KILOCODE_API_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = std::env::var("KILOCODE_MODEL")
            .unwrap_or_else(|_| "gpt-3.5-turbo".to_string());
        
        Self { api_key, base_url, model }
    }

    /// Create a new KiloCode API client with custom base URL and model
    pub fn with_config(api_key: String, base_url: String, model: String) -> Self {
        Self { api_key, base_url, model }
    }

    /// Make an HTTP POST request to the API
    fn make_request(&self, endpoint: &str, body: Vec<u8>) -> Result<String, String> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let request = HttpRequestBuilder::new()
            .method(HttpMethod::Post)
            .url(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("User-Agent", "KiloCode-Zed-Extension/0.1.0")
            .body(body)
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;

        let response = http_client::fetch(&request).map_err(|e| format!("HTTP request failed: {}", e))?;

        // Parse response body
        let response_text = String::from_utf8(response.body)
            .map_err(|e| format!("Failed to parse response as UTF-8: {}", e))?;

        // Check for errors in response body
        if response_text.contains("\"error\":") {
            if let Ok(chat_response) = serde_json::from_str::<ChatResponse>(&response_text) {
                if let Some(error) = chat_response.error {
                    return Err(format!("API Error: {}", error.message));
                }
            }
            return Err(format!("API Error: {}", response_text));
        }

        Ok(response_text)
    }

    /// Build system prompt for different tasks
    fn build_system_prompt(task: &str) -> String {
        match task {
            "explain" => "You are KiloCode, an AI coding assistant. Explain the provided code in detail, including its purpose, how it works, and any potential issues. Be clear and educational.".to_string(),
            "generate" => "You are KiloCode, an AI coding assistant. Generate code based on the user's description. Provide clean, well-commented, and efficient code. Include necessary imports and structure.".to_string(),
            "refactor" => "You are KiloCode, an AI coding assistant. Suggest refactoring improvements for the provided code to make it more readable, maintainable, and efficient. Explain your changes.".to_string(),
            "fix" => "You are KiloCode, an AI coding assistant. Analyze the provided code for bugs and issues, then provide fixes. Explain what was wrong and how your fix addresses it.".to_string(),
            "docs" => "You are KiloCode, an AI coding assistant. Generate comprehensive documentation for the provided code, including function descriptions, parameter explanations, return values, and usage examples. Use standard documentation format.".to_string(),
            _ => "You are KiloCode, an AI coding assistant. Help users with their coding questions, explain code, generate code, refactor, fix bugs, and generate documentation. Be concise and helpful.".to_string(),
        }
    }

    /// Build messages for chat completion
    fn build_messages(&self, system_prompt: &str, user_message: &str, code_context: Option<&str>) -> Vec<ChatMessage> {
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            }
        ];

        if let Some(code) = code_context {
            messages.push(ChatMessage {
                role: "user".to_string(),
                content: format!("Here is the code context:\n```\n{}\n```\n\n{}", code, user_message),
            });
        } else {
            messages.push(ChatMessage {
                role: "user".to_string(),
                content: user_message.to_string(),
            });
        }

        messages
    }

    /// Make a chat completion request
    fn chat_completion(&self, messages: Vec<ChatMessage>) -> Result<String, String> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(2000),
        };

        let body = serde_json::to_vec(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let response_text = self.make_request("chat/completions", body)?;

        // Parse response
        let chat_response: ChatResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        chat_response.choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| "No response from API".to_string())
    }

    /// Generate code from a prompt
    pub fn generate_code(&self, prompt: &str, context: Option<&str>, language: Option<&str>) -> Result<String, String> {
        let system_prompt = Self::build_system_prompt("generate");
        let user_message = if let Some(lang) = language {
            format!("Generate {} code for: {}", lang, prompt)
        } else {
            format!("Generate code for: {}", prompt)
        };
        
        let messages = self.build_messages(&system_prompt, &user_message, context);
        self.chat_completion(messages)
    }

    /// Explain code
    pub fn explain_code(&self, code: &str, language: Option<&str>) -> Result<String, String> {
        let system_prompt = Self::build_system_prompt("explain");
        let user_message = if let Some(lang) = language {
            format!("Explain this {} code in detail.", lang)
        } else {
            "Explain this code in detail.".to_string()
        };
        
        let messages = self.build_messages(&system_prompt, &user_message, Some(code));
        self.chat_completion(messages)
    }

    /// Refactor code
    pub fn refactor_code(&self, code: &str, instructions: &str, language: Option<&str>) -> Result<String, String> {
        let system_prompt = Self::build_system_prompt("refactor");
        let user_message = if let Some(lang) = language {
            format!("Refactor this {} code. {}", lang, instructions)
        } else {
            format!("Refactor this code. {}", instructions)
        };
        
        let messages = self.build_messages(&system_prompt, &user_message, Some(code));
        self.chat_completion(messages)
    }

    /// Fix bugs in code
    pub fn fix_code(&self, code: &str, error_message: Option<&str>, language: Option<&str>) -> Result<String, String> {
        let system_prompt = Self::build_system_prompt("fix");
        let user_message = if let Some(error) = error_message {
            format!("Fix the bugs in this code. Error: {}", error)
        } else {
            "Fix the bugs in this code.".to_string()
        };
        
        if let Some(lang) = language {
            let user_message = format!("{} (Language: {})", user_message, lang);
            let messages = self.build_messages(&system_prompt, &user_message, Some(code));
            self.chat_completion(messages)
        } else {
            let messages = self.build_messages(&system_prompt, &user_message, Some(code));
            self.chat_completion(messages)
        }
    }

    /// Generate documentation for code
    pub fn generate_docs(&self, code: &str, language: Option<&str>, style: Option<&str>) -> Result<String, String> {
        let system_prompt = Self::build_system_prompt("docs");
        let user_message = if let Some(lang) = language {
            if let Some(doc_style) = style {
                format!("Generate {} documentation for this {} code.", doc_style, lang)
            } else {
                format!("Generate documentation for this {} code.", lang)
            }
        } else {
            "Generate documentation for this code.".to_string()
        };
        
        let messages = self.build_messages(&system_prompt, &user_message, Some(code));
        self.chat_completion(messages)
    }

    /// General chat interaction
    pub fn chat(&self, message: &str, context: Option<&str>, _history: Option<Vec<ChatMessage>>) -> Result<String, String> {
        let system_prompt = Self::build_system_prompt("chat");
        let messages = self.build_messages(&system_prompt, message, context);
        self.chat_completion(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = KiloCodeClient::new("test_key".to_string());
        assert_eq!(client.api_key, "test_key");
        assert_eq!(client.base_url, "https://api.openai.com/v1");
        assert_eq!(client.model, "gpt-3.5-turbo");
    }

    #[test]
    fn test_client_with_config() {
        let client = KiloCodeClient::with_config(
            "test_key".to_string(),
            "https://api.groq.com/openai/v1".to_string(),
            "llama3-70b-8192".to_string()
        );
        assert_eq!(client.api_key, "test_key");
        assert_eq!(client.base_url, "https://api.groq.com/openai/v1");
        assert_eq!(client.model, "llama3-70b-8192");
    }

    #[test]
    fn test_build_system_prompt() {
        let prompt = KiloCodeClient::build_system_prompt("explain");
        assert!(prompt.contains("Explain the provided code"));
        
        let prompt = KiloCodeClient::build_system_prompt("generate");
        assert!(prompt.contains("Generate code"));
    }

    #[test]
    fn test_build_messages() {
        let client = KiloCodeClient::new("test_key".to_string());
        let messages = client.build_messages("System prompt", "User message", Some("code context"));
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
        assert!(messages[1].content.contains("code context"));
    }

    #[test]
    fn test_chat_message_serialization() {
        let message = ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };
        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("Hello"));
    }
}
