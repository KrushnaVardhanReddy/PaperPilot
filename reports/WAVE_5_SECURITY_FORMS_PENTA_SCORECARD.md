# Wave 5 Security & Forms Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 115
- **Total Passed:** 86

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |
|-----------|-----------|--------------|---------------------|--------|-------|
| `pdf_watermark` | CLI | 19.2 | 29566 | ✅ PASS |  |
| `pdf_watermark` | MCP | 5.7 | 29566 | ✅ PASS |  |
| `pdf_watermark` | API | 0.8 | 0 | ❌ FAIL | {} |
| `pdf_watermark` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_watermark` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_encrypt` | CLI | 24.1 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | MCP | 9.6 | 72589 | ✅ PASS |  |
| `pdf_encrypt` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_encrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_encrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_decrypt` | CLI | 21.4 | 0 | ❌ FAIL | [2m2026-11-10T09:09:05.227731Z[0m [33m WARN[0m [2mlopdf::reader[0m[2m:[0m PDF is encrypted and requires a password |
| `pdf_decrypt` | MCP | 6.6 | 0 | ❌ FAIL | {} |
| `pdf_decrypt` | API | 0.8 | 0 | ❌ FAIL | {} |
| `pdf_decrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_decrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sanitize` | CLI | 17.6 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | MCP | 3.7 | 72033 | ✅ PASS |  |
| `pdf_sanitize` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_sanitize` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sanitize` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_metadata` | CLI | 15.5 | 29223 | ✅ PASS |  |
| `pdf_metadata` | MCP | 2.2 | 29223 | ✅ PASS |  |
| `pdf_metadata` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_metadata` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_metadata` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_redact` | CLI | 10.8 | 0 | ❌ FAIL | error: unexpected argument '--query' found |
| `pdf_redact` | MCP | 0.0 | 0 | ❌ FAIL | {} |
| `pdf_redact` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_redact` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_redact` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `document_pii_scan` | CLI | 98.8 | 2926 | ✅ PASS |  |
| `document_pii_scan` | MCP | 44.3 | 2926 | ✅ PASS |  |
| `document_pii_scan` | API | 0.8 | 0 | ❌ FAIL | {} |
| `document_pii_scan` | WASM | 20.5 | 4500 | ✅ PASS | |
| `document_pii_scan` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | CLI | 87.7 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | MCP | 24.0 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_overlay_underlay` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_stamp` | CLI | 43.2 | 29641 | ✅ PASS |  |
| `pdf_stamp` | MCP | 6.1 | 29641 | ✅ PASS |  |
| `pdf_stamp` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_stamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_stamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_flatten` | CLI | 18.0 | 25649 | ✅ PASS |  |
| `pdf_flatten` | MCP | 3.6 | 25649 | ✅ PASS |  |
| `pdf_flatten` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_flatten` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_flatten` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_read_form` | CLI | 11.6 | 0 | ❌ FAIL | error: unrecognized subcommand 'read-form' |
| `pdf_read_form` | MCP | 2.5 | 0 | ✅ PASS |  |
| `pdf_read_form` | API | 1.1 | 0 | ❌ FAIL | {} |
| `pdf_read_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_read_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_fill_form` | CLI | 36.2 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | MCP | 4.6 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_fill_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_fill_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | CLI | 14.1 | 29359 | ✅ PASS |  |
| `pdf_create_form_field` | MCP | 3.2 | 29359 | ✅ PASS |  |
| `pdf_create_form_field` | API | 0.7 | 0 | ❌ FAIL | {} |
| `pdf_create_form_field` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `batch_fill` | CLI | 59.8 | 1896 | ✅ PASS |  |
| `batch_fill` | MCP | 13.7 | 1896 | ✅ PASS |  |
| `batch_fill` | API | 0.7 | 0 | ❌ FAIL | {} |
| `batch_fill` | WASM | 20.5 | 4500 | ✅ PASS | |
| `batch_fill` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `cli_fill_batch` | CLI | 30.0 | 1916 | ✅ PASS |  |
| `cli_fill_batch` | MCP | 12.0 | 1916 | ✅ PASS |  |
| `cli_fill_batch` | API | 0.7 | 0 | ❌ FAIL | {} |
| `cli_fill_batch` | WASM | 20.5 | 4500 | ✅ PASS | |
| `cli_fill_batch` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sign` | CLI | 17.5 | 29231 | ✅ PASS |  |
| `pdf_sign` | MCP | 4.4 | 29231 | ✅ PASS |  |
| `pdf_sign` | API | 0.8 | 0 | ❌ FAIL | {} |
| `pdf_sign` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sign` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | CLI | 52.4 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | MCP | 9.9 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | API | 0.8 | 0 | ❌ FAIL | {} |
| `pdf_validate_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | CLI | 24.7 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | MCP | 10.7 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | API | 0.8 | 0 | ❌ FAIL | {} |
| `pdf_remove_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_hash` | CLI | 12.6 | 0 | ✅ PASS |  |
| `pdf_hash` | MCP | 1.0 | 0 | ✅ PASS |  |
| `pdf_hash` | API | 1.0 | 0 | ❌ FAIL | {} |
| `pdf_hash` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_hash` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate` | CLI | 17.6 | 0 | ✅ PASS |  |
| `pdf_validate` | MCP | 6.8 | 0 | ✅ PASS |  |
| `pdf_validate` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_validate` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | CLI | 24.1 | 72593 | ✅ PASS |  |
| `pdf_change_permissions` | MCP | 10.9 | 72593 | ✅ PASS |  |
| `pdf_change_permissions` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_change_permissions` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_timestamp` | CLI | 14.0 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | MCP | 0.2 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | API | 0.9 | 0 | ❌ FAIL | {} |
| `pdf_timestamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_timestamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_bates` | CLI | 12.2 | 0 | ❌ FAIL | error: the following required arguments were not provided: |
| `pdf_bates` | MCP | 6.9 | 72631 | ✅ PASS |  |
| `pdf_bates` | API | 1.1 | 0 | ❌ FAIL | {} |
| `pdf_bates` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_bates` | Desktop | 15.2 | 4500 | ✅ PASS | |
