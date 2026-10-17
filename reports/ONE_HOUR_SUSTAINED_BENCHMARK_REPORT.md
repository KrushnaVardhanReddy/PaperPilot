# PaperPilot — 1-Hour Sustained Soak & Endurance Benchmark Report
Generated: 2026-10-16 19:43:43
Test Type: 1-Hour Continuous Multi-Cycle Stress & Memory Stability Test

---

## 1. Executive Summary

| Metric | Measured Value | Analysis / Verdict |
|---|---|---|
| **Test Duration** | **3600.3 seconds** (~60.0 minutes) | Continuous sustained stress across 50 tools |
| **Total Cycles Completed** | **2,641 cycles** | Continuous multi-operation loop |
| **Total Operations Executed** | **132,050 operations** | Verified across 50 operation types |
| **Success Rate** | **100.00%** (132,050/132,050) | **100% Stability — Zero Crashes / Panics** |
| **Sustained Throughput** | **36.7 ops / second** | High throughput without throttling |
| **Overall Mean Latency** | **27.26 ms** | Core operation performance |
| **P50 Latency (Median)** | **19.66 ms** | Typical responsive latency |
| **P90 Latency** | **49.84 ms** | 90% of requests finish in under 49.8 ms |
| **P95 Latency** | **74.64 ms** | 95% of requests finish in under 74.6 ms |
| **P99 Latency** | **81.00 ms** | Predictable tail execution |
| **P99.9 Latency** | **85.79 ms** | Deep tail execution |
| **Min / Max Latency** | **14.28 ms / 153.92 ms** | Bounded execution time |

---

## 2. Per-Operation Latency Breakdown (132,050 Total Executions)

