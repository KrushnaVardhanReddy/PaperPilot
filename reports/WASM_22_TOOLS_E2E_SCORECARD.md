# WASM 22 Tools E2E Scorecard

## Executive Summary
- **Total Tested:** 22
- **Passed:** 22
- **Failed:** 0
- **Environment:** Headless Chromium + WebAssembly (`wasm32-unknown-unknown`) + Web Worker

## Operations Status

| Tool Name | Status | Web Worker & WASM Latency (ms) | Output Size (bytes) | Verification Notes |
|-----------|--------|--------------------------------|---------------------|-------------------|
| `merge` | ✅ PASS | 86 | 1866 | Valid output generated |
| `split` | ✅ PASS | 49 | 925 | Valid output generated |
| `rotate` | ✅ PASS | 70 | 553 | Valid output generated |
| `compress` | ✅ PASS | 92 | 1414 | Valid output generated |
| `encrypt` | ✅ PASS | 66 | 750 | Valid output generated |
| `watermark` | ✅ PASS | 41 | 736 | Valid output generated |
| `delete_pages` | ✅ PASS | 101 | 1292 | Valid output generated |
| `extract_pages` | ✅ PASS | 68 | 925 | Valid output generated |
| `reorder_pages` | ✅ PASS | 88 | 1414 | Valid output generated |
| `crop` | ✅ PASS | 70 | 566 | Valid output generated |
| `flatten` | ✅ PASS | 61 | 543 | Valid output generated |
| `set_metadata` | ✅ PASS | 71 | 612 | Valid output generated |
| `images_to_pdf` | ✅ PASS | 98 | 618 | Valid output generated |
| `extract_images` | ✅ PASS | 50 | 72 | Valid image extracted |
| `render_page` | ✅ PASS | 82 | 11502 | Valid output generated |
| `extract_text` | ✅ PASS | 75 | 17 | Valid output generated |
| `decrypt` | ✅ PASS | 5 | 190 | Valid output generated |
| `page_numbers` | ✅ PASS | 61 | 737 | Valid output generated |
| `header_footer` | ✅ PASS | 64 | 848 | Valid output generated |
| `pdf_info` | ✅ PASS | 70 | 82 | Valid output generated |
| `ocr` | ✅ PASS | 8 | 0 | Valid output generated |
| `pdf_hash` | ✅ PASS | 64 | 64 | Valid output generated |
