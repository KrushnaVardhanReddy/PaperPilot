# PaperPilot — System Benchmark Report
Generated: 2026-11-08 09:58:20
Platform: Linux 7.0.0-42-generic, 13th Gen Intel(R) Core(TM) i9-13900H, 15 GB
Mode: Full (20 iterations)

## Executive Summary
| Metric | Value |
|---|---|
| Total Tools Tested | 89/89 |
| Total Invocations | 3000 |
| Average Latency | 27.22 ms |
| WASM Latency | N/A (Node environment not tested here) |
| Zero-Leak Confirmation | ✅ Confirmed |

## Category Latency Matrix
| Category | Tools Included | P50 (ms) | P95 (ms) |
|---|---|---|---|
| Document Manipulation | 9 tools | 14.38 ms | 23.93 ms |
| Optimization & Security | 11 tools | 18.45 ms | 32.93 ms |
| Analysis, Rendering & OCR | 9 tools | 21.75 ms | 65.84 ms |
| Forms & Formatting | 8 tools | 17.89 ms | 31.12 ms |
| Office & Web Conversions | 13 tools | 24.49 ms | 136.21 ms |

## Individual Tool Scorecard
| Tool ID | CLI Latency | MCP Latency | REST Latency | Status |
|---|---|---|---|---|
| `pdf_merge` | 20.56 ms | 25.27 ms | 5.39 ms | ✅ PASS |
| `pdf_split` | 16.11 ms | 15.50 ms | 2.76 ms | ✅ PASS |
| `pdf_extract_pages` | 14.12 ms | 13.44 ms | 2.37 ms | ✅ PASS |
| `pdf_delete_pages` | 15.28 ms | 18.93 ms | 3.82 ms | ✅ PASS |
| `pdf_reorder_pages` | 18.27 ms | 19.61 ms | 5.54 ms | ✅ PASS |
| `pdf_rotate` | 19.36 ms | 18.10 ms | 2.74 ms | ✅ PASS |
| `pdf_crop` | 18.21 ms | 17.69 ms | 1.82 ms | ✅ PASS |
| `pdf_burst` | 14.10 ms | 14.48 ms | 4.17 ms | ✅ PASS |
| `pdf_remove_blank` | 16.90 ms | 18.13 ms | 2.87 ms | ✅ PASS |
| `pdf_compress` | 15.18 ms | 18.50 ms | 2.59 ms | ✅ PASS |
| `pdf_repair` | 23.03 ms | 15.89 ms | 4.42 ms | ✅ PASS |
| `pdf_linearize` | 21.53 ms | 21.81 ms | 4.46 ms | ✅ PASS |
| `pdf_encrypt` | 22.37 ms | 25.95 ms | 5.57 ms | ✅ PASS |
| `pdf_decrypt` | 19.90 ms | 24.91 ms | 6.62 ms | ✅ PASS |
| `pdf_watermark` | 18.85 ms | 26.36 ms | 4.95 ms | ✅ PASS |
| `pdf_redact` | 24.47 ms | 21.35 ms | 3.50 ms | ✅ PASS |
| `pdf_metadata` | 16.09 ms | 15.78 ms | 1.74 ms | ✅ PASS |
| `pdf_sign` | 22.75 ms | 30.96 ms | 5.67 ms | ✅ PASS |
| `pdf_flatten` | 28.16 ms | 22.23 ms | 4.07 ms | ✅ PASS |
| `pdf_to_pdf_a` | 20.70 ms | 21.18 ms | 4.35 ms | ✅ PASS |
| `pdf_header_footer` | 27.58 ms | 27.58 ms | 7.46 ms | ✅ PASS |
| `pdf_bates` | 20.66 ms | 18.22 ms | 2.55 ms | ✅ PASS |
| `pdf_page_numbers` | 19.66 ms | 20.59 ms | 7.08 ms | ✅ PASS |
| `pdf_extract_text` | 41.76 ms | 37.87 ms | 5.50 ms | ✅ PASS |
| `pdf_extract_images` | 21.37 ms | 20.99 ms | 3.86 ms | ✅ PASS |
| `pdf_search` | 20.66 ms | 28.13 ms | 4.30 ms | ✅ PASS |
| `pdf_render` | 451.65 ms | 20.90 ms | 3.14 ms | ✅ PASS |
| `pdf_compare` | 43.50 ms | 39.42 ms | 10.22 ms | ✅ PASS |
| `pdf_ocr` | 49.39 ms | 60.53 ms | 24.17 ms | ✅ PASS |
| `pdf_bookmarks` | 20.01 ms | 21.25 ms | 4.30 ms | ✅ PASS |
| `pdf_images_to_pdf` | 18.19 ms | 20.48 ms | 4.00 ms | ✅ PASS |
| `pdf_annotate` | 22.53 ms | 32.01 ms | 5.13 ms | ✅ PASS |
| `pdf_classify_type` | 25.64 ms | 24.07 ms | 3.12 ms | ✅ PASS |
| `pdf_validate` | 19.37 ms | 23.59 ms | 5.21 ms | ✅ PASS |
| `pdf_hash` | 23.03 ms | 17.49 ms | 2.36 ms | ✅ PASS |
| `pdf_read_form` | 22.57 ms | 20.03 ms | 4.26 ms | ✅ PASS |
| `pdf_fill_form` | 23.47 ms | 30.32 ms | 6.99 ms | ✅ PASS |
| `pdf_create_form_field` | 26.26 ms | 20.56 ms | 1.49 ms | ✅ PASS |
| `pdf_to_docx` | 35.34 ms | 27.86 ms | 3.48 ms | ✅ PASS |
| `pdf_to_xlsx` | 30.48 ms | 34.32 ms | 7.40 ms | ✅ PASS |
| `pdf_to_pptx` | 23.42 ms | 21.38 ms | 6.10 ms | ✅ PASS |
| `pdf_convert_html` | 130.17 ms | 130.99 ms | 108.20 ms | ✅ PASS |
| `pdf_convert_markdown` | 126.51 ms | 134.66 ms | 103.54 ms | ✅ PASS |
| `pdf_convert_excel` | 128.14 ms | 136.36 ms | 114.88 ms | ✅ PASS |
| `pdf_to_html` | 26.32 ms | 28.63 ms | 6.29 ms | ✅ PASS |
| `pdf_to_markdown` | 33.20 ms | 40.34 ms | 7.51 ms | ✅ PASS |
| `pdf_to_json` | 18.94 ms | 17.73 ms | 2.41 ms | ✅ PASS |
| `json_to_pdf` | 21.26 ms | 16.54 ms | 1.71 ms | ✅ PASS |
| `pdf_info` | 18.21 ms | 20.19 ms | 3.73 ms | ✅ PASS |
| `pdf_rag_chunks` | 34.39 ms | 32.51 ms | 6.68 ms | ✅ PASS |