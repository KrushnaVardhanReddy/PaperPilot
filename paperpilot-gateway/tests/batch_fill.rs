use axum::{Json, body::to_bytes, http::StatusCode};
use paperpilot_gateway::batch_routes::fill;
use serde_json::{Value, json};
use std::{fs, path::PathBuf};
fn template() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../apps/desktop/tests/fixtures/automation/form.pdf")
}
#[tokio::test]
async fn rest_preserves_per_row_results_and_partial_status() {
    let dir = tempfile::tempdir().unwrap();
    let response=fill(Json(serde_json::from_value(json!({"template":template(),"output_dir":dir.path(),"records":[{"customer":"Ada"},false],"keep_editable":true})).unwrap())).await;
    assert_eq!(response.status(), StatusCode::MULTI_STATUS);
    let r: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    assert_eq!(r["failed"], 1);
    assert_eq!(r["succeeded"], 1);
    let bytes = fs::read(r["results"][0]["output_path"].as_str().unwrap()).unwrap();
    assert_eq!(
        paperpilot_pdf::operations::form_fill::read_fields(
            &lopdf::Document::load_mem(&bytes).unwrap()
        )
        .unwrap()["customer"],
        "Ada"
    );
    assert!(r["results"][1]["error"].is_string());
}
#[tokio::test]
async fn rest_file_input_and_invalid_request_status() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("rows.json");
    fs::write(&data, r#"[{"customer":"Grace"}]"#).unwrap();
    let r = fill(Json(
        serde_json::from_value(
            json!({"template":template(),"output_dir":dir.path().join("out"),"data":data}),
        )
        .unwrap(),
    ))
    .await;
    assert_eq!(r.status(), StatusCode::OK);
    let r = fill(Json(
        serde_json::from_value(
            json!({"template":template(),"output_dir":dir.path().join("bad"),"records":[]}),
        )
        .unwrap(),
    ))
    .await;
    assert_eq!(r.status(), StatusCode::BAD_REQUEST);
    assert!(!dir.path().join("bad").exists());
}
