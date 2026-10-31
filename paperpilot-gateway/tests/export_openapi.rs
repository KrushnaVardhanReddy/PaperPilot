use paperpilot_gateway::server::ApiDoc;
use utoipa::OpenApi;

#[test]
fn test_export_openapi_json() {
    let doc = ApiDoc::openapi();
    let json = doc.to_pretty_json().expect("Failed to serialize OpenAPI spec");
    assert!(json.contains("openapi"), "Spec missing openapi field");
    assert!(json.contains("/health"), "Spec missing /health route");
    assert!(
        json.contains("PaperPilot") || json.contains("info"),
        "Spec missing info metadata"
    );

    let _ = std::fs::create_dir_all("target");
    let _ = std::fs::write("target/openapi.json", json);
}
