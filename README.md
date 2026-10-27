# spell-checkers

A smart spell checker powered by Ollama that identifies and corrects spelling, grammar, and style errors in your text.

## Features

- **Spell Checking**: Detects and corrects spelling mistakes
- **Grammar Correction**: Identifies and fixes grammatical errors
- **Word Diff**: Removed words in red strikethrough, added words in green
- **Fast**: Thinking is off, the model is preloaded at startup and stays in VRAM
- **Offline**: Everything runs on a local Ollama instance
- **Multi-line Input**: Paste several lines; they are checked as one text
- **Code and URL Protection**: `inline code`, fenced code blocks, and URLs are never changed

## Prerequisites

- Rust 2024 Edition or later
- Ollama running locally (default: http://localhost:11434)
- An Ollama model installed (default: gemma4:e4b, install with `ollama pull gemma4:e4b`)

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd spell-checkers

# Build the project
cargo build --release

# Run the spell checker
cargo run
```

## Usage

### Interactive Mode

Run the application and type or paste your text. A typed line is checked when you press Enter. A pasted block of lines is checked as one text:

```bash
cargo run
```

The application will display:
- Model and host information
- Prompt to enter text
- After checking, it will show either "✓ No errors found!" or a word diff of the corrections

Press `Ctrl+D` or `Ctrl+C` to exit the application.

### Pipe Redirection

The spell checker can be used with pipe redirects for powerful workflows:

```bash
# Check text from a file
cat document.txt | spellcheck-rs

# Check and save corrected output
echo "Their are grammer mistakes here." | spellcheck-rs > corrected.txt

# Wayland clipboard integration (requires wl-clipboard)
# Copy text from clipboard, check spelling, and copy back corrected text
wl-paste | spellcheck-rs | wl-copy

# X11 clipboard integration (requires xclip)
xclip -o | spellcheck-rs | xclip -selection clipboard

# Check multiple files
cat *.txt | spellcheck-rs
```

### Example

```bash
✏️  Smart Spell Checker
Model: gemma4:e4b
Host: http://localhost:11434
Type your text to check for spelling and grammar errors.
Press Ctrl+D or Ctrl+C to exit
----------------------------------------

> Their are grammer mistakes here.
~~Their~~There are ~~grammer~~grammar mistakes here.
(0.3s)
```

For text without errors:
```bash
> This sentence is correct.
✓ No errors found!
(0.2s)
```

## Configuration

If `~/.config/spell-checkers/config.json` does not exist, the application uses the defaults and writes them to that file. Every field is optional:

```json
{
  "ollama_host": "http://localhost:11434",
  "model": "gemma4:e4b",
  "keep_alive": "1h"
}
```

- `ollama_host`: Ollama server URL
- `model`: Ollama model to use. Fast choices on a 16 GB GPU: `gemma4:e4b` (default), `gemma4:e2b` (fastest), `gemma4:12b` (largest)
- `keep_alive`: how long the model stays loaded after a check (default `"1h"`), for example `"30m"`, `"2h"`, or `-1` for always
- `system_prompt`: replaces the built-in prompt. Leave it out to get prompt improvements in new versions
