#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use fixture::{PageKind::Text, pdf};
use paperpilot_gateway::server::{WatcherRegistry, build_app};
use paperpilot_pdf::operations::watch::WatcherManager;
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tower::ServiceExt;

#[tokio::test]
async fn start_list_and_delete_watchers_report_real_filesystem_work() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("invoice.pdf"), pdf(&[Text])).unwrap();
    let registry: WatcherRegistry = Arc::new(WatcherManager::default());
    let app = build_app(registry.clone()).await;
    let request = Request::builder().uri("/v1/automation/watchers").method("POST").header("Content-Type", "application/json")
        .body(Body::from(serde_json::json!({"dir":root.path(),"recipe":"preset-legal-discovery","settle_delay_ms":50}).to_string())).unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let data: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    let id = data["watcher_id"].as_str().unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while registry.list()[0].succeeded == 0 {
        assert!(
            Instant::now() < deadline,
            "HTTP-started watcher must actually process the file"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let document = lopdf::Document::load(root.path().join("processed/invoice.pdf")).unwrap();
    assert_eq!(document.get_pages().len(), 1);
    assert!(
        document
            .extract_text(&[1])
            .unwrap()
            .contains("Original invoice")
    );
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/automation/watchers")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let list: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    assert_eq!(list["watchers"][0]["total_processed"], 1);
    assert_eq!(list["watchers"][0]["succeeded"], 1);
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/v1/automation/watchers/{id}"))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert!(registry.list().is_empty());
}

#[tokio::test]
async fn invalid_watch_paths_and_recipes_do_not_return_false_start_success() {
    let root = tempfile::tempdir().unwrap();
    let registry = Arc::new(WatcherManager::default());
    let app = build_app(registry.clone()).await;
    for body in [
        serde_json::json!({"dir":root.path().join("missing"),"recipe":"preset-legal-discovery"}),
        serde_json::json!({"dir":root.path(),"recipe":"missing-recipe"}),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/automation/watchers")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(registry.list().is_empty());
    }
}

#[tokio::test]
async fn multipart_recipe_run_accepts_real_binary_pdf_without_lossy_utf8_conversion() {
    use base64::Engine;
    let app = build_app(Arc::new(WatcherManager::default())).await;
    let boundary = "paperpilot-automation-fixture";
    let mut body = format!("--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"input.pdf\"\r\nContent-Type: application/pdf\r\n\r\n").into_bytes();
    body.extend(pdf(&[Text]));
    body.extend(format!("\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"recipe\"\r\n\r\npreset-legal-discovery\r\n--{boundary}--\r\n").into_bytes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/automation/run")
                .method("POST")
                .header(
                    "Content-Type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let result: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    let output = base64::prelude::BASE64_STANDARD
        .decode(result["output_bytes"].as_str().unwrap())
        .unwrap();
    let doc = lopdf::Document::load_mem(&output).unwrap();
    assert_eq!(doc.get_pages().len(), 1);
    assert!(doc.extract_text(&[1]).unwrap().contains("Original invoice"));
    assert_eq!(result["receipt"]["steps_completed"], 3);
}
