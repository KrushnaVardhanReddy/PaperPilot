#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::*, pdf};
use serde_json::Value;
use std::{fs, process::Command};
#[test]
fn separate_cli_processes_can_query_completed_and_failed_workflows() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");
    let recipe = dir.path().join("flow.ppflow");
    fs::write(&input, pdf(&[Text])).unwrap();
    fs::write(&recipe, "name='CLI'\n[[stage]]\nid='r'\nop='rotate'").unwrap();
    let command = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .arg("run")
        .arg(&recipe)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .arg("--json")
        .output()
        .unwrap();
    assert!(command.status.success());
    let first: Value = serde_json::from_slice(&command.stdout).unwrap();
    let path = std::path::Path::new(first["workflow"]["receipt_path"].as_str().unwrap());
    let command = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .args(["workflow", "history", "--directory"])
        .arg(path.parent().unwrap())
        .arg("--json")
        .output()
        .unwrap();
    assert!(command.status.success());
    let history: Value = serde_json::from_slice(&command.stdout).unwrap();
    assert_eq!(
        history["workflow"][0]["job_id"],
        first["workflow"]["job_id"]
    );
    fs::write(&recipe,"name='Fail'\n[[stage]]\nid='gate'\nop='rotate'\ncondition={if_page_count_gt=50}\non_no_match='error'").unwrap();
    let command = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .arg("run")
        .arg(&recipe)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .arg("--json")
        .output()
        .unwrap();
    assert!(!command.status.success());
    let failure: Value = serde_json::from_slice(&command.stdout).unwrap();
    let p = std::path::Path::new(failure["workflow"]["receipt_path"].as_str().unwrap());
    let command = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .args(["workflow", "history", "--directory"])
        .arg(p.parent().unwrap())
        .arg("--id")
        .arg(p.file_name().unwrap())
        .arg("--json")
        .output()
        .unwrap();
    assert!(command.status.success());
    let history: Value = serde_json::from_slice(&command.stdout).unwrap();
    assert_eq!(history["workflow"]["failed_stage"], "gate");
    assert_eq!(lopdf::Document::load(output).unwrap().get_pages().len(), 1);
}

#[test]
fn existing_batch_receipts_are_read_without_migrating_or_losing_row_details() {
    let dir = tempfile::tempdir().unwrap();
    let template = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../apps/desktop/tests/fixtures/automation/form.pdf");
    let data = dir.path().join("rows.json");
    let output = dir.path().join("batch");
    fs::write(&data, r#"[{"customer":"Ada"}]"#).unwrap();
    let command = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .arg("fill-batch")
        .arg("--keep-editable")
        .arg("--template")
        .arg(template)
        .arg("--data")
        .arg(data)
        .arg("--output-dir")
        .arg(&output)
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        command.status.success(),
        "{}",
        String::from_utf8_lossy(&command.stderr)
    );
    let before = fs::read(output.join("execution_receipt.json")).unwrap();
    let command = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .args(["workflow", "history", "--directory"])
        .arg(&output)
        .args(["--id", "execution_receipt.json", "--json"])
        .output()
        .unwrap();
    assert!(command.status.success());
    let history: Value = serde_json::from_slice(&command.stdout).unwrap();
    assert_eq!(history["workflow"]["source_schema"], "batch/legacy");
    assert_eq!(history["workflow"]["details"]["succeeded"], 1);
    assert_eq!(
        fs::read(output.join("execution_receipt.json")).unwrap(),
        before
    );
    let document = lopdf::Document::load(output.join("0001.pdf")).unwrap();
    assert_eq!(
        paperpilot_pdf::operations::form_fill::read_fields(&document).unwrap()["customer"],
        "Ada"
    );
}
