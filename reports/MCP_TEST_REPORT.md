# PaperPilot MCP Server — E2E Test Report
Generated: 2026-09-30T13:37:03.307022

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
| pdf_merge | 60 | OK |
| pdf_split | 234 | OK |
| pdf_rotate | 176 | OK |
| pdf_extract_pages | 5 | OK |
| pdf_delete_pages | 252 | OK |
| pdf_reorder_pages | 229 | OK |
| pdf_burst | 12 | OK |
| pdf_crop | 3 | OK |
| pdf_encrypt | 7 | OK |
| pdf_decrypt | 227 | OK |
| pdf_redact | 7 | OK |
| pdf_watermark | 56 | OK |
| pdf_header_footer | 49 | OK |
| pdf_metadata | 53 | OK |
| pdf_compress | 32 | OK |
| pdf_linearize | 7 | OK |
| pdf_flatten | 2 | OK |
| pdf_repair | 3 | OK |
| pdf_to_pdf_a | 3 | OK |
| pdf_sign | 8 | OK |
| pdf_extract_text | 10 | OK |
| pdf_extract_images | 5 | OK |
| pdf_images_to_pdf | 28 | OK |
| pdf_bates | 104 | OK |
| pdf_render | 3 | OK |
| pdf_ocr | 2 | OK |
| pdf_compare | 3 | OK |
| pdf_search | 4 | OK |
| pdf_bookmarks | 2 | OK |
| pdf_read_form | 2 | OK |
| pdf_to_docx | 85 | OK |
| pdf_classify_type | 11 | OK |

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

