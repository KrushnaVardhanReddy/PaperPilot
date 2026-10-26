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
            let results =
                cases::pdf_validate_signatures::run_pdf_validate_signatures_suite().await?;
            println!(
                "Validate Digital Signatures finished: {} tests",
                results.len()
            );
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
            println!(
                "Booklet Imposition finished: {} tests",
                res_pdf_booklet.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Booklet Imposition)",
                &res_pdf_booklet,
                "reports/PENTA_INTERFACE_E2E_PDF_BOOKLET_REPORT.md",
            )?;
        }
        "pdf_auto_rotate" | "5.3.10" => {
            let res = cases::pdf_auto_rotate::run_pdf_auto_rotate_suite().await?;
            println!("Auto Upright Orientation finished: {} tests", res.len());
            generate_markdown_report(
                "Phase 5.3.10 — Rust-Native Penta-Interface Real Semantic Assertions Suite (Auto Upright Orientation)",
                &res,
                "reports/PENTA_INTERFACE_E2E_PDF_AUTO_ROTATE_REPORT.md",
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
            println!(
                "Booklet Imposition finished: {} tests",
                res_pdf_booklet.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Booklet Imposition)",
                &res_pdf_booklet,
                "reports/PENTA_INTERFACE_E2E_PDF_BOOKLET_REPORT.md",
            )?;

            let res_pdf_nup = cases::pdf_nup::run_pdf_nup_suite().await?;
            println!("N-Up Page Imposition finished: {} tests", res_pdf_nup.len());
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (N-Up Page Imposition)",
                &res_pdf_nup,
                "reports/PENTA_INTERFACE_E2E_PDF_NUP_REPORT.md",
            )?;

            let res_pdf_scale_pages = cases::pdf_scale_pages::run_pdf_scale_pages_suite().await?;
            println!(
                "Scale Page Dimensions finished: {} tests",
                res_pdf_scale_pages.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Scale Page Dimensions)",
                &res_pdf_scale_pages,
                "reports/PENTA_INTERFACE_E2E_PDF_SCALE_PAGES_REPORT.md",
            )?;

            let res_pdf_remove_signatures =
                cases::pdf_remove_signatures::run_pdf_remove_signatures_suite().await?;
            println!(
                "Remove Digital Signatures finished: {} tests",
                res_pdf_remove_signatures.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Remove Digital Signatures)",
                &res_pdf_remove_signatures,
                "reports/PENTA_INTERFACE_E2E_PDF_REMOVE_SIGNATURES_REPORT.md",
            )?;

            let res_pdf_stamp = cases::pdf_stamp::run_pdf_stamp_suite().await?;
            println!("Stamp Image & Logo finished: {} tests", res_pdf_stamp.len());
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Stamp Image & Logo)",
                &res_pdf_stamp,
                "reports/PENTA_INTERFACE_E2E_PDF_STAMP_REPORT.md",
            )?;

            let res_pdf_overlay_underlay =
                cases::pdf_overlay_underlay::run_pdf_overlay_underlay_suite().await?;
            println!(
                "Overlay & Underlay Stationary finished: {} tests",
                res_pdf_overlay_underlay.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Overlay & Underlay Stationary)",
                &res_pdf_overlay_underlay,
                "reports/PENTA_INTERFACE_E2E_PDF_OVERLAY_UNDERLAY_REPORT.md",
            )?;

            let res_pdf_auto_rotate = cases::pdf_auto_rotate::run_pdf_auto_rotate_suite().await?;
            println!(
                "Auto Upright Orientation finished: {} tests",
                res_pdf_auto_rotate.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Auto Upright Orientation)",
                &res_pdf_auto_rotate,
                "reports/PENTA_INTERFACE_E2E_PDF_AUTO_ROTATE_REPORT.md",
            )?;

            let res_pdf_stitch_canvas =
                cases::pdf_stitch_canvas::run_pdf_stitch_canvas_suite().await?;
            println!(
                "Stitch Continuous Canvas finished: {} tests",
                res_pdf_stitch_canvas.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Stitch Continuous Canvas)",
                &res_pdf_stitch_canvas,
                "reports/PENTA_INTERFACE_E2E_PDF_STITCH_CANVAS_REPORT.md",
            )?;

            let res_pdf_attachments = cases::pdf_attachments::run_pdf_attachments_suite().await?;
            println!(
                "PDF Portfolio Attachments finished: {} tests",
                res_pdf_attachments.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (PDF Portfolio Attachments)",
                &res_pdf_attachments,
                "reports/PENTA_INTERFACE_E2E_PDF_ATTACHMENTS_REPORT.md",
            )?;

            let res_pdf_timestamp = cases::pdf_timestamp::run_pdf_timestamp_suite().await?;
            println!(
                "Cryptographic Timestamp TSA finished: {} tests",
                res_pdf_timestamp.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Cryptographic Timestamp TSA)",
                &res_pdf_timestamp,
                "reports/PENTA_INTERFACE_E2E_PDF_TIMESTAMP_REPORT.md",
            )?;

            let res_pdf_change_permissions =
                cases::pdf_change_permissions::run_pdf_change_permissions_suite().await?;
            println!(
                "Modify PDF Permissions finished: {} tests",
                res_pdf_change_permissions.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Modify PDF Permissions)",
                &res_pdf_change_permissions,
                "reports/PENTA_INTERFACE_E2E_PDF_CHANGE_PERMISSIONS_REPORT.md",
            )?;

            let res_pdf_split_scanned_book =
                cases::pdf_split_scanned_book::run_pdf_split_scanned_book_suite().await?;
            println!(
                "Split Scanned Book Spine finished: {} tests",
                res_pdf_split_scanned_book.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Split Scanned Book Spine)",
                &res_pdf_split_scanned_book,
                "reports/PENTA_INTERFACE_E2E_PDF_SPLIT_SCANNED_BOOK_REPORT.md",
            )?;

            let res_pdf_comic_archive =
                cases::pdf_comic_archive::run_pdf_comic_archive_suite().await?;
            println!(
                "Comic Archive Conversion finished: {} tests",
                res_pdf_comic_archive.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Comic Archive Conversion)",
                &res_pdf_comic_archive,
                "reports/PENTA_INTERFACE_E2E_PDF_COMIC_ARCHIVE_REPORT.md",
            )?;

            let res_pdf_split_by_outline =
                cases::pdf_split_by_outline::run_pdf_split_by_outline_suite().await?;
            println!(
                "Split PDF by Outline finished: {} tests",
                res_pdf_split_by_outline.len()
            );
            generate_markdown_report(
                "Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Split PDF by Outline)",
                &res_pdf_split_by_outline,
                "reports/PENTA_INTERFACE_E2E_PDF_SPLIT_BY_OUTLINE_REPORT.md",
            )?;

            let mut all = Vec::new();
            all.extend(a);
            all.extend(b);
            all.extend(c);
            all.extend(d);
            all.extend(s);
            all.extend(pii);
            all.extend(res_pdf_split_by_outline);
            all.extend(res_pdf_comic_archive);
            all.extend(res_pdf_split_scanned_book);
            all.extend(res_pdf_change_permissions);
            all.extend(res_pdf_timestamp);
            all.extend(res_pdf_attachments);
            all.extend(res_pdf_stitch_canvas);
            all.extend(res_pdf_auto_rotate);
            all.extend(res_pdf_overlay_underlay);
            all.extend(res_pdf_stamp);
            all.extend(res_pdf_remove_signatures);
            all.extend(res_pdf_scale_pages);
            all.extend(res_pdf_nup);
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
