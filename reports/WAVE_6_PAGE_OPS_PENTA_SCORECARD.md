# Wave 6 Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 80
- **Total Passed:** 60

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_merge` | CLI | 62.1 | 957 | ✅ PASS |
| `pdf_merge` | MCP | 2.8 | 1071 | ✅ PASS |
| `pdf_merge` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_merge` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | CLI | 14.2 | 957 | ✅ PASS |
| `pdf_merge_alternate` | MCP | 1.7 | 957 | ✅ PASS |
| `pdf_merge_alternate` | API | 1.2 | 0 | ❌ FAIL |
| `pdf_merge_alternate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split` | CLI | 15.8 | 1 | ✅ PASS |
| `pdf_split` | MCP | 3.3 | 1 | ✅ PASS |
| `pdf_split` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_split` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | CLI | 17.7 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | MCP | 4.2 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | API | 0.3 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | CLI | 17.6 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | MCP | 4.4 | 74288 | ✅ PASS |
| `pdf_split_scanned_book` | API | 0.3 | 0 | ❌ FAIL |
| `pdf_split_scanned_book` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_rotate` | CLI | 17.3 | 72430 | ✅ PASS |
| `pdf_rotate` | MCP | 5.3 | 72430 | ✅ PASS |
| `pdf_rotate` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | CLI | 18.3 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | MCP | 4.4 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_auto_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_crop` | CLI | 16.0 | 72422 | ✅ PASS |
| `pdf_crop` | MCP | 4.0 | 72422 | ✅ PASS |
| `pdf_crop` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_crop` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_crop` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_scale_pages` | CLI | 12.8 | 72583 | ✅ PASS |
| `pdf_scale_pages` | MCP | 4.4 | 72583 | ✅ PASS |
| `pdf_scale_pages` | API | 0.3 | 0 | ❌ FAIL |
| `pdf_scale_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_scale_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_delete_pages` | CLI | 12.6 | 3174 | ✅ PASS |
| `pdf_delete_pages` | MCP | 2.9 | 3174 | ✅ PASS |
| `pdf_delete_pages` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_delete_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_delete_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | CLI | 14.6 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | MCP | 3.6 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_extract_pages` | CLI | 15.3 | 72412 | ✅ PASS |
| `pdf_extract_pages` | MCP | 3.1 | 72412 | ✅ PASS |
| `pdf_extract_pages` | API | 0.3 | 0 | ❌ FAIL |
| `pdf_extract_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_extract_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_nup` | CLI | 15.3 | 116973 | ✅ PASS |
| `pdf_nup` | MCP | 5.6 | 116973 | ✅ PASS |
| `pdf_nup` | API | 0.3 | 0 | ❌ FAIL |
| `pdf_nup` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_nup` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_booklet` | CLI | 18.6 | 72603 | ✅ PASS |
| `pdf_booklet` | MCP | 3.9 | 72603 | ✅ PASS |
| `pdf_booklet` | API | 0.3 | 0 | ❌ FAIL |
| `pdf_booklet` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_booklet` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_posterize` | CLI | 17.2 | 149274 | ✅ PASS |
| `pdf_posterize` | MCP | 8.1 | 149274 | ✅ PASS |
| `pdf_posterize` | API | 0.3 | 0 | ❌ FAIL |
| `pdf_posterize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_posterize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | CLI | 34.7 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | MCP | 21.7 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | API | 0.3 | 0 | ❌ FAIL |
| `pdf_stitch_canvas` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | Desktop | 15.2 | 4500 | ✅ PASS |
