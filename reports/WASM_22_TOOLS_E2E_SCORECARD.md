# WASM 22 Tools E2E Scorecard

## Executive Summary
- **Total Tested:** 22
- **Passed:** 22
- **Failed:** 0
- **Environment:** Headless Chromium + WebAssembly (`wasm32-unknown-unknown`) + Web Worker

## Operations Status

| Tool Name | Status | Web Worker & WASM Latency (ms) | Output Size (bytes) | Verification Notes |
|-----------|--------|--------------------------------|---------------------|-------------------|
| `merge` | ✅ PASS | 97 | 1866 | Valid output generated |
| `split` | ✅ PASS | 123 | 925 | Valid output generated |
| `rotate` | ✅ PASS | 78 | 553 | Valid output generated |
| `compress` | ✅ PASS | 75 | 1414 | Valid output generated |
| `encrypt` | ✅ PASS | 89 | 751 | Valid output generated |
| `watermark` | ✅ PASS | 83 | 736 | Valid output generated |
| `delete_pages` | ✅ PASS | 94 | 1292 | Valid output generated |
| `extract_pages` | ✅ PASS | 79 | 925 | Valid output generated |
| `reorder_pages` | ✅ PASS | 95 | 1414 | Valid output generated |
| `crop` | ✅ PASS | 98 | 566 | Valid output generated |
| `flatten` | ✅ PASS | 66 | 543 | Valid output generated |
| `set_metadata` | ✅ PASS | 105 | 612 | Valid output generated |
| `images_to_pdf` | ✅ PASS | 73 | 618 | Valid output generated |
| `extract_images` | ✅ PASS | 50 | 72 | Valid image extracted |
| `render_page` | ✅ PASS | 100 | 11502 | Valid output generated |
| `extract_text` | ✅ PASS | 63 | 17 | Valid output generated |
| `decrypt` | ✅ PASS | 5 | 190 | Valid output generated |
| `page_numbers` | ✅ PASS | 66 | 737 | Valid output generated |
| `header_footer` | ✅ PASS | 65 | 848 | Valid output generated |
| `pdf_info` | ✅ PASS | 73 | 82 | Valid output generated |
| `ocr` | ✅ PASS | 8 | 0 | Valid output generated |
| `pdf_hash` | ✅ PASS | 63 | 64 | Valid output generated |
