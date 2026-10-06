# WASM 22 Tools E2E Scorecard

## Executive Summary
- **Total Tested:** 22
- **Passed:** 22
- **Failed:** 0
- **Environment:** Headless Chromium + WebAssembly (`wasm32-unknown-unknown`) + Web Worker

## Operations Status

| Tool Name | Status | Web Worker & WASM Latency (ms) | Output Size (bytes) | Verification Notes |
|-----------|--------|--------------------------------|---------------------|-------------------|
| `merge` | ✅ PASS | 98 | 1866 | Valid output generated |
| `split` | ✅ PASS | 86 | 925 | Valid output generated |
| `rotate` | ✅ PASS | 70 | 553 | Valid output generated |
| `compress` | ✅ PASS | 75 | 1414 | Valid output generated |
| `encrypt` | ✅ PASS | 74 | 750 | Valid output generated |
| `watermark` | ✅ PASS | 82 | 736 | Valid output generated |
| `delete_pages` | ✅ PASS | 78 | 1292 | Valid output generated |
| `extract_pages` | ✅ PASS | 67 | 925 | Valid output generated |
| `reorder_pages` | ✅ PASS | 108 | 1414 | Valid output generated |
| `crop` | ✅ PASS | 70 | 566 | Valid output generated |
| `flatten` | ✅ PASS | 70 | 543 | Valid output generated |
| `set_metadata` | ✅ PASS | 70 | 612 | Valid output generated |
| `images_to_pdf` | ✅ PASS | 65 | 618 | Valid output generated |
| `extract_images` | ✅ PASS | 50 | 72 | Valid image extracted |
| `render_page` | ✅ PASS | 96 | 11502 | Valid output generated |
| `extract_text` | ✅ PASS | 73 | 17 | Valid output generated |
| `decrypt` | ✅ PASS | 5 | 190 | Valid output generated |
| `page_numbers` | ✅ PASS | 49 | 737 | Valid output generated |
| `header_footer` | ✅ PASS | 68 | 848 | Valid output generated |
| `pdf_info` | ✅ PASS | 54 | 82 | Valid output generated |
| `ocr` | ✅ PASS | 9 | 0 | Valid output generated |
| `pdf_hash` | ✅ PASS | 63 | 64 | Valid output generated |
