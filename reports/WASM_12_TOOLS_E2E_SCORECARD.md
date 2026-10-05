# WASM 12 Tools E2E Scorecard

## Executive Summary
- **Total Tested:** 12
- **Passed:** 12
- **Failed:** 0
- **Environment:** Headless Chromium + WebAssembly (`wasm32-unknown-unknown`) + Web Worker

## Operations Status

| Tool Name | Status | Web Worker & WASM Latency (ms) | Output Size (bytes) | Verification Notes |
|-----------|--------|--------------------------------|---------------------|-------------------|
| `merge` | ✅ PASS | 46 | 1866 | Valid %PDF- header |
| `split` | ✅ PASS | 43 | 925 | Valid %PDF- header |
| `rotate` | ✅ PASS | 35 | 553 | Valid %PDF- header |
| `compress` | ✅ PASS | 38 | 1414 | Valid %PDF- header |
| `encrypt` | ✅ PASS | 38 | 750 | Valid %PDF- header |
| `watermark` | ✅ PASS | 37 | 736 | Valid %PDF- header |
| `delete_pages` | ✅ PASS | 36 | 1292 | Valid %PDF- header |
| `extract_pages` | ✅ PASS | 42 | 925 | Valid %PDF- header |
| `reorder_pages` | ✅ PASS | 36 | 1414 | Valid %PDF- header |
| `crop` | ✅ PASS | 37 | 566 | Valid %PDF- header |
| `flatten` | ✅ PASS | 36 | 543 | Valid %PDF- header |
| `set_metadata` | ✅ PASS | 33 | 612 | Valid %PDF- header |
