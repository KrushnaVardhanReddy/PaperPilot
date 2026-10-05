# WASM 12 Tools E2E Scorecard

## Executive Summary
- **Total Tested:** 12
- **Passed:** 12
- **Failed:** 0
- **Environment:** Headless Chromium + WebAssembly (`wasm32-unknown-unknown`) + Web Worker

## Operations Status

| Tool Name | Status | Web Worker & WASM Latency (ms) | Output Size (bytes) | Verification Notes |
|-----------|--------|--------------------------------|---------------------|-------------------|
| `merge` | ✅ PASS | 52 | 1866 | Valid %PDF- header |
| `split` | ✅ PASS | 34 | 925 | Valid %PDF- header |
| `rotate` | ✅ PASS | 21 | 553 | Valid %PDF- header |
| `compress` | ✅ PASS | 40 | 1414 | Valid %PDF- header |
| `encrypt` | ✅ PASS | 36 | 751 | Valid %PDF- header |
| `watermark` | ✅ PASS | 33 | 736 | Valid %PDF- header |
| `delete_pages` | ✅ PASS | 35 | 1292 | Valid %PDF- header |
| `extract_pages` | ✅ PASS | 18 | 925 | Valid %PDF- header |
| `reorder_pages` | ✅ PASS | 25 | 1414 | Valid %PDF- header |
| `crop` | ✅ PASS | 36 | 566 | Valid %PDF- header |
| `flatten` | ✅ PASS | 28 | 543 | Valid %PDF- header |
| `set_metadata` | ✅ PASS | 42 | 612 | Valid %PDF- header |
