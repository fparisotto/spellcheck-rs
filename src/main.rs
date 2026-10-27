mod config;
mod mask;
mod spell_checker;

use anyhow::Result;
use std::io::{self, BufRead, IsTerminal, Read, Write};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::timeout;

use crate::config::Config;
use crate::spell_checker::{SpellChecker, format_diff};

/// Lines that arrive within this gap belong to the same pasted text.
const PASTE_GAP: Duration = Duration::from_millis(50);

const USAGE: &str = "\
Usage: spellcheck-rs [OPTIONS]

Reads text from stdin when it is piped, otherwise starts an interactive prompt.

Options:
  -p, --plain    Interactive mode: print only the corrected text, ready to copy
  -h, --help     Print this help";

#[tokio::main]
async fn main() -> Result<()> {
    let mut plain = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-p" | "--plain" => plain = true,
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            _ => anyhow::bail!("unknown argument '{arg}'\n\n{USAGE}"),
        }
    }

    let config = Config::load()?;
    let spell_checker = SpellChecker::new(config.clone())?;

    if io::stdin().is_terminal() {
        run_interactive_mode(&config, Arc::new(spell_checker), plain).await
    } else {
        run_piped_mode(&spell_checker).await
    }
}

async fn run_interactive_mode(
    config: &Config,
    spell_checker: Arc<SpellChecker>,
    plain: bool,
) -> Result<()> {
    println!("✏️  Smart Spell Checker");
    println!("Model: {}", config.model);
    println!("Host: {}", config.ollama_host);
    println!("Type or paste text (multiple lines are fine) to check spelling and grammar.");
    println!("Press Ctrl+D or Ctrl+C to exit");
    println!("----------------------------------------\n");

    let warm_up = Arc::clone(&spell_checker);
    tokio::spawn(async move {
        if let Err(e) = warm_up.warm_up().await {
            eprintln!("\n❌ {e:#}\n   Is Ollama running? Is the model pulled?");
        }
    });

    let mut lines = spawn_line_reader();
    loop {
        print!("> ");
        io::stdout().flush()?;

        let Some(first) = lines.recv().await else {
            println!();
            return Ok(());
        };
        let mut input = first;
        while let Ok(Some(line)) = timeout(PASTE_GAP, lines.recv()).await {
            input.push('\n');
            input.push_str(&line);
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let started = Instant::now();
        match spell_checker.check_text(input).await {
            Ok(corrected) if plain && corrected == input => println!("✓ No errors found!\n"),
            Ok(corrected) if plain => println!("{corrected}\n"),
            Ok(corrected) => {
                println!("{}", format_diff(input, &corrected));
                println!("({:.1}s)\n", started.elapsed().as_secs_f32());
            }
            Err(e) => eprintln!("❌ Error checking text: {e:#}\n"),
        }
    }
}

/// Reads stdin on a plain thread, so that waiting for the next line can time out.
fn spawn_line_reader() -> mpsc::UnboundedReceiver<String> {
    let (tx, rx) = mpsc::unbounded_channel();
    std::thread::spawn(move || {
        for line in io::stdin().lock().lines() {
            let Ok(line) = line else { break };
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    rx
}

async fn run_piped_mode(spell_checker: &SpellChecker) -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let text = input.trim();
    if text.is_empty() {
        return Ok(());
    }

    let corrected = spell_checker.check_text(text).await?;
    print!("{corrected}");
    if input.ends_with('\n') {
        println!();
    }
    Ok(())
}
