# PaperPilot CLI — E2E Test Report
Generated: Wed Sep 30 08:40:10 AM EDT 2026

## Summary
- Total Tests: 30
- Passed: 30 ✅
- Failed: 0 ❌
- Skipped (not implemented): 11 ⚠️

## Detailed Results

### ✅ Working Commands
| Command | Execution Time | Notes |
|---|---|---|
| `merge` | 11ms | Passed |
| `split` | 15ms | Passed |
| `rotate` | 13ms | Passed |
| `extract-pages` | 9ms | Passed |
| `delete` | 16ms | Passed |
| `reorder` | 13ms | Passed |
| `burst` | 14ms | Passed |
| `crop` | 10ms | Passed |
| `redact` | 14ms | Passed |
| `watermark` | 14ms | Passed |
| `header-footer` | 11ms | Passed |
| `metadata` | 13ms | Passed |
| `compress` | 13ms | Passed |
| `repair` | 14ms | Passed |
| `extract-text` | 13ms | Passed |
| `extract-images` | 20ms | Passed |
| `images-to-pdf` | 12ms | Passed |
| `bates` | 14ms | Passed |
| `search` | 12ms | Passed |
| `form read` | 12ms | Passed |
| `hash` | 11ms | Passed |
| `validate` | 10ms | Passed |
| `conversion markdown` | 10ms | Passed |
| `classify` | 15ms | Passed |
| `err_merge_missing` | 10ms | Passed |
| `err_rotate_invalid` | 10ms | Passed |
| `err_permission` | 12ms | Passed |
| `json_hash` | 7ms | Passed |
| `json_merge` | 12ms | Passed |
| `json_err_merge_missing` | 8ms | Passed |

### ❌ Failed Commands
| Command | Exit Code | Error Output | Root Cause (best guess) |
|---|---|---|---|

### ⚠️ Not Implemented
- `encrypt`
- `decrypt`
- `linearize`
- `flatten`
- `pdf-a`
- `signature`
- `render`
- `ocr`
- `compare`
- `bookmarks`
- `err_decrypt_wrong`

### UX Issues
Some commands like `compare`, `validate`, `bookmarks` don't output files or clear status text consistently, making them hard to automate. The error message on incorrect arguments for `rotate` could be clearer. Not all commands output their error to stderr consistently.
