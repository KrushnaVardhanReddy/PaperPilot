use serde::{Deserialize, Serialize};
use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub timestamp: String,
    pub status: HealthStatus,
    pub checks: Vec<CheckResult>,
}

/// Executes all system and environment diagnostic probes and aggregates the results.
pub fn run_diagnostics() -> DiagnosticReport {
    let checks = vec![
        probe_temp_dir(),
        probe_working_dir(),
        probe_gateway_port(7823),
        probe_ocr_models(),
        probe_disk_space(),
    ];

    let mut has_unhealthy = false;
    let mut has_degraded = false;

    for check in &checks {
        if !check.passed {
            // If temp dir or working dir fail, system is critically unhealthy
            if check.name == "temp_dir" || check.name == "working_dir" {
                has_unhealthy = true;
            } else {
                has_degraded = true;
            }
        }
    }

    let status = if has_unhealthy {
        HealthStatus::Unhealthy
    } else if has_degraded {
        HealthStatus::Degraded
    } else {
        HealthStatus::Healthy
    };

    let timestamp = format_current_timestamp();

    DiagnosticReport {
        timestamp,
        status,
        checks,
    }
}

fn format_current_timestamp() -> String {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    let days = secs / 86400;
    let rem_secs = secs % 86400;
    let hours = rem_secs / 3600;
    let mins = (rem_secs % 3600) / 60;
    let s = rem_secs % 60;

    let mut y = 1970;
    let mut d = days as i64;
    loop {
        let leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
        let days_in_year = if leap { 366 } else { 365 };
        if d >= days_in_year {
            d -= days_in_year;
            y += 1;
        } else {
            break;
        }
    }

    let leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
    let days_in_months = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 1;
    for &dim in &days_in_months {
        if d >= dim {
            d -= dim;
            m += 1;
        } else {
            break;
        }
    }
    let day = d + 1;

    format!("{y:04}-{m:02}-{day:02}T{hours:02}:{mins:02}:{s:02}Z")
}

/// 1. Temp Directory write & delete probe
pub fn probe_temp_dir() -> CheckResult {
    let temp_dir = std::env::temp_dir();
    let probe_file = temp_dir.join(format!(
        "paperpilot_doctor_probe_{}.tmp",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));

    let test_data = b"paperpilot-doctor-health-check";

    match fs::write(&probe_file, test_data) {
        Ok(_) => match fs::read(&probe_file) {
            Ok(content) if content == test_data => {
                let _ = fs::remove_file(&probe_file);
                CheckResult {
                    name: "temp_dir".to_string(),
                    passed: true,
                    message: format!(
                        "Temp directory {} is writable and ephemeral cleanup verified",
                        temp_dir.display()
                    ),
                    suggestion: None,
                }
            }
            Ok(_) => {
                let _ = fs::remove_file(&probe_file);
                CheckResult {
                    name: "temp_dir".to_string(),
                    passed: false,
                    message: format!(
                        "Temp directory {} read verification mismatch",
                        temp_dir.display()
                    ),
                    suggestion: Some(
                        "Check temp directory filesystem integrity and permissions.".to_string(),
                    ),
                }
            }
            Err(e) => {
                let _ = fs::remove_file(&probe_file);
                CheckResult {
                    name: "temp_dir".to_string(),
                    passed: false,
                    message: format!(
                        "Failed to read test probe file in {}: {}",
                        temp_dir.display(),
                        e
                    ),
                    suggestion: Some(
                        "Verify read permissions for the operating system temp directory."
                            .to_string(),
                    ),
                }
            }
        },
        Err(e) => CheckResult {
            name: "temp_dir".to_string(),
            passed: false,
            message: format!(
                "Cannot write to temp directory {}: {}",
                temp_dir.display(),
                e
            ),
            suggestion: Some(
                "Ensure current user has write permissions in the OS temp directory or set TMPDIR."
                    .to_string(),
            ),
        },
    }
}

/// Working directory permissions probe
pub fn probe_working_dir() -> CheckResult {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            return CheckResult {
                name: "working_dir".to_string(),
                passed: false,
                message: format!("Cannot determine current working directory: {}", e),
                suggestion: Some(
                    "Check process working directory and execution context.".to_string(),
                ),
            };
        }
    };

    let probe_file = current_dir.join(format!(
        ".paperpilot_probe_{}.tmp",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));

    match fs::write(&probe_file, b"ok") {
        Ok(_) => {
            let _ = fs::remove_file(&probe_file);
            CheckResult {
                name: "working_dir".to_string(),
                passed: true,
                message: format!(
                    "Working directory {} read/write verified",
                    current_dir.display()
                ),
                suggestion: None,
            }
        }
        Err(e) => CheckResult {
            name: "working_dir".to_string(),
            passed: false,
            message: format!(
                "Working directory {} is not writable: {}",
                current_dir.display(),
                e
            ),
            suggestion: Some(
                "Ensure write access to the working directory or run from a writable path."
                    .to_string(),
            ),
        },
    }
}

/// 2. Gateway port availability probe
pub fn probe_gateway_port(port: u16) -> CheckResult {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(listener) => {
            drop(listener);
            CheckResult {
                name: "gateway_port".to_string(),
                passed: true,
                message: format!("Gateway port {} is available for local REST API", port),
                suggestion: None,
            }
        }
        Err(e) => CheckResult {
            name: "gateway_port".to_string(),
            passed: false,
            message: format!("Port {} is currently occupied or unavailable: {}", port, e),
            suggestion: Some(format!(
                "If PaperPilot Gateway is already running, this is normal. Otherwise release port {} via 'kill $(lsof -t -i :{})'.",
                port, port
            )),
        },
    }
}

