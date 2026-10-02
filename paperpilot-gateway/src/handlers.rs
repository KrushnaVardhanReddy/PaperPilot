use axum::{
    extract::{Path, Multipart},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, CallToolResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub tools_count: usize,
}

pub async fn health() -> impl IntoResponse {
    let tools_count = match PaperPilotMcpServer::execute_list_tools() {
        Ok(res) => res.tools.len(),
        Err(_) => 0,
    };
    let resp = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        tools_count,
    };
    (StatusCode::OK, Json(resp))
}

#[derive(Deserialize)]
pub struct McpExecRequest {
    pub tool: String,
    pub arguments: Option<Value>,
}

pub async fn mcp_exec(Json(payload): Json<McpExecRequest>) -> impl IntoResponse {
    let mut request = CallToolRequestParams::default();
    request.name = payload.tool.into();
    if let Some(Value::Object(args)) = payload.arguments {
        request.arguments = Some(args);
    } else if payload.arguments.is_some() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "arguments must be an object"})));
    }

    match PaperPilotMcpServer::execute_call_tool(request) {
        Ok(CallToolResponse::Complete(result)) => {
            (StatusCode::OK, Json(serde_json::json!({"success": true, "result": result})))
        },
        Ok(_) => {
             (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": "Unknown response format"})))
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": e.message})))
        }
    }
}

pub async fn handle_tool(
    Path(tool_name): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut args = serde_json::Map::new();
    let mut temp_files = vec![];

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name.is_empty() { continue; }

        let file_name = field.file_name().map(|s| s.to_string());

        let data = match field.bytes().await {
            Ok(d) => d,
            Err(_) => continue,
        };

        if file_name.is_some() || name == "file" || name == "input" {
            let temp_file = NamedTempFile::new().unwrap();
            let path = temp_file.path().to_owned();
            let mut file = File::create(&path).unwrap();
            file.write_all(&data).unwrap();

            if name == "input" || name == "file" {
                args.insert("input".to_string(), Value::String(path.to_string_lossy().to_string()));
            } else if args.contains_key(&name) {
                // If it's an array like merge
                 if let Value::Array(arr) = args.get_mut(&name).unwrap() {
                     arr.push(Value::String(path.to_string_lossy().to_string()));
                 }
            } else {
                args.insert(name.clone(), Value::String(path.to_string_lossy().to_string()));
            }
            temp_files.push(temp_file);
        } else {
            let s = String::from_utf8_lossy(&data).to_string();
            // Try parsing as json if possible, else string
            if let Ok(val) = serde_json::from_str::<Value>(&s) {
                 args.insert(name, val);
            } else {
                 args.insert(name, Value::String(s));
            }
        }
    }

    let mut request = CallToolRequestParams::default();

    let resolved_name = if tool_name.starts_with("pdf_") {
        tool_name.clone()
    } else {
        format!("pdf_{}", tool_name)
    };

    request.name = resolved_name.into();
    request.arguments = Some(args);

    match PaperPilotMcpServer::execute_call_tool(request) {
        Ok(CallToolResponse::Complete(result)) => {
            (StatusCode::OK, Json(serde_json::json!({"success": true, "result": result})))
        },
        Ok(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": "Unknown response type"})))
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": e.message})))
        }
    }
}

pub async fn merge(multipart: Multipart) -> impl IntoResponse {
    handle_tool(Path("merge".to_string()), multipart).await
}

pub async fn split(multipart: Multipart) -> impl IntoResponse {
    handle_tool(Path("split".to_string()), multipart).await
}

pub async fn compress(multipart: Multipart) -> impl IntoResponse {
    handle_tool(Path("compress".to_string()), multipart).await
}

pub async fn extract_text(multipart: Multipart) -> impl IntoResponse {
    handle_tool(Path("extract_text".to_string()), multipart).await
}

pub async fn convert(multipart: Multipart) -> impl IntoResponse {
    handle_tool(Path("convert".to_string()), multipart).await
}

pub async fn watermark(multipart: Multipart) -> impl IntoResponse {
    handle_tool(Path("watermark".to_string()), multipart).await
}

pub async fn info(multipart: Multipart) -> impl IntoResponse {
    handle_tool(Path("metadata".to_string()), multipart).await
}
