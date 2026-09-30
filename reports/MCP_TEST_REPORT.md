# PaperPilot MCP Server — E2E Test Report
Generated: 2026-09-30T17:28:03.949182

## Summary
- Total Tools Tested: 34
- Passed: 32 ✅
- Failed: 0 ❌
- Not Implemented: 0 ⚠️

## Tool Discovery
- Expected tools: 34
- Found tools: 38
- Missing: None

## Detailed Results

### ✅ Working Tools
| Tool | Latency (ms) | Notes |
|---|---|---|
| pdf_merge | 12 | OK |
| pdf_split | 9 | OK |
| pdf_rotate | 5 | OK |
| pdf_extract_pages | 5 | OK |
| pdf_delete_pages | 5 | OK |
| pdf_reorder_pages | 5 | OK |
| pdf_burst | 6 | OK |
| pdf_crop | 3 | OK |
| pdf_encrypt | 5 | OK |
| pdf_decrypt | 3 | OK |
| pdf_redact | 4 | OK |
| pdf_watermark | 3 | OK |
| pdf_header_footer | 4 | OK |
| pdf_metadata | 3 | OK |
| pdf_compress | 9 | OK |
| pdf_linearize | 8 | OK |
| pdf_flatten | 5 | OK |
| pdf_repair | 4 | OK |
| pdf_to_pdf_a | 4 | OK |
| pdf_sign | 5 | OK |
| pdf_extract_text | 6 | OK |
| pdf_extract_images | 5 | OK |
| pdf_images_to_pdf | 6 | OK |
| pdf_bates | 5 | OK |
| pdf_render | 5 | OK |
| pdf_ocr | 5 | OK |
| pdf_compare | 5 | OK |
| pdf_search | 5 | OK |
| pdf_bookmarks | 4 | OK |
| pdf_read_form | 2 | OK |
| pdf_to_docx | 7 | OK |
| pdf_classify_type | 7 | OK |

### ❌ Failed Tools
| Tool | Latency (ms) | Error | Root Cause Guess |
|---|---|---|---|

### ⚠️ Not Implemented
| Tool | Error |
|---|---|

### Schema Issues

### Error Handling
- pdf_rotate missing arguments correctly threw McpError
- pdf_decrypt wrong password correctly threw McpError
- unknown_tool correctly threw McpError

