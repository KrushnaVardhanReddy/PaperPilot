# Wave 1 Core Launch Suite Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 12
- **Total Passed:** 12

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |
|-----------|-----------|--------------|---------------------|--------|-------|
| `pdf_merge` | CLI | 16.0 | 957 | ✅ PASS |  |
| `pdf_split` | CLI | 23.0 | 1046 | ✅ PASS |  |
| `pdf_rotate` | CLI | 19.0 | 553 | ✅ PASS |  |
| `pdf_delete_pages` | CLI | 19.0 | 966 | ✅ PASS |  |
| `pdf_reorder_pages` | CLI | 12.0 | 1414 | ✅ PASS |  |
| `pdf_compress` | CLI | 26.0 | 4742 | ✅ PASS |  |
| `pdf_watermark` | CLI | 15.0 | 887 | ✅ PASS |  |
| `pdf_ocr` | CLI | 16.0 | 0 | ✅ PASS | Handled missing model gracefully |
| `pdf_extract_text` | CLI | 26.0 | 17 | ✅ PASS |  |
| `pdf_to_docx` | CLI | 21.0 | 18364 | ✅ PASS |  |
| `excel_to_json` | CLI | 14.0 | 129 | ✅ PASS |  |
| `excel_to_csv` | CLI | 12.0 | 28 | ✅ PASS |  |
