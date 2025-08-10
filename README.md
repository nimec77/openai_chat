# DeepSeek Chat Console Application

A console-based chat application written in Rust that allows you to interact with DeepSeek's AI models. This application provides a clean, colorful terminal interface for conversing with DeepSeek's language models.

## Features

- 🤖 **Interactive Chat**: Real-time conversation with DeepSeek AI models
- 🎨 **Colorful Interface**: Beautiful terminal UI with colored output
- 📝 **Conversation History**: Maintains context throughout the session
- ⚡ **Async Architecture**: Built with Tokio for efficient async operations
- 🛡️ **Error Handling**: Comprehensive error handling with helpful messages
- 🔧 **Configurable**: Easy configuration via environment variables
- 💬 **Special Commands**: Built-in commands for enhanced user experience

## Prerequisites

- Rust 1.70+ installed
- DeepSeek API key (get one from [DeepSeek Platform](https://platform.deepseek.com/api_keys))

## Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd openai_chat
   ```

2. **Create a `.env` file** in the project root and add your DeepSeek API key:
   ```env
   DEEPSEEK_API_KEY=your_deepseek_api_key_here
   ```

3. **Build and run**:
   ```bash
   cargo run
   ```

## Configuration

The application can be configured using environment variables. Create a `.env` file in the project root:

```env
# Required: Your DeepSeek API key
DEEPSEEK_API_KEY=your_api_key_here

# Optional: API base URL (default: https://api.deepseek.com)
DEEPSEEK_API_BASE=https://api.deepseek.com

# Optional: Model to use (default: deepseek-chat)
DEEPSEEK_MODEL=deepseek-chat

# Optional: Maximum tokens per response (default: 4096)
MAX_TOKENS=4096

# Optional: Temperature for response randomness (default: 0.7)
TEMPERATURE=0.7

# Optional: Request timeout in seconds (default: 300)
TIMEOUT=300
```

## Usage

### Starting the Application

```bash
cargo run
```

### Available Commands

Once the application is running, you can use these special commands:

- `/help` - Show help information
- `/clear` - Clear conversation history
- `/history` - Show conversation history
- `/exit` or `/quit` - Exit the application

### Example Session

```
🤖 DeepSeek Chat Console
============================================================

Welcome to DeepSeek Chat!
Type your message and press Enter to chat.
Special commands:
  /help - Show this help
  /clear - Clear conversation history
  /history - Show conversation history
  /exit, /quit, or Ctrl+C - Exit the application

────────────────────────────────────────────────────────────

💬 Enter your message: Hello, can you explain what Rust is?

👤 You: Hello, can you explain what Rust is?
🤖 DeepSeek: Rust is a systems programming language that focuses on safety, speed, and concurrency...

💬 Enter your message: /exit
👋 Thank you for using DeepSeek Chat!
Goodbye! 🚀
```

## Project Structure

```
src/
├── main.rs         # Main application entry point
├── config.rs       # Configuration management
├── deepseek.rs     # DeepSeek API client
└── console.rs      # Console interface and user interaction
```

### Key Components

- **`config.rs`**: Handles loading and validation of configuration from environment variables
- **`deepseek.rs`**: Implements the DeepSeek API client with proper error handling
- **`console.rs`**: Provides a rich console interface with colored output and command handling
- **`main.rs`**: Orchestrates the application flow and conversation loop

## Dependencies

The project uses several key Rust crates:

- **`tokio`**: Async runtime for handling asynchronous operations
- **`reqwest`**: HTTP client for API requests
- **`serde`**: Serialization/deserialization for JSON handling
- **`anyhow`**: Error handling with context
- **`thiserror`**: Derive macros for error types
- **`dotenv`**: Environment variable management
- **`colored`**: Colored terminal output
- **`crossterm`**: Cross-platform terminal manipulation

## Error Handling

The application includes comprehensive error handling:

- **API Errors**: Detailed error messages for API failures
- **Network Issues**: Helpful suggestions for connection problems
- **Configuration Errors**: Clear guidance for setup issues
- **Rate Limiting**: Informative messages about API limits

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Troubleshooting

### Common Issues

1. **"DEEPSEEK_API_KEY environment variable is required"**
   - Make sure you've created a `.env` file with your API key
   - Verify the API key is correct and active

2. **Network connection errors**
   - Check your internet connection
   - Verify the API base URL is correct
   - Check if your firewall is blocking the connection

3. **Rate limit errors**
   - Wait a moment before making more requests
   - Consider upgrading your DeepSeek API plan

4. **Build errors**
   - Ensure you have Rust 1.70+ installed
   - Run `cargo update` to update dependencies

## Support

For issues and questions:
- Check the troubleshooting section above
- Review the [DeepSeek API documentation](https://platform.deepseek.com/docs)
- Open an issue in this repository

---

**Happy chatting with DeepSeek! 🚀**
