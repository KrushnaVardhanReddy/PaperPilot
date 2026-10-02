use axum::{
    extract::{Path, Multipart, Request},
    http::{StatusCode, header},
    response::{IntoResponse, Json},
};
use axum::extract::FromRequest;
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
    req: Request,
) -> impl IntoResponse {
    let mut args = serde_json::Map::new();
    let mut temp_files = vec![];

    let content_type = req.headers()
        .get(header::CONTENT_TYPE)
        .and_then(|val| val.to_str().ok())
        .unwrap_or("")
        .to_string();

    if content_type.starts_with("application/json") {
        const MAX_BODY_LIMIT: usize = 10 * 1024 * 1024; // 10MB
        if let Ok(bytes) = axum::body::to_bytes(req.into_body(), MAX_BODY_LIMIT).await {
            if let Ok(Value::Object(map)) = serde_json::from_slice(&bytes) {
                args = map;
            } else {
                return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Invalid JSON"}))).into_response();
            }
        } else {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Failed to read body or body too large"}))).into_response();
        }
    } else if content_type.starts_with("multipart/form-data") {
        let mut multipart = match Multipart::from_request(req, &()).await {
            Ok(m) => m,
            Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Invalid multipart data"}))).into_response(),
        };

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
                     if let Value::Array(arr) = args.get_mut(&name).unwrap() {
                         arr.push(Value::String(path.to_string_lossy().to_string()));
                     }
                } else {
                    args.insert(name.clone(), Value::String(path.to_string_lossy().to_string()));
                }
                temp_files.push(temp_file);
            } else {
                let s = String::from_utf8_lossy(&data).to_string();
                if let Ok(val) = serde_json::from_str::<Value>(&s) {
                     args.insert(name, val);
                } else {
                     args.insert(name, Value::String(s));
                }
            }
        }
    } else {
        return (StatusCode::UNSUPPORTED_MEDIA_TYPE, Json(serde_json::json!({"error": "Unsupported media type"}))).into_response();
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
            (StatusCode::OK, Json(serde_json::json!({"success": true, "result": result}))).into_response()
        },
        Ok(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": "Unknown response type"}))).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": e.message}))).into_response()
        }
    }
}

pub async fn merge(req: Request) -> impl IntoResponse {
    handle_tool(Path("merge".to_string()), req).await
}

pub async fn split(req: Request) -> impl IntoResponse {
    handle_tool(Path("split".to_string()), req).await
}

pub async fn compress(req: Request) -> impl IntoResponse {
    handle_tool(Path("compress".to_string()), req).await
}

pub async fn extract_text(req: Request) -> impl IntoResponse {
    handle_tool(Path("extract_text".to_string()), req).await
}

pub async fn convert(req: Request) -> impl IntoResponse {
    handle_tool(Path("convert".to_string()), req).await
}

pub async fn watermark(req: Request) -> impl IntoResponse {
    handle_tool(Path("watermark".to_string()), req).await
}

pub async fn info(mut multipart: Multipart) -> impl IntoResponse {
    let mut temp_files = vec![];
    let mut input_path = None;

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
                input_path = Some(path.to_path_buf());
            }
            temp_files.push(temp_file);
        }
    }

    if let Some(path) = input_path {
        if let Ok(mut doc) = paperpilot_pdf::document::LopdfDocument::load(&path) {
            use paperpilot_core::traits::PdfOperation;
            let op = paperpilot_pdf::operations::info::InfoOperation::new();
            if op.execute(&mut doc).is_ok() {
                if let Some(res) = op.result.lock().unwrap().clone() {
                    return (StatusCode::OK, Json(serde_json::json!({
                        "success": true,
                        "page_count": res.page_count,
                        "version": res.version
                    }))).into_response();
                }
            }
        }
    }

    (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": "Failed to extract info"}))).into_response()
}
