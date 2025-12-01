use clap::{Parser, Subcommand};
use std::io::{self, Read};

mod fetch;
mod config;

use ollama_rs::get_local_models;

fn print_banner() {
    println!("ollama-rs v0.1.0");
}

#[derive(Parser)]
#[command(author = "samsit mad", version = "0.1.0", about = "Ollama-rs: A Rust client for Ollama LLM server")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Model to use (default: llama3.2)
    #[arg(short, long, default_value = "llama3.2", global = true)]
    model: String,

    /// Prompt text. If omitted, reads from stdin.
    #[arg(short, long, global = true)]
    prompt: Option<String>,

    /// Override host (e.g. 127.0.0.1)
    #[arg(long, global = true)]
    host: Option<String>,

    /// Override port (default: 11434)
    #[arg(long, global = true)]
    port: Option<u16>,

    /// Show version and configuration info
    #[arg(long, global = true)]
    info: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate text from a prompt
    Generate {
        /// Prompt text
        #[arg(value_name = "PROMPT")]
        prompt: Option<String>,
    },
    /// List available local models
    Models,
}

fn read_host_port_from_config() -> Option<(String, u16)> {
    config::read_host_port_from_config().and_then(|hp| {
        let parts: Vec<&str> = hp.split(':').collect();
        if parts.len() == 2 {
            parts[1].parse::<u16>().ok().map(|port| (parts[0].to_string(), port))
        } else {
            None
        }
    })
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Ensure config exists with defaults
    config::ensure_config_with_defaults()?;

    let (cfg_host, cfg_port) = read_host_port_from_config().unwrap_or(("127.0.0.1".to_string(), 11434));
    let host = args.host.unwrap_or(cfg_host);
    let port = args.port.unwrap_or(cfg_port);

    // Handle --info flag
    if args.info {
        print_banner();
        println!("Author: samsit mad");
        println!("Version: 0.1.0");
        println!("\nConfiguration:");
        println!("   Host: {}", host);
        println!("   Port: {}", port);
        println!("   Default Model: {}", args.model);
        println!("\nUsage:");
        println!("   ollama-rs generate \"Your prompt here\"");
        println!("   ollama-rs models");
        println!("   ollama-rs --info");
        return Ok(());
    }

    match args.command {
        Some(Commands::Models) => {
            println!("Fetching available models from {}:{}...", host, port);
            match get_local_models(&host, port) {
                Ok(models) => {
                    if models.is_empty() {
                        println!("No models found. Make sure Ollama is running.");
                    } else {
                        println!("Available Models:");
                        for (i, model) in models.iter().enumerate() {
                            println!("   {}. {}", i + 1, model);
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error fetching models: {}", e);
                    eprintln!("   Is Ollama running on {}:{}?", host, port);
                    Err(e)
                }
            }
        }
        Some(Commands::Generate { prompt: cmd_prompt }) => {
            let prompt = cmd_prompt
                .or(args.prompt)
                .unwrap_or_else(|| {
                    println!("Enter your prompt (Ctrl+D to send):");
                    let mut input = String::new();
                    io::stdin()
                        .read_to_string(&mut input)
                        .unwrap_or_default();
                    input
                });

            if prompt.trim().is_empty() {
                eprintln!("Empty prompt. Provide a prompt to generate.");
                return Ok(());
            }

            println!("Generating response from {} using {}...", host, args.model);
            let host_port = format!("{}:{}", host, port);
            let resp = fetch::send_generate(&host_port, &args.model, &prompt)?;
            println!("Response:\n");
            println!("{}", resp);
            Ok(())
        }
        None => {
            let prompt = args.prompt
                .unwrap_or_else(|| {
                    println!("Enter your prompt (Ctrl+D to send):");
                    let mut input = String::new();
                    io::stdin()
                        .read_to_string(&mut input)
                        .unwrap_or_default();
                    input
                });

            if prompt.trim().is_empty() {
                eprintln!("Empty prompt. Provide a prompt to generate.");
                return Ok(());
            }

            println!("Generating response from {} using {}...", host, args.model);
            let host_port = format!("{}:{}", host, port);
            let resp = fetch::send_generate(&host_port, &args.model, &prompt)?;
            println!("Response:\n");
            println!("{}", resp);
            Ok(())
        }
    }
}
