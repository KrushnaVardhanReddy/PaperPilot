use paperpilot_ai::supervisor::{LlamafileSupervisor, SupervisorError};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Create a dummy script/batch file to simulate the llamafile binary
#[allow(unused_variables)]
fn create_mock_llamafile(sh_content: &str, bat_content: &str) -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = env::temp_dir().join(format!("llamafile_test_{}_{}", std::process::id(), id));
    fs::create_dir_all(&dir).unwrap();

    #[cfg(unix)]
    let file_path = {
        let path = dir.join("mock_llamafile.sh");
        fs::write(&path, sh_content).unwrap();
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(perms.mode() | 0o111);
        fs::set_permissions(&path, perms).unwrap();
        path
    };

    #[cfg(windows)]
    let file_path = {
        let path = dir.join("mock_llamafile.bat");
        fs::write(&path, bat_content).unwrap();
        path
    };

    file_path
}

#[tokio::test]
async fn test_supervisor_start_and_stop() {
    // A simple mock binary that just sleeps
    let mock_bin = create_mock_llamafile(
        r#"#!/bin/bash
sleep 10
"#,
        r#"@echo off
timeout /t 10 /nobreak >nul
"#,
    );

    // Setup a mock server to simulate the health endpoint
    let mock_server = MockServer::start().await;
    let port = mock_server.address().port();

    // Before the binary is started, the endpoint shouldn't be matched
    // Wait for the mock binary to be "ready"
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let supervisor = LlamafileSupervisor::new(mock_bin.clone(), port);

    // Initial state
    let status = supervisor.status();
    assert!(!status.is_running);

    // Start
    let start_result = supervisor.start(Duration::from_secs(5)).await;
    assert!(start_result.is_ok());

    let status = supervisor.status();
    assert!(status.is_running);
    assert!(status.pid.is_some());

    // Stop
    let stop_result = supervisor.stop();
    assert!(stop_result.is_ok());

    let status = supervisor.status();
    assert!(!status.is_running);
}

#[tokio::test]
async fn test_supervisor_port_conflict_reuse() {
    // Create a mock script that shouldn't even be called because port is taken
    let mock_bin = create_mock_llamafile(
        r#"#!/bin/bash
exit 1
"#,
        r#"@echo off
exit /b 1
"#,
    );

    // Setup a mock server that simulates an ALREADY RUNNING llamafile
    let mock_server = MockServer::start().await;
    let port = mock_server.address().port();

    // Respond with 200 on /v1/models to simulate existing service
    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let supervisor = LlamafileSupervisor::new(mock_bin.clone(), port);

    let start_result = supervisor.start(Duration::from_secs(5)).await;
    assert!(start_result.is_ok());

    // Since we attached to an existing process, our child PID will be None,
    // but the service is considered successfully started.
    let status = supervisor.status();
    assert_eq!(status.pid, None);
}

#[tokio::test]
async fn test_supervisor_spawn_failed() {
    // Create a mock script that exits immediately with an error
    let mock_bin = create_mock_llamafile(
        r#"#!/bin/bash
exit 1
"#,
        r#"@echo off
exit /b 1
"#,
    );

    // Setup a mock server
    let mock_server = MockServer::start().await;
    let port = mock_server.address().port();

    let supervisor = LlamafileSupervisor::new(mock_bin.clone(), port);

    // Start should fail
    let start_result = supervisor.start(Duration::from_secs(2)).await;
    assert!(matches!(start_result, Err(SupervisorError::SpawnFailed)));
}

#[tokio::test]
async fn test_supervisor_timeout() {
    // Create a mock script that sleeps but we NEVER configure the mock server to respond 200
    let mock_bin = create_mock_llamafile(
        r#"#!/bin/bash
sleep 5
"#,
        r#"@echo off
timeout /t 5 /nobreak >nul
"#,
    );

    // Setup a mock server
    let mock_server = MockServer::start().await;
    let port = mock_server.address().port();

    // No mocks mounted, so it will return 404
    let supervisor = LlamafileSupervisor::new(mock_bin.clone(), port);

    // Start should timeout
    let start_result = supervisor.start(Duration::from_millis(500)).await;
    assert!(matches!(start_result, Err(SupervisorError::Timeout)));
}

#[tokio::test]
async fn test_supervisor_drop_stops_process() {
    let mock_bin = create_mock_llamafile(
        r#"#!/bin/bash
sleep 10
"#,
        r#"@echo off
timeout /t 10 /nobreak >nul
"#,
    );

    let mock_server = MockServer::start().await;
    let port = mock_server.address().port();

    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let pid;
    {
        let supervisor = LlamafileSupervisor::new(mock_bin.clone(), port);
        supervisor.start(Duration::from_secs(5)).await.unwrap();
        pid = supervisor.status().pid.unwrap();
        // supervisor goes out of scope here and should be dropped
    }

    // Wait a little bit for the process to be killed
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Check if the process is still running.
    // A cross-platform way without unsafe is checking if we can kill it again or looking it up.
    // However, a simple cross platform way is to check the OS specific process list.
    #[cfg(unix)]
    {
        let check_cmd = std::process::Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .status()
            .unwrap();
        assert!(!check_cmd.success(), "Process should have been killed");
    }

    #[cfg(windows)]
    {
        let check_cmd = std::process::Command::new("tasklist")
            .arg("/FI")
            .arg(format!("PID eq {}", pid))
            .output()
            .unwrap();
        let output = String::from_utf8_lossy(&check_cmd.stdout);
        assert!(
            !output.contains(&pid.to_string()),
            "Process should have been killed"
        );
    }
}
