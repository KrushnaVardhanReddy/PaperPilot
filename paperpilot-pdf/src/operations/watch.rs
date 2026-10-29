#![allow(clippy::collapsible_if)]
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use log::{error, info};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::workflow::WorkflowExecutionReport;
use paperpilot_core::workflow::{get_preset_by_id, parse_workflow};
use crate::operations::workflow::run_workflow_bytes_detailed;
use reqwest::blocking::Client;
use serde::Serialize;
use tokio_util::sync::CancellationToken;

#[derive(Serialize)]
struct WebhookPayload {
    success: bool,
    operation: String,
    file: String,
    error: Option<String>,
}

#[derive(Serialize)]
struct ExecutionReceipt {
    timestamp: String,
    file: String,
    sha256: String,
    success: bool,
    error: Option<String>,
}

fn execute_recipe(
    file_path: &Path,
    recipe_path: &Option<PathBuf>,
    _output_dir: &Option<PathBuf>,
) -> OperationResult<(Vec<u8>, WorkflowExecutionReport)> {
    let recipe_p = recipe_path.as_ref().ok_or_else(|| {
        PdfError::InvalidInput("No recipe specified for watch".to_string())
    })?;

    let workflow = if recipe_p.exists() {
        let recipe_str = std::fs::read_to_string(recipe_p).map_err(|e| PdfError::IoError(e))?;
        parse_workflow(&recipe_str)?
    } else {
        let recipe_str = recipe_p.to_string_lossy().to_string();
        get_preset_by_id(&recipe_str).ok_or_else(|| {
            PdfError::InvalidInput(format!("Preset not found: {}", recipe_str))
        })?
    };

    let input_bytes = std::fs::read(file_path).map_err(|e| PdfError::IoError(e))?;

    run_workflow_bytes_detailed(&input_bytes, &workflow).map_err(|failure| failure.cause)
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
    recipe: Option<PathBuf>,
    webhook: Option<String>,
    settle_delay_ms: u64,
    move_original: Option<PathBuf>,
    error_dir: Option<PathBuf>,
    once: bool,
    cancel_token: Option<CancellationToken>,
) -> OperationResult<()> {
    if !directory.exists() {
        return Err(PdfError::InvalidInput(format!(
            "Directory does not exist: {:?}",
            directory
        )));
    }

    info!("Starting watch daemon on {:?}...", directory);

    if let Some(ref out_dir) = output {
        if !out_dir.exists() {
            std::fs::create_dir_all(out_dir).map_err(PdfError::IoError)?;
        }
    }

    if let Some(ref move_dir) = move_original {
        if !move_dir.exists() {
            std::fs::create_dir_all(move_dir).map_err(PdfError::IoError)?;
        }
    }

    let actual_error_dir = error_dir.unwrap_or_else(|| directory.join(".error"));
    if !actual_error_dir.exists() {
        std::fs::create_dir_all(&actual_error_dir).map_err(PdfError::IoError)?;
    }

    let settle_duration = Duration::from_millis(settle_delay_ms);
    let mut pending_files: HashMap<PathBuf, Instant> = HashMap::new();

    let mut initial_files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().eq_ignore_ascii_case("pdf") {
                        initial_files.push(path.clone());
                        pending_files.insert(path, Instant::now() - settle_duration - Duration::from_secs(1));
                    }
                }
            }
        }
    }

    initial_files.sort();

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| PdfError::IoError(std::io::Error::other(e)))?;

    if !once {
        watcher
            .watch(&directory, RecursiveMode::NonRecursive)
            .map_err(|e| PdfError::IoError(std::io::Error::other(e)))?;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if !once {
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            info!("Received SIGINT, shutting down watch daemon gracefully...");
            r.store(false, Ordering::SeqCst);
        });
    }

    let process_file = |path: &PathBuf| {
        info!("Processing settled file: {:?}", path);
        let op_name = "recipe".to_string();

        let result = execute_recipe(path, &recipe, &output);
        let success = result.is_ok();
        let err_msg = result.as_ref().err().map(|e| e.to_string());

        let file_stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

        if let Ok((bytes, _report)) = result {
            info!("Successfully processed: {:?}", path);
            let target_output_dir = output.clone().unwrap_or_else(|| directory.clone());
            let target_output_file = if output.is_some() {
                target_output_dir.join(path.file_name().unwrap())
            } else {
                target_output_dir.join(format!("{}_processed.pdf", file_stem))
            };

            if let Err(e) = std::fs::write(&target_output_file, bytes) {
                error!("Failed to write output file {:?}: {}", target_output_file, e);
            }

            if let Some(ref move_dir) = move_original {
                if let Some(file_name) = path.file_name() {
                    let target_path = move_dir.join(file_name);
                    if let Err(e) = std::fs::rename(path, &target_path) {
                        error!(
                            "Failed to move original file {:?} to {:?}: {}",
                            path, target_path, e
                        );
                    } else {
                        info!("Moved original file to {:?}", target_path);
                    }
                }
            }
        } else {
            error!(
                "Failed to process {:?}: {}",
                path,
                err_msg.as_deref().unwrap_or("Unknown error")
            );

            let target_path = actual_error_dir.join(path.file_name().unwrap());
            if let Err(e) = std::fs::rename(path, &target_path) {
                error!("Failed to move corrupt file {:?} to error dir: {}", path, e);
            }

            let receipt = ExecutionReceipt {
                timestamp: chrono::Utc::now().to_rfc3339(),
                file: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                sha256: "".to_string(),
                success: false,
                error: err_msg.clone(),
            };

            let receipt_path = actual_error_dir.join(format!("{}_receipt.json", file_stem));
            if let Ok(json) = serde_json::to_string_pretty(&receipt) {
                let _ = std::fs::write(receipt_path, json);
            }
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
    };

    while running.load(Ordering::SeqCst) {
        if let Some(token) = &cancel_token {
            if token.is_cancelled() {
                info!("Watcher cancelled via token");
                break;
            }
        }

        if once && pending_files.is_empty() {
            break;
        }

        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(event)) => {
                let Event { kind, paths, .. } = event;
                if matches!(kind, EventKind::Create(_) | EventKind::Modify(_)) {
                    for path in paths {
                        // Ignore subdirs like .error, processed, etc.
                        let is_in_watch_dir = path.parent().map_or(false, |p| p == directory.as_path());
                        if !is_in_watch_dir {
                            continue;
                        }

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
        to_process.sort();

        for path in to_process {
            pending_files.remove(&path);
            process_file(&path);
        }
    }

    Ok(())
}
