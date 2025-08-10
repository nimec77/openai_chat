mod config;
mod console;
mod deepseek;

use anyhow::{Context, Result};
use console::{CommandResult, Console};
use deepseek::{DeepSeekClient, Message};
use std::process;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the application
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<()> {
    // Load configuration
    let config = config::Config::from_env().context("Failed to load configuration")?;

    // Initialize console interface
    let mut console = Console::new();
    console.print_welcome();

    // Initialize DeepSeek client
    let client = DeepSeekClient::new(config).context("Failed to initialize DeepSeek client")?;

    // Conversation history for maintaining context
    let mut conversation_history: Vec<Message> = Vec::new();

    // Add system message to set context
    conversation_history.push(Message::system(
        "You are DeepSeek, a helpful AI assistant. Provide clear, informative, and engaging responses. \
         Be concise but thorough in your explanations."
    ));

    // Set up graceful shutdown handler
    let shutdown_handler = async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        println!("\n\nReceived interrupt signal...");
    };

    // Main conversation loop
    tokio::select! {
        result = conversation_loop(&mut console, &client, &mut conversation_history) => {
            if let Err(e) = result {
                console.print_error(&format!("Conversation error: {}", e));
            }
        }
        _ = shutdown_handler => {
            console.print_info("Shutting down gracefully...");
        }
    }

    console.print_goodbye();
    Ok(())
}

async fn conversation_loop(
    console: &mut Console,
    client: &DeepSeekClient,
    conversation_history: &mut Vec<Message>,
) -> Result<()> {
    loop {
        // Get user input
        let input = console.get_input().await?;

        // Skip empty input
        if input.trim().is_empty() {
            continue;
        }

        // Handle special commands
        match console.handle_command(&input) {
            CommandResult::Exit => break,
            CommandResult::Handled => continue,
            CommandResult::NotHandled => {
                // Process as normal chat message
            }
        }

        // Display user message
        console.print_user_message(&input);

        // Add user message to conversation history
        conversation_history.push(Message::user(&input));

        // Show thinking indicator
        console.print_thinking();

        // Get response from DeepSeek
        match client.get_response_with_history(conversation_history.clone()).await {
            Ok(response) => {
                console.clear_thinking();
                console.print_assistant_message(&response);

                // Add assistant response to conversation history
                conversation_history.push(Message::assistant(&response));

                // Limit conversation history to prevent context overflow
                // Keep system message + last 20 exchanges (40 messages)
                if conversation_history.len() > 41 {
                    // Keep system message (index 0) and remove oldest user-assistant pairs
                    let system_msg = conversation_history[0].clone();
                    conversation_history.drain(1..conversation_history.len() - 20);
                    conversation_history[0] = system_msg;
                }
            }
            Err(e) => {
                console.clear_thinking();
                console.print_error(&format!("Failed to get response: {}", e));
                
                // Provide helpful suggestions based on error type
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("unauthorized") || error_str.contains("401") {
                    console.print_info("Please check your DEEPSEEK_API_KEY in the .env file");
                } else if error_str.contains("network") || error_str.contains("timeout") {
                    console.print_info("Please check your internet connection and try again");
                } else if error_str.contains("rate limit") || error_str.contains("429") {
                    console.print_info("Rate limit exceeded. Please wait a moment before trying again");
                }
            }
        }
    }

    Ok(())
}
