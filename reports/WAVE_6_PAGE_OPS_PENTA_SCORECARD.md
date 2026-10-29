# Wave 6 Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 80
- **Total Passed:** 68

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_merge` | CLI | 23.8 | 0 | ❌ FAIL |
| `pdf_merge` | MCP | 0.8 | 0 | ❌ FAIL |
| `pdf_merge` | API | 5.8 | 0 | ❌ FAIL |
| `pdf_merge` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | CLI | 21.7 | 0 | ❌ FAIL |
| `pdf_merge_alternate` | MCP | 0.1 | 0 | ❌ FAIL |
| `pdf_merge_alternate` | API | 4.2 | 0 | ❌ FAIL |
| `pdf_merge_alternate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split` | CLI | 43.2 | 1 | ✅ PASS |
| `pdf_split` | MCP | 9.9 | 1 | ✅ PASS |
| `pdf_split` | API | 13.8 | 1 | ✅ PASS |
| `pdf_split` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | CLI | 29.0 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | MCP | 6.8 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | API | 11.7 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | CLI | 30.6 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | MCP | 9.9 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | API | 11.8 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_rotate` | CLI | 31.4 | 72430 | ✅ PASS |
| `pdf_rotate` | MCP | 9.5 | 72430 | ✅ PASS |
| `pdf_rotate` | API | 15.1 | 72430 | ✅ PASS |
| `pdf_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | CLI | 32.4 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | MCP | 15.1 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | API | 17.9 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_crop` | CLI | 36.2 | 72422 | ✅ PASS |
| `pdf_crop` | MCP | 17.2 | 72422 | ✅ PASS |
| `pdf_crop` | API | 21.2 | 72422 | ✅ PASS |
| `pdf_crop` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_crop` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_scale_pages` | CLI | 54.3 | 72583 | ✅ PASS |
| `pdf_scale_pages` | MCP | 13.4 | 72583 | ✅ PASS |
| `pdf_scale_pages` | API | 16.0 | 72583 | ✅ PASS |
| `pdf_scale_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_scale_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_delete_pages` | CLI | 31.5 | 3174 | ✅ PASS |
| `pdf_delete_pages` | MCP | 7.2 | 3174 | ✅ PASS |
| `pdf_delete_pages` | API | 10.7 | 3174 | ✅ PASS |
| `pdf_delete_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_delete_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | CLI | 30.9 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | MCP | 8.8 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | API | 31.2 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_extract_pages` | CLI | 34.6 | 72412 | ✅ PASS |
| `pdf_extract_pages` | MCP | 19.8 | 72412 | ✅ PASS |
| `pdf_extract_pages` | API | 14.5 | 72412 | ✅ PASS |
| `pdf_extract_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_extract_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_nup` | CLI | 37.5 | 116973 | ✅ PASS |
| `pdf_nup` | MCP | 18.0 | 116973 | ✅ PASS |
| `pdf_nup` | API | 18.5 | 116973 | ✅ PASS |
| `pdf_nup` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_nup` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_booklet` | CLI | 32.3 | 72603 | ✅ PASS |
| `pdf_booklet` | MCP | 9.6 | 72603 | ✅ PASS |
| `pdf_booklet` | API | 11.9 | 72603 | ✅ PASS |
| `pdf_booklet` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_booklet` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_posterize` | CLI | 38.1 | 149274 | ✅ PASS |
| `pdf_posterize` | MCP | 14.0 | 149274 | ✅ PASS |
| `pdf_posterize` | API | 16.0 | 149274 | ✅ PASS |
| `pdf_posterize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_posterize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | CLI | 85.0 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | MCP | 65.3 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | API | 65.7 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | Desktop | 15.2 | 4500 | ✅ PASS |
