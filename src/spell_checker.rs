use anyhow::Result;
use colored::*;
use futures_util::StreamExt;
use ollama_rs::generation::chat::{ChatMessage, request::ChatMessageRequest};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::ollama_client::OllamaClient;

pub struct SpellChecker {
    client: OllamaClient,
    config: Config,
}

impl SpellChecker {
    pub fn new(config: Config) -> Result<Self> {
        let client = OllamaClient::new(&config.ollama_host)?;
        Ok(Self { client, config })
    }

    pub async fn check_text(&self, text: &str) -> Result<SpellCheckResult> {
        // Build messages with proper roles
        let mut messages = vec![];
        
        // Add system prompt if configured
        if let Some(system_prompt) = &self.config.system_prompt {
            if !system_prompt.is_empty() {
                messages.push(ChatMessage::system(system_prompt.to_string()));
            }
        }
        
        // Add user message with text to check
        messages.push(ChatMessage::user(format!("Text to check: {}", text)));

        let history = Arc::new(Mutex::new(messages.clone()));
        let request = ChatMessageRequest::new(self.config.model.clone(), messages);

        let mut stream = self
            .client
            .send_chat_messages_with_history_stream(history, request)
            .await?;

        let mut response_content = String::new();
        while let Some(res) = stream.next().await {
            if let Ok(chunk) = res {
                response_content.push_str(&chunk.message.content);
            }
        }

        self.parse_response(&response_content, text)
    }

    fn parse_response(&self, response: &str, original_text: &str) -> Result<SpellCheckResult> {
        let response = response.trim();

        let corrected_text = if response == "No errors found." {
            original_text.to_string()
        } else {
            response.to_string()
        };

        Ok(SpellCheckResult {
            original_text: original_text.to_string(),
            corrected_text,
        })
    }

    pub fn format_result(&self, result: &SpellCheckResult) -> String {
        self.format_simple(result)
    }

    fn format_simple(&self, result: &SpellCheckResult) -> String {
        if result.original_text == result.corrected_text {
            format!("{}", "✓ No errors found!".green())
        } else {
            let mut output = format!("{}\n", "Corrected text:".cyan().bold());
            output.push_str(&result.corrected_text.green().to_string());
            output
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellCheckResult {
    pub original_text: String,
    pub corrected_text: String,
}
