mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use serde::Serialize;
use std::process;

#[derive(Serialize)]
struct OperationOutput {
    success: bool,
    operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Validate { path } => commands::validate::execute_validate(path),
    };

    let op_name = match &cli.command {
        Commands::Validate { .. } => "validate",
    };

    let (success, error_msg) = match result {
        Ok(_) => (true, None),
        Err(e) => (false, Some(e.to_string())),
    };

    let output = OperationOutput {
        success,
        operation: op_name.to_string(),
        error: error_msg,
    };

    if cli.json || cli.webhook.is_some() {
        let json_payload = serde_json::to_string(&output).unwrap_or_else(|_| {
            "{\"success\":false,\"operation\":\"unknown\",\"error\":\"Failed to serialize output\"}".to_string()
        });

        if cli.json {
            println!("{}", json_payload);
        }

        if let Some(webhook_url) = cli.webhook {
            let client = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default();

            let res = client
                .post(&webhook_url)
                .header("Content-Type", "application/json")
                .body(json_payload)
                .send();

            #[allow(clippy::collapsible_if)]
            if let Err(e) = res {
                if !cli.json {
                    eprintln!("Warning: Webhook notification failed: {}", e);
                }
            }
        }
    } else if success {
        println!("Operation '{}' completed successfully.", op_name);
    } else {
        eprintln!(
            "Operation '{}' failed: {}",
            op_name,
            output.error.as_deref().unwrap_or("Unknown error")
        );
    }

    if !success {
        process::exit(1);
    }
}
