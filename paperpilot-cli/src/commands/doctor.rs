use paperpilot_core::diagnostics::{HealthStatus, run_diagnostics};

pub fn run(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let report = run_diagnostics();

    if json {
        let json_output = serde_json::to_string_pretty(&report)?;
        println!("{}", json_output);
        std::process::exit(0);
    }

    println!(
        "\x1b[1mPaperPilot System Diagnostics (v{})\x1b[0m",
        env!("CARGO_PKG_VERSION")
    );
    println!("==================================================");
    println!("Status:    {:?}", report.status);
    println!("Timestamp: {}", report.timestamp);
    println!();

    for check in &report.checks {
        let icon = if check.passed {
            "\x1b[32m[✓]\x1b[0m"
        } else {
            "\x1b[31m[✗]\x1b[0m"
        };
        println!("{} {}", icon, check.name);

        if !check.passed {
            println!("    \x1b[31mReason:\x1b[0m {}", check.message);
            if let Some(ref suggestion) = check.suggestion {
                println!("    \x1b[33mFix:\x1b[0m    {}", suggestion);
            }
        }
    }

    println!();
    match report.status {
        HealthStatus::Healthy => {
            println!("\x1b[32mResult: All system diagnostics passed. PaperPilot is ready.\x1b[0m");
        }
        HealthStatus::Degraded => {
            println!("\x1b[33mResult: Some non-critical warnings or optional dependencies are missing.\x1b[0m");
        }
        HealthStatus::Unhealthy => {
            println!("\x1b[31mResult: Critical issues detected that may prevent PaperPilot from operating.\x1b[0m");
        }
    }

    if report.status == HealthStatus::Unhealthy {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
