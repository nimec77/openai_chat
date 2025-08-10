use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};

/// Console interface for the chat application
pub struct Console {
    conversation_history: Vec<String>,
}

impl Console {
    /// Create a new console interface
    pub fn new() -> Self {
        Self {
            conversation_history: Vec::new(),
        }
    }

    /// Print the welcome message
    pub fn print_welcome(&self) {
        println!("{}", "=".repeat(60).bright_cyan());
        println!("{}", "🤖 DeepSeek Chat Console".bright_green().bold());
        println!("{}", "=".repeat(60).bright_cyan());
        println!();
        println!("{}", "Welcome to DeepSeek Chat!".bright_white());
        println!("{}", "Type your message and press Enter to chat.".white());
        println!("{}", "Special commands:".yellow());
        println!("  {} - Show this help", "/help".bright_yellow());
        println!("  {} - Clear conversation history", "/clear".bright_yellow());
        println!("  {} - Show conversation history", "/history".bright_yellow());
        println!("  {} - Exit the application", "/exit or Ctrl+C".bright_yellow());
        println!();
        println!("{}", "─".repeat(60).bright_black());
        println!();
    }

    /// Print a user message
    pub fn print_user_message(&mut self, message: &str) {
        let formatted = format!("👤 You: {}", message);
        println!("{}", formatted.bright_blue());
        self.conversation_history.push(formatted);
    }

    /// Print an assistant message
    pub fn print_assistant_message(&mut self, message: &str) {
        let formatted = format!("🤖 DeepSeek: {}", message);
        println!("{}", formatted.bright_green());
        self.conversation_history.push(formatted);
        println!();
    }

    /// Print an error message
    pub fn print_error(&self, error: &str) {
        println!("{} {}", "❌ Error:".bright_red(), error.red());
        println!();
    }

    /// Print an info message
    pub fn print_info(&self, info: &str) {
        println!("{} {}", "ℹ️  Info:".bright_cyan(), info.cyan());
        println!();
    }

    /// Print a warning message
    #[allow(dead_code)]
    pub fn print_warning(&self, warning: &str) {
        println!("{} {}", "⚠️  Warning:".bright_yellow(), warning.yellow());
        println!();
    }

    /// Get user input with a prompt
    pub async fn get_input(&self) -> Result<String> {
        print!("{} ", "💬 Enter your message:".bright_white());
        io::stdout().flush()?;

        let stdin = tokio::io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        if let Some(line) = lines.next_line().await? {
            Ok(line.trim().to_string())
        } else {
            Ok(String::new())
        }
    }

    /// Handle special commands
    pub fn handle_command(&mut self, input: &str) -> CommandResult {
        match input.trim().to_lowercase().as_str() {
            "/help" => {
                self.print_help();
                CommandResult::Handled
            }
            "/clear" => {
                self.clear_history();
                CommandResult::Handled
            }
            "/history" => {
                self.show_history();
                CommandResult::Handled
            }
            "/exit" | "/quit" => CommandResult::Exit,
            _ if input.starts_with('/') => {
                self.print_error(&format!("Unknown command: {}", input));
                self.print_info("Type /help to see available commands");
                CommandResult::Handled
            }
            _ => CommandResult::NotHandled,
        }
    }

    /// Print help information
    fn print_help(&self) {
        println!("{}", "📖 Available Commands:".bright_cyan().bold());
        println!();
        println!("  {} - Show this help message", "/help".bright_yellow());
        println!("  {} - Clear conversation history", "/clear".bright_yellow());
        println!("  {} - Show conversation history", "/history".bright_yellow());
        println!("  {} - Exit the application", "/exit".bright_yellow());
        println!();
        println!("{}", "💡 Tips:".bright_cyan().bold());
        println!("  • Press Ctrl+C to exit at any time");
        println!("  • Your conversation history is maintained during the session");
        println!("  • Use clear, specific questions for better responses");
        println!();
    }

    /// Clear conversation history
    fn clear_history(&mut self) {
        self.conversation_history.clear();
        self.print_info("Conversation history cleared!");
    }

    /// Show conversation history
    fn show_history(&self) {
        if self.conversation_history.is_empty() {
            self.print_info("No conversation history yet.");
            return;
        }

        println!("{}", "📜 Conversation History:".bright_cyan().bold());
        println!();
        for (i, entry) in self.conversation_history.iter().enumerate() {
            println!("{}. {}", (i + 1).to_string().bright_black(), entry);
        }
        println!();
    }

    /// Print a thinking/loading message
    pub fn print_thinking(&self) {
        print!("{}", "🤔 DeepSeek is thinking...".bright_yellow());
        io::stdout().flush().unwrap();
    }

    /// Clear the thinking message
    pub fn clear_thinking(&self) {
        print!("\r{}\r", " ".repeat(30));
        io::stdout().flush().unwrap();
    }

    /// Print goodbye message
    pub fn print_goodbye(&self) {
        println!();
        println!("{}", "👋 Thank you for using DeepSeek Chat!".bright_green());
        println!("{}", "Goodbye! 🚀".bright_cyan());
        println!();
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of command handling
#[derive(Debug, PartialEq)]
pub enum CommandResult {
    /// Command was handled
    Handled,
    /// Command was not recognized, should be processed as normal input
    NotHandled,
    /// User requested to exit
    Exit,
}
