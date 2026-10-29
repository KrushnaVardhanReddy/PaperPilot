//! Native hot-folder automation. All frontends share the managed worker and
//! processor below; the async compatibility entry point owns no polling threads.
mod manager;
pub use manager::{
    EventSink, WatchConfig, WatchEvent, WatchSnapshot, WatchStatus, WatcherManager,
    resolve_watch_recipe, shared_watcher_manager,
};
use paperpilot_core::error::{OperationResult, PdfError};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

#[allow(clippy::too_many_arguments)]
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
    let manager = Arc::new(WatcherManager::default());
    let start_manager = manager.clone();
    let config = WatchConfig {
        directory,
        output_dir: output,
        recipe: recipe
            .ok_or_else(|| PdfError::InvalidInput("A recipe or preset is required".into()))?
            .to_string_lossy()
            .into(),
        settle_delay_ms,
        move_original,
        error_dir,
        once,
    };
    let sink: EventSink = Arc::new(move |event| {
        log::info!("Watcher {}: {}", event.watcher.id, event.kind);
        if let Some(error) = &event.error {
            log::error!("{error}");
        }
        if matches!(event.kind.as_str(), "file_processed" | "error") {
            if let Some(url) = &webhook {
                // Webhooks are opt-in, supplied explicitly by the CLI caller.
                // Retain the existing completion payload, adding artifact paths.
                let payload = serde_json::json!({
                    "success": event.kind == "file_processed", "operation": "recipe",
                    "file": event.file, "error": event.error,
                    "output": event.output, "receipt": event.receipt,
                });
                if let Err(error) = reqwest::blocking::Client::new()
                    .post(url)
                    .timeout(Duration::from_secs(30))
                    .json(&payload)
                    .send()
                    .and_then(reqwest::blocking::Response::error_for_status)
                {
                    log::error!("Webhook failed: {error}");
                }
            }
        }
    });
    tokio::task::spawn_blocking(move || start_manager.start("cli".into(), config, sink))
        .await
        .map_err(|e| PdfError::Other(e.to_string()))??;
    let token = cancel_token.unwrap_or_default();
    loop {
        let done = manager
            .list()
            .iter()
            .all(|s| matches!(s.status, WatchStatus::Stopped | WatchStatus::Error));
        if done {
            break;
        }
        tokio::select! {
            _ = token.cancelled() => break,
            signal = tokio::signal::ctrl_c(), if !once => { signal?; break; },
            _ = tokio::time::sleep(Duration::from_millis(50)) => {}
        }
    }
    let result = tokio::task::spawn_blocking(move || manager.stop("cli"))
        .await
        .map_err(|e| PdfError::Other(e.to_string()))??;
    if result.status == WatchStatus::Error || result.failed > 0 {
        return Err(PdfError::Other(
            result
                .last_error
                .unwrap_or_else(|| "One or more files failed".into()),
        ));
    }
    Ok(())
}
