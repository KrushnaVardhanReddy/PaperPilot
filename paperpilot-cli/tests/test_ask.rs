use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_ask_dry_run_rotate() {
    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("ask")
        .arg("rotate 90 degrees")
        .arg("../fixtures/Get_Started_With_Smallpdf.pdf")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Intent: Rotate"));
}

#[test]
fn test_ask_unknown_query() {
    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("ask")
        .arg("x x x x x x x x x x x x x x")
        .arg("../fixtures/Get_Started_With_Smallpdf.pdf")
        .arg("--dry-run");

    // The current resolver logic might interpret it as something or explicitly fail
    // We just check it fails gracefully
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not determine intent").or(
            predicate::str::contains(
                "Failed to resolve natural language query: Could not identify a PDF operation",
            ),
        ));
}

#[test]
fn test_ask_non_interactive_execution_missing_params() {
    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("ask").arg("merge").arg("--yes");

    cmd.assert().failure().stderr(
        predicate::str::contains("requires at least one PDF file path").or(
            predicate::str::contains("Failed to resolve natural language query"),
        ),
    );
}

#[test]
fn test_ask_non_interactive_execution_success() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("rotated.pdf");
    let mut cmd = Command::cargo_bin("paperpilot").unwrap();
    cmd.arg("ask")
        .arg("rotate 90 degrees")
        .arg("../fixtures/Get_Started_With_Smallpdf.pdf")
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .arg("--yes");

    cmd.assert().success();
    assert!(output_path.exists());
}
