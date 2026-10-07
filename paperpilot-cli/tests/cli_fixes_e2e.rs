use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;
use tempfile::tempdir;

fn get_fixture_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("tests")
        .join("e2e_fixtures")
        .join(filename)
}

#[test]
fn test_cli_fixes_help() {
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd.arg("remove-blank").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("remove-blank"));

    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd.arg("page-numbers").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("page-numbers"));
}

#[test]
fn test_cli_fixes_execution() {
    let dir = tempdir().unwrap();
    let multi_page = get_fixture_path("multi_page.pdf");

    // remove-blank
    let output_rb = dir.path().join("rb.pdf");
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd.args([
        "remove-blank",
        "--input",
        multi_page.to_str().unwrap(),
        "--output",
        output_rb.to_str().unwrap(),
    ]);
    cmd.assert().success();
    assert!(output_rb.exists());

    // page-numbers
    let output_pn = dir.path().join("pn.pdf");
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd.args([
        "page-numbers",
        "--input",
        multi_page.to_str().unwrap(),
        "--output",
        output_pn.to_str().unwrap(),
    ]);
    cmd.assert().success();
    assert!(output_pn.exists());

    // burst with output-dir alias
    let output_dir = dir.path().join("burst_dir");
    std::fs::create_dir_all(&output_dir).unwrap();
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd.args([
        "burst",
        "--input",
        multi_page.to_str().unwrap(),
        "--output-dir",
        output_dir.to_str().unwrap(),
    ]);
    cmd.assert().success();

    // crop with x, y, width, height
    let single_page = get_fixture_path("single_page.pdf");
    let output_crop = dir.path().join("crop.pdf");
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd.args([
        "crop",
        "--input",
        single_page.to_str().unwrap(),
        "--x",
        "0",
        "--y",
        "0",
        "--width",
        "100",
        "--height",
        "100",
        "--output",
        output_crop.to_str().unwrap(),
    ]);
    cmd.assert().success();
    assert!(output_crop.exists());
}