| Operation | Invocations | Mean | P50 (Median) | P90 | P95 | P99 | P99.9 | Min | Max |
|---|---|---|---|---|---|---|---|---|---|
| `pdf_merge` | 2,641 | 20.66 ms | 20.52 ms | 21.56 ms | 21.91 ms | 23.20 ms | 28.82 ms | 18.49 ms | 41.36 ms |
| `pdf_split` | 2,641 | 20.16 ms | 20.05 ms | 21.18 ms | 21.56 ms | 22.85 ms | 30.23 ms | 18.18 ms | 43.74 ms |
| `pdf_extract_pages` | 2,641 | 19.35 ms | 19.23 ms | 20.32 ms | 20.73 ms | 21.84 ms | 28.19 ms | 16.78 ms | 50.03 ms |
| `pdf_delete_pages` | 2,641 | 19.36 ms | 19.22 ms | 20.37 ms | 20.78 ms | 21.93 ms | 26.51 ms | 17.27 ms | 127.73 ms |
| `pdf_reorder_pages` | 2,641 | 19.37 ms | 19.28 ms | 20.36 ms | 20.75 ms | 21.85 ms | 26.47 ms | 16.93 ms | 36.04 ms |
| `pdf_rotate` | 2,641 | 18.70 ms | 18.58 ms | 19.69 ms | 20.06 ms | 21.06 ms | 25.81 ms | 16.51 ms | 43.49 ms |
| `pdf_crop` | 2,641 | 18.77 ms | 18.68 ms | 19.79 ms | 20.18 ms | 21.28 ms | 24.46 ms | 16.71 ms | 25.99 ms |
| `pdf_burst` | 2,641 | 20.60 ms | 20.50 ms | 21.75 ms | 22.21 ms | 23.72 ms | 27.35 ms | 17.98 ms | 37.93 ms |
| `pdf_remove_blank` | 2,641 | 19.24 ms | 19.12 ms | 20.27 ms | 20.75 ms | 22.25 ms | 27.33 ms | 17.06 ms | 30.27 ms |
| `pdf_compress` | 2,641 | 19.90 ms | 19.80 ms | 20.89 ms | 21.30 ms | 22.40 ms | 26.93 ms | 17.88 ms | 28.64 ms |
| `pdf_repair` | 2,641 | 18.55 ms | 18.43 ms | 19.51 ms | 19.92 ms | 21.17 ms | 25.61 ms | 16.55 ms | 49.62 ms |
| `pdf_linearize` | 2,641 | 19.81 ms | 19.72 ms | 20.83 ms | 21.24 ms | 22.42 ms | 26.75 ms | 17.82 ms | 27.56 ms |
| `pdf_encrypt` | 2,641 | 20.73 ms | 20.61 ms | 21.74 ms | 22.20 ms | 23.35 ms | 28.41 ms | 18.83 ms | 30.72 ms |
| `pdf_decrypt` | 2,641 | 22.63 ms | 22.50 ms | 23.53 ms | 23.87 ms | 25.18 ms | 34.04 ms | 20.79 ms | 60.34 ms |
| `pdf_watermark` | 2,641 | 18.93 ms | 18.82 ms | 19.86 ms | 20.23 ms | 21.44 ms | 26.56 ms | 16.77 ms | 28.68 ms |
| `pdf_redact` | 2,641 | 18.74 ms | 18.66 ms | 19.75 ms | 20.14 ms | 20.96 ms | 25.79 ms | 16.89 ms | 26.80 ms |
| `pdf_metadata` | 2,641 | 18.64 ms | 18.53 ms | 19.63 ms | 20.03 ms | 21.05 ms | 25.00 ms | 16.05 ms | 31.90 ms |
| `pdf_sign` | 2,641 | 18.59 ms | 18.49 ms | 19.59 ms | 19.97 ms | 21.30 ms | 24.66 ms | 16.67 ms | 26.33 ms |
| `pdf_flatten` | 2,641 | 18.83 ms | 18.71 ms | 19.84 ms | 20.22 ms | 21.59 ms | 27.19 ms | 16.77 ms | 40.15 ms |
| `pdf_to_pdf_a` | 2,641 | 18.58 ms | 18.48 ms | 19.55 ms | 19.95 ms | 21.09 ms | 23.81 ms | 16.39 ms | 29.61 ms |
| `pdf_header_footer` | 2,641 | 19.34 ms | 19.20 ms | 20.40 ms | 20.79 ms | 22.56 ms | 27.53 ms | 16.96 ms | 62.77 ms |
| `pdf_bates` | 2,641 | 19.36 ms | 19.25 ms | 20.37 ms | 20.82 ms | 21.98 ms | 27.10 ms | 17.47 ms | 38.80 ms |
| `pdf_page_numbers` | 2,641 | 19.31 ms | 19.22 ms | 20.31 ms | 20.72 ms | 21.98 ms | 25.26 ms | 17.26 ms | 29.69 ms |
| `pdf_extract_text` | 2,641 | 47.15 ms | 46.83 ms | 48.56 ms | 49.21 ms | 52.33 ms | 75.53 ms | 44.24 ms | 101.11 ms |
| `pdf_extract_images` | 2,641 | 19.29 ms | 19.17 ms | 20.09 ms | 20.44 ms | 21.74 ms | 26.73 ms | 17.47 ms | 61.60 ms |
| `pdf_search` | 2,641 | 18.88 ms | 18.77 ms | 19.76 ms | 20.12 ms | 21.37 ms | 26.37 ms | 16.67 ms | 60.11 ms |
| `pdf_render` | 2,641 | 47.16 ms | 46.82 ms | 48.56 ms | 49.29 ms | 53.06 ms | 71.62 ms | 44.53 ms | 76.32 ms |
| `pdf_compare` | 2,641 | 51.13 ms | 50.83 ms | 52.77 ms | 53.35 ms | 56.34 ms | 68.50 ms | 47.58 ms | 80.01 ms |
| `pdf_ocr` | 2,641 | 81.13 ms | 80.65 ms | 83.27 ms | 84.34 ms | 89.82 ms | 117.99 ms | 76.55 ms | 153.92 ms |
| `pdf_bookmarks` | 2,641 | 20.74 ms | 20.56 ms | 21.70 ms | 22.21 ms | 23.81 ms | 31.04 ms | 19.23 ms | 49.17 ms |
| `pdf_images_to_pdf` | 2,641 | 19.20 ms | 19.09 ms | 19.97 ms | 20.33 ms | 21.37 ms | 25.90 ms | 17.42 ms | 31.95 ms |
| `pdf_annotate` | 2,641 | 19.06 ms | 18.91 ms | 20.02 ms | 20.43 ms | 21.59 ms | 26.78 ms | 17.39 ms | 28.08 ms |
| `pdf_classify_type` | 2,641 | 22.21 ms | 22.07 ms | 23.23 ms | 23.64 ms | 25.02 ms | 30.17 ms | 20.18 ms | 35.76 ms |
| `pdf_validate` | 2,641 | 18.15 ms | 18.05 ms | 19.08 ms | 19.47 ms | 20.49 ms | 23.49 ms | 16.24 ms | 44.87 ms |
| `pdf_hash` | 2,641 | 15.88 ms | 15.81 ms | 16.61 ms | 16.94 ms | 17.59 ms | 21.77 ms | 14.28 ms | 36.72 ms |
| `pdf_read_form` | 2,641 | 18.74 ms | 18.62 ms | 19.68 ms | 20.12 ms | 21.15 ms | 23.67 ms | 16.95 ms | 28.37 ms |
| `pdf_fill_form` | 2,641 | 19.19 ms | 19.10 ms | 20.20 ms | 20.59 ms | 21.73 ms | 25.22 ms | 16.88 ms | 26.32 ms |
| `pdf_create_form_field` | 2,641 | 18.97 ms | 18.86 ms | 19.98 ms | 20.37 ms | 21.50 ms | 24.89 ms | 16.89 ms | 27.39 ms |
| `pdf_to_docx` | 2,641 | 21.11 ms | 20.97 ms | 22.19 ms | 22.56 ms | 24.25 ms | 27.33 ms | 18.87 ms | 34.07 ms |
| `pdf_to_xlsx` | 2,641 | 36.52 ms | 36.29 ms | 37.84 ms | 38.42 ms | 41.02 ms | 51.69 ms | 33.89 ms | 55.37 ms |
| `pdf_to_pptx` | 2,641 | 20.86 ms | 20.76 ms | 21.77 ms | 22.20 ms | 23.08 ms | 27.39 ms | 18.86 ms | 53.34 ms |
| `pdf_convert_html` | 2,641 | 73.78 ms | 73.28 ms | 76.08 ms | 77.56 ms | 80.67 ms | 99.30 ms | 69.97 ms | 108.48 ms |
| `pdf_convert_markdown` | 2,641 | 74.40 ms | 73.83 ms | 76.95 ms | 78.29 ms | 83.18 ms | 93.88 ms | 70.50 ms | 103.86 ms |
| `pdf_convert_excel` | 2,641 | 77.61 ms | 77.01 ms | 79.90 ms | 81.61 ms | 85.83 ms | 110.03 ms | 74.03 ms | 116.03 ms |
| `pdf_to_html` | 2,641 | 24.36 ms | 24.21 ms | 25.31 ms | 25.71 ms | 27.16 ms | 34.00 ms | 22.46 ms | 41.78 ms |
| `pdf_to_markdown` | 2,641 | 47.28 ms | 47.03 ms | 48.69 ms | 49.36 ms | 51.70 ms | 66.29 ms | 44.32 ms | 117.41 ms |
| `pdf_to_json` | 2,641 | 19.22 ms | 19.08 ms | 19.99 ms | 20.34 ms | 21.56 ms | 25.69 ms | 17.56 ms | 65.15 ms |
| `json_to_pdf` | 2,641 | 16.95 ms | 16.85 ms | 17.71 ms | 18.02 ms | 19.26 ms | 25.00 ms | 15.31 ms | 65.69 ms |
| `pdf_info` | 2,641 | 18.77 ms | 18.63 ms | 19.68 ms | 20.11 ms | 21.60 ms | 25.79 ms | 16.98 ms | 52.53 ms |
| `pdf_rag_chunks` | 2,641 | 47.12 ms | 46.80 ms | 48.54 ms | 49.22 ms | 53.17 ms | 73.62 ms | 44.28 ms | 100.06 ms |

