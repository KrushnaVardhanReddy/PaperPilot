# Wave 6 Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 80
- **Total Passed:** 64

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_merge` | CLI | 17.2 | 1409 | ✅ PASS |
| `pdf_merge` | MCP | 4.4 | 1528 | ✅ PASS |
| `pdf_merge` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_merge` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | CLI | 14.5 | 1409 | ✅ PASS |
| `pdf_merge_alternate` | MCP | 2.5 | 1409 | ✅ PASS |
| `pdf_merge_alternate` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_merge_alternate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_merge_alternate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split` | CLI | 16.3 | 2 | ✅ PASS |
| `pdf_split` | MCP | 1.6 | 2 | ✅ PASS |
| `pdf_split` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_split` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | CLI | 18.5 | 2946 | ✅ PASS |
| `pdf_split_by_outline` | MCP | 1.4 | 2946 | ✅ PASS |
| `pdf_split_by_outline` | API | 0.9 | 0 | ❌ FAIL |
| `pdf_split_by_outline` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_by_outline` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | CLI | 12.3 | 872 | ✅ PASS |
| `pdf_split_scanned_book` | MCP | 1.1 | 872 | ✅ PASS |
| `pdf_split_scanned_book` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_split_scanned_book` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_split_scanned_book` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_rotate` | CLI | 18.9 | 72430 | ✅ PASS |
| `pdf_rotate` | MCP | 4.6 | 72430 | ✅ PASS |
| `pdf_rotate` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | CLI | 22.2 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | MCP | 3.4 | 72429 | ✅ PASS |
| `pdf_auto_rotate` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_auto_rotate` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_auto_rotate` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_crop` | CLI | 16.4 | 29182 | ✅ PASS |
| `pdf_crop` | MCP | 5.1 | 29182 | ✅ PASS |
| `pdf_crop` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_crop` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_crop` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_scale_pages` | CLI | 13.2 | 72583 | ✅ PASS |
| `pdf_scale_pages` | MCP | 4.7 | 72583 | ✅ PASS |
| `pdf_scale_pages` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_scale_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_scale_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_delete_pages` | CLI | 16.8 | 1230 | ✅ PASS |
| `pdf_delete_pages` | MCP | 1.8 | 1230 | ✅ PASS |
| `pdf_delete_pages` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_delete_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_delete_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | CLI | 16.9 | 1680 | ✅ PASS |
| `pdf_reorder_pages` | MCP | 1.3 | 1680 | ✅ PASS |
| `pdf_reorder_pages` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_reorder_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_reorder_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_extract_pages` | CLI | 13.1 | 1001 | ✅ PASS |
| `pdf_extract_pages` | MCP | 1.4 | 1001 | ✅ PASS |
| `pdf_extract_pages` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_extract_pages` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_extract_pages` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_nup` | CLI | 16.1 | 116973 | ✅ PASS |
| `pdf_nup` | MCP | 4.3 | 116973 | ✅ PASS |
| `pdf_nup` | API | 0.6 | 0 | ❌ FAIL |
| `pdf_nup` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_nup` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_booklet` | CLI | 19.2 | 72603 | ✅ PASS |
| `pdf_booklet` | MCP | 5.4 | 72603 | ✅ PASS |
| `pdf_booklet` | API | 0.9 | 0 | ❌ FAIL |
| `pdf_booklet` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_booklet` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_posterize` | CLI | 18.0 | 149274 | ✅ PASS |
| `pdf_posterize` | MCP | 6.5 | 149274 | ✅ PASS |
| `pdf_posterize` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_posterize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_posterize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | CLI | 39.3 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | MCP | 24.7 | 64331 | ✅ PASS |
| `pdf_stitch_canvas` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_stitch_canvas` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_stitch_canvas` | Desktop | 15.2 | 4500 | ✅ PASS |
