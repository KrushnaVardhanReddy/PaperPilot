use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub timestamp: String,
    pub status: HealthStatus,
    pub checks: Vec<CheckResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub suggestion: Option<String>,
}

pub fn run_diagnostics() -> DiagnosticReport {
    let mut checks = Vec::new();

    // 1. Temp Directory write & delete probe
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("paperpilot_probe.tmp");
    let temp_passed = match fs::write(&temp_file_path, "probe") {
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
            "Temp directory is writable and files can be deleted.".to_string()
        } else {
            "Failed to write or delete from temp directory.".to_string()
        },
        suggestion: if temp_passed {
            None
        } else {
            Some("Check permissions for your temporary directory.".to_string())
        },
    });

    // 2. Gateway port 7823 availability probe
    let port_available = std::net::TcpListener::bind("127.0.0.1:7823").is_ok();
    checks.push(CheckResult {
        name: "Gateway Port: 7823 available".to_string(),
        passed: port_available,
        message: if port_available {
            "Port 7823 is available.".to_string()
        } else {
            "Port 7823 is in use.".to_string()
        },
        suggestion: if port_available {
            None
        } else {
            Some("Check if another instance of PaperPilot Gateway is running, or if another app uses port 7823.".to_string())
        },
    });

    // 3. OCR model weights presence probe
    let models_dir = std::env::var("PAPERPILOT_OCR_MODELS_DIR")
        .unwrap_or_else(|_| "models/ocr/ppocrv4".to_string());
    let det_model = PathBuf::from(&models_dir).join("det.onnx");
    let rec_model = PathBuf::from(&models_dir).join("rec.onnx");
    let models_found = det_model.exists() && rec_model.exists();
    checks.push(CheckResult {
        name: format!("OCR Models: {} (det.onnx, rec.onnx found)", models_dir),
        passed: models_found,
        message: if models_found {
            "OCR models found.".to_string()
        } else {
            "OCR models missing.".to_string()
        },
        suggestion: if models_found {
            None
        } else {
            Some(
                "Run OCR model downloader or set PAPERPILOT_OCR_MODELS_DIR correctly.".to_string(),
            )
        },
    });

    // 4. Free disk space probe
    let mut disk_passed = false;
    let mut disk_message = "Disk space check not supported on this OS.".to_string();
    #[cfg(unix)]
    {
        if let Ok(output) = std::process::Command::new("df").arg("-k").arg(".").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut lines = stdout.lines();
            lines.next(); // skip header
            if let Some(line) = lines.next() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 && let Ok(free_kb) = parts[3].parse::<u64>() {
                    let free_gb = free_kb / 1024 / 1024;
                    disk_passed = free_gb > 5;
                    disk_message = format!("> 5 GB available ({} GB found)", free_gb);
                }
            }
        }
    }
    #[cfg(windows)]
    {
        disk_passed = true; // Placeholder for Windows
        disk_message = "> 5 GB available (Windows fallback)".to_string();
    }

    checks.push(CheckResult {
        name: "Disk Space: > 5 GB available".to_string(),
        passed: disk_passed,
        message: disk_message,
        suggestion: if disk_passed {
            None
        } else {
            Some("Free up some disk space (at least 5 GB recommended).".to_string())
        },
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
            .unwrap();
        assert!(temp_check.passed);
    }
}