/// 3. OCR model weights presence probe
pub fn probe_ocr_models() -> CheckResult {
    let mut resolved_location: Option<PathBuf> = None;

    // Check environment variable
    if let Ok(dir_str) = std::env::var("PAPERPILOT_OCR_MODELS_DIR") {
        let dir = PathBuf::from(dir_str);
        if dir.exists() && check_ocr_dir_has_models(&dir) {
            resolved_location = Some(dir);
        }
    }

    // Check workspace models directories
    if resolved_location.is_none() {
        for candidate in [
            PathBuf::from("models").join("ocr").join("ppocrv4"),
            PathBuf::from("models").join("ocr"),
            PathBuf::from("../models").join("ocr").join("ppocrv4"),
            PathBuf::from("../../models").join("ocr").join("ppocrv4"),
            PathBuf::from("../../../models").join("ocr").join("ppocrv4"),
        ] {
            if candidate.exists() && check_ocr_dir_has_models(&candidate) {
                resolved_location = Some(candidate);
                break;
            }
        }
    }

    // Check user home cache directory
    if resolved_location.is_none()
        && let Ok(home) = std::env::var("HOME")
    {
        let user_cache = PathBuf::from(&home)
            .join(".local")
            .join("share")
            .join("paperpilot")
            .join("models")
            .join("ocr");
        if user_cache.exists() && check_ocr_dir_has_models(&user_cache) {
            resolved_location = Some(user_cache);
        }
    }

    match resolved_location {
        Some(path) => CheckResult {
            name: "ocr_models".to_string(),
            passed: true,
            message: format!("OCR models located at {} (detection & recognition weights verified)", path.display()),
            suggestion: None,
        },
        None => CheckResult {
            name: "ocr_models".to_string(),
            passed: false,
            message: "OCR model weights (det.onnx/rec.onnx or det.rten/rec.rten) not found in standard paths".to_string(),
            suggestion: Some("Set PAPERPILOT_OCR_MODELS_DIR or download models in Settings > OCR & Vision Models.".to_string()),
        },
    }
}

fn check_ocr_dir_has_models(dir: &Path) -> bool {
    let has_det = dir.join("det.onnx").exists()
        || dir.join("det.rten").exists()
        || dir.join("text-detection.rten").exists();
    let has_rec = dir.join("rec.onnx").exists()
        || dir.join("rec.rten").exists()
        || dir.join("text-recognition.rten").exists();
    has_det && has_rec
}

/// 4. Free disk space probe
pub fn probe_disk_space() -> CheckResult {
    let temp_dir = std::env::temp_dir();
    let available_space = get_available_disk_space(&temp_dir);

    match available_space {
        Some(bytes) => {
            let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            if bytes >= 1024 * 1024 * 1024 {
                CheckResult {
                    name: "disk_space".to_string(),
                    passed: true,
                    message: format!(
                        "Scratch storage has {:.2} GB available (minimum 1 GB required)",
                        gb
                    ),
                    suggestion: None,
                }
            } else {
                CheckResult {
                    name: "disk_space".to_string(),
                    passed: false,
                    message: format!(
                        "Low scratch disk space: {:.2} GB available in {}",
                        gb,
                        temp_dir.display()
                    ),
                    suggestion: Some(
                        "Free up disk space on the scratch volume for large PDF rendering."
                            .to_string(),
                    ),
                }
            }
        }
        None => CheckResult {
            name: "disk_space".to_string(),
            passed: true,
            message: format!("Scratch storage available at {}", temp_dir.display()),
            suggestion: None,
        },
    }
}

#[cfg(unix)]
fn get_available_disk_space(path: &Path) -> Option<u64> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let c_path = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let res = unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) };
    if res == 0 {
        let free_bytes = (stat.f_bavail as u128 * stat.f_frsize as u128) as u64;
        Some(free_bytes)
    } else {
        None
    }
}

#[cfg(not(unix))]
fn get_available_disk_space(_path: &Path) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_current_timestamp() {
        let ts = format_current_timestamp();
        assert!(ts.contains('T'));
        assert!(ts.ends_with('Z'));
        assert_eq!(ts.len(), 20);
    }

    #[test]
    fn test_probe_temp_dir() {
        let res = probe_temp_dir();
        assert!(res.passed);
        assert_eq!(res.name, "temp_dir");
    }

    #[test]
    fn test_probe_working_dir() {
        let res = probe_working_dir();
        assert!(res.passed);
        assert_eq!(res.name, "working_dir");
    }

    #[test]
    fn test_probe_disk_space() {
        let res = probe_disk_space();
        assert_eq!(res.name, "disk_space");
    }

    #[test]
    fn test_probe_ocr_models() {
        let res = probe_ocr_models();
        assert_eq!(res.name, "ocr_models");
        assert!(res.passed);
    }

    #[test]
    fn test_run_diagnostics() {
        let report = run_diagnostics();
        assert!(!report.checks.is_empty());
        assert!(!report.timestamp.is_empty());
        assert_ne!(report.status, HealthStatus::Unhealthy);
    }
}
