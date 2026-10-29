use paperpilot_mcp::server::{PaperPilotMcpServer, WatcherRegistryTrait};
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

struct DummyRegistry {
    pub watchers: Mutex<HashMap<String, String>>,
}

impl WatcherRegistryTrait for DummyRegistry {
    fn add_watcher(&self, id: String, _dir: String, _recipe: String, _child: Option<std::process::Child>) {
        self.watchers.lock().unwrap().insert(id.clone(), id);
    }

    fn remove_watcher(&self, id: &str) -> bool {
        self.watchers.lock().unwrap().remove(id).is_some()
    }

    fn list_watchers(&self) -> Vec<serde_json::Value> {
        let map = self.watchers.lock().unwrap();
        map.keys().map(|k| serde_json::json!({"watcher_id": k})).collect()
    }
}

#[tokio::test]
async fn test_mcp_watcher_start_stop() {
    let registry = Arc::new(DummyRegistry { watchers: Mutex::new(HashMap::new()) });
    let server = PaperPilotMcpServer::new(Some(registry.clone()));

    let req: CallToolRequestParams = serde_json::from_value(serde_json::json!({
        "name": "watcher_start",
        "arguments": {
            "dir": "/tmp",
            "recipe": "preset-clean-scan"
        }
    })).unwrap();

    let res = server.execute_call_tool(req).unwrap();
    let watcher_id = match res {
        CallToolResponse::Complete(result) => {
            if let Some(ContentBlock::Text(t)) = result.content.first() {
                let parsed: serde_json::Value = serde_json::from_str(&t.text).unwrap();
                assert_eq!(parsed["status"], "watching");
                parsed["watcher_id"].as_str().unwrap().to_string()
            } else {
                panic!("No text content");
            }
        },
        _ => panic!("Expected complete"),
    };

    // Stop
    let stop_req: CallToolRequestParams = serde_json::from_value(serde_json::json!({
        "name": "watcher_stop",
        "arguments": {
            "watcher_id": watcher_id
        }
    })).unwrap();

    let stop_res = server.execute_call_tool(stop_req).unwrap();
    match stop_res {
        CallToolResponse::Complete(result) => {
            if let Some(ContentBlock::Text(t)) = result.content.first() {
                let parsed: serde_json::Value = serde_json::from_str(&t.text).unwrap();
                assert_eq!(parsed["success"], true);
            }
        },
        _ => panic!("Expected complete"),
    }
}

#[tokio::test]
async fn test_mcp_watcher_list() {
    let registry = Arc::new(DummyRegistry { watchers: Mutex::new(HashMap::new()) });
    let server = PaperPilotMcpServer::new(Some(registry.clone()));

    registry.watchers.lock().unwrap().insert("watcher_1".into(), "1".into());
    registry.watchers.lock().unwrap().insert("watcher_2".into(), "2".into());

    let req: CallToolRequestParams = serde_json::from_value(serde_json::json!({
        "name": "watcher_list"
    })).unwrap();

    let res = server.execute_call_tool(req).unwrap();
    match res {
        CallToolResponse::Complete(result) => {
            if let Some(ContentBlock::Text(t)) = result.content.first() {
                let parsed: serde_json::Value = serde_json::from_str(&t.text).unwrap();
                assert_eq!(parsed["watchers"].as_array().unwrap().len(), 2);
            }
        },
        _ => panic!("Expected complete"),
    }
}

#[tokio::test]
async fn test_mcp_recipe_run() {
    let server = PaperPilotMcpServer::new(None);

    let temp_dir = tempfile::tempdir().unwrap();
    let fixture_path = temp_dir.path().join("input.pdf");

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
    doc.save(&fixture_path).unwrap();

    let out_path = temp_dir.path().join("out.pdf");

    let req: CallToolRequestParams = serde_json::from_value(serde_json::json!({
        "name": "recipe_run",
        "arguments": {
            "input": fixture_path.to_string_lossy().to_string(),
            "recipe": "preset-clean-scan",
            "output": out_path.to_string_lossy().to_string()
        }
    })).unwrap();

    let res = server.execute_call_tool(req).unwrap();
    match res {
        CallToolResponse::Complete(result) => {
            if let Some(ContentBlock::Text(t)) = result.content.first() {
                let parsed: serde_json::Value = serde_json::from_str(&t.text).unwrap();
                assert_eq!(parsed["success"], true);
                assert!(out_path.exists());
            }
        },
        _ => panic!("Expected complete"),
    }
}
