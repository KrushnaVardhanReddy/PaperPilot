use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use paperpilot_gateway::server::build_app;
use tower::ServiceExt;
use paperpilot_pdf::operations::watch::shared_watcher_manager;

#[tokio::test]
async fn test_gateway_x_zero_trace_header() {
    let app = build_app(shared_watcher_manager()).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/tools")
                .header("X-Zero-Trace", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("X-Zero-Trace-Enforced").unwrap(),
        "true"
    );
}

#[tokio::test]
async fn test_gateway_x_log_level_off_header() {
    let app = build_app(shared_watcher_manager()).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/tools")
                .header("X-Log-Level", "off")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("X-Zero-Trace-Enforced").unwrap(),
        "true"
    );
}

#[tokio::test]
async fn test_gateway_query_param_zero_trace() {
    let app = build_app(shared_watcher_manager()).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/tools?zero_trace=true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("X-Zero-Trace-Enforced").unwrap(),
        "true"
    );
}

#[tokio::test]
async fn test_gateway_normal_request_retains_logging() {
    let app = build_app(shared_watcher_manager()).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/tools")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get("X-Zero-Trace-Enforced").is_none());
    assert!(response.headers().get("X-Request-ID").is_some());
}
