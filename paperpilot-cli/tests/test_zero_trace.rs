use std::process::Command;

#[test]
fn test_cli_log_level_off_emits_zero_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_paperpilot-cli"))
        .arg("--log-level")
        .arg("off")
        .arg("watch") // Running a quick dummy command
        .arg("--directory")
        .arg("/tmp")
        .arg("--once")
        .output()
        .expect("failed to execute cli");

    assert!(!output.status.success());
    assert!(output.stderr.is_empty(), "stderr should be empty, but was: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_cli_no_log_env_emits_zero_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_paperpilot-cli"))
        .env("NO_LOG", "1")
        .arg("watch")
        .arg("--directory")
        .arg("/tmp")
        .arg("--once")
        .output()
        .expect("failed to execute cli");

    assert!(!output.status.success());
    assert!(output.stderr.is_empty(), "stderr should be empty, but was: {}", String::from_utf8_lossy(&output.stderr));
}
