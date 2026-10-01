use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

fn get_fixture_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join(filename)
}

#[test]
fn test_hash_command() {
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    let pdf_path = get_fixture_path("simple.pdf");

    // Compute expected hash manually
    let data = fs::read(&pdf_path).unwrap();
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let expected_hash = hex::encode(hasher.finalize());

    cmd.arg("hash")
        .arg("--input")
        .arg(&pdf_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(&expected_hash));
}

#[test]
fn test_verify_command_success() {
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    let pdf_path = get_fixture_path("simple.pdf");

    // Compute valid hash manually
    let data = fs::read(&pdf_path).unwrap();
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let valid_hash = hex::encode(hasher.finalize());

    cmd.arg("verify")
        .arg("--input")
        .arg(&pdf_path)
        .arg("--expected-hash")
        .arg(&valid_hash)
        .assert()
        .success();
}

#[test]
fn test_verify_command_failure() {
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    let pdf_path = get_fixture_path("simple.pdf");

    let invalid_hash = "0000000000000000000000000000000000000000000000000000000000000000";

    cmd.arg("verify")
        .arg("--input")
        .arg(&pdf_path)
        .arg("--expected-hash")
        .arg(invalid_hash)
        .assert()
        .failure(); // exit code 1
}

#[test]
fn test_extract_text_json_format() {
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    let pdf_path = get_fixture_path("simple.pdf");

    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.json");

    cmd.arg("extract-text")
        .arg("--input")
        .arg(&pdf_path)
        .arg("--output")
        .arg(&output_path)
        .arg("--format")
        .arg("json")
        .assert()
        .success();

    let output_content = fs::read_to_string(&output_path).unwrap();

    // Attempt to parse it as JSON array
    let parsed: serde_json::Value = serde_json::from_str(&output_content).unwrap();

    assert!(parsed.is_array());
    let array = parsed.as_array().unwrap();
    assert!(!array.is_empty());
}

#[test]
fn test_annotate_command() {
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    let pdf_path = get_fixture_path("simple.pdf");

    let dir = tempdir().unwrap();
    let data_path = dir.path().join("annotations.json");
    let output_path = dir.path().join("annotated.pdf");

    // Write a dummy annotation data file
    fs::write(
        &data_path,
        "[\n    {\n        \"id\": \"1\",\n        \"type\": \"highlight\",\n        \"page\": 1,\n        \"x\": 100.0,\n        \"y\": 100.0,\n        \"w\": 50.0,\n        \"h\": 20.0,\n        \"color\": \"#ffff00\",\n        \"content\": \"Test annotation\"\n    }\n]"
    )
    .unwrap();

    cmd.arg("annotate")
        .arg("--input")
        .arg(&pdf_path)
        .arg("--data")
        .arg(&data_path)
        .arg("--output")
        .arg(&output_path)
        .assert()
        .success();

    assert!(output_path.exists());
}

#[test]
fn test_form_commands() {
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    let pdf_path = get_fixture_path("simple.pdf");

    let dir = tempdir().unwrap();
    let added_field_path = dir.path().join("added_field.pdf");

    cmd.arg("form")
        .arg("add-field")
        .arg(&pdf_path)
        .arg("--name")
        .arg("TestField")
        .arg("--rect")
        .arg("100,100,200,120")
        .arg("--output")
        .arg(&added_field_path)
        .assert()
        .success();

    assert!(added_field_path.exists());

    let mut cmd2 = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd2.arg("form")
        .arg("read")
        .arg(&added_field_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("TestField"));

    let data_path = dir.path().join("form_data.json");
    fs::write(&data_path, r#"{"TestField": "Hello World"}"#).unwrap();

    let filled_path = dir.path().join("filled.pdf");
    let mut cmd3 = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd3.arg("form")
        .arg("fill")
        .arg(&added_field_path)
        .arg("--data")
        .arg(&data_path)
        .arg("--output")
        .arg(&filled_path)
        .assert()
        .success();

    assert!(filled_path.exists());
}
