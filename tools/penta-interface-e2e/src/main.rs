#![allow(warnings)]

mod assertions;
mod cases;
mod fixtures;
mod runner;

use anyhow::Result;
use clap::Parser;
use runner::TestExecutionResult;

#[derive(Parser, Debug)]
#[command(
    name = "penta-interface-e2e",
    version = "0.1.0",
    about = "Rust-Native Penta-Interface Real Semantic Assertions Suite"
)]
struct Args {
    #[arg(long, default_value = "all")]
    group: String,
}

fn generate_markdown_report(
    title: &str,
    results: &[TestExecutionResult],
    report_path: &str,
) -> Result<()> {
    let mut md = String::new();
    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();

    md.push_str(&format!("# {}\n\n", title));
    md.push_str("## Executive Scorecard\n");
    md.push_str(&format!("- **Total Tests Evaluated:** {}\n", total));
    md.push_str(&format!(
        "- **Total Passed:** {} / {} ({:.1}%)\n\n",
        passed,
        total,
        (passed as f64 / total as f64) * 100.0
    ));

    md.push_str("## Detailed Test Matrix\n\n");
    md.push_str("| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |\n");
    md.push_str("|---|---|---|---|---|---|---|---|\n");

    for r in results {
        let verdict = if r.passed { "✅ PASS" } else { "❌ FAIL" };
        let invocation = r.get_invocation().replace('|', "\\|");
        let assertion =
            format!("{}: {}", r.assertion_checked, r.expected_result).replace('|', "\\|");
        let actual = r.actual_result.replace('|', "\\|");
        md.push_str(&format!(
            "| `{}` | **{}** | {} | `{}` | `{:.2} ms` | {} | {} | {} |\n",
            r.tool_id,
            r.interface.label(),
            r.tier.label(),
            invocation,
            r.latency_ms,
            assertion,
            actual,
            verdict
        ));
    }

    std::fs::create_dir_all("reports")?;
    std::fs::write(report_path, md)?;
    println!("Report written to: {}", report_path);
    Ok(())
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
            generate_markdown_report(
                "Phase 5.9.4A — Rust-Native Penta-Interface Real Semantic Assertions Suite (Page Operations)",
                &results,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4A.md"
            )?;
        }
        "security_forms" | "5.9.4B" => {
            let results = cases::security_forms::run_security_forms_suite().await?;
            println!("Security & forms finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.4B — Rust-Native Penta-Interface Real Semantic Assertions Suite (Security & Forms)",
                &results,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4B.md"
            )?;
        }
        "analysis" | "5.9.4C" => {
            let results = cases::analysis::run_analysis_suite().await?;
            println!("Analysis finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.4C — Rust-Native Penta-Interface Real Semantic Assertions Suite (Extraction & Analysis)",
                &results,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4C.md"
            )?;
        }
        "conversions" | "5.9.4D" => {
            let results = cases::conversions::run_conversions_suite().await?;
            println!("Conversions finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.4D — Rust-Native Penta-Interface Real Semantic Assertions Suite (Conversions)",
                &results,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4D.md"
            )?;
        }
        "edge_cases" => {
            let results = cases::edge_cases::run_edge_cases_suite().await?;
            println!("Edge cases finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.5 — Rust-Native Penta-Interface Real Semantic Assertions Suite (Edge Cases)",
                &results,
                "reports/PENTA_INTERFACE_E2E_EDGE_CASES_REPORT.md"
            )?;
        }
        "bench" => {
            let results = cases::bench::run_benchmarks().await?;
            println!("Bench finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.7 — Multimodal PDF-to-JSON Benchmark",
                &results,
                "reports/PDF_TO_JSON_MULTIMODAL_BENCH.md",
            )?;
        }
        _ => {
            println!("Running all suites...");
            let a = cases::page_ops::run_page_ops_suite().await?;
            println!("Page ops finished: {} tests", a.len());
            generate_markdown_report(
                "Phase 5.9.4A — Rust-Native Penta-Interface Real Semantic Assertions Suite (Page Operations)",
                &a,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4A.md",
            )?;

            let b = cases::security_forms::run_security_forms_suite().await?;
            println!("Security & forms finished: {} tests", b.len());
            generate_markdown_report(
                "Phase 5.9.4B — Rust-Native Penta-Interface Real Semantic Assertions Suite (Security & Forms)",
                &b,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4B.md",
            )?;

            let c = cases::analysis::run_analysis_suite().await?;
            println!("Analysis finished: {} tests", c.len());
            generate_markdown_report(
                "Phase 5.9.4C — Rust-Native Penta-Interface Real Semantic Assertions Suite (Extraction & Analysis)",
                &c,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4C.md",
            )?;

            let d = cases::conversions::run_conversions_suite().await?;
            println!("Conversions finished: {} tests", d.len());
            generate_markdown_report(
                "Phase 5.9.4D — Rust-Native Penta-Interface Real Semantic Assertions Suite (Conversions)",
                &d,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4D.md",
            )?;

            let mut all = Vec::new();
            all.extend(a);
            all.extend(b);
            all.extend(c);
            all.extend(d);
            generate_markdown_report(
                "Master Phase 5.9.4 — Rust-Native Penta-Interface Real Semantic Assertions Suite (All 44 Tools, 880 Tests)",
                &all,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT.md",
            )?;
        }
    }

    Ok(())
}
