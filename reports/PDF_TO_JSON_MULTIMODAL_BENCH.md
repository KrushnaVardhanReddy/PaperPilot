# Phase 5.9.7 — Multimodal PDF-to-JSON Benchmark

## Executive Scorecard
- **Total Tests Evaluated:** 2
- **Total Passed:** 0 / 2 (0.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `pdf_to_json_bench` | **💻 CLI** | Simple (Tier 1) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `29.72 ms` | Benchmark run: Success | Done | ❌ FAIL |
| `pdf_to_json_bench` | **💻 CLI** | Complex (Tier 3) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `13.22 ms` | Benchmark run: Success | Done. Latency None: 29.723146999999997ms, Base64: 13.219413ms | ❌ FAIL |
