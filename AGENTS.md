# AGENTS.md

This file provides guidance to coding agents when working with code in this repository.

## Project Overview

This is a Rust-based spell checker application that uses Ollama for AI-powered text correction. It identifies and corrects spelling, grammar, and style errors through an interactive CLI interface.

## Development Commands

### Building and Running
```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run

# Check code without building
cargo check
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Nix Development Environment
If using Nix with flakes:
```bash
# Enter development shell
nix develop

# Build the package
nix build
```

## Architecture Overview

### Core Module Structure

The application follows a modular architecture with clear separation of concerns:

1. **`main.rs`**: Entry point and main loop
   - Interactive mode (TTY): a stdin reader thread feeds lines into a channel; lines that arrive within `PASTE_GAP` (a paste) form one check; word diff output, or the plain corrected text with `--plain`; preloads the model at startup
   - Piped mode: reads all of stdin, prints only the corrected text
   - Exit with Ctrl+D or Ctrl+C

2. **`spell_checker.rs`**: Core spell checking logic
   - `SpellChecker` struct orchestrates the checking process
   - Sends the user text wrapped in `<text>` tags (prompt-injection guard)
   - Requests use `think: false`, `temperature: 0`, and the configured `keep_alive`
   - Returns the corrected text; also accepts a "No errors found." sentinel from custom prompts
   - `format_diff` renders a word diff with the `similar` crate

3. **`mask.rs`**: replaces code spans and URLs with `⟦n⟧` placeholders before the check and restores them after; if the model alters a placeholder, `check_text` checks the unmasked text

4. **`config.rs`**: Configuration management
   - Loads `~/.config/spell-checkers/config.json`; writes the defaults when the file is missing
   - Configurable: host, model, keep_alive, optional system prompt override
   - `DEFAULT_SYSTEM_PROMPT` is used when `system_prompt` is unset


### Data Flow

1. User input → Main loop → SpellChecker::check_text()
2. SpellChecker sends the system prompt and the tagged user text to Ollama (non-streaming)
3. The response is the corrected text; a "No errors found." sentinel maps to the original text
4. Interactive mode prints a word diff; piped mode prints the corrected text

### Key Design Decisions

- **Speed first**: thinking off, model kept in VRAM, preload on startup
- **No sentinel by default**: the model returns the text unchanged when it is correct, so the result is a plain string comparison
- **Minimal parsing**: No complex error classification, just corrected text

## Prerequisites

- Rust 2024 Edition or later
- Ollama must be running locally (default: http://localhost:11434)
- An Ollama model must be installed (default: gemma4:e4b)
- To install a model: `ollama pull gemma4:e4b`

## Configuration

The application auto-creates configuration at first run. Key settings:
- `ollama_host`: Ollama server URL
- `model`: Which Ollama model to use
- `keep_alive`: How long Ollama keeps the model loaded (default "1h"; "30m", "2h", -1)
- `system_prompt`: Optional override of the built-in prompt

## Common Development Tasks

### Modifying Spell Checking Behavior
1. Update `DEFAULT_SYSTEM_PROMPT` in `config.rs`
2. The prompt instructs the model to return only the corrected text, or the text unchanged

### Adding Features
The application is intentionally kept simple. Potential enhancements:
1. Split long input by paragraph (the model context limit can cut off long text)
2. Implement detailed error classification (would require updating prompt and parsing)
3. Add command handling (e.g., "file <filename>", "quit")
4. Enhanced output formatting with error explanations