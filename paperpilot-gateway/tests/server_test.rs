use axum::{
    body::Body,
    http::{Request, StatusCode},
};
async fn build_app() -> axum::Router {
    paperpilot_gateway::server::build_app(std::sync::Arc::new(
        paperpilot_pdf::operations::watch::WatcherManager::default(),
    ))
    .await
}
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_endpoint() {
    let app = build_app().await;

    let request = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert!(json["tools_count"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn test_mcp_exec_tool() {
    let app = build_app().await;

    // We can test pdf_split or something trivial if we had test files,
    // but we can test missing argument errors on mcp_exec easily
    let payload = serde_json::json!({
        "tool": "pdf_metadata",
        "arguments": {
            "input": "nonexistent.pdf"
        }
    });

    let request = Request::builder()
        .uri("/api/v1/pdf/mcp-exec")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // The tool should return an internal server error or explicit json error due to missing file
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["success"], false);
    assert!(json["error"].is_string());
}

#[tokio::test]
async fn test_multipart_and_json_endpoints() {
    let app = build_app().await;

    // Test JSON Endpoint on a multipart route
    let payload = serde_json::json!({
        "input": "tests/e2e_fixtures/secret_999.pdf",
        "output": "/tmp/test.pdf",
        "password": "test"
    });

    let request = Request::builder()
        .uri("/api/v1/pdf/tools/pdf_encrypt")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Just check the parsing didn't fail with BAD_REQUEST
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_swagger_ui_endpoint() {
    let app = build_app().await;

    let request = Request::builder()
        .uri("/api-docs/openapi.json")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["info"]["title"], "PaperPilot Gateway API & MCP Server");
    assert!(json["paths"]["/health"].is_object());
    assert!(json["paths"]["/api/v1/pdf/mcp-exec"].is_object());
    for path in ["/tika", "/tika/text", "/tika/json/md", "/rmeta/text"] {
        assert_eq!(
            json["paths"][path]["put"]["requestBody"]["content"]["application/pdf"]["schema"]["$ref"],
            "#/components/schemas/TikaPdfBody"
        );
    }
    assert_eq!(
        json["components"]["schemas"]["TikaPdfBody"]["type"],
        "string"
    );
    assert_eq!(
        json["components"]["schemas"]["TikaPdfBody"]["format"],
        "binary"
    );
    assert!(json["paths"]["/api/v1/tika/rmeta/text"]["post"].is_object());

    // Export static docs/api/openapi.json
    let _ = std::fs::create_dir_all("../docs/api");
    let _ = std::fs::write(
        "../docs/api/openapi.json",
        serde_json::to_string_pretty(&json).unwrap(),
    );
    let _ = std::fs::create_dir_all("docs/api");
    let _ = std::fs::write(
        "docs/api/openapi.json",
        serde_json::to_string_pretty(&json).unwrap(),
    );
}

/// A real, serialized two-page PDF with selectable text and a ruled table.
fn ingestion_pdf() -> Vec<u8> {
    use lopdf::{Document, Object, Stream, dictionary};
    let mut pdf = Document::with_version("1.7");
    let pages_id = pdf.new_object_id();
    let font_id = pdf.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica",
    });
    let resources_id = pdf.add_object(dictionary! { "Font" => dictionary! { "F1" => font_id } });
    let mut pages = Vec::new();
    for content in [
        "BT /F1 12 Tf 72 740 Td (PaperPilot local ingestion) Tj ET\n\
         72 640 348 60 re S 260 640 m 260 700 l S 72 680 m 420 680 l S 72 660 m 420 660 l S\n\
         BT /F1 12 Tf 82 686 Td (Item) Tj ET\n\
         BT /F1 12 Tf 270 686 Td (Amount) Tj ET\n\
         BT /F1 12 Tf 82 666 Td (Invoice) Tj ET\n\
         BT /F1 12 Tf 270 666 Td (125) Tj ET\n\
         BT /F1 12 Tf 82 646 Td (Tax) Tj ET\n\
         BT /F1 12 Tf 270 646 Td (25) Tj ET",
        "BT /F1 12 Tf 72 720 Td (Second page citation) Tj ET",
    ] {
        let content_id = pdf.add_object(Stream::new(dictionary! {}, content.as_bytes().to_vec()));
        let page_id = pdf.add_object(dictionary! {
            "Type" => "Page", "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Resources" => resources_id, "Contents" => content_id,
        });
        pages.push(Object::Reference(page_id));
    }
    pdf.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages", "Kids" => pages, "Count" => 2,
        }),
    );
    let catalog_id = pdf.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    pdf.trailer.set("Root", catalog_id);
    let mut bytes = Vec::new();
    pdf.save_to(&mut bytes).unwrap();
    bytes
}

