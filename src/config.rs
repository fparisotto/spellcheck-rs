use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_ollama_host")]
    pub ollama_host: String,

    #[serde(default = "default_model")]
    pub model: String,

    #[serde(default)]
    pub system_prompt: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

            let config: Config = serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {:?}", config_path))?;

            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let content = serde_json::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Failed to get config directory")?;
        Ok(config_dir.join("spell-checkers").join("config.json"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ollama_host: default_ollama_host(),
            model: default_model(),
            system_prompt: Some(default_system_prompt()),
        }
    }
}

fn default_ollama_host() -> String {
    "http://localhost:11434".to_string()
}

fn default_model() -> String {
    "gemma3:4b".to_string()
}

fn default_system_prompt() -> String {
    r#"You are a spell checker. Your ONLY job is to correct spelling and grammar errors.

RESPOND RULES:
- If NO errors found: say exactly "No errors found."
- If errors found: output ONLY the corrected text
- DO NOT explain, chat, or be conversational
- DO NOT say "here's the corrected version" or similar
- DO NOT add extra content or suggestions

Examples:
Input: "lets go too the stor"
Output: "let's go to the store"

Input: "This sentence is correct."
Output: "No errors found.""#
        .to_string()
}
