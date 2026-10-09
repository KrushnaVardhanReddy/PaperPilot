use crate::runner::{cli::CliRunner, ComplexityTier, InterfaceType, TestExecutionResult};
use anyhow::Result;

pub async fn run_benchmarks() -> Result<Vec<TestExecutionResult>> {
    let mut results = Vec::new();
    let out_dir = std::path::PathBuf::from("target/e2e_output/bench");
    std::fs::create_dir_all(&out_dir)?;

    let out_path_none = out_dir.join("pdf_to_json_bench_none.json");
    let (success_none, _, latency_none) = CliRunner::run(&[
        "convert",
        "--format",
        "json",
        "--input",
        "tests/e2e_fixtures/real/multimodal.pdf",
        "--output",
        out_path_none.to_str().unwrap(),
        "--image-mode",
        "none",
    ])
    .await?;

    results.push(TestExecutionResult {
        tool_id: "pdf_to_json_bench".into(),
        interface: InterfaceType::Cli,
        tier: ComplexityTier::Simple,
        data_sent: "Bench none".into(),
        assertion_checked: "Benchmark run".into(),
        expected_result: "Success".into(),
        actual_result: "Done".into(),
        passed: success_none,
        latency_ms: latency_none,
    });

    let out_path_b64 = out_dir.join("pdf_to_json_bench_base64.json");
    let (success_b64, _, latency_b64) = CliRunner::run(&[
        "convert",
        "--format",
        "json",
        "--input",
        "tests/e2e_fixtures/real/multimodal.pdf",
        "--output",
        out_path_b64.to_str().unwrap(),
        "--image-mode",
        "base64",
    ])
    .await?;

    results.push(TestExecutionResult {
        tool_id: "pdf_to_json_bench".into(),
        interface: InterfaceType::Cli,
        tier: ComplexityTier::Complex,
        data_sent: "Bench base64".into(),
        assertion_checked: "Benchmark run".into(),
        expected_result: "Success".into(),
        actual_result: format!(
            "Done. Latency None: {}ms, Base64: {}ms",
            latency_none, latency_b64
        ),
        passed: success_b64,
        latency_ms: latency_b64,
    });

    Ok(results)
}
