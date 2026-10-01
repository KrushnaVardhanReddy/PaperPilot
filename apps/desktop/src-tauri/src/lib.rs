use rmcp::model::CallToolRequestParams;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod error;
use crate::error::DesktopError;

lazy_static::lazy_static! {
    static ref CANCEL_FLAGS: Arc<Mutex<HashMap<String, bool>>> =
        Arc::new(Mutex::new(HashMap::new()));
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

    invoke_mcp_tool(
        "pdf_annotate".to_string(),
        serde_json::Value::Object(args),
    )
    .await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, invoke_mcp_tool, cancel_job, save_annotations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::{cancel_job, invoke_mcp_tool, CANCEL_FLAGS};
    use serde_json::json;

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
}
