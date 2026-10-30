pub mod cli;
pub mod commands;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "paperpilot watch")]
pub struct WatchCli {
    /// Directory to monitor for incoming PDF files
    #[arg(short = 'd', long)]
    pub directory: PathBuf,

    /// Path to .ppflow TOML or JSON recipe to execute per file
    #[arg(short = 'r', long)]
    pub recipe: Option<PathBuf>,

    /// Output directory for successfully processed PDFs
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Optional output basename template using input, UTC date, hash and index tokens
    #[arg(long)]
    pub name_template: Option<String>,

    /// Directory to move successfully processed originals (optional)
    #[arg(long)]
    pub move_original: Option<PathBuf>,

    /// Directory for failure receipts; failed sources stay in place (default: <dir>/.error)
    #[arg(long)]
    pub error_dir: Option<PathBuf>,

    /// Milliseconds of filesystem inactivity before a file is considered ready
    #[arg(long, default_value_t = 500)]
    pub settle_delay_ms: u64,

    /// Optional webhook URL to POST the completion result and receipt path
    #[arg(long)]
    pub webhook: Option<String>,

    /// Process existing files and exit (useful for one-shot batch and testing)
    #[arg(long)]
    pub once: bool,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    batch: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow: Option<serde_json::Value>,
}

pub fn run() {
    #[cfg(windows)]
    {
        // Windows default thread stack is only 1MB which overflows on large Clap parser hierarchies.
        // We spawn a worker thread with an 8MB stack (standard Linux/macOS stack size).
        let handler = std::thread::Builder::new()
            .stack_size(8 * 1024 * 1024)
            .spawn(run_impl)
            .expect("Failed to spawn CLI thread with 8MB stack");
        let _ = handler.join();
    }
    #[cfg(not(windows))]
    {
        run_impl();
    }
}

fn run_impl() {
    let cli = Cli::parse();

    let log_level = cli
        .log_level
        .clone()
        .or_else(|| std::env::var("RUST_LOG").ok())
        .unwrap_or_else(|| "info".to_string());

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&log_level));

    let log_format = cli
        .log_format
        .clone()
        .or_else(|| std::env::var("LOG_FORMAT").ok())
        .unwrap_or_else(|| "text".to_string())
        .to_lowercase();

    match log_format.as_str() {
        "json" => {
            let _ = tracing_subscriber::fmt()
                .json()
                .with_writer(std::io::stderr)
                .with_env_filter(env_filter)
                .try_init();
        }
        "compact" => {
            let _ = tracing_subscriber::fmt()
                .compact()
                .with_writer(std::io::stderr)
                .with_env_filter(env_filter)
                .try_init();
        }
        "pretty" => {
            let _ = tracing_subscriber::fmt()
                .pretty()
                .with_writer(std::io::stderr)
                .with_env_filter(env_filter)
                .try_init();
        }
        _ => {
            let _ = tracing_subscriber::fmt()
                .with_writer(std::io::stderr)
                .with_env_filter(env_filter)
                .try_init();
        }
    }

    let mut args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "watch" {
        args.remove(1);
        let watch_cli = WatchCli::parse_from(args);

        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        if let Err(e) = rt.block_on(paperpilot_pdf::operations::watch::handle_watch_with_naming(
            watch_cli.directory,
            watch_cli.output,
            watch_cli.recipe,
            watch_cli.webhook,
            watch_cli.settle_delay_ms,
            watch_cli.move_original,
            watch_cli.error_dir,
            watch_cli.once,
            None,
            watch_cli.name_template,
        )) {
            tracing::error!(error = %e, "Watch loop failed");
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    let _ = env_logger::try_init();

    let mut workflow = None;
    let (result, batch) = match &cli.command {
        cli::Commands::Workflow { .. } | cli::Commands::Run { .. } => {
            let (result, report) = commands::workflow::execute(&cli.command, cli.json);
            workflow = report;
            (result, None)
        }
        #[cfg(feature = "pdf")]
        cli::Commands::FillBatch(args) => match commands::fill_batch::run(args) {
            Ok(receipt) => (
                receipt.outcome(),
                Some(serde_json::to_value(receipt).unwrap()),
            ),
            Err(error) => (Err(error), None),
        },
        _ => (commands::execute_command(&cli.command), None),
    };

    let op_name = format!("{:?}", cli.command)
        .split('(')
        .next()
        .unwrap_or("Unknown")
        .split_whitespace()
        .next()
        .unwrap_or("Unknown")
        .to_string();

    tracing::debug!(subcommand = %op_name, "Dispatching CLI subcommand");

    let payload = match result {
        Ok(_) => CommandResult {
            success: true,
            operation: op_name.clone(),
            error: None,
            batch,
            workflow,
        },
        Err(e) => {
            tracing::error!(subcommand = %op_name, error = %e, "CLI execution failed");
            CommandResult {
                success: false,
                operation: op_name.clone(),
                error: Some(e.to_string()),
                batch,
                workflow,
            }
        }
    };

    if cli.json {
        let json_output = serde_json::to_string(&payload).unwrap();
        println!("{}", json_output);
    } else {
        if let Some(batch) = &payload.batch {
            eprintln!(
                "Batch: {} succeeded, {} failed. Receipt: {}",
                batch["succeeded"],
                batch["failed"],
                batch["receipt_path"].as_str().unwrap_or("")
            );
        }
        match &payload.error {
            Some(err) => error!("Operation failed: {}", err),
            None => info!("Operation completed successfully."),
        }
    }

    if let Some(webhook_url) = cli.webhook {
        let should_fire = if cli.webhook_on_success && cli.webhook_on_failure {
            true // If both are set, logic usually dictates firing on both, or it's contradictory. Let's fire always if no filters, or if both are on.
        } else if cli.webhook_on_success {
            payload.success
        } else if cli.webhook_on_failure {
            !payload.success
        } else {
            true // No filters specified, fire always
        };

        if should_fire {
            let client = Client::new();
            let mut request = client.post(&webhook_url).json(&payload);

            if let Some(secret) = cli.webhook_secret {
                use hmac::{Hmac, Mac};
                use sha2::Sha256;
                type HmacSha256 = Hmac<Sha256>;

                let payload_bytes = serde_json::to_vec(&payload).unwrap();
                let mut mac = <HmacSha256 as hmac::KeyInit>::new_from_slice(secret.as_bytes())
                    .expect("HMAC can take key of any size");
                mac.update(&payload_bytes);
                let result = mac.finalize();
                let signature = hex::encode(result.into_bytes());
                request = request.header("x-paperpilot-signature", signature);
            }

            match request.send() {
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
    }

    if !payload.success {
        process::exit(1);
    }
}
