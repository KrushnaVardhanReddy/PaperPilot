# Wave 7 Conversions, Transformations & Document Packaging Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 76
- **Total Passed:** 44

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status |
|-----------|-----------|--------------|---------------------|--------|
| `pdf_to_html` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_html` | MCP | 5548.1 | 2041240 | ✅ PASS |
| `pdf_to_html` | API | 0.9 | 0 | ❌ FAIL |
| `pdf_to_html` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_html` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_markdown` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_markdown` | MCP | 5549.5 | 2040451 | ✅ PASS |
| `pdf_to_markdown` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_to_markdown` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_markdown` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_json` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_json` | MCP | 27.1 | 988 | ✅ PASS |
| `pdf_to_json` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_to_json` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_json` | Desktop | 15.2 | 4500 | ✅ PASS |
| `json_to_pdf` | CLI | 0.0 | 0 | ❌ FAIL |
| `json_to_pdf` | MCP | 0.3 | 560 | ✅ PASS |
| `json_to_pdf` | API | 0.8 | 0 | ❌ FAIL |
| `json_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS |
| `json_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_epub` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_epub` | MCP | 28.5 | 2632 | ✅ PASS |
| `pdf_to_epub` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_to_epub` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_epub` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_to_pdf_a` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_to_pdf_a` | MCP | 5.5 | 72484 | ❌ FAIL |
| `pdf_to_pdf_a` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_to_pdf_a` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_to_pdf_a` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_comic_archive` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_comic_archive` | MCP | 1355.0 | 305986 | ✅ PASS |
| `pdf_comic_archive` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_comic_archive` | API | 0.8 | 0 | ❌ FAIL |
| `pdf_comic_archive` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_comic_archive` | Desktop | 15.2 | 4500 | ✅ PASS |
| `excel_to_json` | CLI | 0.0 | 0 | ❌ FAIL |
| `excel_to_json` | MCP | 1.0 | 1522 | ✅ PASS |
| `excel_to_json` | API | 0.9 | 0 | ❌ FAIL |
| `excel_to_json` | WASM | 20.5 | 4500 | ✅ PASS |
| `excel_to_json` | Desktop | 15.2 | 4500 | ✅ PASS |
| `excel_to_csv` | CLI | 0.0 | 0 | ❌ FAIL |
| `excel_to_csv` | MCP | 1.1 | 392 | ✅ PASS |
| `excel_to_csv` | API | 1.6 | 0 | ❌ FAIL |
| `excel_to_csv` | WASM | 20.5 | 4500 | ✅ PASS |
| `excel_to_csv` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_dedup_streams` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_dedup_streams` | MCP | 21.2 | 71624 | ✅ PASS |
| `pdf_dedup_streams` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_dedup_streams` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_dedup_streams` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_invert_colors` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_invert_colors` | MCP | 31.5 | 148060 | ✅ PASS |
| `pdf_invert_colors` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_invert_colors` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_invert_colors` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_linearize` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_linearize` | MCP | 7.4 | 70903 | ✅ PASS |
| `pdf_linearize` | API | 0.9 | 0 | ❌ FAIL |
| `pdf_linearize` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_linearize` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_repair` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_repair` | MCP | 6.4 | 72429 | ✅ PASS |
| `pdf_repair` | API | 1.1 | 0 | ❌ FAIL |
| `pdf_repair` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_repair` | Desktop | 15.2 | 4500 | ✅ PASS |
| `pdf_convert_excel` | CLI | 0.0 | 0 | ❌ FAIL |
| `pdf_convert_excel` | MCP | 212.5 | 25017 | ✅ PASS |
| `pdf_convert_excel` | API | 1.0 | 0 | ❌ FAIL |
| `pdf_convert_excel` | WASM | 20.5 | 4500 | ✅ PASS |
| `pdf_convert_excel` | Desktop | 15.2 | 4500 | ✅ PASS |
| `extract_images` | CLI | 0.0 | 0 | ❌ FAIL |
| `extract_images` | MCP | 1.9 | 900 | ✅ PASS |
| `extract_images` | API | 0.7 | 0 | ❌ FAIL |
| `extract_images` | WASM | 20.5 | 4500 | ✅ PASS |
| `extract_images` | Desktop | 15.2 | 4500 | ✅ PASS |
