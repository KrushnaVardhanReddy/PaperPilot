# Wave 6 Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 80
- **Total Passed:** 62

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_merge` | CLI | 47.5 | 1409 | ✅ PASS |
| `pdf_merge` | MCP | 3.3 | 1528 | ✅ PASS |
| `pdf_merge` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_merge` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | CLI | 12.9 | 1409 | ✅ PASS |
| `pdf_merge_alternate` | MCP | 2.0 | 1409 | ✅ PASS |
| `pdf_merge_alternate` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_merge_alternate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split` | CLI | 14.7 | 2 | ✅ PASS |
| `pdf_split` | MCP | 1.6 | 2 | ✅ PASS |
| `pdf_split` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_split` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | CLI | 14.9 | 2946 | ✅ PASS |
| `pdf_split_by_outline` | MCP | 1.6 | 2946 | ✅ PASS |
| `pdf_split_by_outline` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | CLI | 46.4 | 872 | ✅ PASS |
| `pdf_split_scanned_book` | MCP | 1.0 | 872 | ✅ PASS |
| `pdf_split_scanned_book` | API | 0.9 | 0 | ❌ FAIL |
| `pdf_split_scanned_book` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_rotate` | CLI | 18.8 | 72430 | ✅ PASS |
| `pdf_rotate` | MCP | 3.8 | 72430 | ✅ PASS |
| `pdf_rotate` | API | 1.7 | 0 | ❌ FAIL |
| `pdf_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | CLI | 16.1 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | MCP | 3.9 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_auto_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_crop` | CLI | 16.5 | 72422 | ✅ PASS |
| `pdf_crop` | MCP | 2.9 | 72422 | ✅ PASS |
| `pdf_crop` | API | 0.9 | 0 | ❌ FAIL |
| `pdf_crop` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_crop` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_scale_pages` | CLI | 14.4 | 72583 | ✅ PASS |
| `pdf_scale_pages` | MCP | 3.6 | 72583 | ✅ PASS |
| `pdf_scale_pages` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_scale_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_scale_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_delete_pages` | CLI | 18.4 | 3174 | ✅ PASS |
| `pdf_delete_pages` | MCP | 2.7 | 3174 | ✅ PASS |
| `pdf_delete_pages` | API | 0.6 | 0 | ❌ FAIL |
| `pdf_delete_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_delete_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | CLI | 16.7 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | MCP | 3.3 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_extract_pages` | CLI | 23.7 | 72412 | ✅ PASS |
| `pdf_extract_pages` | MCP | 3.8 | 72412 | ✅ PASS |
| `pdf_extract_pages` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_extract_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_extract_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_nup` | CLI | 21.3 | 116973 | ✅ PASS |
| `pdf_nup` | MCP | 4.9 | 116973 | ✅ PASS |
| `pdf_nup` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_nup` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_nup` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_booklet` | CLI | 13.8 | 72603 | ✅ PASS |
| `pdf_booklet` | MCP | 3.6 | 72603 | ✅ PASS |
| `pdf_booklet` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_booklet` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_booklet` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_posterize` | CLI | 21.0 | 149274 | ✅ PASS |
| `pdf_posterize` | MCP | 7.3 | 149274 | ✅ PASS |
| `pdf_posterize` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_posterize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_posterize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | CLI | 34.9 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | MCP | 21.4 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | API | 0.6 | 0 | ❌ FAIL |
| `pdf_stitch_canvas` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | Desktop | 15.2 | 4500 | ✅ PASS |
