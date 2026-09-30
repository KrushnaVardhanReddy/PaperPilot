# PaperPilot CLI — E2E Test Report
Generated: Wed Sep 30 09:22:50 UTC 2026

## Summary
- Total Tests: 40
- Passed: 29 ✅
- Failed: 11 ❌
- Skipped (not implemented): 1 ⚠️

## Detailed Results

### ✅ Working Commands
| Command | Execution Time | Notes |
|---|---|---|
| `merge` | 21ms | Passed |
| `split` | 21ms | Passed |
| `rotate` | 19ms | Passed |
| `extract-pages` | 18ms | Passed |
| `delete` | 19ms | Passed |
| `reorder` | 18ms | Passed |
| `burst` | 20ms | Passed |
| `crop` | 18ms | Passed |
| `decrypt` | 18ms | Passed |
| `redact` | 18ms | Passed |
| `watermark` | 18ms | Passed |
| `header-footer` | 19ms | Passed |
| `metadata` | 18ms | Passed |
| `compress` | 19ms | Passed |
| `repair` | 19ms | Passed |
| `extract-text` | 19ms | Passed |
| `extract-images` | 23ms | Passed |
| `bates` | 21ms | Passed |
| `search` | 15ms | Passed |
| `form read` | 16ms | Passed |
| `hash` | 15ms | Passed |
| `validate` | 16ms | Passed |
| `conversion markdown` | 18ms | Passed |
| `classify` | 16ms | Passed |
| `err_merge_missing` | 16ms | Passed |
| `err_rotate_invalid` | 15ms | Passed |
| `err_permission` | 16ms | Passed |
| `json_hash` | 15ms | Passed |
| `json_merge` | 18ms | Passed |

### ❌ Failed Commands
| Command | Exit Code | Error Output | Root Cause (best guess) |
|---|---|---|---|
| `encrypt` | 1 | `Command failed. Stderr: [2026-09-30T09:22:50Z ERROR paperpilot_cli] Operation failed: Unsupported operation: Encrypt operation is not natively supported by lopdf Stdout:  ` | Root Cause |
| `linearize` | 1 | `Command failed. Stderr: [2026-09-30T09:22:50Z ERROR paperpilot_cli] Operation failed: Unsupported operation: Linearization not yet supported by lopdf backend Stdout:  ` | Root Cause |
| `flatten` | 1 | `Command failed. Stderr: [2026-09-30T09:22:50Z ERROR paperpilot_cli] Operation failed: Unsupported operation: Flattening interactive forms not yet supported natively Stdout:  ` | Root Cause |
| `pdf-a` | 1 | `Command failed. Stderr: [2026-09-30T09:22:51Z ERROR paperpilot_cli] Operation failed: Unsupported operation: PDF/A conversion not yet supported natively Stdout:  ` | Root Cause |
| `signature` | 1 | `Command failed. Stderr: [2026-09-30T09:22:51Z ERROR paperpilot_cli] Operation failed: Unsupported operation: Digital signatures not yet supported natively Stdout:  ` | Root Cause |
| `images-to-pdf` | 1 | `Command failed. Stderr: [2026-09-30T09:22:51Z ERROR paperpilot_cli] Operation failed: Parse error: Trailer missing Root Stdout:  ` | Root Cause |
| `render` | 1 | `Command failed. Stderr: [2026-09-30T09:22:51Z ERROR paperpilot_cli] Operation failed: Unsupported operation: Rendering requires an external engine like pdfium Stdout:  ` | Root Cause |
| `ocr` | 1 | `Command failed. Stderr: [2026-09-30T09:22:51Z ERROR paperpilot_cli] Operation failed: Unsupported operation: OCR requires tesseract Stdout:  ` | Root Cause |
| `compare` | 1 | `Command failed. Stderr: [2026-09-30T09:22:51Z ERROR paperpilot_cli] Operation failed: Unsupported operation: Compare operation is too complex for basic structural diff Stdout:  ` | Root Cause |
| `err_decrypt_wrong` | 0 | `Expected failure but got exit code 0 ` | Root Cause |
| `json_err_merge_missing` | 1 | `Command failed. Stderr:  Stdout: {"success":false,"operation":"Merge","error":"Parse error: IO error"} ` | Root Cause |

### ⚠️ Not Implemented
- `bookmarks`

### UX Issues
Some commands like `compare`, `validate`, `bookmarks` don't output files or clear status text consistently, making them hard to automate. The error message on incorrect arguments for `rotate` could be clearer. Not all commands output their error to stderr consistently.
