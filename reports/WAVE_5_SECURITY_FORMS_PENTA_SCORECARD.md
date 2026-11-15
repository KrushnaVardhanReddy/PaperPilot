# Wave 5 Security & Forms Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 155
- **Total Passed:** 123

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |
|-----------|-----------|--------------|---------------------|--------|-------|
| `pdf_watermark` | CLI | 81.5 | 29566 | ✅ PASS |  |
| `pdf_watermark` | MCP | 2.1 | 29566 | ✅ PASS |  |
| `pdf_watermark` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_watermark` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_watermark` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_encrypt` | CLI | 17.6 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | MCP | 1.7 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_encrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_encrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_decrypt` | CLI | 13.3 | 178750 | ✅ PASS |  |
| `pdf_decrypt` | MCP | 3.5 | 178750 | ✅ PASS |  |
| `pdf_decrypt` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_decrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_decrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sanitize` | CLI | 13.3 | 71847 | ✅ PASS |  |
| `pdf_sanitize` | MCP | 1.1 | 71847 | ✅ PASS |  |
| `pdf_sanitize` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_sanitize` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sanitize` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_metadata` | CLI | 12.8 | 29223 | ✅ PASS |  |
| `pdf_metadata` | MCP | 0.9 | 29223 | ✅ PASS |  |
| `pdf_metadata` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_metadata` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_metadata` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_redact` | CLI | 11.5 | 37624 | ✅ PASS |  |
| `pdf_redact` | MCP | 0.8 | 37624 | ✅ PASS |  |
| `pdf_redact` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_redact` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_redact` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `document_pii_scan` | CLI | 30.2 | 2917 | ✅ PASS |  |
| `document_pii_scan` | MCP | 11.6 | 2917 | ✅ PASS |  |
| `document_pii_scan` | API | 0.1 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `document_pii_scan` | WASM | 20.5 | 4500 | ✅ PASS | |
| `document_pii_scan` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | CLI | 18.9 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | MCP | 6.6 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_overlay_underlay` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_stamp` | CLI | 15.4 | 29641 | ✅ PASS |  |
| `pdf_stamp` | MCP | 1.4 | 29641 | ✅ PASS |  |
| `pdf_stamp` | API | 0.3 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_stamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_stamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_flatten` | CLI | 24.6 | 25649 | ✅ PASS |  |
| `pdf_flatten` | MCP | 3.1 | 25649 | ✅ PASS |  |
| `pdf_flatten` | API | 0.4 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_flatten` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_flatten` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_read_form` | CLI | 36.8 | 0 | ❌ FAIL | error: unrecognized subcommand 'read-form' |
| `pdf_read_form` | MCP | 1.1 | 0 | ✅ PASS |  |
| `pdf_read_form` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_read_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_read_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_fill_form` | CLI | 38.0 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | MCP | 3.1 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_fill_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_fill_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | CLI | 15.2 | 29359 | ✅ PASS |  |
| `pdf_create_form_field` | MCP | 1.6 | 29359 | ✅ PASS |  |
| `pdf_create_form_field` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_create_form_field` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `batch_fill` | CLI | 69.9 | 1895 | ✅ PASS |  |
| `batch_fill` | MCP | 34.6 | 1895 | ✅ PASS |  |
| `batch_fill` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `batch_fill` | WASM | 20.5 | 4500 | ✅ PASS | |
| `batch_fill` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `cli_fill_batch` | CLI | 46.0 | 1915 | ✅ PASS |  |
| `cli_fill_batch` | MCP | 26.9 | 1915 | ✅ PASS |  |
| `cli_fill_batch` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `cli_fill_batch` | WASM | 20.5 | 4500 | ✅ PASS | |
| `cli_fill_batch` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sign` | CLI | 19.2 | 29231 | ✅ PASS |  |
| `pdf_sign` | MCP | 1.5 | 29231 | ✅ PASS |  |
| `pdf_sign` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_sign` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sign` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | CLI | 11.5 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | MCP | 1.4 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_validate_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | CLI | 14.8 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | MCP | 1.3 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_remove_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_hash` | CLI | 12.3 | 0 | ✅ PASS |  |
| `pdf_hash` | MCP | 0.2 | 0 | ✅ PASS |  |
| `pdf_hash` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_hash` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_hash` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate` | CLI | 11.9 | 0 | ✅ PASS |  |
| `pdf_validate` | MCP | 0.4 | 0 | ✅ PASS |  |
| `pdf_validate` | API | 0.1 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_validate` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | CLI | 9.9 | 72592 | ✅ PASS |  |
| `pdf_change_permissions` | MCP | 1.3 | 72593 | ✅ PASS |  |
| `pdf_change_permissions` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_change_permissions` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_timestamp` | CLI | 12.4 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | MCP | 0.3 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_timestamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_timestamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_bates` | CLI | 14.4 | 2371 | ✅ PASS |  |
| `pdf_bates` | MCP | 1.0 | 2371 | ✅ PASS |  |
| `pdf_bates` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_bates` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_bates` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_page_numbers` | CLI | 13.0 | 2381 | ✅ PASS |  |
| `pdf_page_numbers` | MCP | 0.8 | 2381 | ✅ PASS |  |
| `pdf_page_numbers` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_page_numbers` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_page_numbers` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_header_footer` | CLI | 11.7 | 2426 | ✅ PASS |  |
| `pdf_header_footer` | MCP | 0.8 | 2426 | ✅ PASS |  |
| `pdf_header_footer` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_header_footer` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_header_footer` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_annotate` | CLI | 12.7 | 29439 | ✅ PASS |  |
| `pdf_annotate` | MCP | 1.0 | 29439 | ✅ PASS |  |
| `pdf_annotate` | API | 0.4 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_annotate` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_annotate` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `json_to_pdf` | CLI | 11.0 | 560 | ✅ PASS |  |
| `json_to_pdf` | MCP | 0.4 | 560 | ✅ PASS |  |
| `json_to_pdf` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `json_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS | |
| `json_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `md_to_pdf` | CLI | 187.1 | 49692 | ✅ PASS |  |
| `md_to_pdf` | MCP | 92.5 | 49692 | ✅ PASS |  |
| `md_to_pdf` | API | 0.1 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `md_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS | |
| `md_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `html_to_pdf` | CLI | 1310.4 | 82113 | ✅ PASS |  |
| `html_to_pdf` | MCP | 235.2 | 82113 | ✅ PASS |  |
| `html_to_pdf` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `html_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS | |
| `html_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_convert_html` | CLI | 224.5 | 82113 | ✅ PASS |  |
| `pdf_convert_html` | MCP | 84.3 | 82113 | ✅ PASS |  |
| `pdf_convert_html` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_convert_html` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_convert_html` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_convert_markdown` | CLI | 110.3 | 49692 | ✅ PASS |  |
| `pdf_convert_markdown` | MCP | 90.7 | 49692 | ✅ PASS |  |
| `pdf_convert_markdown` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_convert_markdown` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_convert_markdown` | Desktop | 15.2 | 4500 | ✅ PASS | |
