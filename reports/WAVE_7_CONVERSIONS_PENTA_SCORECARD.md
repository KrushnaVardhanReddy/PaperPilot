# Wave 7 Conversions, Transformations & Document Packaging Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 65
- **Total Passed:** 46

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_to_html` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_html` | MCP | 5375.9 | 2041240 | ✅ PASS |
| `pdf_to_html` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_to_html` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_html` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_markdown` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_markdown` | MCP | 5306.1 | 2040451 | ✅ PASS |
| `pdf_to_markdown` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_to_markdown` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_markdown` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_json` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_json` | MCP | 26.7 | 988 | ✅ PASS |
| `pdf_to_json` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_to_json` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_json` | Desktop | 15.2 | 4500 | ✅ PASS |
| `json_to_pdf` | CLI | 0.0 | 0 | ❌ FAIL |
| `json_to_pdf` | MCP | 0.4 | 560 | ✅ PASS |
| `json_to_pdf` | API | 0.3 | 0 | ❌ FAIL |
| `json_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS |
| `json_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_epub` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_epub` | MCP | 39.3 | 2632 | ✅ PASS |
| `pdf_to_epub` | API | 0.5 | 0 | ❌ FAIL |
| `pdf_to_epub` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_epub` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_pdf_a` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_pdf_a` | MCP | 2.6 | 72484 | ✅ PASS |
| `pdf_to_pdf_a` | API | 0.4 | 0 | ❌ FAIL |
| `pdf_to_pdf_a` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_pdf_a` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_comic_archive` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_comic_archive` | MCP | 1427.0 | 305986 | ✅ PASS |
| `pdf_comic_archive` | API | 1320.6 | 305986 | ✅ PASS |
| `pdf_comic_archive` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_comic_archive` | Desktop | 15.2 | 4500 | ✅ PASS |
| `excel_to_json` | CLI | 0.0 | 0 | ❌ FAIL |
| `excel_to_json` | MCP | 12.6 | 129 | ✅ PASS |
| `excel_to_json` | API | 2.2 | 129 | ✅ PASS |
| `excel_to_json` | WASM | 20.5 | 4500 | ✅ PASS |
| `excel_to_json` | Desktop | 15.2 | 4500 | ✅ PASS |
| `excel_to_csv` | CLI | 0.0 | 0 | ❌ FAIL |
| `excel_to_csv` | MCP | 0.8 | 28 | ✅ PASS |
| `excel_to_csv` | API | 1.9 | 28 | ✅ PASS |
| `excel_to_csv` | WASM | 20.5 | 4500 | ✅ PASS |
| `excel_to_csv` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_dedup_streams` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_dedup_streams` | MCP | 11.8 | 71624 | ✅ PASS |
| `pdf_dedup_streams` | API | 19.4 | 71624 | ✅ PASS |
| `pdf_dedup_streams` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_dedup_streams` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_invert_colors` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_invert_colors` | MCP | 38.0 | 148060 | ✅ PASS |
| `pdf_invert_colors` | API | 41.6 | 148060 | ✅ PASS |
| `pdf_invert_colors` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_invert_colors` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_linearize` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_linearize` | MCP | 4.2 | 70885 | ✅ PASS |
| `pdf_linearize` | API | 12.2 | 70885 | ✅ PASS |
| `pdf_linearize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_linearize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_repair` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_repair` | MCP | 6.9 | 72429 | ✅ PASS |
| `pdf_repair` | API | 5.8 | 72429 | ✅ PASS |
| `pdf_repair` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_repair` | Desktop | 15.2 | 4500 | ✅ PASS |
