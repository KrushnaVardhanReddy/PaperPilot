# Wave 8 Analysis Intelligence Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 75
- **Total Passed:** 5

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error Notes |
|-----------|-----------|--------------|---------------------|--------|-------------|
| `pdf_extract_text` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_extract_text` | MCP | 1.9 | 0 | ❌ FAIL | ErrorData { code: ErrorCode(-32602), message: "Missing or invalid 'output' parameter", data: None } |
| `pdf_extract_text` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_text` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_text` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_ocr` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_ocr` | MCP | 11.8 | 0 | ❌ FAIL | ErrorData { code: ErrorCode(-32602), message: "Other error: OCR models not found. Run 'paperpilot ocr setup' or download the model pack from settings.", data: None } |
| `pdf_ocr` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_ocr` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_ocr` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_tables` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_extract_tables` | MCP | 0.0 | 0 | ❌ FAIL | ErrorData { code: ErrorCode(-32602), message: "Unknown tool", data: None } |
| `pdf_extract_tables` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_tables` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_extract_tables` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `document_pii_scan` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `document_pii_scan` | MCP | 22.1 | 0 | ❌ FAIL | Output file not generated |
| `document_pii_scan` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `document_pii_scan` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `document_pii_scan` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bookmarks` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_bookmarks` | MCP | 2.8 | 103 | ✅ PASS |  |
| `pdf_bookmarks` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bookmarks` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bookmarks` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bates` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_bates` | MCP | 0.0 | 0 | ❌ FAIL | ErrorData { code: ErrorCode(-32602), message: "Missing or invalid 'start_number' parameter", data: None } |
| `pdf_bates` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bates` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_bates` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_header_footer` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_header_footer` | MCP | 2.3 | 0 | ❌ FAIL | Output file not generated |
| `pdf_header_footer` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_header_footer` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_header_footer` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_classify_type` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_classify_type` | MCP | 3.1 | 101 | ✅ PASS |  |
| `pdf_classify_type` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_classify_type` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_classify_type` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_compare` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_compare` | MCP | 34.1 | 175 | ✅ PASS |  |
| `pdf_compare` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_compare` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_compare` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_hash` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_hash` | MCP | 0.7 | 208 | ✅ PASS |  |
| `pdf_hash` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_hash` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_hash` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_validate` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_validate` | MCP | 0.7 | 92 | ✅ PASS |  |
| `pdf_validate` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_validate` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_validate` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_render` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_render` | MCP | 4.5 | 0 | ❌ FAIL | Output file not generated |
| `pdf_render` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_render` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_render` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_rag_chunks` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `pdf_rag_chunks` | MCP | 0.0 | 0 | ❌ FAIL | ErrorData { code: ErrorCode(-32602), message: "Missing or invalid 'output' parameter", data: None } |
| `pdf_rag_chunks` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_rag_chunks` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `pdf_rag_chunks` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `workspace_search` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `workspace_search` | MCP | 0.0 | 0 | ❌ FAIL | ErrorData { code: ErrorCode(-32602), message: "I/O error: Is a directory (os error 21)", data: None } |
| `workspace_search` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `workspace_search` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `workspace_search` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `semantic_search` | CLI | 0.0 | 0 | ❌ FAIL | Command failed |
| `semantic_search` | MCP | 0.0 | 0 | ❌ FAIL | ErrorData { code: ErrorCode(-32602), message: "Unknown tool", data: None } |
| `semantic_search` | API | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `semantic_search` | WASM | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
| `semantic_search` | Desktop | 0.0 | 0 | ❌ FAIL | Not tested in Rust MCP |
