use assert_cmd::Command;
use paperpilot_pdf::document::LopdfDocument;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;
const FIXTURE_FORM_FIELDS: &str = "../tests/fixtures/form_fields.pdf";

#[test]
fn test_fill_batch_csv_success() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("data.csv");
    let output_dir = temp_dir.path().join("out");

    let csv_content = "Name,Age\nAlice,30\nBob,25\nCharlie,35\n";
    fs::write(&csv_path, csv_content).unwrap();

    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("fill-batch")
        .arg("--template")
        .arg(FIXTURE_FORM_FIELDS)
        .arg("--data")
        .arg(&csv_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .assert()
        .success();

    let entries: Vec<_> = fs::read_dir(&output_dir).unwrap().collect();
    assert_eq!(entries.len(), 3);
    assert!(output_dir.join("0000.pdf").exists());
    assert!(output_dir.join("0001.pdf").exists());
    assert!(output_dir.join("0002.pdf").exists());
}

#[test]
fn test_fill_batch_json_success() {
    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("data.json");
    let output_dir = temp_dir.path().join("out");

    let json_content = r#"[{"Name":"Alice","Age":"30"},{"Name":"Bob","Age":"25"}]"#;
    fs::write(&json_path, json_content).unwrap();

    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("fill-batch")
        .arg("--template")
        .arg(FIXTURE_FORM_FIELDS)
        .arg("--data")
        .arg(&json_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .assert()
        .success();

    let entries: Vec<_> = fs::read_dir(&output_dir).unwrap().collect();
    assert_eq!(entries.len(), 2);
    assert!(output_dir.join("0000.pdf").exists());
    assert!(output_dir.join("0001.pdf").exists());
}

#[test]
fn test_fill_batch_flatten() {
    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("data.json");
    let output_dir = temp_dir.path().join("out");

    let json_content = r#"[{"Name":"Alice"}]"#;
    fs::write(&json_path, json_content).unwrap();

    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("fill-batch")
        .arg("--template")
        .arg(FIXTURE_FORM_FIELDS)
        .arg("--data")
        .arg(&json_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--flatten")
        .assert()
        .success();

    let output_pdf = output_dir.join("0000.pdf");
    assert!(output_pdf.exists());

    let doc = LopdfDocument::load(&output_pdf).unwrap();
    let catalog = doc.inner.catalog().unwrap();
    assert!(
        !catalog.has(b"AcroForm"),
        "AcroForm should be removed when flattened"
    );
}

#[test]
fn test_fill_batch_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("data.json");
    let output_dir = temp_dir.path().join("out");

    let json_content = r#"[{"Name":"Alice"},{"Name":"Bob"}]"#;
    fs::write(&json_path, json_content).unwrap();

    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("fill-batch")
        .arg("--template")
        .arg(FIXTURE_FORM_FIELDS)
        .arg("--data")
        .arg(&json_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("[dry-run] would write:"));

    assert!(
        !output_dir.exists(),
        "Output directory should not be created in dry-run mode"
    );
}

#[test]
fn test_fill_batch_missing_column_token() {
    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("data.json");
    let output_dir = temp_dir.path().join("out");

    let json_content = r#"[{"Name":"Alice"}]"#;
    fs::write(&json_path, json_content).unwrap();

    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("fill-batch")
        .arg("--template")
        .arg(FIXTURE_FORM_FIELDS)
        .arg("--data")
        .arg(&json_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--name-pattern")
        .arg("{nonexistent}_{_index}.pdf")
        .assert()
        .success();

    assert!(output_dir.join("{nonexistent}_0000.pdf").exists());
}

#[test]
fn test_fill_batch_fail_fast_bad_template() {
    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("data.json");
    let output_dir = temp_dir.path().join("out");

    let json_content = r#"[{"Name":"Alice"}]"#;
    fs::write(&json_path, json_content).unwrap();

    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("fill-batch")
        .arg("--template")
        .arg("non_existent_template.pdf")
        .arg("--data")
        .arg(&json_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--fail-fast")
        .assert()
        .failure();

    let entries: Vec<_> = fs::read_dir(&output_dir).unwrap().collect();
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_fill_batch_custom_pattern_date() {
    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("data.json");
    let output_dir = temp_dir.path().join("out");

    let json_content = r#"[{"Name":"Alice"}]"#;
    fs::write(&json_path, json_content).unwrap();

    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("fill-batch")
        .arg("--template")
        .arg(FIXTURE_FORM_FIELDS)
        .arg("--data")
        .arg(&json_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--name-pattern")
        .arg("{Name}_{_date}.pdf")
        .assert()
        .success();

    let date = chrono::Local::now().format("%Y%m%d").to_string();
    assert!(output_dir.join(format!("Alice_{}.pdf", date)).exists());
}

#[test]
fn test_fill_batch_auto_create_output_dir() {
    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("data.json");
    let output_dir = temp_dir.path().join("non_existent_out");

    let json_content = r#"[{"Name":"Alice"}]"#;
    fs::write(&json_path, json_content).unwrap();

    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("fill-batch")
        .arg("--template")
        .arg(FIXTURE_FORM_FIELDS)
        .arg("--data")
        .arg(&json_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .assert()
        .success();

    assert!(output_dir.exists());
    assert!(output_dir.join("0000.pdf").exists());
}
