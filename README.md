# spell-checkers

A smart spell checker powered by Ollama that identifies and corrects spelling, grammar, and style errors in your text.

## Features

- **Spell Checking**: Detects and corrects spelling mistakes
- **Grammar Correction**: Identifies and fixes grammatical errors
- **Style Improvements**: Suggests better sentence structure and clarity
- **Simple Output**: Clean, concise correction format
- **Colored Output**: Visual highlighting of errors and corrections
- **Interactive Mode**: Real-time spell checking as you type

## Prerequisites

- Rust (latest stable version)
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

Simply run the application and type your text:

```bash
cargo run
> Enter text to check (or command): This is a test sentense with mispelled words.
```

Press `Ctrl+C` to exit the application.

### Example

```bash
> Enter text to check (or command): Their are grammer mistakes here.

=== Spell Check Results ===

Original: Their are grammer mistakes here.
Corrected: There are grammar mistakes here.

Corrections:
1. [Grammar] "Their" → "There"
   Incorrect use of possessive pronoun

2. [Spelling] "grammer" → "grammar"
   Misspelled word
```

## Configuration

The application creates a configuration file at `~/.config/spell-checkers/config.json` with the following options:

- `ollama_host`: Ollama server URL (default: "http://localhost:11434")
- `model`: Ollama model to use (default: "gemma3:4b")
- `system_prompt`: Custom prompt for the AI model
