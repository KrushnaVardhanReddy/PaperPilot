# Spec 025: 1-Hour Sustained Soak, Stress & Endurance Benchmark (Post Zero-Chrome)

## Status: APPROVED / IN IMPLEMENTATION (Phase 5.7)

## 1. Executive Summary & Purpose
Following the migration from `headless_chrome` to pure-Rust vector engines (`genpdf`, `typst`, `fulgur` in Spec 024), PaperPilot operates 100% in pure Rust across all 44 operations.

To guarantee that PaperPilot satisfies enterprise-grade reliability, SLA uptime, and zero-degradation criteria, this specification defines a **1-Hour (3,600 Seconds) Sustained Soak & Endurance Test** covering **ALL 44 operations** across the complete tool suite.

The objective is to:
1. Subject the full suite of **all 44 operations** to ~250,000+ continuous multi-cycle executions without pause.
2. Measure long-term **Resident Set Size (RSS)** memory stability to mathematically confirm zero memory leaks, heap bloat, or memory fragmentation.
3. Quantify throughput consistency over time (proving zero CPU throttling or resource exhaustion).
4. Measure true statistical latency percentiles (**P50, P90, P95, P99, and P99.9 tail latencies**) individually for **every single one of the 44 tools**.

---

## 2. Test Architecture & Methodology

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. SUSTAINED SOAK HARNESS: scripts/stress_test.py (3,600s / 1 Hour)         │
│    • Complete Coverage: Loops across ALL 44 distinct operation definitions  │
│      imported directly from test_tri_interface_e2e.py                       │
│    • Ingestion: 50 fixture files, 129 PDF pages, all document formats       │
│    • Execution layer: Compiled release binary (target/release/paperpilot-cli)│
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. CONTINUOUS TELEMETRY TRACKING (Every 60 Seconds)                         │
│    • Process RSS memory (MB) via /proc/self/status                          │
│    • Total cycles and successful operations count                           │
│    • Instantaneous throughput (ops/sec)                                     │
│    • Error count and unexpected stderr output detection                     │
├─────────────────────────────────────────────────────────────────────────────┤
│ 3. POST-MIGRATION LATENCY DISTRIBUTIONS                                     │
│    • Full percentile evaluation: Min, P50, P90, P95, P99, P99.9, Max        │
│    • Verification that Excel, Markdown, and HTML conversions stay < 50ms    │
├─────────────────────────────────────────────────────────────────────────────┤
│ 4. AUTOMATED DELIVERABLE                                                    │
│    • reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md                         │
│    • wiki/19-One-Hour-Endurance-Benchmark.md                                │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Exit Criteria & Pass Requirements

1. **Zero Failures**: 100.00% success rate across all operations over the entire 60-minute duration.
2. **Zero Memory Leaks**: Process RSS must remain flat and bounded (no continuous upward memory trajectory).
3. **Sub-50ms Universal Execution**: All operations (including conversions) must achieve sub-50ms average latencies.
4. **Stable Throughput**: Sustained throughput must not degrade by more than 5% between minute 1 and minute 60.
