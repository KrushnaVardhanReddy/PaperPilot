# PaperPilot MCP Server — E2E Test Report
Generated: 2026-09-30T11:51:42.734508

## Summary
- Total Tools Tested: 34
- Passed: 7 ✅
- Failed: 19 ❌
- Not Implemented: 8 ⚠️

## Tool Discovery
- Expected tools: 34
- Found tools: 36
- Missing: pdf_reorder, pdf_pdf_a, pdf_form, pdf_hash, pdf_validate, pdf_convert, pdf_classify

## Detailed Results

### ✅ Working Tools
| Tool | Latency (ms) | Notes |
|---|---|---|
| pdf_decrypt | 9 | OK |
| pdf_watermark | 3 | OK |
| pdf_metadata | 3 | OK |
| pdf_compress | 7 | OK |
| pdf_repair | 3 | OK |
| pdf_bates | 3 | OK |
| pdf_search | 5 | OK |

### ❌ Failed Tools
| Tool | Latency (ms) | Error | Root Cause Guess |
|---|---|---|---|
| pdf_merge | 2 | MCPError: Missing or invalid 'inputs' parameter | |
| pdf_split | 8 | File not found: /app/tests/e2e_fixtures/multi_page_part_1.pdf | |
| pdf_rotate | 1 | MCPError: Missing or invalid 'pages' parameter | |
| pdf_extract_pages | 1 | MCPError: Missing or invalid 'pages' parameter | |
| pdf_delete_pages | 1 | MCPError: Missing or invalid 'pages' parameter | |
| pdf_reorder | 0 | MCPError: Unknown tool | |
| pdf_burst | 6 | File not found: /app/tests/e2e_fixtures/multi_page_page_1.pdf | |
| pdf_crop | 1 | MCPError: Missing or invalid 'box' parameter | |
| pdf_redact | 1 | MCPError: Missing or invalid 'page' parameter | |
| pdf_header_footer | 1 | MCPError: Missing or invalid 'position' parameter | |
| pdf_pdf_a | 1 | MCPError: Unknown tool | |
| pdf_extract_text | 1 | MCPError: Missing or invalid 'output' parameter | |
| pdf_extract_images | 3 | File not found: /app/tests/e2e_fixtures/image_doc_img_1.png | |
| pdf_images_to_pdf | 1 | MCPError: Parse error: Trailer missing Root | |
| pdf_form | 1 | MCPError: Unknown tool | |
| pdf_hash | 1 | MCPError: Unknown tool | |
| pdf_validate | 1 | MCPError: Unknown tool | |
| pdf_convert | 1 | MCPError: Unknown tool | |
| pdf_classify | 1 | MCPError: Unknown tool | |

### ⚠️ Not Implemented
| Tool | Error |
|---|---|
| pdf_encrypt | Unsupported operation: Encrypt operation is not natively supported by lopdf |
| pdf_linearize | Unsupported operation: Linearization not yet supported by lopdf backend |
| pdf_flatten | Unsupported operation: Flattening interactive forms not yet supported natively |
| pdf_sign | Unsupported operation: Digital signatures not yet supported natively |
| pdf_render | Unsupported operation: Rendering requires an external engine like pdfium |
| pdf_ocr | Unsupported operation: OCR requires tesseract |
| pdf_compare | Unsupported operation: Compare operation is too complex for basic structural diff |
| pdf_bookmarks | Unsupported operation: Bookmarks operation is not supported yet |

### Schema Issues
- pdf_to_docx: missing description for required 'input'
- pdf_to_docx: missing description for required 'output'
- pdf_to_xlsx: missing description for required 'input'
- pdf_to_xlsx: missing description for required 'output'
- pdf_to_pptx: missing description for required 'input'
- pdf_to_pptx: missing description for required 'output'
- pdf_classify_type: missing description for required 'input'
- pdf_read_form: missing description for required 'input'
- pdf_fill_form: missing description for required 'input'
- pdf_fill_form: missing description for required 'output'
- pdf_fill_form: missing description for required 'values'
- pdf_create_form_field: missing description for required 'field_name'
- pdf_create_form_field: missing description for required 'field_type'
- pdf_create_form_field: missing description for required 'height'
- pdf_create_form_field: missing description for required 'input'
- pdf_create_form_field: missing description for required 'output'
- pdf_create_form_field: missing description for required 'page'
- pdf_create_form_field: missing description for required 'width'
- pdf_create_form_field: missing description for required 'x'
- pdf_create_form_field: missing description for required 'y'
- pdf_redact: missing description for required 'height'
- pdf_redact: missing description for required 'width'
- pdf_redact: missing description for required 'x'
- pdf_redact: missing description for required 'y'
- pdf_header_footer: missing description for required 'input'
- pdf_header_footer: missing description for required 'output'
- pdf_header_footer: missing description for required 'text'
- pdf_bates: missing description for required 'input'
- pdf_bates: missing description for required 'output'
- pdf_bates: missing description for required 'padding'
- pdf_bates: missing description for required 'prefix'
- pdf_bates: missing description for required 'start_number'
- pdf_render: missing description for required 'input'
- pdf_render: missing description for required 'output'
- pdf_images_to_pdf: missing description for required 'inputs'
- pdf_images_to_pdf: missing description for required 'output'
- pdf_compare: missing description for required 'input_a'
- pdf_compare: missing description for required 'input_b'
- pdf_compare: missing description for required 'output'
- pdf_bookmarks: missing description for required 'input'
- pdf_bookmarks: missing description for required 'output'
- pdf_ocr: missing description for required 'input'
- pdf_ocr: missing description for required 'output'

### Error Handling
- pdf_rotate missing arguments correctly threw MCPError
- pdf_decrypt wrong password DID NOT FAIL
- unknown_tool correctly threw MCPError
