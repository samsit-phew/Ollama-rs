# 🦙 Ollama-rs - Rust CLI Client

A lightweight, fast Ollama CLI client written in Rust with zero unnecessary dependencies. Interact with local LLMs directly from your terminal.

**Version:** 0.1.0  
**Author:** samsit mad  
**License:** MIT

---

## Features

✨ **Fast & Lightweight** - Minimal dependencies, quick startup  
🎯 **Simple Config** - Auto-creates `~/.config/ollama-rs/var.json` with sensible defaults  
📋 **List Models** - View all available local models with one command  
💬 **Generate Text** - Send prompts and get formatted responses  
🎨 **Beautiful CLI** - ASCII art, emoji feedback, and colored output  
⚙️ **Flexible** - Override host, port, and model via CLI flags  

---

## Installation

### Build from Source

```bash
git clone https://github.com/yourusername/ollama-rs.git
cd ollama-rs
cargo build --release
```

The binary will be available at `./target/release/ollama-rs`

### Optional: Install Globally

```bash
cargo install --path .
```

---

## Quick Start

### 1. Ensure Ollama is Running

Make sure Ollama server is running locally (default: `127.0.0.1:11434`):

```bash
ollama serve
```

### 2. Generate Text

```bash
ollama-rs generate "Tell me a joke about penguins"
```

Or use the shorthand (defaults to `generate` subcommand):

```bash
ollama-rs "Write a haiku about Rust"
```

### 3. List Available Models

```bash
ollama-rs models
```

### 4. Check Configuration

```bash
ollama-rs info
```

---

## Configuration

Ollama-rs automatically creates a configuration directory and file on first run:

**File Location:** `~/.config/ollama-rs/var.json`

**Default Contents:**

```json
{
  "ollama-host": "127.0.0.1",
  "ollama-port": 11434
}
```

### Customize Configuration

Edit `~/.config/ollama-rs/var.json` to change the default host and port:

```json
{
  "ollama-host": "192.168.1.100",
  "ollama-port": 11434
}
```

---

## Usage

### Subcommands

#### `generate` - Generate text from a prompt (default)

```bash
# Explicit subcommand
ollama-rs generate "What is Rust?"

# Implicit (no subcommand defaults to generate)
ollama-rs "Explain quantum computing"

# Read from stdin
cat prompt.txt | ollama-rs
```

#### `models` - List available models

```bash
ollama-rs models
```

Output:
```
🔍 Fetching available models from 127.0.0.1:11434...

✅ Available Models:
   1. llama3.2:latest
   2. mistral:latest
   3. neural-chat:latest
```

#### `info` - Show configuration and usage info

```bash
ollama-rs info
```

Output:
```
╔═══════════════════════════════════════════╗
║  🦙  Ollama CLI Client v0.1.0            ║
║      Fast Local LLM Interactions          ║
║      by samsit mad                        ║
╚═══════════════════════════════════════════╝

📋 Configuration:
   Host: 127.0.0.1
   Port: 11434
   Default Model: llama3.2

💡 Usage:
   ollama-rs generate "Your prompt here"
   ollama-rs models
   ollama-rs info
```

### Global Options

```bash
# Specify a custom model
ollama-rs --model mistral "What is AI?"

# Override host and port
ollama-rs --host 192.168.1.5 --port 11434 "Hello"

# Pass prompt via stdin
echo "Write code" | ollama-rs --model llama3.2

# Combined options
ollama-rs generate --model neural-chat --host localhost "Explain Rust ownership"
```

---

## Examples

### Example 1: Basic Prompt

```bash
$ ollama-rs "What is the capital of France?"
```

### Example 2: Change Model

```bash
$ ollama-rs --model mistral "Tell me about machine learning"
```

### Example 3: Remote Ollama Server

```bash
$ ollama-rs --host 192.168.1.100 --port 11434 "Hello from remote!"
```

### Example 4: Read Prompt from File

```bash
$ ollama-rs "$(cat my_prompt.txt)"
```

### Example 5: Pipe Prompt

