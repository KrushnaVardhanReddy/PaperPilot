#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use fixture::{PageKind::Text, pdf};
use serde_json::{Value, json};
use std::{
    fs,
    sync::Arc,
    time::{Duration, Instant},
};
use tower::ServiceExt;
fn post(uri: &str, value: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(value.to_string()))
        .unwrap()
}
#[tokio::test]
async fn rest_recipe_reports_named_output_and_preserves_collision() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("source.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let out = dir.path().join("out");
    let app = paperpilot_gateway::server::build_app(Arc::new(
        paperpilot_pdf::operations::watch::WatcherManager::default(),
    ))
    .await;
    let args = json!({"input":input,"output":out,"preset":"preset-legal-discovery","name_template":"{input_stem}-{index:04}.pdf"});
    let response = app
        .clone()
        .oneshot(post("/api/v1/workflow/run-json", args.clone()))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let value: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    let path = value["report"]["output"].as_str().unwrap();
    assert_eq!(std::path::Path::new(path), out.join("source-0001.pdf"));
    let saved = fs::read(path).unwrap();
    assert!(
        lopdf::Document::load_mem(&saved)
            .unwrap()
            .extract_text(&[1])
            .unwrap()
            .contains("Original invoice")
    );
    let response = app
        .oneshot(post("/api/v1/workflow/run-json", args))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(fs::read(path).unwrap(), saved);
}
#[tokio::test]
async fn rest_watcher_validates_and_applies_custom_naming() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("source.pdf"), pdf(&[Text])).unwrap();
    let manager = Arc::new(paperpilot_pdf::operations::watch::WatcherManager::default());
    let app = paperpilot_gateway::server::build_app(manager.clone()).await;
    let response=app.clone().oneshot(post("/v1/automation/watchers",json!({"dir":dir.path(),"recipe":"preset-legal-discovery","name_template":"../escape.pdf"}))).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(manager.list().is_empty());
    let response=app.oneshot(post("/v1/automation/watchers",json!({"dir":dir.path(),"recipe":"preset-legal-discovery","settle_delay_ms":50,"name_template":"{input_stem}-{index:04}.pdf"}))).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let value: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while manager.list()[0].succeeded == 0 {
        assert!(Instant::now() < deadline);
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    manager
        .remove(value["watcher_id"].as_str().unwrap())
        .unwrap();
    assert!(lopdf::Document::load(dir.path().join("processed/source-0001.pdf")).is_ok());
}
