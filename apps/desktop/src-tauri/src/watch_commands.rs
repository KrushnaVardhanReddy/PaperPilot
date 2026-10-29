use paperpilot_pdf::operations::document_inspection::{inspect_document_bytes, DocumentInspection};
use paperpilot_pdf::operations::watch::{EventSink, WatchConfig, WatchSnapshot, WatcherManager};
use std::sync::Arc;
use tauri::{Emitter, State};

#[tauri::command]
pub async fn start_watch_folder(
    app: tauri::AppHandle,
    manager: State<'_, Arc<WatcherManager>>,
    id: String,
    config: WatchConfig,
) -> Result<WatchSnapshot, String> {
    let manager = manager.inner().clone();
    let sink: EventSink = Arc::new(move |event| {
        let channel = match event.kind.as_str() {
            "file_processed" => "watch:file_processed",
            "error" => "watch:error",
            _ => "watch:status",
        };
        let _ = app.emit(channel, &event);
    });
    tauri::async_runtime::spawn_blocking(move || manager.start(id, config, sink))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_watch_folder(
    manager: State<'_, Arc<WatcherManager>>,
    folder_id: String,
) -> Result<WatchSnapshot, String> {
    let manager = manager.inner().clone();
    tauri::async_runtime::spawn_blocking(move || manager.stop(&folder_id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_watched_folders(manager: State<'_, Arc<WatcherManager>>) -> Vec<WatchSnapshot> {
    manager.list()
}

#[tauri::command]
pub async fn remove_watch_folder(
    manager: State<'_, Arc<WatcherManager>>,
    folder_id: String,
) -> Result<(), String> {
    let manager = manager.inner().clone();
    tauri::async_runtime::spawn_blocking(move || manager.remove(&folder_id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn inspect_document(
    path: Option<String>,
    bytes: Option<Vec<u8>>,
) -> Result<DocumentInspection, String> {
    tauri::async_runtime::spawn_blocking(move || {
        const LIMIT: u64 = 128 * 1024 * 1024;
        let bytes = match (path, bytes) {
            (Some(path), None) => {
                let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
                if metadata.len() > LIMIT {
                    return Err("Document inspection is limited to 128 MiB".to_string());
                }
                std::fs::read(path).map_err(|e| e.to_string())?
            }
            (None, Some(bytes)) if bytes.len() as u64 <= LIMIT => bytes,
            _ => {
                return Err(
                    "Provide one local document path or byte buffer (up to 128 MiB)".into(),
                );
            }
        };
        inspect_document_bytes(&bytes).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn list_watch_recipes() -> Vec<serde_json::Value> {
    paperpilot_core::workflow::get_default_presets()
        .into_iter()
        .filter(|workflow| {
            paperpilot_pdf::operations::workflow::validate_workflow_definition(workflow).is_ok()
        })
        .map(|workflow| serde_json::json!({"id":workflow.id,"name":workflow.name}))
        .collect()
}
