# Spec 060: Pure-Rust 1-Hour Sustained Soak & Endurance Benchmark Harness (`paperpilot-bench`)

## 1. Overview & Motivation
Currently, our 1-hour sustained soak benchmark and telemetry reporting is implemented in Python (`scripts/stress_test.py` and `scripts/test_tri_interface_e2e.py`).

While the Python script executes the compiled Rust binary via subprocess, having the benchmark harness in Python has notable downsides:
1. **Subprocess Overhead**: Process invocation overhead in Python skews high-frequency micro-benchmark latencies.
2. **External Runtime Dependency**: Running benchmarks in minimal CI/CD or Alpine Docker containers requires installing Python 3 and its libraries.
3. **No Direct In-Engine Memory Profiling**: Python cannot directly measure memory allocator statistics (e.g. jemalloc / mimalloc / system allocator stats) inside the PaperPilot engine.
4. **Pure-Rust Philosophy**: PaperPilot is 100% Rust across core, PDF engine, CLI, MCP, and Gateway. The endurance benchmark harness should also be an authentic, pure-Rust CLI tool in `tools/penta-bench` or `tools/soak-bench`.

---

## 2. Requirements & Architecture

### Requirement 1: New Rust Crate `tools/soak-bench` (or embedded in `tools/penta-interface-e2e`)
- Implement a dedicated benchmark CLI binary:
  ```bash
  cargo run --release -p soak-bench -- --duration 3600
  ```
- Flags supported:
  - `--duration <SECONDS>` (default: 3600, or 60 for quick sanity tests)
  - `--output <PATH>` (default: `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md`)
  - `--wiki <PATH>` (default: `wiki/19-One-Hour-Endurance-Benchmark.md`)
  - `--concurrency <N>` (default: 1, sequential cycle test)

### Requirement 2: All 50 Tools Coverage
- The runner must define and execute all 50 operations covering:
  - Page Manipulation (merge, split, rotate, delete, reorder, crop, extract, burst, remove-blank)
  - Content Transformation (watermark, header/footer, bates, page-numbers)
  - Security & Metadata (encrypt, decrypt, sign, flatten, redact, metadata, hash, validate)
  - Document Conversions (pdf_to_docx, pdf_to_xlsx, pdf_to_html, pdf_to_markdown, pdf_to_json, json_to_pdf, excel_to_json, excel_to_csv, office_to_markdown, office_extract_text, office_modernize_legacy)
  - Search & Analysis (pdf_search, pdf_compare, pdf_classify_type, pdf_bookmarks, pdf_info, pdf_rag_chunks, workspace_search)
  - Image Operations (pdf_images_to_pdf, pdf_render, extract_images)
  - Optimization (compress, linearize, repair)

### Requirement 3: High-Precision Statistics & Telemetry
- Measure time using `std::time::Instant` with sub-microsecond precision.
- Record:
  - Total cycles, total successful operations, error count (must be 0).
  - Exact percentiles per tool and globally: Min, Mean, Median (P50), P90, P95, P99, P99.9, Max.
  - Process RSS memory sampling: On Linux via `/proc/self/statm` or `libc::getrusage`, on macOS via `mach_task_basic_info`, on Windows via `GetProcessMemoryInfo`.
  - Throughput (operations per second).

### Requirement 4: Markdown Report Generation
- Format and write `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md` and `wiki/19-One-Hour-Endurance-Benchmark.md` matching the existing table format.
- Deprecate / retire `scripts/stress_test.py` once the pure-Rust harness is in place.

---

## 3. Success Criteria
- [ ] `cargo run --release -p soak-bench -- --duration 60` completes with 100% success rate across all 50 tools.
- [ ] Latency percentiles (P50, P90, P95, P99, P99.9) are calculated with high statistical accuracy in pure Rust.
- [ ] Memory RSS stability is tracked and reported.
- [ ] Generates clean Markdown report at `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md`.
- [ ] Zero dependency on Python runtime.
