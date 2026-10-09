use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::AiError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LlmConfig {
    pub endpoint_url: String, // e.g. "http://localhost:11434/v1" or "http://127.0.0.1:8080/v1"
    pub model: String,        // e.g. "llama-3.2-3b-instruct" or "qwen2.5-coder:7b"
    pub api_key: Option<String>,
    pub temperature: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ToolDefinition {
    pub r#type: String, // typically "function"
    pub function: FunctionDefinition,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON schema for parameters
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: AssistantMessage,
    pub finish_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssistantMessage {
    pub role: String,
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String, // JSON string
}

#[derive(Clone)]
pub struct UniversalLlmClient {
    config: LlmConfig,
    http: Client,
}

impl UniversalLlmClient {
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            http: Client::new(),
        }
    }

    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        tools: Option<Vec<ToolDefinition>>,
    ) -> Result<ChatCompletionResponse, AiError> {
        let url = format!("{}/chat/completions", self.config.endpoint_url.trim_end_matches('/'));

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(api_key) = &self.config.api_key
            && !api_key.trim().is_empty()
            && let Ok(auth_val) = HeaderValue::from_str(&format!("Bearer {}", api_key.trim()))
        {
            headers.insert(AUTHORIZATION, auth_val);
        }

        let tool_choice = if tools.as_ref().map(|t| !t.is_empty()).unwrap_or(false) {
            Some("auto".to_string())
        } else {
            None
        };

        let request_body = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages,
            temperature: self.config.temperature,
            tools,
            tool_choice,
        };

        let response = self
            .http
            .post(&url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AiError::Api(format!("HTTP {}: {}", status, text)));
        }

        let resp_json: ChatCompletionResponse = response.json().await?;
        Ok(resp_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_initialization() {
        let config = LlmConfig {
            endpoint_url: "http://localhost:11434/v1".to_string(),
            model: "llama-3.2-3b-instruct".to_string(),
            api_key: None,
            temperature: Some(0.1),
        };
        let client = UniversalLlmClient::new(config);
        assert_eq!(client.config.endpoint_url, "http://localhost:11434/v1");
        assert_eq!(client.config.model, "llama-3.2-3b-instruct");
    }
}
