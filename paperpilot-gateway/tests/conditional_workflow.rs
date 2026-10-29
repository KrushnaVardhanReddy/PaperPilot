#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use axum::{Json, body::to_bytes, http::StatusCode, response::IntoResponse};
use fixture::{PageKind::*, pdf};
use paperpilot_gateway::handlers::{WorkflowRunJsonRequest, workflow_run_json};
use serde_json::{Value, json};
use std::fs;

#[tokio::test]
async fn gateway_json_recipe_reports_conditions_and_returns_real_output() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let output = dir.path().join("out.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let r = workflow_run_json(Json(WorkflowRunJsonRequest {
        name_template: None, input: input.to_string_lossy().into(), output: output.to_string_lossy().into(), preset: None,
        recipe: Some(json!({"id":"api","name":"API","steps":[{"id":"skip","operationId":"rotate","condition":{"if_page_count_gt":50}},{"id":"finish","operationId":"watermark","params":{"text":"VERIFIED"}}]})),
    })).await.into_response();
    assert_eq!(r.status(), StatusCode::OK);
    let response: Value =
        serde_json::from_slice(&to_bytes(r.into_body(), 1024 * 1024).await.unwrap()).unwrap();
    assert_eq!(response["report"]["steps_skipped"], 1);
    let doc = lopdf::Document::load(output).unwrap();
    let text = doc.extract_text(&[1]).unwrap();
    assert!(text.contains("VERIFIED") && text.contains("Original invoice"));
}

#[tokio::test]
async fn gateway_toml_failure_returns_report_without_overwriting_output() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let output = dir.path().join("out.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let previous = pdf(&[Image]);
    fs::write(&output, &previous).unwrap();
    let r = workflow_run_json(Json(WorkflowRunJsonRequest {
        name_template: None, input: input.to_string_lossy().into(), output: output.to_string_lossy().into(), preset: None,
        recipe: Some(json!("name='Fail'\n[[stage]]\nid='gate'\nop='rotate'\ncondition={if_page_count_gt=50}\non_no_match='error'")),
    })).await.into_response();
    assert_eq!(r.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let response: Value =
        serde_json::from_slice(&to_bytes(r.into_body(), 1024 * 1024).await.unwrap()).unwrap();
    assert_eq!(response["report"]["stages"][0]["status"], "failed");
    assert_eq!(fs::read(output).unwrap(), previous);
}
