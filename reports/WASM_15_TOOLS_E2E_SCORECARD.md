# WASM 15 Tools E2E Scorecard

## Executive Summary
- **Total Tested:** 15
- **Passed:** 15
- **Failed:** 0
- **Environment:** Headless Chromium + WebAssembly (`wasm32-unknown-unknown`) + Web Worker

## Operations Status

| Tool Name | Status | Web Worker & WASM Latency (ms) | Output Size (bytes) | Verification Notes |
|-----------|--------|--------------------------------|---------------------|-------------------|
| `merge` | ✅ PASS | 136 | 1866 | Valid %PDF- header |
| `split` | ✅ PASS | 94 | 925 | Valid %PDF- header |
| `rotate` | ✅ PASS | 77 | 553 | Valid %PDF- header |
| `compress` | ✅ PASS | 96 | 1414 | Valid %PDF- header |
| `encrypt` | ✅ PASS | 60 | 750 | Valid %PDF- header |
| `watermark` | ✅ PASS | 49 | 736 | Valid %PDF- header |
| `delete_pages` | ✅ PASS | 53 | 1292 | Valid %PDF- header |
| `extract_pages` | ✅ PASS | 47 | 925 | Valid %PDF- header |
| `reorder_pages` | ✅ PASS | 53 | 1414 | Valid %PDF- header |
| `crop` | ✅ PASS | 57 | 566 | Valid %PDF- header |
| `flatten` | ✅ PASS | 54 | 543 | Valid %PDF- header |
| `set_metadata` | ✅ PASS | 89 | 612 | Valid %PDF- header |
| `images_to_pdf` | ✅ PASS | 57 | 618 | Valid %PDF- header |
| `extract_images` | ✅ PASS | 50 | 72 | Valid image extracted |
| `pdf_hash` | ✅ PASS | 41 | 64 | Valid %PDF- header |
