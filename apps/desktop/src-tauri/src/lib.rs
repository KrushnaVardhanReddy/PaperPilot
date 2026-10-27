#[cfg(feature = "desktop-features")]
use rmcp::model::CallToolRequestParams;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

pub mod ai_commands;
pub mod error;
pub mod mobile_bridge;
pub mod ocr_commands;
pub mod pack_commands;
pub mod storage;
use crate::error::DesktopError;

lazy_static::lazy_static! {
    static ref CANCEL_FLAGS: Arc<Mutex<HashMap<String, bool>>> =
        Arc::new(Mutex::new(HashMap::new()));

    static ref GATEWAY_RUNNING: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref GATEWAY_SHUTDOWN_TX: Arc<Mutex<Option<oneshot::Sender<()>>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
fn cancel_job(job_id: String) -> Result<(), DesktopError> {
    let mut flags = CANCEL_FLAGS
        .lock()
        .map_err(|e| DesktopError::LockError(e.to_string()))?;
    flags.insert(job_id, true);
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[cfg(feature = "desktop-features")]
async fn invoke_mcp_tool(
    tool_name: String,
    arguments: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let mut request = CallToolRequestParams::default();
    let cwd = std::env::current_dir().unwrap_or_default();
    println!(">>> Invoking MCP tool: {}", tool_name);
    println!(">>> CWD: {}", cwd.display());
    println!(">>> Arguments: {}", arguments);
    request.name = tool_name.into();
    if let serde_json::Value::Object(map) = arguments {
        request.arguments = Some(map);
    }

    match paperpilot_mcp::server::PaperPilotMcpServer::execute_call_tool(request) {
        Ok(response) => {
            if let rmcp::model::CallToolResponse::Complete(result) = response {
                if let Some(rmcp::model::ContentBlock::Text(text_content)) = result.content.first()
                {
                    if let Ok(parsed) =
                        serde_json::from_str::<serde_json::Value>(&text_content.text)
                    {
                        return Ok(parsed);
                    }
                }
            }
            Err("Failed to parse MCP response".to_string())
        }
        Err(e) => Err(e.message.to_string()),
    }
}

#[tauri::command]
async fn save_annotations(
    file_path: String,
    output_path: String,
    annotations: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let mut args = serde_json::Map::new();
    args.insert("input".to_string(), serde_json::Value::String(file_path));
    args.insert("output".to_string(), serde_json::Value::String(output_path));
    args.insert("annotations".to_string(), annotations);

    invoke_mcp_tool("pdf_annotate".to_string(), serde_json::Value::Object(args)).await
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct FileMetadata {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
}

#[tauri::command]
async fn get_file_metadata(path: String) -> Result<FileMetadata, String> {
    let p = std::path::Path::new(&path);
    let meta = std::fs::metadata(p).map_err(|e| e.to_string())?;
    let name = p
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "document.pdf".to_string());
    Ok(FileMetadata {
        path,
        name,
        size_bytes: meta.len(),
    })
}

#[tauri::command]
async fn read_file_bytes(path: String) -> Result<Vec<u8>, String> {
    std::fs::read(&path).map_err(|e| e.to_string())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct RemoteFileResult {
    pub name: String,
    pub bytes: Vec<u8>,
}

#[tauri::command]
async fn download_remote_file(url: String) -> Result<RemoteFileResult, String> {
    let client = reqwest::Client::builder()
        .user_agent("PaperPilot/0.6.0")
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!(
            "HTTP Error {}: Failed to fetch file",
            resp.status()
        ));
    }

    let mut filename = "document.pdf".to_string();
    if let Some(cd) = resp.headers().get("content-disposition") {
        if let Ok(cd_str) = cd.to_str() {
            if let Some(idx) = cd_str.find("filename=") {
                let name_part = &cd_str[idx + 9..];
                let clean_name = name_part
                    .trim_matches('"')
                    .trim_matches('\'')
                    .split(';')
                    .next()
                    .unwrap_or("document.pdf");
                if !clean_name.is_empty() {
                    filename = clean_name.to_string();
                }
            }
        }
    } else if let Ok(parsed) = reqwest::Url::parse(&url) {
        if let Some(seg) = parsed.path_segments().and_then(|mut s| s.next_back()) {
            if !seg.is_empty() {
                filename = seg.to_string();
            }
        }
    }

    let bytes = resp.bytes().await.map_err(|e| e.to_string())?.to_vec();
    Ok(RemoteFileResult {
        name: filename,
        bytes,
    })
}

#[derive(serde::Serialize)]
struct GatewayStatus {
    is_running: bool,
    port: u16,
    swagger_url: String,
}

#[tauri::command]
#[cfg(feature = "desktop-features")]
fn get_gateway_status() -> GatewayStatus {
    let is_running = *GATEWAY_RUNNING.lock().unwrap();
    GatewayStatus {
        is_running,
        port: 7823,
        swagger_url: "http://127.0.0.1:7823/swagger-ui".to_string(),
    }
}

#[tauri::command]
#[cfg(feature = "desktop-features")]
async fn start_gateway() -> Result<(), String> {
    let mut is_running = GATEWAY_RUNNING.lock().unwrap();
    if *is_running {
        return Ok(());
    }

    let (tx, rx) = oneshot::channel();

    // Spawn the gateway
    tokio::spawn(async move {
        let shutdown_signal = async {
            let _ = rx.await;
        };
        if let Err(e) = paperpilot_gateway::server::start(7823, "127.0.0.1", shutdown_signal).await
        {
            eprintln!("Gateway server failed: {}", e);
        }

        *GATEWAY_RUNNING.lock().unwrap() = false;
    });

    *is_running = true;
    *GATEWAY_SHUTDOWN_TX.lock().unwrap() = Some(tx);

    Ok(())
}

#[tauri::command]
#[cfg(feature = "desktop-features")]
async fn stop_gateway() -> Result<(), String> {
    let mut tx_lock = GATEWAY_SHUTDOWN_TX.lock().unwrap();
    if let Some(tx) = tx_lock.take() {
        let _ = tx.send(());
    }

    // The tokio task will clean up the GATEWAY_RUNNING flag
    Ok(())
}

#[tauri::command]
#[cfg(not(feature = "desktop-features"))]
async fn invoke_mcp_tool(
    _tool_name: String,
    _arguments: serde_json::Value,
) -> Result<serde_json::Value, String> {
    Err(String::from("MCP tools are not supported on mobile"))
}

#[tauri::command]
#[cfg(not(feature = "desktop-features"))]
fn get_gateway_status() -> GatewayStatus {
    GatewayStatus {
        is_running: false,
        port: 7823,
        swagger_url: String::from("http://127.0.0.1:7823/swagger-ui"),
    }
}

#[tauri::command]
#[cfg(not(feature = "desktop-features"))]
async fn start_gateway() -> Result<(), String> {
    Err(String::from("Gateway is not supported on mobile"))
}

#[tauri::command]
#[cfg(not(feature = "desktop-features"))]
async fn stop_gateway() -> Result<(), String> {
    Err(String::from("Gateway is not supported on mobile"))
}

#[tauri::command]
async fn resolve_natural_language(
    query: String,
    context: Option<paperpilot_nlp::traits::ResolverContext>,
) -> Result<serde_json::Value, String> {
    use paperpilot_nlp::traits::NlpResolver;
    let resolver = paperpilot_nlp::resolver::OfflineNlpResolver::new();
    let ctx = context.unwrap_or_default();
    match resolver.resolve_with_context(&query, &ctx) {
        Ok(plan) => serde_json::to_value(&plan).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn query_documentation_rag(
    query: String,
    engine: tauri::State<paperpilot_nlp::rag::DocumentationRagEngine>,
) -> Result<paperpilot_nlp::rag::RagAnswer, String> {
    if let Some(ans) = engine.query(&query) {
        Ok(ans)
    } else {
        Err("No matching documentation found.".into())
    }
}

#[tauri::command]
async fn get_jobs_history(
    limit: Option<usize>,
) -> Result<Vec<paperpilot_storage::models::JobRecord>, String> {
    let engine = storage::get_storage_engine().await?;
    engine
        .list_jobs(limit.unwrap_or(50))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_audit_events(
    limit: Option<usize>,
) -> Result<Vec<paperpilot_storage::models::AuditEvent>, String> {
    let engine = storage::get_storage_engine().await?;
    engine
        .list_audit_events(limit.unwrap_or(50))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_document_vectors(
    query_embedding: Vec<f32>,
    limit: Option<usize>,
) -> Result<Vec<paperpilot_storage::models::SemanticSearchResult>, String> {
    let engine = storage::get_storage_engine().await?;
    engine
        .search_chunks(&query_embedding, limit.unwrap_or(10), None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn store_document_chunks(
    chunks: Vec<paperpilot_storage::models::NewDocumentChunk>,
) -> Result<usize, String> {
    let engine = storage::get_storage_engine().await?;
    engine
        .store_chunks(&chunks)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn clear_jobs_history() -> Result<usize, String> {
    let engine = storage::get_storage_engine().await?;
    engine.clear_all_jobs().await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                // Drop LlamafileSupervisor to kill process when the window closes
                if window.label() == "main" {
                    tauri::async_runtime::block_on(async {
                        let mut supervisor_lock = ai_commands::GLOBAL_LLAMAFILE_SUPERVISOR.lock().await;
                        *supervisor_lock = None;
                    });
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri::Manager;
                let icon_bytes = include_bytes!("../icons/icon.png");
                if let Ok(icon) = tauri::image::Image::from_bytes(icon_bytes) {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.set_icon(icon.clone());
                    }

                    let _ = tauri::tray::TrayIconBuilder::new()
                        .icon(icon)
                        .tooltip("PaperPilot")
                        .build(app);
                }
            }
            let _ = app;
            Ok(())
        })
        .manage(paperpilot_nlp::rag::DocumentationRagEngine::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            invoke_mcp_tool,
            cancel_job,
            query_documentation_rag,
            save_annotations,
            read_file_bytes,
            download_remote_file,
            get_file_metadata,
            get_gateway_status,
            start_gateway,
            stop_gateway,
            resolve_natural_language,
            get_jobs_history,
            clear_jobs_history,
            get_audit_events,
            search_document_vectors,
            store_document_chunks,
            ai_commands::get_available_models,
            ai_commands::download_model,
            ai_commands::cancel_model_download,
            ai_commands::save_ai_config,
            ai_commands::get_ai_config,
            ai_commands::get_ai_execution_status,
            ai_commands::test_ai_endpoint,
            ocr_commands::get_available_ocr_models,
            ocr_commands::download_ocr_model,
            ocr_commands::cancel_ocr_model_download,
            ocr_commands::set_active_ocr_model,
            ocr_commands::get_active_ocr_model,
            pack_commands::is_portable_mode_cmd,
            pack_commands::get_feature_packs,
            pack_commands::download_feature_pack,
            pack_commands::set_feature_pack_enabled
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::{cancel_job, get_file_metadata, CANCEL_FLAGS};
    #[cfg(feature = "desktop-features")]
    #[cfg(feature = "desktop-features")]
    use crate::{
        get_gateway_status, invoke_mcp_tool, start_gateway, stop_gateway, GATEWAY_RUNNING,
        GATEWAY_SHUTDOWN_TX,
    };
    #[cfg(feature = "desktop-features")]
    use serde_json::json;
    use std::io::Write;

    #[tokio::test]
    #[cfg(feature = "desktop-features")]
    #[cfg(feature = "desktop-features")]
    async fn test_invoke_mcp_tool_validation_error() {
        // Test with a tool that expects an input file that doesn't exist
        let tool_name = "pdf_metadata".to_string();
        let args = json!({
            "input": "non_existent_file.pdf"
        });

        let result = invoke_mcp_tool(tool_name, args).await;

        // Since the file does not exist, the MCP server should attempt to read it
        // and return a graceful error message indicating the failure.
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(!err_msg.is_empty(), "Error message should not be empty");
    }

    #[tokio::test]
    #[cfg(feature = "desktop-features")]
    #[cfg(feature = "desktop-features")]
    async fn test_invoke_mcp_tool_invalid_tool() {
        let tool_name = "invalid_tool_name".to_string();
        let args = json!({});

        let result = invoke_mcp_tool(tool_name, args).await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("Unknown tool"));
    }

    #[test]
    fn test_cancel_job() {
        let job_id = "test-job-123".to_string();
        let res = cancel_job(job_id.clone());
        assert!(res.is_ok());

        let flags = CANCEL_FLAGS.lock().unwrap();
        assert_eq!(flags.get(&job_id), Some(&true));
    }

    #[tokio::test]
    async fn test_get_file_metadata_valid() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_metadata.pdf");
        let content = b"dummy pdf content";
        {
            let mut f = std::fs::File::create(&file_path).unwrap();
            f.write_all(content).unwrap();
        }

        let res = get_file_metadata(file_path.to_string_lossy().to_string()).await;
        assert!(res.is_ok());
        let metadata = res.unwrap();
        assert_eq!(metadata.name, "test_metadata.pdf");
        assert_eq!(metadata.size_bytes, content.len() as u64);

        std::fs::remove_file(file_path).unwrap();
    }

    #[tokio::test]
    async fn test_get_file_metadata_invalid() {
        let res = get_file_metadata("non_existent_file_path.pdf".to_string()).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    #[cfg(feature = "desktop-features")]
    #[cfg(feature = "desktop-features")]
    async fn test_gateway_lifecycle() {
        // Reset state
        *GATEWAY_RUNNING.lock().unwrap() = false;
        *GATEWAY_SHUTDOWN_TX.lock().unwrap() = None;

        // Ensure status is initially false
        let status1 = get_gateway_status();
        assert!(!status1.is_running);
        assert_eq!(status1.port, 7823);

        // Start gateway
        let start_res = start_gateway().await;
        assert!(start_res.is_ok());

        // Check state after start
        let status2 = get_gateway_status();
        assert!(status2.is_running);
        assert!(GATEWAY_SHUTDOWN_TX.lock().unwrap().is_some());

        // Calling start again should also return Ok (no-op)
        let start_res2 = start_gateway().await;
        assert!(start_res2.is_ok());

        // Stop gateway
        let stop_res = stop_gateway().await;
        assert!(stop_res.is_ok());

        // Verify TX is taken (the async task itself resets running state eventually,
        // but TX should be gone immediately)
        assert!(GATEWAY_SHUTDOWN_TX.lock().unwrap().is_none());

        // Give the spawned task a tiny bit of time to shutdown and reset the flag
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let status3 = get_gateway_status();
        assert!(!status3.is_running);
    }

    #[tokio::test]
    async fn test_resolve_natural_language() {
        use crate::resolve_natural_language;
        let result = resolve_natural_language("merge a.pdf and b.pdf".to_string(), None).await;
        assert!(result.is_ok());
        let val = result.unwrap();

        let intent = val.get("intent").and_then(|i| i.as_str());
        assert_eq!(intent, Some("Merge"));
        let inputs = val.get("input_files").and_then(|i| i.as_array());
        assert!(inputs.is_some());
    }

    #[tokio::test]
    async fn test_resolve_natural_language_with_context() {
        use crate::resolve_natural_language;
        use paperpilot_nlp::traits::ResolverContext;

        let ctx = ResolverContext {
            active_document: Some("active_doc.pdf".to_string()),
            open_documents: vec!["active_doc.pdf".to_string()],
        };

        let result = resolve_natural_language("rotate 90 degrees".to_string(), Some(ctx)).await;
        assert!(result.is_ok());
        let val = result.unwrap();

        let intent = val.get("intent").and_then(|i| i.as_str());
        assert_eq!(intent, Some("Rotate"));
    }
}
