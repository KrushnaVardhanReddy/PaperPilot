# PaperPilot — System Benchmark Report
Generated: 2026-11-07 10:42:13
Platform: Linux 6.8.0, Intel(R) Xeon(R) Processor @ 2.30GHz, 7 GB
Mode: Quick (1 iteration)

## Executive Summary
| Metric | Value |
|---|---|
| Total Tools Tested | 89/89 |
| Total Invocations | 150 |
| Average Latency | 23.89 ms |
| WASM Latency | N/A (Node environment not tested here) |
| Zero-Leak Confirmation | ✅ Confirmed |

## Category Latency Matrix
| Category | Tools Included | P50 (ms) | P95 (ms) |
|---|---|---|---|
| Document Manipulation | 9 tools | 20.05 ms | 62.81 ms |
| Optimization & Security | 11 tools | 20.44 ms | 30.91 ms |
| Analysis, Rendering & OCR | 9 tools | 19.89 ms | 60.68 ms |
| Forms & Formatting | 8 tools | 19.68 ms | 27.93 ms |
| Office & Web Conversions | 13 tools | 19.90 ms | 81.28 ms |

## Individual Tool Scorecard
| Tool ID | CLI Latency | MCP Latency | REST Latency | Status |
|---|---|---|---|---|
| `pdf_merge` | 33.75 ms | 292.58 ms | 62.81 ms | ❌ FAIL |
| `pdf_split` | 26.05 ms | 21.74 ms | 6.64 ms | ❌ FAIL |
| `pdf_extract_pages` | 24.61 ms | 20.05 ms | 7.58 ms | ❌ FAIL |
| `pdf_delete_pages` | 30.77 ms | 19.92 ms | 4.82 ms | ❌ FAIL |
| `pdf_reorder_pages` | 29.86 ms | 19.43 ms | 6.38 ms | ❌ FAIL |
| `pdf_rotate` | 23.49 ms | 19.02 ms | 6.78 ms | ❌ FAIL |
| `pdf_crop` | 23.26 ms | 19.61 ms | 4.63 ms | ❌ FAIL |
| `pdf_burst` | 25.58 ms | 27.44 ms | 6.97 ms | ❌ FAIL |
| `pdf_remove_blank` | 27.44 ms | 19.88 ms | 5.58 ms | ❌ FAIL |
| `pdf_compress` | 29.56 ms | 19.28 ms | 6.22 ms | ❌ FAIL |
| `pdf_repair` | 30.91 ms | 20.44 ms | 4.49 ms | ❌ FAIL |
| `pdf_linearize` | 24.42 ms | 27.04 ms | 5.05 ms | ❌ FAIL |
| `pdf_encrypt` | 25.65 ms | 19.00 ms | 6.05 ms | ❌ FAIL |
| `pdf_decrypt` | 26.58 ms | 27.47 ms | 8.36 ms | ❌ FAIL |
| `pdf_watermark` | 23.29 ms | 30.95 ms | 4.64 ms | ❌ FAIL |
| `pdf_redact` | 26.30 ms | 24.37 ms | 4.38 ms | ❌ FAIL |
| `pdf_metadata` | 22.79 ms | 18.63 ms | 4.17 ms | ❌ FAIL |
| `pdf_sign` | 22.88 ms | 19.61 ms | 4.27 ms | ❌ FAIL |
| `pdf_flatten` | 23.19 ms | 19.75 ms | 4.30 ms | ❌ FAIL |
| `pdf_to_pdf_a` | 22.49 ms | 20.68 ms | 4.25 ms | ❌ FAIL |
| `pdf_header_footer` | 23.27 ms | 18.63 ms | 4.54 ms | ❌ FAIL |
| `pdf_bates` | 33.41 ms | 19.88 ms | 4.82 ms | ❌ FAIL |
| `pdf_page_numbers` | 27.93 ms | 19.72 ms | 4.84 ms | ❌ FAIL |
| `pdf_extract_text` | 60.15 ms | 18.81 ms | 30.23 ms | ❌ FAIL |
| `pdf_extract_images` | 24.52 ms | 21.31 ms | 4.58 ms | ❌ FAIL |
| `pdf_search` | 23.26 ms | 19.02 ms | 4.53 ms | ❌ FAIL |
| `pdf_render` | 49.03 ms | 20.07 ms | 5.85 ms | ❌ FAIL |
| `pdf_compare` | 52.96 ms | 19.89 ms | 10.71 ms | ❌ FAIL |
| `pdf_ocr` | 102.26 ms | 19.72 ms | 60.68 ms | ❌ FAIL |
| `pdf_bookmarks` | 25.30 ms | 19.98 ms | 5.58 ms | ❌ FAIL |
| `pdf_images_to_pdf` | 26.10 ms | 19.17 ms | 7.11 ms | ❌ FAIL |
| `pdf_annotate` | 20.47 ms | 19.50 ms | 4.58 ms | ❌ FAIL |
| `pdf_classify_type` | 26.66 ms | 19.86 ms | 7.72 ms | ❌ FAIL |
| `pdf_validate` | 23.14 ms | 19.29 ms | 4.10 ms | ❌ FAIL |
| `pdf_hash` | 20.65 ms | 19.35 ms | 3.06 ms | ❌ FAIL |
| `pdf_read_form` | 23.47 ms | 19.68 ms | 3.98 ms | ❌ FAIL |
| `pdf_fill_form` | 23.62 ms | 18.86 ms | 4.34 ms | ❌ FAIL |
| `pdf_create_form_field` | 23.58 ms | 19.39 ms | 4.12 ms | ❌ FAIL |
| `pdf_to_docx` | 53.54 ms | 19.40 ms | 8.48 ms | ❌ FAIL |
| `pdf_to_xlsx` | 40.95 ms | 19.38 ms | 19.63 ms | ❌ FAIL |
| `pdf_to_pptx` | 24.76 ms | 19.41 ms | 5.55 ms | ❌ FAIL |
| `pdf_convert_html` | 102.42 ms | 19.80 ms | 56.13 ms | ❌ FAIL |
| `pdf_convert_markdown` | 77.19 ms | 19.90 ms | 53.70 ms | ❌ FAIL |
| `pdf_convert_excel` | 81.28 ms | 19.29 ms | 57.45 ms | ❌ FAIL |
| `pdf_to_html` | 28.97 ms | 19.69 ms | 3.46 ms | ❌ FAIL |
| `pdf_to_markdown` | 60.38 ms | 20.59 ms | 3.13 ms | ❌ FAIL |
| `pdf_to_json` | 23.19 ms | 18.80 ms | 4.55 ms | ❌ FAIL |
| `json_to_pdf` | 22.40 ms | 20.09 ms | 3.30 ms | ❌ FAIL |
| `pdf_info` | 23.46 ms | 18.76 ms | 3.66 ms | ❌ FAIL |
| `pdf_rag_chunks` | 48.41 ms | 19.97 ms | 6.49 ms | ❌ FAIL |