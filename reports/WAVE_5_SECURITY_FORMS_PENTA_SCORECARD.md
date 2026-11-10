# Wave 5 Security & Forms Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 105
- **Total Passed:** 72

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |
|-----------|-----------|--------------|---------------------|--------|-------|
| `pdf_watermark` | CLI | 16.3 | 72768 | ✅ PASS |  |
| `pdf_watermark` | MCP | 5.1 | 72768 | ✅ PASS |  |
| `pdf_watermark` | API | 0.6 | 0 | ❌ FAIL | {} |
| `pdf_watermark` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_watermark` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_encrypt` | CLI | 24.1 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | MCP | 8.9 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_encrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_encrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_decrypt` | CLI | 19.2 | 0 | ❌ FAIL | [2m2026-11-10T08:37:51.236797Z[0m [33m WARN[0m [2mlopdf::reader[0m[2m:[0m PDF is encrypted and requires a password |
| `pdf_decrypt` | MCP | 6.5 | 0 | ❌ FAIL | {} |
| `pdf_decrypt` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_decrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_decrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sanitize` | CLI | 17.6 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | MCP | 4.5 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_sanitize` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sanitize` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_metadata` | CLI | 17.7 | 29223 | ✅ PASS |  |
| `pdf_metadata` | MCP | 2.2 | 29223 | ✅ PASS |  |
| `pdf_metadata` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_metadata` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_metadata` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_redact` | CLI | 12.3 | 0 | ❌ FAIL | error: unexpected argument '--query' found |
| `pdf_redact` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_redact` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_redact` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_redact` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `document_pii_scan` | CLI | 134.2 | 2917 | ✅ PASS |  |
| `document_pii_scan` | MCP | 46.3 | 2917 | ✅ PASS |  |
| `document_pii_scan` | API | 0.7 | 0 | ❌ FAIL | {} |
| `document_pii_scan` | WASM | 20.5 | 4500 | ✅ PASS | |
| `document_pii_scan` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | CLI | 14.3 | 0 | ❌ FAIL | error: unexpected argument '--template' found |
| `pdf_overlay_underlay` | MCP | 21.6 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_overlay_underlay` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_stamp` | CLI | 11.6 | 0 | ❌ FAIL | error: unexpected argument '--image' found |
| `pdf_stamp` | MCP | 4.8 | 29641 | ✅ PASS |  |
| `pdf_stamp` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_stamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_stamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_flatten` | CLI | 15.9 | 25649 | ✅ PASS |  |
| `pdf_flatten` | MCP | 3.3 | 25649 | ✅ PASS |  |
| `pdf_flatten` | API | 1.0 | 0 | ❌ FAIL | {} |
| `pdf_flatten` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_flatten` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_read_form` | CLI | 12.9 | 0 | ❌ FAIL | error: unrecognized subcommand 'read-form' |
| `pdf_read_form` | MCP | 3.6 | 0 | ✅ PASS |  |
| `pdf_read_form` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_read_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_read_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_fill_form` | CLI | 10.6 | 0 | ❌ FAIL | error: unrecognized subcommand 'fill-form' |
| `pdf_fill_form` | MCP | 5.1 | 23442 | ✅ PASS |  |
| `pdf_fill_form` | API | 1.1 | 0 | ❌ FAIL | {} |
| `pdf_fill_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_fill_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sign` | CLI | 15.7 | 29231 | ✅ PASS |  |
| `pdf_sign` | MCP | 3.3 | 29231 | ✅ PASS |  |
| `pdf_sign` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_sign` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sign` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | CLI | 9.7 | 0 | ❌ FAIL | error: unrecognized subcommand 'validate-signatures' |
| `pdf_validate_signatures` | MCP | 9.4 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_validate_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | CLI | 12.1 | 0 | ❌ FAIL | error: unrecognized subcommand 'remove-signatures' |
| `pdf_remove_signatures` | MCP | 6.3 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_remove_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_hash` | CLI | 12.9 | 0 | ✅ PASS |  |
| `pdf_hash` | MCP | 0.6 | 0 | ✅ PASS |  |
| `pdf_hash` | API | 1.1 | 0 | ❌ FAIL | {} |
| `pdf_hash` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_hash` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate` | CLI | 21.3 | 0 | ✅ PASS |  |
| `pdf_validate` | MCP | 5.3 | 0 | ✅ PASS |  |
| `pdf_validate` | API | 1.1 | 0 | ❌ FAIL | {} |
| `pdf_validate` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | CLI | 22.6 | 72592 | ✅ PASS |  |
| `pdf_change_permissions` | MCP | 9.0 | 72593 | ✅ PASS |  |
| `pdf_change_permissions` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_change_permissions` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | CLI | 11.0 | 0 | ❌ FAIL | error: unrecognized subcommand 'create-form-field' |
| `pdf_create_form_field` | MCP | 4.9 | 29350 | ✅ PASS |  |
| `pdf_create_form_field` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_create_form_field` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_timestamp` | CLI | 11.9 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | MCP | 0.2 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | API | 0.6 | 0 | ❌ FAIL | {} |
| `pdf_timestamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_timestamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_bates` | CLI | 10.7 | 0 | ❌ FAIL | error: the following required arguments were not provided: |
| `pdf_bates` | MCP | 5.7 | 72631 | ✅ PASS |  |
| `pdf_bates` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_bates` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_bates` | Desktop | 15.2 | 4500 | ✅ PASS | |
