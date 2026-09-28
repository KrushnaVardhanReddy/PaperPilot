use lopdf::{Document, Object, Stream, dictionary};
use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::CallToolRequestParams;
use rmcp::model::CallToolResponse;
use serde_json::Value;
use std::path::Path;

fn create_test_pdf(path: &Path) {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });

    let content = Stream::new(
        dictionary! {},
        b"BT /F1 12 Tf 10 10 Td (Test PDF Content) Tj ET".to_vec(),
    );
    let content_id = doc.add_object(content);
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    });

    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        }),
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.save(path).unwrap();
}

#[tokio::test]
async fn test_pdf_merge_e2e() {
    let _server = PaperPilotMcpServer::new();
    let temp_dir = tempfile::tempdir().unwrap();
    let input1 = temp_dir.path().join("simple.pdf");
    let input2 = temp_dir.path().join("multi_page.pdf");
    create_test_pdf(&input1);
    create_test_pdf(&input2);

    let output_path = temp_dir.path().join("merged.pdf");

    let mut args = serde_json::Map::new();
    args.insert(
        "inputs".to_string(),
        Value::Array(vec![
            Value::String(input1.to_string_lossy().into_owned()),
            Value::String(input2.to_string_lossy().into_owned()),
        ]),
    );
    args.insert(
        "output".to_string(),
        Value::String(output_path.to_string_lossy().into_owned()),
    );

    let mut req = CallToolRequestParams::default();
    req.name = "pdf_merge".into();
    req.arguments = Some(args);

    let res = PaperPilotMcpServer::execute_call_tool(req).unwrap();

    if let CallToolResponse::Complete(result) = res {
        assert!(
            !result.is_error.unwrap_or(false),
            "Tool execution returned an error"
        );
        let content_str = &result.content[0].as_text().unwrap().text;
        let json_res: serde_json::Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(json_res.get("success").unwrap().as_bool(), Some(true));
    } else {
        panic!("Expected CallToolResponse::Complete");
    }

    assert!(output_path.exists());
}

#[tokio::test]
async fn test_pdf_extract_text_e2e() {
    let _server = PaperPilotMcpServer::new();
    let temp_dir = tempfile::tempdir().unwrap();
    let input = temp_dir.path().join("simple.pdf");
    create_test_pdf(&input);
    let output_path = temp_dir.path().join("out.txt");

    let mut args = serde_json::Map::new();
    args.insert(
        "input".to_string(),
        Value::String(input.to_string_lossy().into_owned()),
    );
    args.insert(
        "output".to_string(),
        Value::String(output_path.to_string_lossy().into_owned()),
    );

    let mut req = CallToolRequestParams::default();
    req.name = "pdf_extract_text".into();
    req.arguments = Some(args);

    let res = PaperPilotMcpServer::execute_call_tool(req).unwrap();

    if let CallToolResponse::Complete(result) = res {
        assert!(
            !result.is_error.unwrap_or(false),
            "Tool execution returned an error"
        );
        let content_str = &result.content[0].as_text().unwrap().text;
        let json_res: serde_json::Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(json_res.get("success").unwrap().as_bool(), Some(true));
    } else {
        panic!("Expected CallToolResponse::Complete");
    }

    assert!(output_path.exists());
}

#[tokio::test]
async fn test_pdf_metadata_e2e() {
    let _server = PaperPilotMcpServer::new();
    let temp_dir = tempfile::tempdir().unwrap();
    let input = temp_dir.path().join("simple.pdf");
    create_test_pdf(&input);

    let mut args = serde_json::Map::new();
    args.insert(
        "input".to_string(),
        Value::String(input.to_string_lossy().into_owned()),
    );

    let mut req = CallToolRequestParams::default();
    req.name = "pdf_metadata".into();
    req.arguments = Some(args);

    let res = PaperPilotMcpServer::execute_call_tool(req).unwrap();

    if let CallToolResponse::Complete(result) = res {
        assert!(
            !result.is_error.unwrap_or(false),
            "Tool execution returned an error"
        );
        let content_str = &result.content[0].as_text().unwrap().text;
        let json_res: serde_json::Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(json_res.get("success").unwrap().as_bool(), Some(true));
    } else {
        panic!("Expected CallToolResponse::Complete");
    }
}
