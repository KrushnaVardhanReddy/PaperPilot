//! Security Dynamic Application Security Testing (DAST) & Fuzzing Harness
//! Validates body limit overflow, security headers, malformed PDF handling,
//! path traversal protection, and zero-panic resilience against adversarial inputs.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

async fn build_app() -> axum::Router {
    paperpilot_gateway::server::build_app(std::sync::Arc::new(
        paperpilot_pdf::operations::watch::WatcherManager::default(),
    ))
    .await
}

fn sample_minimal_pdf() -> Vec<u8> {
    b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n\
2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n\
3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\n\
xref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \n\
trailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n190\n%%EOF\n"
        .to_vec()
}

#[tokio::test]
async fn test_dast_security_headers_enforced() {
    let app = build_app().await;

    let req = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let headers = res.headers();
    assert_eq!(
        headers.get("x-content-type-options").and_then(|v| v.to_str().ok()),
        Some("nosniff"),
        "Missing X-Content-Type-Options: nosniff"
    );
    assert_eq!(
        headers.get("x-frame-options").and_then(|v| v.to_str().ok()),
        Some("DENY"),
        "Missing X-Frame-Options: DENY"
    );
    assert_eq!(
        headers.get("referrer-policy").and_then(|v| v.to_str().ok()),
        Some("no-referrer"),
        "Missing Referrer-Policy: no-referrer"
    );
    assert_eq!(
        headers.get("cross-origin-resource-policy").and_then(|v| v.to_str().ok()),
        Some("same-origin"),
        "Missing Cross-Origin-Resource-Policy: same-origin"
    );
}

#[tokio::test]
async fn test_dast_body_limit_overflow_protection() {
    let app = build_app().await;

    // Craft a payload that exceeds 100 MB body limit
    let mut oversized = sample_minimal_pdf();
    oversized.resize(100 * 1024 * 1024 + 1024, b'A');

    let req = Request::builder()
        .uri("/tika")
        .method("PUT")
        .header("content-type", "application/pdf")
        .body(Body::from(oversized))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(
        res.status(),
        StatusCode::PAYLOAD_TOO_LARGE,
        "Server must reject bodies >100MB with 413 Payload Too Large"
    );
}

#[tokio::test]
async fn test_dast_malformed_pdf_fuzz_matrix() {
    let app = build_app().await;

    let malformed_cases: Vec<(&str, Vec<u8>)> = vec![
        ("empty_payload", b"".to_vec()),
        ("corrupted_header", b"%PDF".to_vec()),
        ("invalid_magic_bytes", b"NOT_A_PDF_DOCUMENT_12345".to_vec()),
        ("high_byte_binary_noise", vec![0xFF, 0xFE, 0x00, 0x1F, 0x88, 0x99, 0xAA, 0xBB]),
        (
            "truncated_xref",
            b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\ntrailer\n<< /Root 1 0 R >>\nstartxref\n99999\n%%EOF".to_vec(),
        ),
        (
            "declared_stream_bomb",
            b"%PDF-1.4\n1 0 obj\n<< /Length 1073741824 >>\nstream\n12345\nendstream\nendobj\ntrailer\n<<>>\n%%EOF".to_vec(),
        ),
    ];

    for (name, payload) in malformed_cases {
        let app = app.clone();
        let req = Request::builder()
            .uri("/tika/text")
            .method("PUT")
            .header("content-type", "application/pdf")
            .body(Body::from(payload))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        // Malformed inputs must yield 4xx client errors, NEVER crash or return 500
        assert!(
            res.status().is_client_error(),
            "Malformed test case '{}' returned unexpected status: {}",
            name,
            res.status()
        );
    }
}

#[tokio::test]
async fn test_dast_malformed_multipart_resilience() {
    let app = build_app().await;

    // Send malformed multipart payload missing boundary to /v1/automation/run
    let req = Request::builder()
        .uri("/v1/automation/run")
        .method("POST")
        .header("content-type", "multipart/form-data; boundary=invalid_boundary")
        .body(Body::from(b"--invalid_boundary\r\nContent-Disposition: form-data\r\n\r\ncorrupted data\r\n".to_vec()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    // Must handle gracefully without panic
    assert!(
        res.status().is_client_error(),
        "Malformed multipart must return client error, got: {}",
        res.status()
    );
}

#[tokio::test]
async fn test_dast_path_traversal_blocked_on_receipts() {
    let app = build_app().await;

    let traversal_ids = vec![
        "..%2f..%2fetc%2fpasswd",
        "nested%2fpath%2frun-123.json",
        "run-123.json%00malicious",
        "absolute%2fetc%2fshadow",
        "nonexistent.json",
    ];

    for id in traversal_ids {
        let app = app.clone();
        let req = Request::builder()
            .uri(format!("/v1/automation/receipts/{}?directory=/tmp", id))
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert!(
            res.status().is_client_error(),
            "Path traversal '{}' was not blocked! Status: {}",
            id,
            res.status()
        );
    }
}

#[tokio::test]
async fn test_dast_zero_trace_suppression_and_header_audit() {
    let app = build_app().await;

    let req = Request::builder()
        .uri("/health")
        .method("GET")
        .header("x-zero-trace", "true")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers()
            .get("x-zero-trace-enforced")
            .and_then(|v| v.to_str().ok()),
        Some("true"),
        "x-zero-trace-enforced header must be set when requested"
    );
    // Request ID should be suppressed when zero-trace is enforced
    assert!(
        res.headers().get("X-Request-ID").is_none(),
        "X-Request-ID must be omitted during zero-trace"
    );
}
