use axum::{
    extract::Query,
    response::{
        IntoResponse, Json,
        sse::{Event, Sse},
    },
};
use futures::stream::Stream;
use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, CallToolResponse};
use serde::Deserialize;
use serde_json::Value;
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt;

#[derive(Deserialize)]
pub struct SessionQuery {
    #[allow(dead_code)]
    session_id: Option<String>,
}

pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let session_id = uuid::Uuid::new_v4().to_string();

    let stream = tokio_stream::iter(vec![Ok(Event::default()
        .event("endpoint")
        .data(format!("?sessionId={}", session_id)))])
    .chain(
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(15)))
            .map(|_| Ok(Event::default().comment("ping"))),
    );

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}

pub async fn messages_handler(
    Query(_query): Query<SessionQuery>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    // For a minimal HTTP adapter, we assume standard jsonrpc payload for CallTool
    if let Some(method) = payload.get("method").and_then(|m| m.as_str()) {
        if method == "tools/call" {
            if let Some(params) = payload.get("params") {
                let name = params
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string();
                let args = params.get("arguments").and_then(|a| a.as_object()).cloned();

                let mut req = CallToolRequestParams::default();
                req.name = name.into();
                req.arguments = args;

                match PaperPilotMcpServer::execute_call_tool(req) {
                    Ok(CallToolResponse::Complete(result)) => {
                        return Json(serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": payload.get("id"),
                            "result": result
                        }));
                    }
                    Ok(_) => {
                        return Json(serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": payload.get("id"),
                            "error": {
                                "code": -32603,
                                "message": "Unknown response type"
                            }
                        }));
                    }
                    Err(e) => {
                        return Json(serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": payload.get("id"),
                            "error": {
                                "code": e.code.0,
                                "message": e.message
                            }
                        }));
                    }
                }
            }
        }
    }

    // For other methods, we mock a response for now to allow clients to initialize
    // A robust adapter would proxy everything to a real Server instance.
    if payload.get("method").and_then(|m| m.as_str()) == Some("initialize") {
        return Json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": payload.get("id"),
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "paperpilot-mcp",
                    "version": "1.0.0"
                }
            }
        }));
    } else if payload.get("method").and_then(|m| m.as_str()) == Some("tools/list") {
        let tools_result = PaperPilotMcpServer::execute_list_tools().unwrap();
        let tools_list: Vec<_> = tools_result
            .tools
            .into_iter()
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": t.input_schema
                })
            })
            .collect();
        return Json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": payload.get("id"),
            "result": {
                "tools": tools_list
            }
        }));
    }

    Json(serde_json::json!({
        "jsonrpc": "2.0",
        "id": payload.get("id"),
        "error": {
            "code": -32601,
            "message": "Method not found"
        }
    }))
}
