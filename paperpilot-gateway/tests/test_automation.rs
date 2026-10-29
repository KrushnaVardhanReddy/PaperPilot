use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use paperpilot_gateway::server::{build_app, WatcherRegistry, WatcherHandle};
use tower::ServiceExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[tokio::test]
async fn test_post_watcher_start() {
    let registry: WatcherRegistry = Arc::new(Mutex::new(HashMap::new()));
    let app = build_app(registry.clone()).await;

    let payload = serde_json::json!({
        "dir": "/tmp/watch_test",
        "recipe": "preset-clean-scan"
    });

    let request = Request::builder()
        .uri("/v1/automation/watchers")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Check registry
    let map = registry.lock().await;
    assert_eq!(map.len(), 1);
}

#[tokio::test]
async fn test_get_watcher_list() {
    let registry: WatcherRegistry = Arc::new(Mutex::new(HashMap::new()));

    // Add a fake watcher
    {
        let mut map = registry.lock().await;
        map.insert("watcher_123".to_string(), WatcherHandle {
            watcher_id: "watcher_123".to_string(),
            dir: std::path::PathBuf::from("/tmp"),
            recipe: std::path::PathBuf::from("fake_recipe"),
            cancel: tokio_util::sync::CancellationToken::new(),
            child_proc: None,
            last_processed_file: None,
            last_processed_at: None,
            total_processed: 0,
        });
    }

    let app = build_app(registry.clone()).await;

    let request = Request::builder()
        .uri("/v1/automation/watchers")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body["watchers"].as_array().unwrap().len(), 1);
    assert_eq!(body["watchers"][0]["watcher_id"], "watcher_123");
}

#[tokio::test]
async fn test_delete_watcher_stop() {
    let registry: WatcherRegistry = Arc::new(Mutex::new(HashMap::new()));

    // Add a fake watcher
    {
        let mut map = registry.lock().await;
        map.insert("watcher_123".to_string(), WatcherHandle {
            watcher_id: "watcher_123".to_string(),
            dir: std::path::PathBuf::from("/tmp"),
            recipe: std::path::PathBuf::from("fake_recipe"),
            cancel: tokio_util::sync::CancellationToken::new(),
            child_proc: None,
            last_processed_file: None,
            last_processed_at: None,
            total_processed: 0,
        });
    }

    let app = build_app(registry.clone()).await;

    let request = Request::builder()
        .uri("/v1/automation/watchers/watcher_123")
        .method("DELETE")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Ensure it's removed
    let map = registry.lock().await;
    assert!(map.is_empty());
}

#[tokio::test]
async fn test_post_automation_run() {
    let registry: WatcherRegistry = Arc::new(Mutex::new(HashMap::new()));
    let app = build_app(registry.clone()).await;

    // Use a small valid PDF byte array for testing
    let mut doc = lopdf::Document::with_version("1.4");
    let pages_id = doc.new_object_id();
    let catalog_id = doc.new_object_id();
    let page_id = doc.new_object_id();

    let mut page_dict = lopdf::Dictionary::new();
    page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
    page_dict.set("Parent", lopdf::Object::Reference(pages_id));
    page_dict.set("MediaBox", lopdf::Object::Array(vec![0.into(), 0.into(), 612.into(), 792.into()]));
    doc.objects.insert(page_id, lopdf::Object::Dictionary(page_dict));

    let mut pages_dict = lopdf::Dictionary::new();
    pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
    pages_dict.set("Kids", lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]));
    pages_dict.set("Count", lopdf::Object::Integer(1));
    doc.objects.insert(pages_id, lopdf::Object::Dictionary(pages_dict));

    let mut catalog_dict = lopdf::Dictionary::new();
    catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
    doc.objects.insert(catalog_id, lopdf::Object::Dictionary(catalog_dict));
    doc.trailer.set("Root", lopdf::Object::Reference(catalog_id));

    let mut pdf_bytes = Vec::new();
    doc.save_to(&mut pdf_bytes).unwrap();

    let boundary = "------------------------boundary123";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"test.pdf\"\r\n\
         Content-Type: application/pdf\r\n\
         \r\n\
         {pdf_data}\r\n\
         --{boundary}\r\n\
         Content-Disposition: form-data; name=\"recipe\"\r\n\
         \r\n\
         preset-clean-scan\r\n\
         --{boundary}--\r\n",
        boundary = boundary,
        pdf_data = unsafe { String::from_utf8_unchecked(pdf_bytes) }
    );

    let request = Request::builder()
        .uri("/v1/automation/run")
        .method("POST")
        .header("Content-Type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body["success"], true);
    assert!(body["output_bytes"].is_string());
    assert!(body["receipt"].is_object());
}
