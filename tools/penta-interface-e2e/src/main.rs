#![allow(warnings)]
#![allow(clippy::all)]

mod assertions;
mod cases;
mod fixtures;
mod runner;

use anyhow::Result;
use clap::Parser;
use runner::TestExecutionResult;
use std::path::Path;

pub fn verify_output_file(path: &Path, expected_ext: &str) -> Result<bool> {
    if !path.exists() || std::fs::metadata(path)?.len() == 0 {
        return Ok(false);
    }
    let content = std::fs::read(path)?;
    if content.is_empty() {
        return Ok(false);
    }
    match expected_ext {
        "pdf" => Ok(content.starts_with(b"%PDF")),
        "xlsx" | "docx" => Ok(content.starts_with(b"PK")),
        "md" => {
            let s = String::from_utf8_lossy(&content);
            Ok(s.contains('|') || s.contains('#'))
        }
        "json" => Ok(serde_json::from_slice::<serde_json::Value>(&content).is_ok()),
        "csv" => {
            let s = String::from_utf8_lossy(&content);
            Ok(s.lines().count() > 1 && (s.contains(',') || s.contains(';')))
        }
        _ => Ok(true),
    }
}

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

    let workspace_root = std::env::current_dir().unwrap();
    let out_dir = workspace_root.join("tests/e2e_fixtures/out/penta_e2e");
    if out_dir.exists() {
        let _ = std::fs::remove_dir_all(&out_dir);
    }
    std::fs::create_dir_all(&out_dir).expect("Failed to create clean penta_e2e out directory");

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
        "table" | "5.9.58" => {
            let results = cases::table::run_table_suite().await?;
            println!("Table extraction finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.58 — Rust-Native Penta-Interface Real Semantic Assertions Suite (Tabular Extraction)",
                &results,
                "reports/PENTA_INTERFACE_E2E_TABLE_REPORT.md"
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
        "workflow" => {
            let results = cases::workflow::run_workflow_suite().await?;
            println!("Workflow finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.69C — Penta-Interface Declarative Workflow Engine & Recipe Execution",
                &results,
                "reports/PENTA_INTERFACE_E2E_WORKFLOW_REPORT.md",
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
        "pii" | "5.9.60" => {
            let results = cases::pii::run_pii_suite().await?;
            println!("PII finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.60 — Rust-Native Penta-Interface Real Semantic Assertions Suite (PII Scan)",
                &results,
                "reports/PENTA_INTERFACE_E2E_PII_SCAN_REPORT.md"
            )?;
        }
        "sanitize" | "5.9.59" => {
            let results = cases::sanitize::run_sanitize_suite().await?;
            println!("Sanitize finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.59 — Rust-Native Penta-Interface Real Semantic Assertions Suite (Sanitize)",
                &results,
                "reports/PENTA_INTERFACE_E2E_SANITIZE_REPORT.md"
            )?;
        }
        "pdf_validate_signatures" | "5.9.61" => {
            let results = cases::pdf_validate_signatures::run_pdf_validate_signatures_suite().await?;
            println!("Validate Digital Signatures finished: {} tests", results.len());
            generate_markdown_report(
                "Phase 5.9.61 — Rust-Native Penta-Interface Real Semantic Assertions Suite (Validate Digital Signatures)",
                &results,
                "reports/PENTA_INTERFACE_E2E_PDF_VALIDATE_SIGNATURES_REPORT.md"
            )?;
        }
        "bench" => {
            let results = cases::bench::run_benchmark_suite().await?;
            println!("Benchmarking suite completed.");
            let mm_results = cases::bench::run_multimodal_benchmarks().await?;
            println!("Multimodal bench finished: {} tests", mm_results.len());
            generate_markdown_report(
                "Phase 5.9.7 — Multimodal PDF-to-JSON Benchmark",
                &mm_results,
                "reports/PDF_TO_JSON_MULTIMODAL_BENCH.md",
            )?;
        }
        "pdf_booklet" => {
            let res_pdf_booklet = cases::pdf_booklet::run_pdf_booklet_suite().await?;
            println!("Booklet Imposition finished: {} tests", res_pdf_booklet.len());
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Booklet Imposition)",
                &res_pdf_booklet,
                "reports/PENTA_INTERFACE_E2E_PDF_BOOKLET_REPORT.md",
            )?;
        }
        "pdf_booklet" => {
            let res = cases::pdf_booklet::run_pdf_booklet_suite().await?;
            println!("Booklet Imposition finished: {} tests", res.len());
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Booklet Imposition)",
                &res,
                "reports/PENTA_INTERFACE_E2E_PDF_BOOKLET_REPORT.md",
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

            let pii = cases::pii::run_pii_suite().await?;
            println!("PII finished: {} tests", pii.len());
            generate_markdown_report(
                "Phase 5.9.60 — Rust-Native Penta-Interface Real Semantic Assertions Suite (PII Scan)",
                &pii,
                "reports/PENTA_INTERFACE_E2E_PII_SCAN_REPORT.md",
            )?;

            let s = cases::sanitize::run_sanitize_suite().await?;
            println!("Sanitize finished: {} tests", s.len());
            generate_markdown_report(
                "Phase 5.9.59 — Rust-Native Penta-Interface Real Semantic Assertions Suite (Sanitize)",
                &s,
                "reports/PENTA_INTERFACE_E2E_SANITIZE_REPORT.md",
            )?;

            let generated_res =
                cases::pdf_validate_signatures::run_pdf_validate_signatures_suite().await?;
            println!(
                "Validate Digital Signatures finished: {} tests",
                generated_res.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Validate Digital Signatures)",
                &generated_res,
                "reports/PENTA_INTERFACE_E2E_PDF_VALIDATE_SIGNATURES_REPORT.md",
            )?;

            let res_pdf_booklet = cases::pdf_booklet::run_pdf_booklet_suite().await?;
            println!("Booklet Imposition finished: {} tests", res_pdf_booklet.len());
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Booklet Imposition)",
                &res_pdf_booklet,
                "reports/PENTA_INTERFACE_E2E_PDF_BOOKLET_REPORT.md",
            )?;

            let mut all = Vec::new();
            all.extend(a);
            all.extend(b);
            all.extend(c);
            all.extend(d);
            all.extend(s);
            all.extend(pii);
            all.extend(res_pdf_booklet);
            all.extend(generated_res);
            generate_markdown_report(
                "Master Phase 5.9.4 — Rust-Native Penta-Interface Real Semantic Assertions Suite (All 44 Tools, 880 Tests)",
                &all,
                "reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT.md",
            )?;
        }
    }

    Ok(())
}
