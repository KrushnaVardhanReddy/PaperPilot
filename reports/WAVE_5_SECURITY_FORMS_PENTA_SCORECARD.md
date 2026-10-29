# Wave 5 Security & Forms Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 75
- **Total Passed:** 42

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |
|-----------|-----------|--------------|---------------------|--------|-------|
| `pdf_watermark` | CLI | 0.4 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_watermark` | MCP | 11.2 | 72768 | ✅ PASS |  |
| `pdf_watermark` | API | 22.6 | 72768 | ✅ PASS |  |
| `pdf_watermark` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_watermark` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_encrypt` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_encrypt` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_encrypt` | API | 3.8 | 0 | ❌ FAIL | {"error":"Missing or invalid 'password' parameter","success":false} |
| `pdf_encrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_encrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_decrypt` | CLI | 0.4 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_decrypt` | MCP | 3.1 | 0 | ❌ FAIL | {} |
| `pdf_decrypt` | API | 6.7 | 0 | ❌ FAIL | {"error":"Unsupported operation: Decryption failed or invalid password: invalid password for encrypted PDF","success":false} |
| `pdf_decrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_decrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sanitize` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_sanitize` | MCP | 7.7 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | API | 12.4 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sanitize` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_redact` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_redact` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_redact` | API | 3.4 | 0 | ❌ FAIL | {"error":"Missing or invalid 'page' parameter","success":false} |
| `pdf_redact` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_redact` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_read_form` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_read_form` | MCP | 2.6 | 0 | ✅ PASS |  |
| `pdf_read_form` | API | 5.3 | 0 | ✅ PASS |  |
| `pdf_read_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_read_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_fill_form` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_fill_form` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_fill_form` | API | 3.2 | 0 | ❌ FAIL | {"error":"Missing 'values'","success":false} |
| `pdf_fill_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_fill_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_flatten` | CLI | 0.4 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_flatten` | MCP | 2.8 | 1952 | ✅ PASS |  |
| `pdf_flatten` | API | 5.7 | 1952 | ✅ PASS |  |
| `pdf_flatten` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_flatten` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_stamp` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_stamp` | MCP | 1.8 | 0 | ❌ FAIL | {} |
| `pdf_stamp` | API | 5.2 | 0 | ❌ FAIL | {"error":"I/O error: No such file or directory (os error 2)","success":false} |
| `pdf_stamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_stamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | CLI | 0.4 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_validate_signatures` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_validate_signatures` | API | 3.4 | 0 | ❌ FAIL | {"error":"Missing or invalid 'output' parameter","success":false} |
| `pdf_validate_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | CLI | 0.5 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_remove_signatures` | MCP | 8.4 | 72429 | ✅ PASS |  |
| `pdf_remove_signatures` | API | 12.2 | 72429 | ✅ PASS |  |
| `pdf_remove_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_change_permissions` | MCP | 17.3 | 72592 | ✅ PASS |  |
| `pdf_change_permissions` | API | 21.1 | 72593 | ✅ PASS |  |
| `pdf_change_permissions` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | CLI | 0.4 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_create_form_field` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_create_form_field` | API | 3.2 | 0 | ❌ FAIL | {"error":"Missing or invalid 'field_name' parameter","success":false} |
| `pdf_create_form_field` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_timestamp` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_timestamp` | MCP | 0.6 | 0 | ❌ FAIL | {} |
| `pdf_timestamp` | API | 3.4 | 0 | ❌ FAIL | {"error":"Unsupported operation: Not yet implemented","success":false} |
| `pdf_timestamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_timestamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_bates` | CLI | 0.3 | 0 | ❌ FAIL | No such file or directory (os error 2) |
| `pdf_bates` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_bates` | API | 3.5 | 0 | ❌ FAIL | {"error":"Missing or invalid 'start_number' parameter","success":false} |
| `pdf_bates` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_bates` | Desktop | 15.2 | 4500 | ✅ PASS | |
