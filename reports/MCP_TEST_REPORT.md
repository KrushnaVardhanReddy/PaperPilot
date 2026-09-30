# PaperPilot MCP Server — E2E Test Report
Generated: 2026-09-30T10:39:05.581411

## Summary
- Total Tools Tested: 34
- Passed: 23 ✅
- Failed: 0 ❌
- Not Implemented: 9 ⚠️

## Tool Discovery
- Expected tools: 34
- Found tools: 38
- Missing: None

## Detailed Results

### ✅ Working Tools
| Tool | Latency (ms) | Notes |
|---|---|---|
| pdf_merge | 5 | OK |
| pdf_split | 2 | OK |
| pdf_rotate | 1 | OK |
| pdf_extract_pages | 2 | OK |
| pdf_delete_pages | 3 | OK |
| pdf_reorder_pages | 2 | OK |
| pdf_burst | 4 | OK |
| pdf_crop | 1 | OK |
| pdf_decrypt | 1 | OK |
| pdf_redact | 2 | OK |
| pdf_watermark | 1 | OK |
| pdf_header_footer | 2 | OK |
| pdf_metadata | 1 | OK |
| pdf_compress | 2 | OK |
| pdf_repair | 1 | OK |
| pdf_extract_text | 2 | OK |
| pdf_extract_images | 2 | OK |
| pdf_images_to_pdf | 2 | OK |
| pdf_bates | 2 | OK |
| pdf_search | 3 | OK |
| pdf_read_form | 1 | OK |
| pdf_to_docx | 4 | OK |
| pdf_classify_type | 3 | OK |

### ❌ Failed Tools
| Tool | Latency (ms) | Error | Root Cause Guess |
|---|---|---|---|

### ⚠️ Not Implemented
| Tool | Error |
|---|---|
| pdf_encrypt | Unsupported operation: Encrypt operation is not natively supported by lopdf |
| pdf_linearize | Unsupported operation: Linearization not yet supported by lopdf backend |
| pdf_flatten | Unsupported operation: Flattening interactive forms not yet supported natively |
| pdf_to_pdf_a | Unsupported operation: PDF/A conversion not yet supported natively |
| pdf_sign | Unsupported operation: Digital signatures not yet supported natively |
| pdf_render | Unsupported operation: Rendering requires an external engine like pdfium |
| pdf_ocr | Unsupported operation: OCR requires tesseract |
| pdf_compare | Unsupported operation: Compare operation is too complex for basic structural diff |
| pdf_bookmarks | Unsupported operation: Bookmarks operation is not supported yet |

### Schema Issues

### Error Handling
- pdf_rotate missing arguments correctly threw McpError
- pdf_decrypt wrong password correctly threw McpError
- unknown_tool correctly threw McpError

