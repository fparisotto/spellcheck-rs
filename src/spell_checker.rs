use anyhow::{Context, Result};
use colored::Colorize;
use ollama_rs::Ollama;
use ollama_rs::generation::chat::{ChatMessage, request::ChatMessageRequest};
use ollama_rs::generation::parameters::ThinkType;
use ollama_rs::models::ModelOptions;
use similar::{ChangeTag, TextDiff};

use crate::config::Config;
use crate::mask::Masked;

pub struct SpellChecker {
    client: Ollama,
    config: Config,
}

impl SpellChecker {
    pub fn new(config: Config) -> Result<Self> {
        let url = config
            .ollama_host
            .parse()
            .context("Invalid Ollama host URL")?;
        Ok(Self {
            client: Ollama::from_url(url),
            config,
        })
    }

    /// Loads the model into VRAM so that the first check does not pay the load time.
    pub async fn warm_up(&self) -> Result<()> {
        self.client
            .send_chat_messages(self.request(vec![]))
            .await
            .with_context(|| format!("Cannot load model '{}'", self.config.model))?;
        Ok(())
    }

    /// Returns the corrected text, or the input unchanged when it has no errors.
    pub async fn check_text(&self, text: &str) -> Result<String> {
        let masked = Masked::new(text);
        let corrected = self.correct(&masked.text).await?;
        match masked.unmask(&corrected) {
            Some(corrected) => Ok(corrected),
            None => self.correct(text).await,
        }
    }

    async fn correct(&self, text: &str) -> Result<String> {
        let messages = vec![
            ChatMessage::system(self.config.system_prompt().to_string()),
            ChatMessage::user(format!("<text>\n{text}\n</text>")),
        ];
        let response = self
            .client
            .send_chat_messages(self.request(messages))
            .await?;
        let corrected = response.message.content.trim();
        let corrected = corrected
            .strip_prefix("<text>")
            .and_then(|s| s.strip_suffix("</text>"))
            .map_or(corrected, str::trim);

        // Custom prompts often ask for this sentinel instead of the unchanged text.
        if corrected
            .trim_matches(['"', '.'])
            .eq_ignore_ascii_case("no errors found")
        {
            return Ok(text.to_string());
        }
        Ok(corrected.to_string())
    }

    fn request(&self, messages: Vec<ChatMessage>) -> ChatMessageRequest {
        ChatMessageRequest::new(self.config.model.clone(), messages)
            .think(ThinkType::False)
            .keep_alive(self.config.keep_alive.clone())
            .options(ModelOptions::default().temperature(0.0))
    }
}

/// Word-level diff: removed words in red strikethrough, added words in green.
pub fn format_diff(original: &str, corrected: &str) -> String {
    if original == corrected {
        return "✓ No errors found!".green().to_string();
    }
    TextDiff::from_words(original, corrected)
        .iter_all_changes()
        .map(|change| match change.tag() {
            ChangeTag::Equal => change.value().to_string(),
            ChangeTag::Delete => change.value().red().strikethrough().to_string(),
            ChangeTag::Insert => change.value().green().bold().to_string(),
        })
        .collect()
}
