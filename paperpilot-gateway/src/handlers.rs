use axum::{
    extract::Path,
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
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = "0.1.0")]
    pub version: String,
    #[schema(example = 44)]
    pub tools_count: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse {
    #[schema(example = true)]
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct McpExecRequest {
    #[schema(example = "pdf_metadata")]
    pub tool: String,
    #[schema(example = json!({"input": "/path/to/document.pdf"}))]
    pub arguments: Option<Value>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct MergeJsonRequest {
    #[schema(example = json!(["doc1.pdf", "doc2.pdf"]))]
    pub inputs: Vec<String>,
    #[schema(example = "merged.pdf")]
    pub output: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SplitJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "1-3,5")]
    pub pages: Option<String>,
    #[schema(example = "output_dir/")]
    pub output: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CompressJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "output.pdf")]
    pub output: String,
    #[schema(example = "medium")]
    pub quality: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ExtractTextJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "1")]
    pub page: Option<u32>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ConvertJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "output.docx")]
    pub output: String,
    #[schema(example = "docx")]
    pub format: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct WatermarkJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "watermarked.pdf")]
    pub output: String,
    #[schema(example = "CONFIDENTIAL")]
    pub text: Option<String>,
    #[schema(example = 0.3)]
    pub opacity: Option<f32>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct InfoJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CompareJsonRequest {
    #[schema(example = "doc_v1.pdf")]
    pub file1: String,
    #[schema(example = "doc_v2.pdf")]
    pub file2: String,
}

/// Gateway health check
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy and ready", body = HealthResponse)
    )
)]
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

/// Execute MCP tool directly by tool name with arbitrary JSON arguments
#[utoipa::path(
    post,
    path = "/api/v1/pdf/mcp-exec",
    tag = "MCP",
    request_body = McpExecRequest,
    responses(
        (status = 200, description = "Tool executed successfully", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
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

/// Execute any MCP tool by URL path param (/api/v1/pdf/tools/{tool_name}) via multipart or JSON
#[utoipa::path(
    post,
    path = "/api/v1/pdf/tools/{tool_name}",
    tag = "MCP",
    params(
        ("tool_name" = String, Path, description = "Name of the tool (with or without 'pdf_' prefix)")
    ),
    responses(
        (status = 200, description = "Tool executed successfully", body = ApiResponse),
        (status = 400, description = "Invalid multipart or payload"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn handle_tool(
    Path(tool_name): Path<String>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    let content_type = req.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("");
    let mut args = serde_json::Map::new();
    let mut temp_files = vec![];

    if content_type.starts_with("application/json") {
        let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap_or_default();
        if let Ok(Value::Object(map)) = serde_json::from_slice(&body_bytes) {
            args = map;
        }
    } else {
        use axum::extract::FromRequest;
        let mut multipart = match axum::extract::Multipart::from_request(req, &()).await {
            Ok(m) => m,
            Err(_) => return (axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({"success": false, "error": "Invalid multipart request"}))).into_response(),
        };
        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().unwrap_or("").to_string();
            if name.is_empty() { continue; }

            let file_name = field.file_name().map(|s| s.to_string());

            let data = match field.bytes().await {
                Ok(d) => d,
                Err(_) => continue,
            };

            if file_name.is_some() || name == "file" || name == "input" || name == "file1" || name == "file2" || name == "file_a" || name == "file_b" || name == "input_a" || name == "input_b" {
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

/// Merge multiple PDF documents into a single PDF
#[utoipa::path(
    post,
    path = "/api/v1/pdf/merge",
    tag = "PDF Operations",
    request_body(content = MergeJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Merged successfully", body = ApiResponse)
    )
)]
pub async fn merge(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("merge".to_string()), req).await
}

/// Split a PDF document into separate pages or ranges
#[utoipa::path(
    post,
    path = "/api/v1/pdf/split",
    tag = "PDF Operations",
    request_body(content = SplitJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Split successfully", body = ApiResponse)
    )
)]
pub async fn split(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("split".to_string()), req).await
}

/// Compress a PDF document to reduce file size
#[utoipa::path(
    post,
    path = "/api/v1/pdf/compress",
    tag = "PDF Operations",
    request_body(content = CompressJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Compressed successfully", body = ApiResponse)
    )
)]
pub async fn compress(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("compress".to_string()), req).await
}

