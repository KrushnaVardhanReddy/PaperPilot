#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::Text, pdf};
use paperpilot_mcp::server::PaperPilotMcpServer;
use paperpilot_pdf::operations::watch::shared_watcher_manager;
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use serde_json::{Value, json};
use std::time::{Duration, Instant};

fn request(name: &str, args: Value) -> CallToolRequestParams {
    let mut r = CallToolRequestParams::default();
    r.name = name.to_string().into();
    r.arguments = args.as_object().cloned();
    r
}
fn call(name: &str, args: Value) -> Value {
    let CallToolResponse::Complete(result) = PaperPilotMcpServer::new(None)
        .execute_call_tool(request(name, args))
        .unwrap()
    else {
        panic!("Expected complete MCP response");
    };
    let ContentBlock::Text(text) = &result.content[0] else {
        panic!("Expected JSON text");
    };
    serde_json::from_str(&text.text).unwrap()
}

#[test]
fn standalone_mcp_starts_a_real_worker_shared_with_the_desktop_runtime() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("input.pdf"), pdf(&[Text])).unwrap();
    let result = call(
        "watcher_start",
        json!({"dir":root.path(),"recipe":"preset-legal-discovery","settle_delay_ms":50}),
    );
    let id = result["watcher_id"].as_str().unwrap();
    let manager = shared_watcher_manager();
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let item = manager.list().into_iter().find(|s| s.id == id).unwrap();
        if item.succeeded == 1 {
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(50));
    }
    let list = call("watcher_list", json!({}));
    let item = list["watchers"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["watcher_id"] == id)
        .unwrap();
    assert_eq!(item["total_processed"], 1);
    let d = lopdf::Document::load(root.path().join("processed/input.pdf")).unwrap();
    assert!(d.extract_text(&[1]).unwrap().contains("Original invoice"));
    assert_eq!(
        call("watcher_stop", json!({"watcher_id":id}))["success"],
        true
    );
    assert!(manager.list().iter().all(|s| s.id != id));
}

#[test]
fn invalid_mcp_watch_request_has_no_phantom_watcher() {
    let root = tempfile::tempdir().unwrap();
    let manager = shared_watcher_manager();
    let result = PaperPilotMcpServer::new(None).execute_call_tool(request(
        "watcher_start",
        json!({"dir":root.path().join("missing"),"recipe":"preset-legal-discovery"}),
    ));
    assert!(result.is_err());
    for args in [
        json!({"dir":root.path(),"recipe":"preset-legal-discovery","output_dir":42}),
        json!({"dir":root.path(),"recipe":"preset-legal-discovery","settle_delay_ms":0}),
    ] {
        assert!(
            PaperPilotMcpServer::new(None)
                .execute_call_tool(request("watcher_start", args))
                .is_err()
        );
    }
    assert!(
        manager
            .list()
            .iter()
            .all(|s| !s.directory.starts_with(root.path()))
    );
}
