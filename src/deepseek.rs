use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::config::Config;

/// Message role in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// A single message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }
}

/// Chat completion request payload
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    pub temperature: f32,
    pub stream: bool,
}

/// Choice in the response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: Option<String>,
}

/// Usage statistics in the response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Chat completion response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// DeepSeek API client
#[derive(Debug)]
pub struct DeepSeekClient {
    client: Client,
    config: Config,
}

impl DeepSeekClient {
    /// Create a new DeepSeek client with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .user_agent("openai_chat/0.1.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, config })
    }

    /// Send a chat completion request
    pub async fn chat_completion(&self, messages: Vec<Message>) -> Result<ChatResponse> {
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: false,
        };

        let url = format!("{}/v1/chat/completions", self.config.api_base);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to DeepSeek API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API request failed with status {}: {}", status, text);
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse response from DeepSeek API")?;

        Ok(chat_response)
    }

    /// Get a simple text response from a user message
    #[allow(dead_code)]
    pub async fn get_response(&self, user_message: &str) -> Result<String> {
        let messages = vec![Message::user(user_message)];
        let response = self.chat_completion(messages).await?;

        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            anyhow::bail!("No response choices received from API")
        }
    }

    /// Get a response with conversation history
    pub async fn get_response_with_history(&self, messages: Vec<Message>) -> Result<String> {
        let response = self.chat_completion(messages).await?;

        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            anyhow::bail!("No response choices received from API")
        }
    }
}
