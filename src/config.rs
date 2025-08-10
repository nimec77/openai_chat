use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

/// Configuration for the DeepSeek API client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// DeepSeek API key
    pub api_key: String,
    /// API base URL
    pub api_base: String,
    /// Model to use for chat completions
    pub model: String,
    /// Maximum tokens per response
    pub max_tokens: u32,
    /// Temperature for response randomness (0.0-2.0)
    pub temperature: f32,
    /// Timeout for API requests
    pub timeout: u64,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok(); // Load .env file if it exists

        let api_key = env::var("DEEPSEEK_API_KEY")
            .context("DEEPSEEK_API_KEY environment variable is required")?;

        let api_base = env::var("DEEPSEEK_API_BASE")
            .unwrap_or_else(|_| "https://api.deepseek.com".to_string());

        let model = env::var("DEEPSEEK_MODEL")
            .unwrap_or_else(|_| "deepseek-chat".to_string());

        let max_tokens = env::var("MAX_TOKENS")
            .unwrap_or_else(|_| "4096".to_string())
            .parse()
            .context("MAX_TOKENS must be a valid number")?;

        let temperature = env::var("TEMPERATURE")
            .unwrap_or_else(|_| "0.7".to_string())
            .parse()
            .context("TEMPERATURE must be a valid number")?;

        let timeout = env::var("TIMEOUT")
            .unwrap_or_else(|_| "300".to_string())
            .parse()
            .context("TIMEOUT must be a valid number")?;

        Ok(Config {
            api_key,
            api_base,
            model,
            max_tokens,
            temperature,
            timeout,
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("API key cannot be empty");
        }

        if self.temperature < 0.0 || self.temperature > 2.0 {
            anyhow::bail!("Temperature must be between 0.0 and 2.0");
        }

        if self.max_tokens == 0 {
            anyhow::bail!("Max tokens must be greater than 0");
        }

        if self.timeout == 0 {
            anyhow::bail!("Timeout must be greater than 0");
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_base: "https://api.deepseek.com".to_string(),
            model: "deepseek-chat".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            timeout: 300,
        }
    }
}
