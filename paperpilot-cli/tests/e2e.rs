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
