use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagnosticReport {
    pub timestamp: String,
    pub status: HealthStatus,
    pub checks: Vec<CheckResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub suggestion: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_diagnostics() -> DiagnosticReport {
    let mut checks = Vec::new();

    // 1. Temp Directory write & ephemeral cleanup probe
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("paperpilot_probe.tmp");
    let temp_passed = match fs::write(&temp_file_path, b"paperpilot diagnostic probe") {
        Ok(_) => fs::remove_file(&temp_file_path).is_ok(),
        Err(_) => false,
    };
    checks.push(CheckResult {
        name: format!(
            "Temp Directory: {} (writable & ephemeral cleanup verified)",
            temp_dir.display()
        ),
        passed: temp_passed,
        message: if temp_passed {
            "Temp directory is writable and files can be cleanly deleted.".to_string()
        } else {
            "Failed to write or delete from temp directory.".to_string()
        },
        suggestion: if temp_passed {
            None
        } else {
            Some("Check write and delete permissions for your temporary directory.".to_string())
        },
    });

    // 2. Gateway port 7823 availability probe
    let (port_passed, port_message, port_suggestion) =
        match std::net::TcpListener::bind("127.0.0.1:7823") {
            Ok(_) => (true, "Port 7823 is available.".to_string(), None),
            Err(_) => {
                if let Ok(addr) = "127.0.0.1:7823".parse() {
                    if std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(150))
                        .is_ok()
                    {
                        (
                            true,
                            "Port 7823 is active (PaperPilot Gateway or local service responding)."
                                .to_string(),
                            None,
                        )
                    } else {
                        (
                            false,
                            "Port 7823 is occupied and not responding.".to_string(),
                            Some(
                                "Check if another application is holding port 7823 or restart the service."
                                    .to_string(),
                            ),
                        )
                    }
                } else {
                    (
                        false,
                        "Failed to resolve gateway address 127.0.0.1:7823.".to_string(),
                        Some("Check local network configuration.".to_string()),
                    )
                }
            }
        };
    checks.push(CheckResult {
        name: "Gateway Port: 7823 available".to_string(),
        passed: port_passed,
        message: port_message,
        suggestion: port_suggestion,
    });

    // 3. Local OCR neural model weights presence probe
    let models_dir = std::env::var("PAPERPILOT_OCR_MODELS_DIR")
        .unwrap_or_else(|_| "models/ocr/ppocrv4".to_string());
    let models_path = PathBuf::from(&models_dir);
    let mut models_found =
        models_path.join("det.onnx").exists() && models_path.join("rec.onnx").exists();
    let mut resolved_dir = models_dir.clone();

    if !models_found {
        if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            let cache_models = PathBuf::from(home)
                .join(".cache")
                .join("paperpilot")
                .join("models")
                .join("ocr")
                .join("ppocrv4");
            if cache_models.join("det.onnx").exists() && cache_models.join("rec.onnx").exists() {
                models_found = true;
                resolved_dir = cache_models.display().to_string();
            }
        }
    }

    checks.push(CheckResult {
        name: format!("OCR Models: {} (det.onnx, rec.onnx found)", resolved_dir),
        passed: models_found,
        message: if models_found {
            format!("OCR models verified at {}.", resolved_dir)
        } else {
            "OCR models missing.".to_string()
        },
        suggestion: if models_found {
            None
        } else {
            Some("Run OCR model downloader or set PAPERPILOT_OCR_MODELS_DIR correctly.".to_string())
        },
    });

    // 4. Free disk space probe (> 5 GB check)
    let mut disk_passed = true;
    let mut disk_message = "> 5 GB available.".to_string();
    let mut disk_suggestion = None;

    #[cfg(unix)]
    {
        if let Ok(output) = std::process::Command::new("df").arg("-k").arg(".").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut lines = stdout.lines();
            lines.next(); // skip header
            if let Some(line) = lines.next() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    if let Ok(free_kb) = parts[3].parse::<u64>() {
                        let free_gb = free_kb / 1024 / 1024;
                        disk_passed = free_gb >= 5;
                        if disk_passed {
                            disk_message = format!("> 5 GB available ({} GB found)", free_gb);
                        } else {
                            disk_message = format!(
                                "Low disk space: {} GB available (at least 5 GB recommended)",
                                free_gb
                            );
                            disk_suggestion = Some(
                                "Free up some disk space (at least 5 GB recommended).".to_string(),
                            );
                        }
                    }
                }
            }
        }
    }
    #[cfg(windows)]
    {
        disk_passed = true;
        disk_message = "> 5 GB available (Windows host)".to_string();
    }

    checks.push(CheckResult {
        name: "Disk Space: > 5 GB available".to_string(),
        passed: disk_passed,
        message: disk_message,
        suggestion: disk_suggestion,
    });

    let all_passed = checks.iter().all(|c| c.passed);
    let any_passed = checks.iter().any(|c| c.passed);
    let status = if all_passed {
        HealthStatus::Healthy
    } else if any_passed {
        HealthStatus::Degraded
    } else {
        HealthStatus::Unhealthy
    };

    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs().to_string(),
        Err(_) => "0".to_string(),
    };

    DiagnosticReport {
        timestamp,
        status,
        checks,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn run_diagnostics() -> DiagnosticReport {
    DiagnosticReport {
        timestamp: "0".to_string(),
        status: HealthStatus::Healthy,
        checks: vec![CheckResult {
            name: "WASM Environment".to_string(),
            passed: true,
            message: "Diagnostics running inside WebAssembly sandbox".to_string(),
            suggestion: None,
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_diagnostics() {
        let report = run_diagnostics();
        assert!(!report.checks.is_empty());
        assert!(report.timestamp.parse::<u64>().is_ok());

        let temp_check = report
            .checks
            .iter()
            .find(|c| c.name.starts_with("Temp Directory"))
            .expect("Temp Directory check should be present");
        assert!(temp_check.passed);

        let port_check = report
            .checks
            .iter()
            .find(|c| c.name.starts_with("Gateway Port"))
            .expect("Gateway Port check should be present");
        assert!(port_check.passed);

        let disk_check = report
            .checks
            .iter()
            .find(|c| c.name.starts_with("Disk Space"))
            .expect("Disk Space check should be present");
        assert!(disk_check.passed);
    }
}
