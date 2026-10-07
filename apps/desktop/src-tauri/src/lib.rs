use rmcp::model::CallToolRequestParams;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

pub mod error;
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

#[derive(serde::Serialize)]
struct GatewayStatus {
    is_running: bool,
    port: u16,
    swagger_url: String,
}

#[tauri::command]
fn get_gateway_status() -> GatewayStatus {
    let is_running = *GATEWAY_RUNNING.lock().unwrap();
    GatewayStatus {
        is_running,
        port: 7823,
        swagger_url: "http://127.0.0.1:7823/swagger-ui".to_string(),
    }
}

#[tauri::command]
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
async fn stop_gateway() -> Result<(), String> {
    let mut tx_lock = GATEWAY_SHUTDOWN_TX.lock().unwrap();
    if let Some(tx) = tx_lock.take() {
        let _ = tx.send(());
    }

    // The tokio task will clean up the GATEWAY_RUNNING flag
    Ok(())
}

#[tauri::command]
fn resolve_natural_language(
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
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
            get_file_metadata,
            get_gateway_status,
            start_gateway,
            stop_gateway,
            resolve_natural_language
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::{
        cancel_job, get_file_metadata, get_gateway_status, invoke_mcp_tool,
        query_documentation_rag, start_gateway, stop_gateway, CANCEL_FLAGS, GATEWAY_RUNNING,
        GATEWAY_SHUTDOWN_TX,
    };
    use serde_json::json;
    use std::io::Write;

    #[tokio::test]
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

    #[test]
    fn test_resolve_natural_language() {
        use crate::resolve_natural_language;
        let result = resolve_natural_language("merge a.pdf and b.pdf".to_string(), None);
        assert!(result.is_ok());
        let val = result.unwrap();
        let intent = val.get("intent").and_then(|i| i.as_str());
        assert_eq!(intent, Some("Merge"));
        let inputs = val.get("input_files").and_then(|i| i.as_array());
        assert!(inputs.is_some());
        // We do not strictly check the array length since it depends on the offline resolver's heuristics
    }

    #[test]
    fn test_resolve_natural_language_with_context() {
        use crate::resolve_natural_language;
        use paperpilot_nlp::traits::ResolverContext;

        let ctx = ResolverContext {
            active_document: Some("active_doc.pdf".to_string()),
            open_documents: vec!["active_doc.pdf".to_string()],
        };

        // Command missing explicit file should use active_document from context
        let result = resolve_natural_language("rotate 90 degrees".to_string(), Some(ctx));
        assert!(result.is_ok());
        let val = result.unwrap();
        let intent = val.get("intent").and_then(|i| i.as_str());
        assert_eq!(intent, Some("Rotate"));

        let inputs = val.get("input_files").and_then(|i| i.as_array());
        assert!(inputs.is_some());
        assert_eq!(inputs.unwrap().len(), 1);
        assert_eq!(inputs.unwrap()[0].as_str(), Some("active_doc.pdf"));
    }
}
