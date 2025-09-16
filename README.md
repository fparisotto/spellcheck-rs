# spell-checkers

A smart spell checker powered by Ollama that identifies and corrects spelling, grammar, and style errors in your text.

## Features

- **Spell Checking**: Detects and corrects spelling mistakes
- **Grammar Correction**: Identifies and fixes grammatical errors
- **Simple Output**: Clean, concise correction format
- **Colored Output**: Visual highlighting with green for corrected text
- **Streaming Response**: Real-time response from Ollama API

## Prerequisites

- Rust 2024 Edition or later
- Ollama running locally (default: http://localhost:11434)
- An Ollama model installed (default: gemma3:4b)

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

Simply run the application and type your text:

```bash
cargo run
```

The application will display:
- Model and host information
- Prompt to enter text
- After checking, it will show either "✓ No errors found!" or the corrected text

Press `Ctrl+C` to exit the application.

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
Model: gemma3:4b
Host: http://localhost:11434
Type your text to check for spelling and grammar errors.
Press Ctrl+C
----------------------------------------

Enter text to check (or command):
> Their are grammer mistakes here.

🔍 Checking text...

Corrected text:
There are grammar mistakes here.
```

For text without errors:
```bash
Enter text to check (or command):
> This sentence is correct.

🔍 Checking text...

✓ No errors found!
```

## Configuration

The application creates a configuration file at `~/.config/spell-checkers/config.json` with the following options:

- `ollama_host`: Ollama server URL (default: "http://localhost:11434")
- `model`: Ollama model to use (default: "gemma3:4b")
- `system_prompt`: Custom prompt for the AI model
