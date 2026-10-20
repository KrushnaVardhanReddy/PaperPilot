# 19. One-Hour Endurance Benchmark

## Overview
This wiki page documents the **1-Hour (3,600 Seconds) Continuous Soak & Endurance Benchmark** performed on the pure-Rust release binary post-`headless_chrome` migration.
The goal of this benchmark is to ensure sustained throughput, stable tail latencies, and zero memory leaks over an extended period.

## Methodology
The `soak-bench` test harness ran continuously for 3600 seconds.
During every cycle, it executes distinct operations representing all major tool categories:
- Document manipulation (merge, split, crop, burst, etc.)
- Optimization & Security (compress, encrypt, watermark, redact, sign, etc.)
- Analysis & OCR (extract text, OCR, search, classify, validate, etc.)
- Formatting & Forms (bates, annotations, read/fill forms, etc.)
- Pure-Rust Conversions (HTML, Markdown, Office to PDF).

## Results Summary
- **Duration**: 3600.0 Seconds
- **Total Cycles**: 340834
- **Total Operations Executed**: 8180016
- **Throughput**: 2272.23 ops/sec
- **Success Rate**: 100.00%
- **Memory Stability**: Zero memory leaks observed. Memory remained stable over the duration.
- **Crash Rate**: 0 panics or unexpected exits.

## Telemetry
The script captured latencies (Mean, P50, P90, P95, P99, P99.9, Min, and Max) across all tools natively. Memory (Peak RSS) was sampled every 60 seconds.
For full tabular breakdown of latencies, refer to `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md`.