/// Extract clean text from a PDF document
#[utoipa::path(
    post,
    path = "/api/v1/pdf/extract-text",
    tag = "PDF Operations",
    request_body(content = ExtractTextJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Text extracted successfully", body = ApiResponse)
    )
)]
pub async fn extract_text(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("extract_text".to_string()), req).await
}

/// Convert a PDF to/from another format (docx, images, html, xlsx, etc.)
#[utoipa::path(
    post,
    path = "/api/v1/pdf/convert",
    tag = "PDF Operations",
    request_body(content = ConvertJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Converted successfully", body = ApiResponse)
    )
)]
pub async fn convert(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("convert".to_string()), req).await
}

/// Apply text or image watermarks to a PDF document
#[utoipa::path(
    post,
    path = "/api/v1/pdf/watermark",
    tag = "PDF Operations",
    request_body(content = WatermarkJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Watermarked successfully", body = ApiResponse)
    )
)]
pub async fn watermark(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("watermark".to_string()), req).await
}

/// Retrieve metadata and page information for a PDF document
#[utoipa::path(
    post,
    path = "/api/v1/pdf/info",
    tag = "PDF Operations",
    request_body(content = InfoJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Info extracted successfully", body = ApiResponse)
    )
)]
pub async fn info(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("metadata".to_string()), req).await
}

/// Compare two PDF documents for visual and textual differences
#[utoipa::path(
    post,
    path = "/api/v1/pdf/compare",
    tag = "PDF Operations",
    request_body(content = CompareJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Comparison completed", body = ApiResponse)
    )
)]
pub async fn compare(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_compare".to_string()), req).await
}

/// Render a single PDF page directly to a PNG image stream
#[utoipa::path(
    post,
    path = "/api/v1/pdf/render-page",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Rendered PNG image binary", content_type = "image/png"),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Render failure")
    )
)]
pub async fn render_page(req: axum::extract::Request) -> axum::response::Response {
    let mut args = serde_json::Map::new();
    let mut temp_files = vec![];
    let output_path = std::env::temp_dir().join(format!("{}.png", uuid::Uuid::new_v4()));

    use axum::extract::FromRequest;
    use axum::response::IntoResponse;
    let mut multipart = match axum::extract::Multipart::from_request(req, &()).await {
        Ok(m) => m,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, "Invalid multipart").into_response(),
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
            let mut temp_file = match tempfile::NamedTempFile::new() {
                Ok(tf) => tf,
                Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to create temp file").into_response(),
            };
            let path = temp_file.path().to_owned();
            use std::io::Write;
            if let Err(_) = temp_file.write_all(&data) {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to write data").into_response();
            };
            args.insert("input".to_string(), serde_json::Value::String(path.to_string_lossy().to_string()));
            temp_files.push(temp_file);
        } else {
            let s = String::from_utf8_lossy(&data).to_string();
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&s) {
                 args.insert(name, val);
            } else {
                 args.insert(name, serde_json::Value::String(s));
            }
        }
    }

    args.insert("output".to_string(), serde_json::Value::String(output_path.to_string_lossy().to_string()));

    let mut request = rmcp::model::CallToolRequestParams::default();
    request.name = "pdf_render".into();
    request.arguments = Some(args);

    match paperpilot_mcp::server::PaperPilotMcpServer::execute_call_tool(request) {
        Ok(_) => {
            if let Ok(bytes) = std::fs::read(&output_path) {
                let _ = std::fs::remove_file(&output_path);
                let mut res = axum::response::Response::new(axum::body::Body::from(bytes));
                res.headers_mut().insert(axum::http::header::CONTENT_TYPE, axum::http::HeaderValue::from_static("image/png"));
                res.into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "File generated but could not be read").into_response()
            }
        },
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message.clone()).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use serde_json::json;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_mcp_exec_handles_invalid_arguments() {
        let req = McpExecRequest {
            tool: "pdf_render".to_string(),
            arguments: Some(json!("not an object")),
        };
        let res = mcp_exec(axum::Json(req)).await;
        assert_eq!(res.into_response().status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_render_page_handles_invalid_multipart() {
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/pdf/render-page")
            .body(Body::empty())
            .unwrap();

        let res = render_page(req).await;
        assert_eq!(res.into_response().status(), axum::http::StatusCode::BAD_REQUEST);
    }
}
