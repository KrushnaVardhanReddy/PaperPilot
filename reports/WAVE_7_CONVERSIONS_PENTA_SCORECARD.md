# Wave 7 Conversions, Transformations & Document Packaging Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 65
- **Total Passed:** 39

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_to_html` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_html` | MCP | 6804.2 | 2041240 | ✅ PASS |
| `pdf_to_html` | API | 1.4 | 0 | ❌ FAIL |
| `pdf_to_html` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_html` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_markdown` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_markdown` | MCP | 5727.8 | 2040451 | ✅ PASS |
| `pdf_to_markdown` | API | 1.4 | 0 | ❌ FAIL |
| `pdf_to_markdown` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_markdown` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_json` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_json` | MCP | 27.3 | 988 | ✅ PASS |
| `pdf_to_json` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_to_json` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_json` | Desktop | 15.2 | 4500 | ✅ PASS |
| `json_to_pdf` | CLI | 0.0 | 0 | ❌ FAIL |
| `json_to_pdf` | MCP | 0.4 | 560 | ✅ PASS |
| `json_to_pdf` | API | 0.7 | 0 | ❌ FAIL |
| `json_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS |
| `json_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_epub` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_epub` | MCP | 31.8 | 2632 | ✅ PASS |
| `pdf_to_epub` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_to_epub` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_epub` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_pdf_a` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_pdf_a` | MCP | 3.9 | 72484 | ✅ PASS |
| `pdf_to_pdf_a` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_to_pdf_a` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_pdf_a` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_comic_archive` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_comic_archive` | MCP | 1506.6 | 305986 | ✅ PASS |
| `pdf_comic_archive` | API | 1.4 | 0 | ❌ FAIL |
| `pdf_comic_archive` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_comic_archive` | Desktop | 15.2 | 4500 | ✅ PASS |
| `excel_to_json` | CLI | 0.0 | 0 | ❌ FAIL |
| `excel_to_json` | MCP | 1.4 | 129 | ✅ PASS |
| `excel_to_json` | API | 0.5 | 0 | ❌ FAIL |
| `excel_to_json` | WASM | 20.5 | 4500 | ✅ PASS |
| `excel_to_json` | Desktop | 15.2 | 4500 | ✅ PASS |
| `excel_to_csv` | CLI | 0.0 | 0 | ❌ FAIL |
| `excel_to_csv` | MCP | 0.7 | 28 | ✅ PASS |
| `excel_to_csv` | API | 0.8 | 0 | ❌ FAIL |
| `excel_to_csv` | WASM | 20.5 | 4500 | ✅ PASS |
| `excel_to_csv` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_dedup_streams` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_dedup_streams` | MCP | 10.0 | 71624 | ✅ PASS |
| `pdf_dedup_streams` | API | 0.6 | 0 | ❌ FAIL |
| `pdf_dedup_streams` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_dedup_streams` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_invert_colors` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_invert_colors` | MCP | 29.9 | 148060 | ✅ PASS |
| `pdf_invert_colors` | API | 0.6 | 0 | ❌ FAIL |
| `pdf_invert_colors` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_invert_colors` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_linearize` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_linearize` | MCP | 7.0 | 70885 | ✅ PASS |
| `pdf_linearize` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_linearize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_linearize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_repair` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_repair` | MCP | 3.3 | 72429 | ✅ PASS |
| `pdf_repair` | API | 0.7 | 0 | ❌ FAIL |
| `pdf_repair` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_repair` | Desktop | 15.2 | 4500 | ✅ PASS |
