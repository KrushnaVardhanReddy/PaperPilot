use paperpilot_core::diagnostics::{self, HealthStatus};
use paperpilot_core::error::OperationResult;

pub fn handle_doctor(json_output: bool) -> OperationResult<()> {
    let report = diagnostics::run_diagnostics();

    if json_output {
        let json_str = serde_json::to_string_pretty(&report).map_err(|e| {
            paperpilot_core::error::PdfError::Other(format!("JSON serialization error: {}", e))
        })?;
        println!("{}", json_str);
    } else {
        println!();
        println!("  PaperPilot Doctor (v{})", env!("CARGO_PKG_VERSION"));
        println!("  ==========================");

        for check in &report.checks {
            if check.passed {
                // Green checkmark
                println!(
                    "  \x1b[32m[✓]\x1b[0m {}: {}",
                    format_check_title(&check.name),
                    check.message
                );
            } else if check.name == "ocr_models" || check.name == "gateway_port" {
                // Amber warning
                println!(
                    "  \x1b[33m[!]\x1b[0m {}: {}",
                    format_check_title(&check.name),
                    check.message
                );
                if let Some(ref suggestion) = check.suggestion {
                    println!("      \x1b[33m↳ Fix:\x1b[0m {}", suggestion);
                }
            } else {
                // Red failure
                println!(
                    "  \x1b[31m[✗]\x1b[0m {}: {}",
                    format_check_title(&check.name),
                    check.message
                );
                if let Some(ref suggestion) = check.suggestion {
                    println!("      \x1b[31m↳ Fix:\x1b[0m {}", suggestion);
                }
            }
        }

        println!();
        match report.status {
            HealthStatus::Healthy => {
                println!(
                    "  \x1b[32mResult: All systems operational. PaperPilot is ready to process documents.\x1b[0m"
                );
            }
            HealthStatus::Degraded => {
                let rec_count = report.checks.iter().filter(|c| !c.passed).count();
                println!(
                    "  \x1b[33mResult: {} recommendation(s) found. PaperPilot is ready to process documents.\x1b[0m",
                    rec_count
                );
            }
            HealthStatus::Unhealthy => {
                let fail_count = report.checks.iter().filter(|c| !c.passed).count();
                println!(
                    "  \x1b[31mResult: {} critical failure(s) detected. Please resolve above issues before processing.\x1b[0m",
                    fail_count
                );
                return Err(paperpilot_core::error::PdfError::Other(
                    "Doctor detected critical system environment issues".to_string(),
                ));
            }
        }
        println!();
    }

    Ok(())
}

fn format_check_title(name: &str) -> &'static str {
    match name {
        "temp_dir" => "Temp Scratch Storage",
        "working_dir" => "Working Directory",
        "gateway_port" => "REST Gateway Port",
        "ocr_models" => "OCR Model Weights",
        "disk_space" => "Scratch Disk Space",
        _ => "System Check",
    }
}
