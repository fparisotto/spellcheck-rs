mod config;
mod ollama_client;
mod spell_checker;

use anyhow::Result;
use std::io::{self, IsTerminal, Write};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

use crate::config::Config;
use crate::spell_checker::SpellChecker;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    let spell_checker = SpellChecker::new(config.clone())?;

    let stdin = std::io::stdin();

    if stdin.is_terminal() {
        run_interactive_mode(&config, &spell_checker).await
    } else {
        run_piped_mode(&spell_checker).await
    }
}

async fn run_interactive_mode(config: &Config, spell_checker: &SpellChecker) -> Result<()> {
    println!("✏️  Smart Spell Checker");
    println!("Model: {}", config.model);
    println!("Host: {}", config.ollama_host);
    println!("Type your text to check for spelling and grammar errors.");
    println!("Press Ctrl+C");
    println!("----------------------------------------\n");

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);

    loop {
        println!("Enter text to check (or command):");
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        reader.read_line(&mut input).await?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        println!("\n🔍 Checking text...");
        check_text(spell_checker, input).await;
    }
}

async fn run_piped_mode(spell_checker: &SpellChecker) -> Result<()> {
    let mut stdin = tokio::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).await?;

    let input = input.trim();
    if !input.is_empty() {
        match spell_checker.check_text(input).await {
            Ok(result) => {
                if result.original_text == result.corrected_text {
                    print!("{}", result.original_text);
                } else {
                    print!("{}", result.corrected_text);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

async fn check_text(spell_checker: &SpellChecker, text: &str) {
    match spell_checker.check_text(text).await {
        Ok(result) => {
            println!("\n{}", spell_checker.format_result(&result));
        }
        Err(e) => {
            eprintln!("\n❌ Error checking text: {}", e);
        }
    }
}