```bash
$ echo "Explain quantum computing" | ollama-rs
```

### Example 6: List Models and Select

```bash
$ ollama-rs models
$ ollama-rs --model llama3.2 "Your prompt"
```

---

## Project Structure

```
ollama-rs/
├── Cargo.toml           # Project manifest & dependencies
├── src/
│   ├── main.rs          # CLI entry point with subcommands
│   ├── lib.rs           # Public library functions (get_local_models)
│   ├── fetch.rs         # HTTP request/response handling
│   └── config.rs        # Config file management
├── target/              # Build artifacts
└── README.md           # This file
```

### Key Modules

- **`main.rs`** - CLI interface with subcommands and ASCII art
- **`lib.rs`** - `get_local_models(host, port)` function for querying Ollama
- **`fetch.rs`** - Raw HTTP POST to `/api/generate`, response formatting
- **`config.rs`** - Config directory/file creation and reading

---

## Architecture

### How It Works

1. **Config Initialization** - On startup, ensures `~/.config/ollama-rs/var.json` exists
2. **Host/Port Resolution** - Reads config or accepts CLI overrides
3. **Request Building** - Constructs HTTP POST to Ollama's `/api/generate`
4. **Response Formatting** - Extracts and pretty-prints JSON responses
5. **Output Display** - Shows model response with emoji feedback

### Network Flow

```
User Input
    ↓
Config Resolution (file or CLI)
    ↓
HTTP POST /api/generate
    ↓
Ollama Server (127.0.0.1:11434)
    ↓
Stream Response
    ↓
Format & Display
```

---

## Dependencies

- **clap** (v4) - CLI argument parsing with subcommands
- **serde_json** (v1) - JSON handling
- **dirs-next** (v2) - XDG config directory support

**Dev Dependencies:**
- **tempfile** (v3) - Temporary directories for tests

---

## Testing

Run the test suite:

```bash
cargo test
```

Run specific test:

```bash
cargo test test_get_local_models
```

Run with output:

```bash
cargo test -- --nocapture
```

---

## Troubleshooting

### "Connection refused"

**Problem:** `Is Ollama running on 127.0.0.1:11434?`

**Solution:** Start Ollama server:
```bash
ollama serve
```

### "No models found"

**Problem:** Models list is empty

**Solution:** 
1. Ensure Ollama is running: `ollama serve`
2. Pull a model: `ollama pull llama3.2`
3. List models: `ollama list`

### Config not being read

**Problem:** `~/.config/ollama-rs/var.json` not created

**Solution:** Run any command once to auto-generate:
```bash
ollama-rs info
```

### Wrong host/port

**Problem:** Connecting to wrong server

**Solution:** Check config or override via CLI:
```bash
ollama-rs --host 192.168.1.100 --port 11434 models
```

---

## Development

### Build

```bash
cargo build
```

### Build Release (Optimized)

```bash
cargo build --release
```

### Run Directly

```bash
cargo run -- generate "Your prompt"
cargo run -- models
cargo run -- info
```

### Format Code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

---

## Contributing

Contributions welcome! Feel free to:

- Report bugs via GitHub Issues
- Submit pull requests with improvements
- Suggest new features

---

## Performance Notes

- **First run:** Auto-creates config (minimal overhead)
- **Model listing:** Direct HTTP query to Ollama (fast)
- **Generation:** Depends on model size and server hardware
- **Binary size:** ~3-5MB (release build)

---

## Future Improvements

- [ ] Streaming responses with real-time output
- [ ] Support for embeddings and chat modes
- [ ] Interactive REPL mode
- [ ] Model management (pull, remove, update)
- [ ] Persistent conversation history
- [ ] Configuration profiles for different servers
- [ ] Output formatting options (JSON, CSV, markdown)
- [ ] Async/parallel request handling

---

## License

MIT License - See LICENSE file for details

---

## Support

For issues, questions, or suggestions:

- **GitHub Issues:** [Create an issue](https://github.com/yourusername/ollama-rs/issues)
- **Author:** samsit mad

Enjoy using Ollama-rs! 🦙✨
