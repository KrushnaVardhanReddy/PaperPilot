mod runner;
mod fixtures;
mod assertions;
mod cases;

use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(name = "penta-interface-e2e", version = "0.1.0", about = "Rust-Native Penta-Interface Real Semantic Assertions Suite")]
struct Args {
    #[arg(long, default_value = "all")]
    group: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("=== PaperPilot Penta-Interface E2E Suite ===");
    println!("Initializing fixtures...");
    let fixture_mgr = fixtures::FixtureManager::new();
    fixture_mgr.ensure_fixtures()?;

    match args.group.as_str() {
        "page_ops" | "5.9.4A" => {
            let results = cases::page_ops::run_page_ops_suite().await?;
            println!("Page ops finished: {} tests", results.len());
        }
        "security_forms" | "5.9.4B" => {
            let results = cases::security_forms::run_security_forms_suite().await?;
            println!("Security & forms finished: {} tests", results.len());
        }
        "analysis" | "5.9.4C" => {
            let results = cases::analysis::run_analysis_suite().await?;
            println!("Analysis finished: {} tests", results.len());
        }
        "conversions" | "5.9.4D" => {
            let results = cases::conversions::run_conversions_suite().await?;
            println!("Conversions finished: {} tests", results.len());
        }
        _ => {
            println!("Running all suites...");
            cases::page_ops::run_page_ops_suite().await?;
            cases::security_forms::run_security_forms_suite().await?;
            cases::analysis::run_analysis_suite().await?;
            cases::conversions::run_conversions_suite().await?;
        }
    }

    Ok(())
}
