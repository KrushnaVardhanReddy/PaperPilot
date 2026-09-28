mod cli;
mod commands;

use clap::Parser;
use cli::Cli;
use log::{error, info};
use reqwest::blocking::Client;
use serde::Serialize;
use std::process;

#[derive(Serialize)]
struct CommandResult {
    success: bool,
    operation: String,
    error: Option<String>,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    let result = commands::execute_command(&cli.command);

    let op_name = format!("{:?}", cli.command)
        .split_whitespace()
        .next()
        .unwrap_or("Unknown")
        .to_string();

    let payload = match result {
        Ok(_) => CommandResult {
            success: true,
            operation: op_name,
            error: None,
        },
        Err(e) => CommandResult {
            success: false,
            operation: op_name,
            error: Some(e.to_string()),
        },
    };

    if cli.json {
        let json_output = serde_json::to_string(&payload).unwrap();
        println!("{}", json_output);
    } else {
        match &payload.error {
            Some(err) => error!("Operation failed: {}", err),
            None => info!("Operation completed successfully."),
        }
    }

    if let Some(webhook_url) = cli.webhook {
        let client = Client::new();
        match client.post(&webhook_url).json(&payload).send() {
            Ok(res) if res.status().is_success() => {
                info!("Successfully posted to webhook: {}", webhook_url);
            }
            Ok(res) => {
                error!(
                    "Webhook returned error status: {} - {}",
                    res.status(),
                    webhook_url
                );
            }
            Err(e) => {
                error!("Failed to post to webhook: {} - {}", e, webhook_url);
            }
        }
    }

    if !payload.success {
        process::exit(1);
    }
}
