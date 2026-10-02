use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use paperpilot_gateway::server::build_app;
use tower::ServiceExt;
use serde_json::Value;

#[tokio::test]
async fn test_health_endpoint() {
    let app = build_app().await;

    let request = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert!(json["tools_count"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn test_mcp_exec_tool() {
    let app = build_app().await;

    // We can test pdf_split or something trivial if we had test files,
    // but we can test missing argument errors on mcp_exec easily
    let payload = serde_json::json!({
        "tool": "pdf_metadata",
        "arguments": {
            "input": "nonexistent.pdf"
        }
    });

    let request = Request::builder()
        .uri("/api/v1/pdf/mcp-exec")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // The tool should return an internal server error or explicit json error due to missing file
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["success"], false);
    assert!(json["error"].is_string());
}

#[tokio::test]
async fn test_multipart_and_json_endpoints() {
    let app = build_app().await;

    // Test JSON Endpoint on a multipart route
    let payload = serde_json::json!({
        "input": "tests/e2e_fixtures/secret_999.pdf",
        "output": "/tmp/test.pdf",
        "password": "test"
    });

    let request = Request::builder()
        .uri("/api/v1/pdf/tools/pdf_encrypt")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Just check the parsing didn't fail with BAD_REQUEST
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}
