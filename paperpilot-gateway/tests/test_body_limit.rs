use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use paperpilot_gateway::server::build_app;
use paperpilot_pdf::operations::watch::WatcherManager;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

static SERIAL_TEST_LOCK: Mutex<()> = Mutex::new(());

fn sample_pdf() -> Vec<u8> {
    b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n\
2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n\
3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\n\
xref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \n\
trailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n190\n%%EOF\n"
        .to_vec()
}

#[tokio::test]
async fn test_custom_body_limit_enforced() {
    let _guard = SERIAL_TEST_LOCK.lock().unwrap();

    // Set body limit to 1 MB
    unsafe {
        std::env::set_var("PAPERPILOT_MAX_BODY_LIMIT_MB", "1");
    }

    let app = build_app(Arc::new(WatcherManager::default())).await;

    // Send payload of 1.5 MB -> should be rejected with 413 Payload Too Large
    let mut oversized = sample_pdf();
    oversized.resize((1.5 * 1024.0 * 1024.0) as usize, b'X');

    let req = Request::builder()
        .uri("/tika/text")
        .method("PUT")
        .header("content-type", "application/pdf")
        .body(Body::from(oversized))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(
        res.status(),
        StatusCode::PAYLOAD_TOO_LARGE,
        "Custom 1MB limit should reject 1.5MB body"
    );

    unsafe {
        std::env::remove_var("PAPERPILOT_MAX_BODY_LIMIT_MB");
    }
}

#[tokio::test]
async fn test_unlimited_body_limit_with_zero() {
    let _guard = SERIAL_TEST_LOCK.lock().unwrap();

    // Set body limit to 0 (unlimited)
    unsafe {
        std::env::set_var("PAPERPILOT_MAX_BODY_LIMIT_MB", "0");
    }

    let app = build_app(Arc::new(WatcherManager::default())).await;

    // Send payload of 2 MB to /tika/text
    let mut payload = sample_pdf();
    payload.resize(2 * 1024 * 1024, b' ');

    let req = Request::builder()
        .uri("/tika/text")
        .method("PUT")
        .header("content-type", "application/pdf")
        .body(Body::from(payload))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    // It should not be 413 PAYLOAD_TOO_LARGE
    assert_ne!(
        res.status(),
        StatusCode::PAYLOAD_TOO_LARGE,
        "Body limit 0 should allow requests without Payload Too Large rejection"
    );

    unsafe {
        std::env::remove_var("PAPERPILOT_MAX_BODY_LIMIT_MB");
    }
}

#[tokio::test]
async fn test_unlimited_body_limit_with_string_unlimited() {
    let _guard = SERIAL_TEST_LOCK.lock().unwrap();

    // Set body limit to "unlimited"
    unsafe {
        std::env::set_var("PAPERPILOT_MAX_BODY_LIMIT_MB", "unlimited");
    }

    let app = build_app(Arc::new(WatcherManager::default())).await;

    let mut payload = sample_pdf();
    payload.resize(2 * 1024 * 1024, b' ');

    let req = Request::builder()
        .uri("/tika/text")
        .method("PUT")
        .header("content-type", "application/pdf")
        .body(Body::from(payload))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_ne!(
        res.status(),
        StatusCode::PAYLOAD_TOO_LARGE,
        "PAPERPILOT_MAX_BODY_LIMIT_MB=unlimited should allow requests without Payload Too Large rejection"
    );

    unsafe {
        std::env::remove_var("PAPERPILOT_MAX_BODY_LIMIT_MB");
    }
}
