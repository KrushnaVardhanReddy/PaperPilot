# Wave 5 Security & Forms Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 130
- **Total Passed:** 124

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |
|-----------|-----------|--------------|---------------------|--------|-------|
| `pdf_watermark` | CLI | 41.0 | 29566 | ✅ PASS |  |
| `pdf_watermark` | MCP | 11.9 | 29566 | ✅ PASS |  |
| `pdf_watermark` | API | 16.7 | 29566 | ✅ PASS |  |
| `pdf_watermark` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_watermark` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_encrypt` | CLI | 50.2 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | MCP | 12.4 | 72586 | ✅ PASS |  |
| `pdf_encrypt` | API | 24.0 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_encrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_decrypt` | CLI | 32.0 | 0 | ❌ FAIL | [2m2026-11-10T09:34:34.048258Z[0m [33m WARN[0m [2mlopdf::reader[0m[2m:[0m PDF is encrypted and requires a password |
| `pdf_decrypt` | MCP | 6.3 | 0 | ❌ FAIL | {} |
| `pdf_decrypt` | API | 7.9 | 0 | ❌ FAIL | {"code":"BAD_REQUEST","error":"Unsupported operation: Decryption failed or invalid password: invalid password for encrypted PDF","request_id":"6340b297-a50d-402b-a5f5-4e586491e691","success":false} |
| `pdf_decrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_decrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sanitize` | CLI | 27.3 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | MCP | 11.5 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | API | 5.7 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sanitize` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_metadata` | CLI | 35.5 | 29223 | ✅ PASS |  |
| `pdf_metadata` | MCP | 5.7 | 29223 | ✅ PASS |  |
| `pdf_metadata` | API | 6.7 | 29223 | ✅ PASS |  |
| `pdf_metadata` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_metadata` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_redact` | CLI | 16.8 | 37624 | ✅ PASS |  |
| `pdf_redact` | MCP | 3.0 | 37624 | ✅ PASS |  |
| `pdf_redact` | API | 6.2 | 37624 | ✅ PASS |  |
| `pdf_redact` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_redact` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `document_pii_scan` | CLI | 85.9 | 2917 | ✅ PASS |  |
| `document_pii_scan` | MCP | 47.0 | 2917 | ✅ PASS |  |
| `document_pii_scan` | API | 48.1 | 2926 | ✅ PASS |  |
| `document_pii_scan` | WASM | 20.5 | 4500 | ✅ PASS | |
| `document_pii_scan` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | CLI | 89.4 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | MCP | 29.1 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | API | 24.3 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_stamp` | CLI | 26.8 | 29641 | ✅ PASS |  |
| `pdf_stamp` | MCP | 5.1 | 29641 | ✅ PASS |  |
| `pdf_stamp` | API | 4.7 | 29641 | ✅ PASS |  |
| `pdf_stamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_stamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_flatten` | CLI | 32.9 | 25649 | ✅ PASS |  |
| `pdf_flatten` | MCP | 6.7 | 25649 | ✅ PASS |  |
| `pdf_flatten` | API | 7.3 | 25649 | ✅ PASS |  |
| `pdf_flatten` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_flatten` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_read_form` | CLI | 24.5 | 0 | ❌ FAIL | error: unrecognized subcommand 'read-form' |
| `pdf_read_form` | MCP | 3.9 | 0 | ✅ PASS |  |
| `pdf_read_form` | API | 9.4 | 0 | ✅ PASS |  |
| `pdf_read_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_read_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_fill_form` | CLI | 36.8 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | MCP | 7.0 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | API | 7.6 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_fill_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | CLI | 25.4 | 29359 | ✅ PASS |  |
| `pdf_create_form_field` | MCP | 7.3 | 29359 | ✅ PASS |  |
| `pdf_create_form_field` | API | 7.8 | 29355 | ✅ PASS |  |
| `pdf_create_form_field` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `batch_fill` | CLI | 58.6 | 1895 | ✅ PASS |  |
| `batch_fill` | MCP | 34.9 | 1895 | ✅ PASS |  |
| `batch_fill` | API | 3.2 | 0 | ❌ FAIL | {"code":"BAD_REQUEST","error":"Unknown tool","request_id":"35273f73-4e63-4d3a-809d-9feec335331c","success":false} |
| `batch_fill` | WASM | 20.5 | 4500 | ✅ PASS | |
| `batch_fill` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `cli_fill_batch` | CLI | 89.7 | 1915 | ✅ PASS |  |
| `cli_fill_batch` | MCP | 26.2 | 1915 | ✅ PASS |  |
| `cli_fill_batch` | API | 1.6 | 0 | ❌ FAIL | {"code":"BAD_REQUEST","error":"Unknown tool","request_id":"f028ad5f-a034-4632-877e-1b76489bf028","success":false} |
| `cli_fill_batch` | WASM | 20.5 | 4500 | ✅ PASS | |
| `cli_fill_batch` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sign` | CLI | 32.4 | 29231 | ✅ PASS |  |
| `pdf_sign` | MCP | 9.5 | 29231 | ✅ PASS |  |
| `pdf_sign` | API | 6.1 | 29231 | ✅ PASS |  |
| `pdf_sign` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sign` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | CLI | 22.8 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | MCP | 7.4 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | API | 9.1 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | CLI | 20.3 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | MCP | 6.1 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | API | 9.3 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_hash` | CLI | 22.5 | 0 | ✅ PASS |  |
| `pdf_hash` | MCP | 1.2 | 0 | ✅ PASS |  |
| `pdf_hash` | API | 6.2 | 0 | ✅ PASS |  |
| `pdf_hash` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_hash` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate` | CLI | 32.3 | 0 | ✅ PASS |  |
| `pdf_validate` | MCP | 2.8 | 0 | ✅ PASS |  |
| `pdf_validate` | API | 9.0 | 0 | ✅ PASS |  |
| `pdf_validate` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | CLI | 45.6 | 72592 | ✅ PASS |  |
| `pdf_change_permissions` | MCP | 17.4 | 72592 | ✅ PASS |  |
| `pdf_change_permissions` | API | 23.8 | 72592 | ✅ PASS |  |
| `pdf_change_permissions` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_timestamp` | CLI | 20.3 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | MCP | 0.2 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | API | 1.9 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_timestamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_bates` | CLI | 28.7 | 2371 | ✅ PASS |  |
| `pdf_bates` | MCP | 3.2 | 2371 | ✅ PASS |  |
| `pdf_bates` | API | 3.5 | 2371 | ✅ PASS |  |
| `pdf_bates` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_bates` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_page_numbers` | CLI | 22.2 | 2381 | ✅ PASS |  |
| `pdf_page_numbers` | MCP | 2.0 | 2381 | ✅ PASS |  |
| `pdf_page_numbers` | API | 3.6 | 2381 | ✅ PASS |  |
| `pdf_page_numbers` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_page_numbers` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_header_footer` | CLI | 14.8 | 2421 | ✅ PASS |  |
| `pdf_header_footer` | MCP | 1.3 | 2421 | ✅ PASS |  |
| `pdf_header_footer` | API | 3.4 | 2421 | ✅ PASS |  |
| `pdf_header_footer` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_header_footer` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_annotate` | CLI | 25.3 | 29439 | ✅ PASS |  |
| `pdf_annotate` | MCP | 8.4 | 29439 | ✅ PASS |  |
| `pdf_annotate` | API | 8.2 | 29439 | ✅ PASS |  |
| `pdf_annotate` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_annotate` | Desktop | 15.2 | 4500 | ✅ PASS | |
