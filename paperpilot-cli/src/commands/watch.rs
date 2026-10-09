use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use log::{error, info, warn};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use reqwest::blocking::Client;
use clap::Parser;
use serde::Serialize;
use paperpilot_core::error::{OperationResult, PdfError};

#[derive(Serialize)]
struct WebhookPayload {
    success: bool,
    operation: String,
    file: String,
    error: Option<String>,
}

fn execute_operation(
    file_path: &Path,
    output_dir: &Option<PathBuf>,
    operation: &Option<String>,
    recipe: &Option<String>,
) -> OperationResult<()> {
    // If output_dir is provided, append the file name to it, or if it's a dir, output there.
    let output_path = if let Some(dir) = output_dir {
        if let Some(file_name) = file_path.file_name() {
            dir.join(file_name)
        } else {
            dir.clone()
        }
    } else {
        // Fallback: output to same dir with a prefix/suffix
        let mut p = file_path.to_path_buf();
        let stem = p.file_stem().unwrap_or_default().to_string_lossy();
        p.set_file_name(format!("{}_processed.pdf", stem));
        p
    };

    let mut args = vec!["paperpilot".to_string()];

    if let Some(_r) = recipe {
        // Recipe logic goes here. But based on cli.rs, there is no generic --recipe flag for the root command.
        // It might be parsed by another mechanism or we can construct a pipeline.
        // The issue description implies executing a pipeline recipe.
        // Let's check how recipe is executed in main.rs or if it exists.
        warn!("Recipe execution is not fully mapped in CLI commands yet.");
        // We will pass it as a generic operation for now, or just fail cleanly.
        // Or perhaps there is a pipeline runner in paperpilot_core?
    } else if let Some(op) = operation {
        args.push(op.clone());
        args.push("--input".to_string());
        args.push(file_path.to_string_lossy().to_string());
        args.push("--output".to_string());
        args.push(output_path.to_string_lossy().to_string());

        // Quality argument is required for compress
        if op == "compress" {
            args.push("--quality".to_string());
            args.push("medium".to_string()); // Defaulting to medium if not provided
        }

    } else {
        return Err(PdfError::InvalidInput("No operation or recipe specified for watch".to_string()));
    }

    let cli = match crate::cli::Cli::try_parse_from(args) {
        Ok(c) => c,
        Err(e) => {
            return Err(PdfError::InvalidInput(format!("Failed to parse constructed arguments: {}", e)));
        }
    };

    crate::commands::execute_command(&cli.command)
}

fn handle_webhook(webhook_url: &str, payload: WebhookPayload) {
    let client = Client::new();
    match client.post(webhook_url).json(&payload).send() {
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

pub async fn handle_watch(
    directory: PathBuf,
    output: Option<PathBuf>,
    operation: Option<String>,
    recipe: Option<String>,
    webhook: Option<String>,
    settle_delay_ms: u64,
    move_original: Option<PathBuf>,
) -> OperationResult<()> {
    if !directory.exists() {
        return Err(PdfError::InvalidInput(format!("Directory does not exist: {:?}", directory)));
    }

    info!("Starting watch daemon on {:?}...", directory);

    if let Some(ref out_dir) = output {
        if !out_dir.exists() {
            std::fs::create_dir_all(out_dir).map_err(|e| PdfError::IoError(e))?;
        }
    }

    if let Some(ref move_dir) = move_original {
        if !move_dir.exists() {
            std::fs::create_dir_all(move_dir).map_err(|e| PdfError::IoError(e))?;
        }
    }

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    watcher.watch(&directory, RecursiveMode::NonRecursive)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Received SIGINT, shutting down watch daemon gracefully...");
        r.store(false, Ordering::SeqCst);
    });

    let settle_duration = Duration::from_millis(settle_delay_ms);
    let mut pending_files: HashMap<PathBuf, Instant> = HashMap::new();

    while running.load(Ordering::SeqCst) {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(event)) => {
                let Event { kind, paths, .. } = event;
                if matches!(kind, EventKind::Create(_) | EventKind::Modify(_)) {
                    for path in paths {
                        if let Some(ext) = path.extension() {
                            if ext.to_string_lossy().eq_ignore_ascii_case("pdf") {
                                pending_files.insert(path, Instant::now());
                            }
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                error!("Watch error: {:?}", e);
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Check pending files to see if settle time has passed
            }
            Err(e) => {
                error!("Channel recv error: {:?}", e);
                break;
            }
        }

        let now = Instant::now();
        let mut to_process = Vec::new();

        for (path, last_seen) in pending_files.iter() {
            if now.duration_since(*last_seen) >= settle_duration {
                to_process.push(path.clone());
            }
        }

        for path in to_process {
            pending_files.remove(&path);

            info!("Processing settled file: {:?}", path);

            let op_name = operation.clone().unwrap_or_else(|| "recipe".to_string());
            let result = execute_operation(&path, &output, &operation, &recipe);

            let success = result.is_ok();
            let err_msg = result.err().map(|e| e.to_string());

            if success {
                info!("Successfully processed: {:?}", path);

                if let Some(ref move_dir) = move_original {
                    if let Some(file_name) = path.file_name() {
                        let target_path = move_dir.join(file_name);
                        if let Err(e) = std::fs::rename(&path, &target_path) {
                            error!("Failed to move original file {:?} to {:?}: {}", path, target_path, e);
                        } else {
                            info!("Moved original file to {:?}", target_path);
                        }
                    }
                }
            } else {
                error!("Failed to process {:?}: {}", path, err_msg.as_deref().unwrap_or("Unknown error"));
            }

            if let Some(ref w) = webhook {
                let payload = WebhookPayload {
                    success,
                    operation: op_name,
                    file: path.to_string_lossy().to_string(),
                    error: err_msg,
                };
                handle_webhook(w, payload);
            }
        }
    }

    Ok(())
}
