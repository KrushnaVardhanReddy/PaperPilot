# Wave 8 Analysis Intelligence Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 85
- **Total Passed:** 10

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error Notes |
|-----------|-----------|--------------|---------------------|--------|-------------|
| `pdf_extract_text` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_extract_text` | MCP | 6.1 | 113 | ✅ PASS |  |
| `pdf_extract_text` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_text` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_text` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_ocr` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_ocr` | MCP | 270.9 | 0 | ❌ FAIL | Output file not generated |
| `pdf_ocr` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_ocr` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_ocr` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_tables` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_extract_tables` | MCP | 3.6 | 0 | ❌ FAIL | Output file not generated |
| `pdf_extract_tables` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_tables` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_tables` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `document_pii_scan` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `document_pii_scan` | MCP | 3.4 | 0 | ❌ FAIL | Output file not generated |
| `document_pii_scan` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `document_pii_scan` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `document_pii_scan` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bookmarks` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_bookmarks` | MCP | 0.0 | 0 | ❌ FAIL | ErrorData { code: ErrorCode(-32602), message: "Missing or invalid 'output' parameter", data: None } |
| `pdf_bookmarks` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bookmarks` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bookmarks` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bates` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_bates` | MCP | 1.2 | 0 | ❌ FAIL | Output file not generated |
| `pdf_bates` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bates` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bates` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_header_footer` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_header_footer` | MCP | 0.4 | 0 | ❌ FAIL | Output file not generated |
| `pdf_header_footer` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_header_footer` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_header_footer` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_classify_type` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_classify_type` | MCP | 1.1 | 132 | ✅ PASS |  |
| `pdf_classify_type` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_classify_type` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_classify_type` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_compare` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_compare` | MCP | 0.9 | 175 | ✅ PASS |  |
| `pdf_compare` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_compare` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_compare` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_hash` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_hash` | MCP | 0.1 | 277 | ✅ PASS |  |
| `pdf_hash` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_hash` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_hash` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_validate` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_validate` | MCP | 0.2 | 220 | ✅ PASS |  |
| `pdf_validate` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_validate` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_validate` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_render` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_render` | MCP | 26.4 | 0 | ❌ FAIL | Output file not generated |
| `pdf_render` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_render` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_render` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_rag_chunks` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_rag_chunks` | MCP | 0.9 | 160 | ✅ PASS |  |
| `pdf_rag_chunks` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_rag_chunks` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_rag_chunks` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `workspace_search` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `workspace_search` | MCP | 63.0 | 2041 | ✅ PASS |  |
| `workspace_search` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `workspace_search` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `workspace_search` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `semantic_search` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `semantic_search` | MCP | 0.4 | 158 | ✅ PASS |  |
| `semantic_search` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `semantic_search` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `semantic_search` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_patterns` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_extract_patterns` | MCP | 0.9 | 189 | ✅ PASS |  |
| `pdf_extract_patterns` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_patterns` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_patterns` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_search` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_search` | MCP | 0.3 | 87 | ✅ PASS |  |
| `pdf_search` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_search` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_search` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
