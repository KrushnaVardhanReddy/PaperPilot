#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use fixture::{PageKind::*, pdf};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt;
#[tokio::test]
async fn rest_run_and_history_share_actual_persisted_receipts() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");
    std::fs::write(&input, pdf(&[Text])).unwrap();
    let app = paperpilot_gateway::server::build_app(Arc::new(
        paperpilot_pdf::operations::watch::WatcherManager::default(),
    ))
    .await;
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/workflow/run-json")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"input":input,"output":output,"preset":"preset-legal-discovery"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let value: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    let path = std::path::Path::new(value["receipt_path"].as_str().unwrap());
    let uri = format!(
        "/v1/automation/receipts?directory={}",
        urlencoding::encode(path.parent().unwrap().to_str().unwrap())
    );
    let response = app
        .clone()
        .oneshot(Request::builder().uri(&uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let entries: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    assert_eq!(entries[0]["job_id"], value["report"]["job_id"]);
    let uri = format!(
        "/v1/automation/receipts/{}?directory={}",
        path.file_name().unwrap().to_str().unwrap(),
        urlencoding::encode(path.parent().unwrap().to_str().unwrap())
    );
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        lopdf::Document::load(output)
            .unwrap()
            .extract_text(&[1])
            .unwrap()
            .contains("Original invoice")
    );
}
