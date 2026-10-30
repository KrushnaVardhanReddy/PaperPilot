# Wave 6 Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 80
- **Total Passed:** 61

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_merge` | CLI | 58.7 | 957 | ✅ PASS |
| `pdf_merge` | MCP | 3.0 | 1071 | ✅ PASS |
| `pdf_merge` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_merge` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | CLI | 16.9 | 957 | ✅ PASS |
| `pdf_merge_alternate` | MCP | 1.6 | 957 | ✅ PASS |
| `pdf_merge_alternate` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_merge_alternate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split` | CLI | 17.1 | 1 | ✅ PASS |
| `pdf_split` | MCP | 5.7 | 1 | ✅ PASS |
| `pdf_split` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_split` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | CLI | 14.9 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | MCP | 5.1 | 72538 | ✅ PASS |
| `pdf_split_by_outline` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | CLI | 13.8 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | MCP | 3.7 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_split_scanned_book` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_rotate` | CLI | 18.6 | 72430 | ✅ PASS |
| `pdf_rotate` | MCP | 5.5 | 72430 | ✅ PASS |
| `pdf_rotate` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | CLI | 26.3 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | MCP | 7.2 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | API | 0.6 | 0 | ❌ FAIL |
| `pdf_auto_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_crop` | CLI | 20.0 | 72422 | ✅ PASS |
| `pdf_crop` | MCP | 4.8 | 72422 | ✅ PASS |
| `pdf_crop` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_crop` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_crop` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_scale_pages` | CLI | 22.6 | 72583 | ✅ PASS |
| `pdf_scale_pages` | MCP | 4.1 | 72583 | ✅ PASS |
| `pdf_scale_pages` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_scale_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_scale_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_delete_pages` | CLI | 17.3 | 3174 | ✅ PASS |
| `pdf_delete_pages` | MCP | 2.5 | 3174 | ✅ PASS |
| `pdf_delete_pages` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_delete_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_delete_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | CLI | 12.3 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | MCP | 2.1 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_extract_pages` | CLI | 13.5 | 72412 | ✅ PASS |
| `pdf_extract_pages` | MCP | 5.8 | 72412 | ✅ PASS |
| `pdf_extract_pages` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_extract_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_extract_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_nup` | CLI | 15.5 | 116973 | ✅ PASS |
| `pdf_nup` | MCP | 4.0 | 116973 | ✅ PASS |
| `pdf_nup` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_nup` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_nup` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_booklet` | CLI | 13.0 | 72603 | ✅ PASS |
| `pdf_booklet` | MCP | 3.6 | 72603 | ✅ PASS |
| `pdf_booklet` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_booklet` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_booklet` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_posterize` | CLI | 22.6 | 149274 | ✅ PASS |
| `pdf_posterize` | MCP | 6.7 | 149274 | ✅ PASS |
| `pdf_posterize` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_posterize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_posterize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | CLI | 31.8 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | MCP | 23.8 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_stitch_canvas` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | Desktop | 15.2 | 4500 | ✅ PASS |
