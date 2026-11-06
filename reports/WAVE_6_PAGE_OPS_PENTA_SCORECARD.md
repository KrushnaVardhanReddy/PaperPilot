# Wave 6 Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 80
- **Total Passed:** 62

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_merge` | CLI | 377.9 | 957 | ✅ PASS |
| `pdf_merge` | MCP | 4.4 | 1071 | ✅ PASS |
| `pdf_merge` | API | 3.4 | 0 | ❌ FAIL |
| `pdf_merge` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | CLI | 25.7 | 957 | ✅ PASS |
| `pdf_merge_alternate` | MCP | 2.5 | 957 | ✅ PASS |
| `pdf_merge_alternate` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_merge_alternate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split` | CLI | 18.9 | 1 | ✅ PASS |
| `pdf_split` | MCP | 6.3 | 1 | ✅ PASS |
| `pdf_split` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_split` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | CLI | 24.3 | 72538 | ✅ PASS |
| `pdf_split_by_outline` | MCP | 5.6 | 72538 | ✅ PASS |
| `pdf_split_by_outline` | API | 1.1 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | CLI | 22.7 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | MCP | 4.1 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | API | 0.9 | 0 | ❌ FAIL |
| `pdf_split_scanned_book` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_rotate` | CLI | 22.1 | 72430 | ✅ PASS |
| `pdf_rotate` | MCP | 4.3 | 72430 | ✅ PASS |
| `pdf_rotate` | API | 1.1 | 0 | ❌ FAIL |
| `pdf_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | CLI | 22.0 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | MCP | 6.5 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | API | 1.5 | 0 | ❌ FAIL |
| `pdf_auto_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_crop` | CLI | 25.0 | 72422 | ✅ PASS |
| `pdf_crop` | MCP | 5.7 | 72422 | ✅ PASS |
| `pdf_crop` | API | 1.8 | 0 | ❌ FAIL |
| `pdf_crop` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_crop` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_scale_pages` | CLI | 34.9 | 72583 | ✅ PASS |
| `pdf_scale_pages` | MCP | 8.7 | 72583 | ✅ PASS |
| `pdf_scale_pages` | API | 1.1 | 0 | ❌ FAIL |
| `pdf_scale_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_scale_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_delete_pages` | CLI | 23.3 | 3174 | ✅ PASS |
| `pdf_delete_pages` | MCP | 14.9 | 3174 | ✅ PASS |
| `pdf_delete_pages` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_delete_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_delete_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | CLI | 24.6 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | MCP | 12.2 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_extract_pages` | CLI | 34.2 | 72412 | ✅ PASS |
| `pdf_extract_pages` | MCP | 11.3 | 72412 | ✅ PASS |
| `pdf_extract_pages` | API | 1.1 | 0 | ❌ FAIL |
| `pdf_extract_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_extract_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_nup` | CLI | 39.0 | 116973 | ✅ PASS |
| `pdf_nup` | MCP | 11.9 | 116973 | ✅ PASS |
| `pdf_nup` | API | 1.5 | 0 | ❌ FAIL |
| `pdf_nup` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_nup` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_booklet` | CLI | 43.0 | 72603 | ✅ PASS |
| `pdf_booklet` | MCP | 17.2 | 72603 | ✅ PASS |
| `pdf_booklet` | API | 2.6 | 0 | ❌ FAIL |
| `pdf_booklet` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_booklet` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_posterize` | CLI | 90.4 | 149274 | ✅ PASS |
| `pdf_posterize` | MCP | 28.8 | 149274 | ✅ PASS |
| `pdf_posterize` | API | 6.1 | 0 | ❌ FAIL |
| `pdf_posterize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_posterize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | CLI | 103.6 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | MCP | 42.1 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | API | 1.4 | 0 | ❌ FAIL |
| `pdf_stitch_canvas` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | Desktop | 15.2 | 4500 | ✅ PASS |
