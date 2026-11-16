# PaperPilot — System Benchmark Report
Generated: 2026-11-16 15:19:37
Platform: Linux 7.0.0-43-generic, 13th Gen Intel(R) Core(TM) i9-13900H, 15 GB
Mode: Full (20 iterations)

## Executive Summary
| Metric | Value |
|---|---|
| Total Tools Tested | 89/89 |
| Total Invocations | 3000 |
| Average Latency | 20.33 ms |
| WASM Latency | N/A (Node environment not tested here) |
| Zero-Leak Confirmation | ✅ Confirmed |

## Category Latency Matrix
| Category | Tools Included | P50 (ms) | P95 (ms) |
|---|---|---|---|
| Document Manipulation | 9 tools | 12.08 ms | 24.92 ms |
| Optimization & Security | 11 tools | 12.56 ms | 25.53 ms |
| Analysis, Rendering & OCR | 9 tools | 16.74 ms | 52.16 ms |
| Forms & Formatting | 8 tools | 14.62 ms | 28.10 ms |
| Office & Web Conversions | 13 tools | 18.46 ms | 126.58 ms |

## Individual Tool Scorecard
| Tool ID | CLI Latency | MCP Latency | REST Latency | Status |
|---|---|---|---|---|
| `pdf_merge` | 18.64 ms | 13.67 ms | 4.10 ms | ✅ PASS |
| `pdf_split` | 19.32 ms | 15.93 ms | 3.03 ms | ✅ PASS |
| `pdf_extract_pages` | 17.01 ms | 18.35 ms | 1.52 ms | ✅ PASS |
| `pdf_delete_pages` | 18.49 ms | 18.22 ms | 4.77 ms | ✅ PASS |
| `pdf_reorder_pages` | 18.95 ms | 14.13 ms | 4.13 ms | ✅ PASS |
| `pdf_rotate` | 19.97 ms | 20.43 ms | 1.46 ms | ✅ PASS |
| `pdf_crop` | 14.34 ms | 10.28 ms | 2.54 ms | ✅ PASS |
| `pdf_burst` | 17.56 ms | 13.40 ms | 2.10 ms | ✅ PASS |
| `pdf_remove_blank` | 10.82 ms | 18.17 ms | 4.19 ms | ✅ PASS |
| `pdf_compress` | 17.95 ms | 16.09 ms | 2.87 ms | ✅ PASS |
| `pdf_repair` | 17.85 ms | 20.46 ms | 1.17 ms | ✅ PASS |
| `pdf_linearize` | 18.46 ms | 22.94 ms | 2.67 ms | ✅ PASS |
| `pdf_encrypt` | 16.44 ms | 10.61 ms | 1.90 ms | ✅ PASS |
| `pdf_decrypt` | 14.92 ms | 17.51 ms | 4.22 ms | ✅ PASS |
| `pdf_watermark` | 11.36 ms | 9.86 ms | 2.75 ms | ✅ PASS |
| `pdf_redact` | 17.71 ms | 18.17 ms | 1.14 ms | ✅ PASS |
| `pdf_metadata` | 20.70 ms | 19.11 ms | 2.38 ms | ✅ PASS |
| `pdf_sign` | 19.33 ms | 16.05 ms | 3.96 ms | ✅ PASS |
| `pdf_flatten` | 17.65 ms | 19.68 ms | 3.67 ms | ✅ PASS |
| `pdf_to_pdf_a` | 20.01 ms | 15.37 ms | 1.55 ms | ✅ PASS |
| `pdf_header_footer` | 13.97 ms | 12.03 ms | 1.65 ms | ✅ PASS |
| `pdf_bates` | 13.91 ms | 24.74 ms | 4.43 ms | ✅ PASS |
| `pdf_page_numbers` | 18.60 ms | 17.22 ms | 1.33 ms | ✅ PASS |
| `pdf_extract_text` | 19.91 ms | 32.53 ms | 3.69 ms | ✅ PASS |
| `pdf_extract_images` | 19.77 ms | 16.45 ms | 4.67 ms | ✅ PASS |
| `pdf_search` | 17.09 ms | 20.41 ms | 3.30 ms | ✅ PASS |
| `pdf_render` | 49.70 ms | 50.95 ms | 32.41 ms | ✅ PASS |
| `pdf_compare` | 31.55 ms | 39.97 ms | 3.55 ms | ✅ PASS |
| `pdf_ocr` | 45.80 ms | 46.38 ms | 19.15 ms | ✅ PASS |
| `pdf_bookmarks` | 20.11 ms | 26.60 ms | 5.03 ms | ✅ PASS |
| `pdf_images_to_pdf` | 21.63 ms | 16.86 ms | 1.46 ms | ✅ PASS |
| `pdf_annotate` | 15.29 ms | 19.04 ms | 1.69 ms | ✅ PASS |
| `pdf_classify_type` | 21.80 ms | 20.24 ms | 2.18 ms | ✅ PASS |
| `pdf_validate` | 10.07 ms | 7.49 ms | 0.67 ms | ✅ PASS |
| `pdf_hash` | 15.44 ms | 12.49 ms | 1.17 ms | ✅ PASS |
| `pdf_read_form` | 19.28 ms | 22.30 ms | 3.39 ms | ✅ PASS |
| `pdf_fill_form` | 18.44 ms | 12.78 ms | 2.19 ms | ✅ PASS |
| `pdf_create_form_field` | 17.33 ms | 21.26 ms | 6.37 ms | ✅ PASS |
| `pdf_to_docx` | 27.60 ms | 34.78 ms | 8.70 ms | ✅ PASS |
| `pdf_to_xlsx` | 33.70 ms | 24.76 ms | 5.35 ms | ✅ PASS |
| `pdf_to_pptx` | 13.62 ms | 10.82 ms | 1.44 ms | ✅ PASS |
| `pdf_convert_html` | 118.08 ms | 115.84 ms | 90.75 ms | ✅ PASS |
| `pdf_convert_markdown` | 126.58 ms | 121.10 ms | 85.63 ms | ✅ PASS |
| `pdf_convert_excel` | 120.47 ms | 114.15 ms | 91.15 ms | ✅ PASS |
| `pdf_to_html` | 21.37 ms | 24.92 ms | 8.08 ms | ✅ PASS |
| `pdf_to_markdown` | 23.38 ms | 21.96 ms | 3.56 ms | ✅ PASS |
| `pdf_to_json` | 13.73 ms | 10.70 ms | 2.44 ms | ✅ PASS |
| `json_to_pdf` | 15.82 ms | 16.69 ms | 1.96 ms | ✅ PASS |
| `pdf_info` | 17.51 ms | 19.22 ms | 0.97 ms | ✅ PASS |
| `pdf_rag_chunks` | 19.10 ms | 20.44 ms | 1.96 ms | ✅ PASS |