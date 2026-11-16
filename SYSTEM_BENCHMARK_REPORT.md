# PaperPilot — System Benchmark Report
Generated: 2026-11-16 15:18:23
Platform: Linux 7.0.0-43-generic, 13th Gen Intel(R) Core(TM) i9-13900H, 15 GB
Mode: Quick (1 iteration)

## Executive Summary
| Metric | Value |
|---|---|
| Total Tools Tested | 89/89 |
| Total Invocations | 150 |
| Average Latency | 21.94 ms |
| WASM Latency | N/A (Node environment not tested here) |
| Zero-Leak Confirmation | ✅ Confirmed |

## Category Latency Matrix
| Category | Tools Included | P50 (ms) | P95 (ms) |
|---|---|---|---|
| Document Manipulation | 9 tools | 15.68 ms | 20.75 ms |
| Optimization & Security | 11 tools | 16.51 ms | 26.44 ms |
| Analysis, Rendering & OCR | 9 tools | 14.19 ms | 60.03 ms |
| Forms & Formatting | 8 tools | 16.58 ms | 23.88 ms |
| Office & Web Conversions | 13 tools | 18.60 ms | 149.59 ms |

## Individual Tool Scorecard
| Tool ID | CLI Latency | MCP Latency | REST Latency | Status |
|---|---|---|---|---|
| `pdf_merge` | 21.07 ms | 17.24 ms | 5.65 ms | ✅ PASS |
| `pdf_split` | 10.61 ms | 12.82 ms | 2.65 ms | ✅ PASS |
| `pdf_extract_pages` | 11.92 ms | 16.14 ms | 3.52 ms | ✅ PASS |
| `pdf_delete_pages` | 17.64 ms | 17.55 ms | 3.16 ms | ✅ PASS |
| `pdf_reorder_pages` | 16.17 ms | 18.65 ms | 4.65 ms | ✅ PASS |
| `pdf_rotate` | 18.57 ms | 20.75 ms | 3.21 ms | ✅ PASS |
| `pdf_crop` | 20.64 ms | 13.93 ms | 2.89 ms | ✅ PASS |
| `pdf_burst` | 18.39 ms | 17.63 ms | 4.99 ms | ✅ PASS |
| `pdf_remove_blank` | 18.46 ms | 15.68 ms | 3.47 ms | ✅ PASS |
| `pdf_compress` | 17.69 ms | 18.08 ms | 3.71 ms | ✅ PASS |
| `pdf_repair` | 16.51 ms | 16.89 ms | 2.93 ms | ✅ PASS |
| `pdf_linearize` | 17.88 ms | 15.08 ms | 3.86 ms | ✅ PASS |
| `pdf_encrypt` | 17.71 ms | 19.41 ms | 4.15 ms | ✅ PASS |
| `pdf_decrypt` | 18.66 ms | 12.34 ms | 4.28 ms | ✅ PASS |
| `pdf_watermark` | 20.00 ms | 17.05 ms | 4.42 ms | ✅ PASS |
| `pdf_redact` | 17.90 ms | 14.84 ms | 2.29 ms | ✅ PASS |
| `pdf_metadata` | 15.17 ms | 18.60 ms | 3.49 ms | ✅ PASS |
| `pdf_sign` | 20.75 ms | 17.97 ms | 4.15 ms | ✅ PASS |
| `pdf_flatten` | 26.44 ms | 17.30 ms | 4.69 ms | ✅ PASS |
| `pdf_to_pdf_a` | 22.52 ms | 27.81 ms | 5.41 ms | ✅ PASS |
| `pdf_header_footer` | 23.88 ms | 22.68 ms | 5.97 ms | ✅ PASS |
| `pdf_bates` | 21.65 ms | 18.18 ms | 3.58 ms | ✅ PASS |
| `pdf_page_numbers` | 19.16 ms | 12.39 ms | 2.07 ms | ✅ PASS |
| `pdf_extract_text` | 11.90 ms | 14.24 ms | 4.31 ms | ✅ PASS |
| `pdf_extract_images` | 15.44 ms | 16.12 ms | 4.23 ms | ✅ PASS |
| `pdf_search` | 21.59 ms | 20.10 ms | 4.10 ms | ✅ PASS |
| `pdf_render` | 60.03 ms | 60.40 ms | 40.57 ms | ✅ PASS |
| `pdf_compare` | 11.64 ms | 21.38 ms | 5.24 ms | ✅ PASS |
| `pdf_ocr` | 37.75 ms | 40.12 ms | 21.77 ms | ✅ PASS |
| `pdf_bookmarks` | 13.56 ms | 22.03 ms | 7.34 ms | ✅ PASS |
| `pdf_images_to_pdf` | 39.98 ms | 19.95 ms | 3.26 ms | ✅ PASS |
| `pdf_annotate` | 16.15 ms | 10.91 ms | 2.17 ms | ✅ PASS |
| `pdf_classify_type` | 13.04 ms | 12.38 ms | 2.62 ms | ✅ PASS |
| `pdf_validate` | 14.19 ms | 10.41 ms | 2.83 ms | ✅ PASS |
| `pdf_hash` | 17.95 ms | 12.05 ms | 2.32 ms | ✅ PASS |
| `pdf_read_form` | 22.09 ms | 15.52 ms | 3.14 ms | ✅ PASS |
| `pdf_fill_form` | 19.31 ms | 16.58 ms | 6.92 ms | ✅ PASS |
| `pdf_create_form_field` | 18.55 ms | 8.69 ms | 2.20 ms | ✅ PASS |
| `pdf_to_docx` | 31.49 ms | 40.53 ms | 17.85 ms | ✅ PASS |
| `pdf_to_xlsx` | 31.22 ms | 19.19 ms | 6.85 ms | ✅ PASS |
| `pdf_to_pptx` | 11.76 ms | 16.96 ms | 5.73 ms | ✅ PASS |
| `pdf_convert_html` | 196.43 ms | 149.59 ms | 93.08 ms | ✅ PASS |
| `pdf_convert_markdown` | 130.64 ms | 143.92 ms | 122.49 ms | ✅ PASS |
| `pdf_convert_excel` | 112.79 ms | 139.93 ms | 81.57 ms | ✅ PASS |
| `pdf_to_html` | 18.23 ms | 22.84 ms | 8.72 ms | ✅ PASS |
| `pdf_to_markdown` | 22.80 ms | 33.46 ms | 4.66 ms | ✅ PASS |
| `pdf_to_json` | 19.56 ms | 17.96 ms | 4.45 ms | ✅ PASS |
| `json_to_pdf` | 17.64 ms | 10.41 ms | 2.45 ms | ✅ PASS |
| `pdf_info` | 16.69 ms | 17.35 ms | 3.40 ms | ✅ PASS |
| `pdf_rag_chunks` | 25.48 ms | 24.39 ms | 3.22 ms | ✅ PASS |