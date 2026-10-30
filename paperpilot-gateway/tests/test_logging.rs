use paperpilot_gateway::server::{WatcherRegistry, build_app};
use paperpilot_pdf::operations::watch::shared_watcher_manager;
use reqwest::Client;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::test]
async fn test_gateway_logging_and_request_id() {
    let registry: WatcherRegistry = shared_watcher_manager();
    let app = build_app(registry).await;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let client = Client::new();
    let url = format!("http://{}/health", addr);

    // Test 1: Auto-generated Request-ID header
    let res = client.get(&url).send().await.unwrap();
    assert_eq!(res.status(), 200);
    assert!(res.headers().contains_key("X-Request-ID"));
    let req_id_1 = res.headers().get("X-Request-ID").unwrap().to_str().unwrap().to_string();
    assert!(!req_id_1.is_empty());

    // Test 2: Custom Request-ID header propagation
    let custom_id = "test-req-id-12345";
    let res2 = client
        .get(&url)
        .header("X-Request-ID", custom_id)
        .send().await.unwrap();
    assert_eq!(res2.status(), 200);
    assert_eq!(
        res2.headers().get("X-Request-ID").unwrap().to_str().unwrap(),
        custom_id
    );

    // Test 3: Failed request returns request_id in JSON body and header
    let tool_url = format!("http://{}/api/v1/pdf/tools/pdf_extract_text", addr);
    let res3 = client
        .post(&tool_url)
        .header("Content-Type", "application/json")
        .header("X-Request-ID", "custom-err-id")
        .body(r#"{"input": "nonexistent.pdf", "output": "out.txt"}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(res3.headers().get("X-Request-ID").unwrap().to_str().unwrap(), "custom-err-id");
    let json_body: serde_json::Value = res3.json().await.unwrap();
    assert_eq!(json_body["request_id"].as_str().unwrap(), "custom-err-id");
    assert_eq!(json_body["success"].as_bool().unwrap(), false);
}
