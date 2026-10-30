use assert_cmd::Command;

#[test]
fn test_cli_logging_flags_and_stderr() {
    let mut cmd = Command::cargo_bin("paperpilot-cli").unwrap();
    cmd.args(["--log-format", "json", "--log-level", "debug", "list-tools"]);

    let assert = cmd.assert().success();
    let output = assert.get_output();

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);

    // Stderr should contain structured NDJSON log line
    assert!(stderr_str.contains("Dispatching CLI subcommand"), "Stderr missing debug log line: {}", stderr_str);
    assert!(stderr_str.contains("\"level\":\"DEBUG\""), "Stderr missing DEBUG level JSON: {}", stderr_str);

    // Parse one JSON log line from stderr to verify valid JSON
    let first_json_line = stderr_str
        .lines()
        .find(|line| line.trim().starts_with('{'))
        .expect("No JSON line found in stderr");

    let parsed: serde_json::Value = serde_json::from_str(first_json_line)
        .expect("Failed to parse stderr NDJSON line as valid JSON");
    assert_eq!(parsed["fields"]["subcommand"], "ListTools");

    // Stdout should remain clean or contain expected command output
    assert!(!stdout_str.contains("\"level\":\"DEBUG\""));
}
