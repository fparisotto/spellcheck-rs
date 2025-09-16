use anyhow::{Context, Result};
use ollama_rs::Ollama;
use ollama_rs::generation::chat::{
    ChatMessage, ChatMessageResponseStream, request::ChatMessageRequest,
};
use std::sync::{Arc, Mutex};

pub struct OllamaClient {
    client: Ollama,
}

impl OllamaClient {
    pub fn new(host: &str) -> Result<Self> {
        let url = host.parse().context("Invalid Ollama host URL")?;
        Ok(Self {
            client: Ollama::from_url(url),
        })
    }

    pub async fn send_chat_messages_with_history_stream(
        &self,
        history: Arc<Mutex<Vec<ChatMessage>>>,
        request: ChatMessageRequest,
    ) -> Result<ChatMessageResponseStream> {
        self.client
            .send_chat_messages_with_history_stream(history, request)
            .await
            .context("Failed to send chat messages")
    }
}
