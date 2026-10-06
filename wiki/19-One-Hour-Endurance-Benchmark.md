# 19. One-Hour Endurance Benchmark

## Overview
This wiki page documents the **1-Hour (3,600 Seconds) Continuous Soak & Endurance Benchmark** performed on the pure-Rust release binary post-`headless_chrome` migration.
The goal of this benchmark is to ensure sustained throughput, stable tail latencies, and zero memory leaks over an extended period.

## Methodology
The `stress_test.py` test harness ran continuously for 3,600 seconds.
During every cycle, it executes 44 distinct operations representing all major tool categories:
- Document manipulation (merge, split, crop, burst, etc.)
- Optimization & Security (compress, encrypt, watermark, redact, sign, etc.)
- Analysis & OCR (extract text, OCR, search, classify, validate, etc.)
- Formatting & Forms (bates, annotations, read/fill forms, etc.)
- Pure-Rust Conversions (HTML, Markdown, Office to PDF).

## Results Summary
- **Duration**: 1 Hour (3,600 seconds)
- **Total Cycles**: 8,043
- **Total Operations Executed**: 353,892
- **Throughput**: 98.30 ops/sec
- **Success Rate**: 100.00%
- **Memory Stability**: Zero memory leaks observed. Memory remained stable over the 60-minute duration.
- **Crash Rate**: 0 panics or unexpected exits.

## Telemetry
The script captured latencies (Mean, P50, P90, P95, P99, P99.9, Min, and Max) across all 44 tools natively. Memory (Peak RSS) was sampled every 60 seconds.
For full tabular breakdown of latencies, refer to `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md`.
