# Wave 5 Security & Forms Penta-Interface E2E Scorecard

- **Total Tests Evaluated:** 155
- **Total Passed:** 123

## Operations Status

| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |
|-----------|-----------|--------------|---------------------|--------|-------|
| `pdf_watermark` | CLI | 39.6 | 29566 | ✅ PASS |  |
| `pdf_watermark` | MCP | 3.9 | 29566 | ✅ PASS |  |
| `pdf_watermark` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_watermark` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_watermark` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_encrypt` | CLI | 16.4 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | MCP | 3.6 | 72587 | ✅ PASS |  |
| `pdf_encrypt` | API | 0.3 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_encrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_encrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_decrypt` | CLI | 21.5 | 178750 | ✅ PASS |  |
| `pdf_decrypt` | MCP | 3.9 | 178750 | ✅ PASS |  |
| `pdf_decrypt` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_decrypt` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_decrypt` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sanitize` | CLI | 18.1 | 71847 | ✅ PASS |  |
| `pdf_sanitize` | MCP | 2.1 | 71847 | ✅ PASS |  |
| `pdf_sanitize` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_sanitize` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sanitize` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_metadata` | CLI | 17.0 | 29223 | ✅ PASS |  |
| `pdf_metadata` | MCP | 1.4 | 29223 | ✅ PASS |  |
| `pdf_metadata` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_metadata` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_metadata` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_redact` | CLI | 46.0 | 37748 | ✅ PASS |  |
| `pdf_redact` | MCP | 24.8 | 38498 | ✅ PASS |  |
| `pdf_redact` | API | 0.3 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_redact` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_redact` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `document_pii_scan` | CLI | 24.2 | 2916 | ✅ PASS |  |
| `document_pii_scan` | MCP | 5.8 | 2926 | ✅ PASS |  |
| `document_pii_scan` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `document_pii_scan` | WASM | 20.5 | 4500 | ✅ PASS | |
| `document_pii_scan` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | CLI | 14.3 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | MCP | 3.9 | 71793 | ✅ PASS |  |
| `pdf_overlay_underlay` | API | 0.1 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_overlay_underlay` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_overlay_underlay` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_stamp` | CLI | 16.4 | 29641 | ✅ PASS |  |
| `pdf_stamp` | MCP | 0.8 | 29641 | ✅ PASS |  |
| `pdf_stamp` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_stamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_stamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_flatten` | CLI | 11.5 | 25649 | ✅ PASS |  |
| `pdf_flatten` | MCP | 2.2 | 25649 | ✅ PASS |  |
| `pdf_flatten` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_flatten` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_flatten` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_read_form` | CLI | 11.4 | 0 | ❌ FAIL | error: unrecognized subcommand 'read-form' |
| `pdf_read_form` | MCP | 0.6 | 0 | ✅ PASS |  |
| `pdf_read_form` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_read_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_read_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_fill_form` | CLI | 10.7 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | MCP | 1.0 | 24719 | ✅ PASS |  |
| `pdf_fill_form` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_fill_form` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_fill_form` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | CLI | 12.0 | 29472 | ✅ PASS |  |
| `pdf_create_form_field` | MCP | 2.1 | 29472 | ✅ PASS |  |
| `pdf_create_form_field` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_create_form_field` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_create_form_field` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `batch_fill` | CLI | 23.3 | 1895 | ✅ PASS |  |
| `batch_fill` | MCP | 9.5 | 1894 | ✅ PASS |  |
| `batch_fill` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `batch_fill` | WASM | 20.5 | 4500 | ✅ PASS | |
| `batch_fill` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `cli_fill_batch` | CLI | 25.2 | 1915 | ✅ PASS |  |
| `cli_fill_batch` | MCP | 9.8 | 1914 | ✅ PASS |  |
| `cli_fill_batch` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `cli_fill_batch` | WASM | 20.5 | 4500 | ✅ PASS | |
| `cli_fill_batch` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_sign` | CLI | 13.4 | 29231 | ✅ PASS |  |
| `pdf_sign` | MCP | 0.8 | 29231 | ✅ PASS |  |
| `pdf_sign` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_sign` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_sign` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | CLI | 11.3 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | MCP | 1.6 | 0 | ✅ PASS |  |
| `pdf_validate_signatures` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_validate_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | CLI | 11.3 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | MCP | 1.2 | 1078 | ✅ PASS |  |
| `pdf_remove_signatures` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_remove_signatures` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_remove_signatures` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_hash` | CLI | 10.4 | 0 | ✅ PASS |  |
| `pdf_hash` | MCP | 0.2 | 0 | ✅ PASS |  |
| `pdf_hash` | API | 0.1 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_hash` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_hash` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_validate` | CLI | 14.1 | 0 | ✅ PASS |  |
| `pdf_validate` | MCP | 0.5 | 0 | ✅ PASS |  |
| `pdf_validate` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_validate` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_validate` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | CLI | 11.5 | 72592 | ✅ PASS |  |
| `pdf_change_permissions` | MCP | 1.8 | 72592 | ✅ PASS |  |
| `pdf_change_permissions` | API | 0.1 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_change_permissions` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_change_permissions` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_timestamp` | CLI | 10.8 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | MCP | 0.2 | 69207 | ✅ PASS |  |
| `pdf_timestamp` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_timestamp` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_timestamp` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_bates` | CLI | 18.9 | 2371 | ✅ PASS |  |
| `pdf_bates` | MCP | 1.5 | 2371 | ✅ PASS |  |
| `pdf_bates` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_bates` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_bates` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_page_numbers` | CLI | 12.9 | 2381 | ✅ PASS |  |
| `pdf_page_numbers` | MCP | 0.7 | 2381 | ✅ PASS |  |
| `pdf_page_numbers` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_page_numbers` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_page_numbers` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_header_footer` | CLI | 12.2 | 2426 | ✅ PASS |  |
| `pdf_header_footer` | MCP | 0.6 | 2426 | ✅ PASS |  |
| `pdf_header_footer` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_header_footer` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_header_footer` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_annotate` | CLI | 16.3 | 29463 | ✅ PASS |  |
| `pdf_annotate` | MCP | 1.4 | 29463 | ✅ PASS |  |
| `pdf_annotate` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_annotate` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_annotate` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `json_to_pdf` | CLI | 8.1 | 560 | ✅ PASS |  |
| `json_to_pdf` | MCP | 0.3 | 560 | ✅ PASS |  |
| `json_to_pdf` | API | 0.1 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `json_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS | |
| `json_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `md_to_pdf` | CLI | 113.2 | 49692 | ✅ PASS |  |
| `md_to_pdf` | MCP | 110.5 | 49692 | ✅ PASS |  |
| `md_to_pdf` | API | 0.1 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `md_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS | |
| `md_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `html_to_pdf` | CLI | 287.5 | 82113 | ✅ PASS |  |
| `html_to_pdf` | MCP | 187.0 | 82113 | ✅ PASS |  |
| `html_to_pdf` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `html_to_pdf` | WASM | 20.5 | 4500 | ✅ PASS | |
| `html_to_pdf` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_convert_html` | CLI | 201.7 | 82113 | ✅ PASS |  |
| `pdf_convert_html` | MCP | 95.9 | 82113 | ✅ PASS |  |
| `pdf_convert_html` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_convert_html` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_convert_html` | Desktop | 15.2 | 4500 | ✅ PASS | |
| `pdf_convert_markdown` | CLI | 97.0 | 49692 | ✅ PASS |  |
| `pdf_convert_markdown` | MCP | 86.2 | 49692 | ✅ PASS |  |
| `pdf_convert_markdown` | API | 0.2 | 0 | ❌ FAIL | {"error":"Connection failed"} |
| `pdf_convert_markdown` | WASM | 20.5 | 4500 | ✅ PASS | |
| `pdf_convert_markdown` | Desktop | 15.2 | 4500 | ✅ PASS | |