---

## 3. Resource Stability & Zero-Leak Verification

- **Memory Leak Analysis**: Measured Peak RSS using child process telemetry. Verified 0 memory leaks or continuous bloat over 60 minutes.
- **CPU & Thermal Stability**: Sustained **~36.7 ops/sec** consistently across the entire 1 hour with no degradation.

### Memory Stability Over Time

| Timestamp | Peak RSS (MB) |
|---|---|
| Minute 1 | 61.16 MB |
| Minute 2 | 61.16 MB |
| Minute 3 | 61.16 MB |
| Minute 4 | 61.16 MB |
| Minute 5 | 61.17 MB |
| Minute 6 | 61.17 MB |
| Minute 7 | 61.26 MB |
| Minute 8 | 61.26 MB |
| Minute 9 | 61.26 MB |
| Minute 10 | 61.26 MB |
| Minute 11 | 61.26 MB |
| Minute 12 | 61.26 MB |
| Minute 13 | 61.26 MB |
| Minute 14 | 61.26 MB |
| Minute 15 | 61.26 MB |
| Minute 16 | 61.26 MB |
| Minute 17 | 61.26 MB |
| Minute 18 | 61.26 MB |
| Minute 19 | 61.26 MB |
| Minute 20 | 61.26 MB |
| Minute 21 | 61.26 MB |
| Minute 22 | 61.26 MB |
| Minute 23 | 61.26 MB |
| Minute 24 | 61.26 MB |
| Minute 25 | 61.26 MB |
| Minute 26 | 61.26 MB |
| Minute 27 | 61.26 MB |
| Minute 28 | 61.26 MB |
| Minute 29 | 61.26 MB |
| Minute 30 | 61.26 MB |
| Minute 31 | 61.26 MB |
| Minute 32 | 61.26 MB |
| Minute 33 | 61.26 MB |
| Minute 34 | 61.26 MB |
| Minute 35 | 61.26 MB |
| Minute 36 | 61.27 MB |
| Minute 37 | 61.27 MB |
| Minute 38 | 61.27 MB |
| Minute 39 | 61.27 MB |
| Minute 40 | 61.27 MB |
| Minute 41 | 61.27 MB |
| Minute 42 | 61.27 MB |
| Minute 43 | 61.27 MB |
| Minute 44 | 61.27 MB |
| Minute 45 | 61.27 MB |
| Minute 46 | 61.27 MB |
| Minute 47 | 61.27 MB |
| Minute 48 | 61.27 MB |
| Minute 49 | 61.27 MB |
| Minute 50 | 61.27 MB |
| Minute 51 | 61.27 MB |
| Minute 52 | 61.27 MB |
| Minute 53 | 61.27 MB |
| Minute 54 | 61.27 MB |
| Minute 55 | 61.27 MB |
| Minute 56 | 61.27 MB |
| Minute 57 | 61.27 MB |
| Minute 58 | 61.27 MB |
| Minute 59 | 61.27 MB |
| Minute 60 | 61.27 MB |
