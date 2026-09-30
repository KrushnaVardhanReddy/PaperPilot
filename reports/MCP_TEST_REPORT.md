# PaperPilot MCP Server — E2E Test Report
Generated: 2026-09-30T14:25:19.768267

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
| pdf_merge | 9 | OK |
| pdf_split | 6 | OK |
| pdf_rotate | 3 | OK |
| pdf_extract_pages | 3 | OK |
| pdf_delete_pages | 4 | OK |
| pdf_reorder_pages | 5 | OK |
| pdf_burst | 6 | OK |
| pdf_crop | 3 | OK |
| pdf_decrypt | 3 | OK |
| pdf_redact | 3 | OK |
| pdf_watermark | 3 | OK |
| pdf_header_footer | 4 | OK |
| pdf_metadata | 3 | OK |
| pdf_compress | 7 | OK |
| pdf_repair | 3 | OK |
| pdf_extract_text | 4 | OK |
| pdf_extract_images | 3 | OK |
| pdf_images_to_pdf | 4 | OK |
| pdf_bates | 4 | OK |
| pdf_search | 5 | OK |
| pdf_read_form | 3 | OK |
| pdf_to_docx | 6 | OK |
| pdf_classify_type | 7 | OK |

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
- pdf_rotate missing arguments correctly threw MCPError
- pdf_decrypt wrong password correctly threw MCPError
- unknown_tool correctly threw MCPError