#[tokio::test]
async fn test_open_webui_tika_loaders() {
    let app = build_app().await;
    for (path, content_key) in [
        ("/tika/text", "X-TIKA:content"),
        ("/tika/json/md", "tk:content"),
    ] {
        // Mirrors Open-WebUI TikaLoader.load(): PUT raw bytes, no Accept header,
        // followed by response.json().get(content_key).
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(path)
                    .header("content-type", "application/pdf")
                    .body(Body::from(ingestion_pdf()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert_eq!(response.headers()["content-type"], "application/json");
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let metadata: Value = serde_json::from_slice(&body).unwrap();
        assert!(metadata.is_object());
        assert_eq!(metadata["Content-Type"], "application/pdf");
        assert_eq!(metadata["xmpTPg:NPages"], "2");
        let text = metadata[content_key].as_str().unwrap();
        for expected in [
            "PaperPilot local ingestion",
            "Second page citation",
            "Item",
            "Amount",
            "Invoice",
            "125",
        ] {
            assert!(text.contains(expected), "Missing {expected}: {text}");
        }
        assert!(text.find("PaperPilot").unwrap() < text.find("Second page").unwrap());
        assert!(
            text.contains('|'),
            "Expected detected Markdown table: {text}"
        );
    }
}

#[tokio::test]
async fn test_tika_recursive_metadata_routes() {
    let app = build_app().await;
    for (method, path) in [
        ("PUT", "/rmeta/text"),
        ("POST", "/rmeta/text"),
        ("PUT", "/tika/rmeta/text"),
        ("POST", "/api/v1/tika/rmeta/text"),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(path)
                    .header("content-type", "application/octet-stream")
                    .body(Body::from(ingestion_pdf()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{method} {path}");
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let metadata: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(metadata.as_array().unwrap().len(), 1);
        assert!(
            metadata[0]["X-TIKA:content"]
                .as_str()
                .unwrap()
                .contains("Second page citation")
        );
    }
}

#[tokio::test]
async fn test_tika_text_and_json_negotiation() {
    let app = build_app().await;
    for accept in ["text/plain", "application/json"] {
        // No Content-Type hint: detection still uses actual PDF bytes.
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/tika")
                    .header("accept", accept)
                    .body(Body::from(ingestion_pdf()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response.headers()["content-type"]
                .to_str()
                .unwrap()
                .starts_with(accept)
        );
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        if accept == "application/json" {
            let metadata: Value = serde_json::from_slice(&body).unwrap();
            assert!(
                metadata["X-TIKA:content"]
                    .as_str()
                    .unwrap()
                    .contains("PaperPilot")
            );
        } else {
            assert!(
                String::from_utf8(body.to_vec())
                    .unwrap()
                    .contains("PaperPilot")
            );
        }
    }
}

#[tokio::test]
async fn test_tika_rejects_invalid_uploads() {
    let app = build_app().await;
    for (mime, bytes, expected) in [
        ("application/pdf", vec![], StatusCode::BAD_REQUEST),
        (
            "text/plain",
            b"hello".to_vec(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
        ),
        (
            "application/pdf",
            b"not a PDF".to_vec(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
        ),
        (
            "application/pdf",
            b"%PDF-1.7\ncorrupt".to_vec(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            "multipart/form-data; boundary=test",
            ingestion_pdf(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
        ),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/tika/text")
                    .header("content-type", mime)
                    .body(Body::from(bytes))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), expected, "{mime}");
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let error: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(error["success"], false);
        assert!(error["error"].is_string());
    }
    // A rejected upload must not break later requests.
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_tika_upload_respects_body_limit() {
    let app = build_app().await;
    let mut bytes = ingestion_pdf();
    bytes.resize(100 * 1024 * 1024 + 1, b' ');
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/tika/text")
                .header("content-type", "application/pdf")
                .body(Body::from(bytes))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_tika_rejects_password_protected_pdf() {
    use paperpilot_core::traits::PdfOperation;
    use paperpilot_pdf::{document::LopdfDocument, operations::encrypt::EncryptOperation};
    let mut document =
        LopdfDocument::from_inner(lopdf::Document::load_mem(&ingestion_pdf()).unwrap());
    EncryptOperation {
        user_password: Some("user-password".to_owned()),
        owner_password: Some("owner-password".to_owned()),
    }
    .execute(&mut document)
    .unwrap();
    let mut bytes = Vec::new();
    document.inner.save_to(&mut bytes).unwrap();
    let response = build_app()
        .await
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/tika/text")
                .header("content-type", "application/pdf")
                .body(Body::from(bytes))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_cors_origin_restriction() {
    let app = build_app().await;

    // Test request from untrusted external origin
    let request_untrusted = Request::builder()
        .uri("/health")
        .method("GET")
        .header("origin", "https://malicious-website.com")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request_untrusted).await.unwrap();
    // Untrusted origin should NOT get an Access-Control-Allow-Origin header matching its origin
    assert_ne!(
        response
            .headers()
            .get("access-control-allow-origin")
            .and_then(|h| h.to_str().ok()),
        Some("https://malicious-website.com")
    );

    // Test request from trusted local origin (Tauri app)
    let request_trusted = Request::builder()
        .uri("/health")
        .method("GET")
        .header("origin", "tauri://localhost")
        .body(Body::empty())
        .unwrap();

    let response_trusted = app.oneshot(request_trusted).await.unwrap();
    assert_eq!(
        response_trusted
            .headers()
            .get("access-control-allow-origin")
            .and_then(|h| h.to_str().ok()),
        Some("tauri://localhost")
    );
}
