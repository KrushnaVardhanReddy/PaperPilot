use paperpilot_core::diagnostics::run_diagnostics;

pub fn run(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let report = run_diagnostics();

    if json {
        let json_output = serde_json::to_string_pretty(&report)?;
        println!("{}", json_output);
        return Ok(());
    }

    println!("PaperPilot System Diagnostics");
    println!("-----------------------------");
    println!("Timestamp: {}", report.timestamp);
    println!("Status:    {:?}", report.status);
    println!();

    for check in report.checks {
        let icon = if check.passed { "[✓]" } else { "[✗]" };
        println!("{} {}", icon, check.name);

        if !check.passed {
            println!("    Reason: {}", check.message);
            if let Some(suggestion) = check.suggestion {
                println!("    Fix:    {}", suggestion);
            }
        }
    }

    Ok(())
}
