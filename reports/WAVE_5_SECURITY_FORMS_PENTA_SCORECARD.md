# Wave 5 Security & Forms Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 80
- **Total Passed:** 54

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |
|-----------|-----------|--------------|---------------------|--------|-------|
| `pdf_watermark` | CLI | 18.5 | 72768 | ✅ PASS |  |
| `pdf_watermark` | MCP | 5.8 | 72768 | ✅ PASS |  |
| `pdf_watermark` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_watermark` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_watermark` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_encrypt` | CLI | 22.2 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | MCP | 7.5 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | API | 0.6 | 0 | ❌ FAIL | {} |
| `pdf_encrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_encrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_decrypt` | CLI | 22.7 | 56699 | ✅ PASS |  |
| `pdf_decrypt` | MCP | 12.1 | 56699 | ✅ PASS |  |
| `pdf_decrypt` | API | 0.6 | 0 | ❌ FAIL | {} |
| `pdf_decrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_decrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sanitize` | CLI | 18.5 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | MCP | 3.0 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | API | 1.0 | 0 | ❌ FAIL | {} |
| `pdf_sanitize` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sanitize` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_metadata` | CLI | 22.6 | 29223 | ✅ PASS |  |
| `pdf_metadata` | MCP | 4.8 | 29223 | ✅ PASS |  |
| `pdf_metadata` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_metadata` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_metadata` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_redact` | CLI | 12.0 | 0 | ❌ FAIL | error: unexpected argument '--query' found |
| `pdf_redact` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_redact` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_redact` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_redact` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_read_form` | CLI | 12.5 | 0 | ❌ FAIL | error: unrecognized subcommand 'read-form' |
| `pdf_read_form` | MCP | 2.9 | 0 | ✅ PASS |  |
| `pdf_read_form` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_read_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_read_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_fill_form` | CLI | 10.5 | 0 | ❌ FAIL | error: unrecognized subcommand 'fill-form' |
| `pdf_fill_form` | MCP | 3.4 | 23442 | ✅ PASS |  |
| `pdf_fill_form` | API | 1.0 | 0 | ❌ FAIL | {} |
| `pdf_fill_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_fill_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_flatten` | CLI | 17.6 | 25649 | ✅ PASS |  |
| `pdf_flatten` | MCP | 4.6 | 25649 | ✅ PASS |  |
| `pdf_flatten` | API | 0.6 | 0 | ❌ FAIL | {} |
| `pdf_flatten` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_flatten` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_stamp` | CLI | 10.9 | 0 | ❌ FAIL | error: unrecognized subcommand 'stamp' |
| `pdf_stamp` | MCP | 4.0 | 29641 | ✅ PASS |  |
| `pdf_stamp` | API | 1.3 | 0 | ❌ FAIL | {} |
| `pdf_stamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_stamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | CLI | 11.1 | 0 | ❌ FAIL | error: unrecognized subcommand 'validate-signatures' |
| `pdf_validate_signatures` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_validate_signatures` | API | 0.8 | 0 | ❌ FAIL | {} |
| `pdf_validate_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | CLI | 14.4 | 0 | ❌ FAIL | error: unrecognized subcommand 'remove-signatures' |
| `pdf_remove_signatures` | MCP | 3.9 | 72429 | ✅ PASS |  |
| `pdf_remove_signatures` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_remove_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | CLI | 22.3 | 72593 | ✅ PASS |  |
| `pdf_change_permissions` | MCP | 7.5 | 72593 | ✅ PASS |  |
| `pdf_change_permissions` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_change_permissions` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | CLI | 10.6 | 0 | ❌ FAIL | error: unrecognized subcommand 'create-form-field' |
| `pdf_create_form_field` | MCP | 4.2 | 29350 | ✅ PASS |  |
| `pdf_create_form_field` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_create_form_field` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_timestamp` | CLI | 11.7 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | MCP | 0.2 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | API | 32.4 | 0 | ❌ FAIL | {} |
| `pdf_timestamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_timestamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_bates` | CLI | 16.2 | 0 | ❌ FAIL | error: the following required arguments were not provided: |
| `pdf_bates` | MCP | 3.0 | 72631 | ✅ PASS |  |
| `pdf_bates` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_bates` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_bates` | Desktop | 15.2 | 4500 | ✅ PASS | |
