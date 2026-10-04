# Phase 4.E2E-R3 — Batch 1: Structural & Page Operations Tri-Interface Test & Living Documentation

This document provides verified copy-pasteable working examples for CLI, MCP, and REST API across PaperPilot operations.

## Master Parity Matrix

| Tool | Synopsis | CLI Status | MCP Status | API Status |
|---|---|---|---|---|
| `pdf_merge` | Merges multiple PDF files into a single document. | ❌ FAIL (2) | ✅ PASS | ✅ PASS |
| `pdf_split` | Splits a multi-page PDF document into smaller documents based on specified pages. | ❌ FAIL (1) | ❌ FAIL (0) | ❌ FAIL (500) |
| `pdf_rotate` | Rotates all pages of a PDF document. | ❌ FAIL (2) | ❌ FAIL (0) | ❌ FAIL (500) |
| `pdf_extract_pages` | Extracts specific pages from a PDF document to create a new PDF. | ✅ PASS | ✅ PASS | ✅ PASS |
| `pdf_delete_pages` | Removes specific pages from a PDF document. | ✅ PASS | ✅ PASS | ✅ PASS |
| `pdf_reorder_pages` | Reorders the pages of a PDF document according to a specified sequence. | ❌ FAIL (1) | ❌ FAIL (0) | ❌ FAIL (500) |
| `pdf_burst` | Bursts a PDF document into individual 1-page PDF files. | ❌ FAIL (2) | ❌ FAIL (0) | ❌ FAIL (500) |
| `pdf_crop` | Crops all pages of a PDF document to the specified dimensions. | ❌ FAIL (2) | ❌ FAIL (0) | ❌ FAIL (500) |
| `pdf_remove_blank` | Removes empty or blank pages from a PDF document. | ❌ FAIL (2) | ✅ PASS | ✅ PASS |
| `pdf_page_numbers` | Adds page numbers to a PDF document. | ❌ FAIL (2) | ✅ PASS | ✅ PASS |

## Operations Detail

### Merge (`pdf_merge`)

**Synopsis:** Merges multiple PDF files into a single document.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 925 | 0 | -925 | 0 | No | 10 | ❌ FAIL |
| MCP | 925 | 1803 | 878 | 1 | Yes | 39 | ✅ PASS |
| API | 925 | 1803 | 878 | 1 | Yes | 62 | ✅ PASS |

**CLI Error:**
```
error: unexpected argument '/app/tests/e2e_fixtures/page_2.pdf' found

Usage: paperpilot-cli merge [OPTIONS] --input <INPUT> --output <OUTPUT>

For more information, try '--help'.

```

#### Examples

**CLI:**
```bash
paperpilot-cli merge --input /app/tests/e2e_fixtures/page_1.pdf /app/tests/e2e_fixtures/page_2.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/merged.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_merge",
    "arguments": {
      "inputs": [
        "/app/tests/e2e_fixtures/page_1.pdf",
        "/app/tests/e2e_fixtures/page_2.pdf"
      ],
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_merged.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_merge \
  -H "Content-Type: application/json" \
  -d '{"inputs": ["/app/tests/e2e_fixtures/page_1.pdf", "/app/tests/e2e_fixtures/page_2.pdf"], "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_merged.pdf"}'
```

---

### Split (`pdf_split`)

**Synopsis:** Splits a multi-page PDF document into smaller documents based on specified pages.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 1409 | 0 | -1409 | 0 | No | 14 | ❌ FAIL |
| MCP | 1409 | 0 | -1409 | 0 | No | 8 | ❌ FAIL |
| API | 1409 | 0 | -1409 | 0 | No | 2 | ❌ FAIL |

**CLI Error:**
```

```

**MCP Error:**
```
Error executing MCP tool: ErrorData { code: ErrorCode(-32602), message: "Missing or invalid 'output_dir' parameter", data: None }

```

**API Response:**
```
{"error":"Missing or invalid 'output_dir' parameter","success":false}
```

#### Examples

**CLI:**
```bash
paperpilot-cli split --input /app/tests/e2e_fixtures/multi_page.pdf --pages 1-2 --output /app/tests/e2e_fixtures/out/tri_e2e/split.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_split",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/multi_page.pdf",
      "pages": "1-2",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_split.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_split \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/multi_page.pdf", "pages": "1-2", "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_split.pdf"}'
```

---

### Rotate (`pdf_rotate`)

**Synopsis:** Rotates all pages of a PDF document.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 539 | 0 | -539 | 0 | No | 10 | ❌ FAIL |
| MCP | 539 | 0 | -539 | 0 | No | 8 | ❌ FAIL |
| API | 539 | 0 | -539 | 0 | No | 2 | ❌ FAIL |

**CLI Error:**
```
error: unexpected argument '--angle' found

  tip: a similar argument exists: '--pages'

Usage: paperpilot-cli rotate --input <INPUT> --pages <PAGES> --degrees <DEGREES> --output <OUTPUT>

For more information, try '--help'.

```

**MCP Error:**
```
Error executing MCP tool: ErrorData { code: ErrorCode(-32602), message: "Missing or invalid 'pages' parameter", data: None }

```

**API Response:**
```
{"error":"Missing or invalid 'pages' parameter","success":false}
```

#### Examples

**CLI:**
```bash
paperpilot-cli rotate --input /app/tests/e2e_fixtures/single_page.pdf --angle 90 --output /app/tests/e2e_fixtures/out/tri_e2e/rotated.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_rotate",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/single_page.pdf",
      "angle": 90,
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_rotated.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_rotate \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/single_page.pdf", "angle": 90, "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_rotated.pdf"}'
```

---

### Extract Pages (`pdf_extract_pages`)

**Synopsis:** Extracts specific pages from a PDF document to create a new PDF.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 1409 | 1046 | -363 | 1 | Yes | 13 | ✅ PASS |
| MCP | 1409 | 1046 | -363 | 1 | Yes | 13 | ✅ PASS |
| API | 1409 | 1046 | -363 | 1 | Yes | 4 | ✅ PASS |

#### Examples

**CLI:**
```bash
paperpilot-cli extract --input /app/tests/e2e_fixtures/multi_page.pdf --pages 1,3 --output /app/tests/e2e_fixtures/out/tri_e2e/extracted.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_pages",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/multi_page.pdf",
      "pages": "1,3",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_extracted.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_pages \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/multi_page.pdf", "pages": "1,3", "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_extracted.pdf"}'
```

---

### Delete Pages (`pdf_delete_pages`)

**Synopsis:** Removes specific pages from a PDF document.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 1409 | 1292 | -117 | 1 | Yes | 13 | ✅ PASS |
| MCP | 1409 | 1292 | -117 | 1 | Yes | 12 | ✅ PASS |
| API | 1409 | 1292 | -117 | 1 | Yes | 4 | ✅ PASS |

#### Examples

**CLI:**
```bash
paperpilot-cli delete --input /app/tests/e2e_fixtures/multi_page.pdf --pages 2 --output /app/tests/e2e_fixtures/out/tri_e2e/deleted.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_delete_pages",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/multi_page.pdf",
      "pages": "2",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_deleted.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_delete_pages \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/multi_page.pdf", "pages": "2", "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_deleted.pdf"}'
```

---

### Reorder Pages (`pdf_reorder_pages`)

**Synopsis:** Reorders the pages of a PDF document according to a specified sequence.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 1409 | 0 | -1409 | 0 | No | 13 | ❌ FAIL |
| MCP | 1409 | 0 | -1409 | 0 | No | 11 | ❌ FAIL |
| API | 1409 | 0 | -1409 | 0 | No | 4 | ❌ FAIL |

**CLI Error:**
```

```

**MCP Error:**
```
Error executing MCP tool: ErrorData { code: ErrorCode(-32602), message: "Parse error: New order length (3) must match document page count (5)", data: None }

```

**API Response:**
```
{"error":"Parse error: New order length (3) must match document page count (5)","success":false}
```

#### Examples

**CLI:**
```bash
paperpilot-cli reorder --input /app/tests/e2e_fixtures/multi_page.pdf --order 3,2,1 --output /app/tests/e2e_fixtures/out/tri_e2e/reordered.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_reorder_pages",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/multi_page.pdf",
      "order": "3,2,1",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_reordered.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_reorder_pages \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/multi_page.pdf", "order": "3,2,1", "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_reordered.pdf"}'
```

---

### Burst (`pdf_burst`)

**Synopsis:** Bursts a PDF document into individual 1-page PDF files.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 1409 | 0 | -1409 | 0 | No | 10 | ❌ FAIL |
| MCP | 1409 | 0 | -1409 | 0 | No | 12 | ❌ FAIL |
| API | 1409 | 0 | -1409 | 0 | No | 4 | ❌ FAIL |

**CLI Error:**
```
error: unexpected argument '--output-dir' found

  tip: a similar argument exists: '--output'

Usage: paperpilot-cli burst --input <INPUT> --output <OUTPUT>

For more information, try '--help'.

```

**MCP Error:**
```
Error executing MCP tool: ErrorData { code: ErrorCode(-32602), message: "I/O error: No such file or directory (os error 2)", data: None }

```

**API Response:**
```
{"error":"I/O error: No such file or directory (os error 2)","success":false}
```

#### Examples

**CLI:**
```bash
paperpilot-cli burst --input /app/tests/e2e_fixtures/multi_page.pdf --output-dir /app/tests/e2e_fixtures/out/tri_e2e/burst
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_burst",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/multi_page.pdf",
      "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_burst"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_burst \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/multi_page.pdf", "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/api_burst"}'
```

---

### Crop (`pdf_crop`)

**Synopsis:** Crops all pages of a PDF document to the specified dimensions.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 539 | 0 | -539 | 0 | No | 10 | ❌ FAIL |
| MCP | 539 | 0 | -539 | 0 | No | 8 | ❌ FAIL |
| API | 539 | 0 | -539 | 0 | No | 2 | ❌ FAIL |

**CLI Error:**
```
error: unexpected argument '--x' found

Usage: paperpilot-cli crop --input <INPUT> --pages <PAGES> --rect <RECT> --output <OUTPUT>

For more information, try '--help'.

```

**MCP Error:**
```
Error executing MCP tool: ErrorData { code: ErrorCode(-32602), message: "Missing or invalid 'box' parameter", data: None }

```

**API Response:**
```
{"error":"Missing or invalid 'box' parameter","success":false}
```

#### Examples

**CLI:**
```bash
paperpilot-cli crop --input /app/tests/e2e_fixtures/single_page.pdf --x 10 --y 10 --width 500 --height 700 --output /app/tests/e2e_fixtures/out/tri_e2e/cropped.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_crop",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/single_page.pdf",
      "x": 10.0,
      "y": 10.0,
      "width": 500.0,
      "height": 700.0,
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_cropped.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_crop \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/single_page.pdf", "x": 10.0, "y": 10.0, "width": 500.0, "height": 700.0, "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_cropped.pdf"}'
```

---

### Remove Blank Pages (`pdf_remove_blank`)

**Synopsis:** Removes empty or blank pages from a PDF document.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 1409 | 0 | -1409 | 0 | No | 10 | ❌ FAIL |
| MCP | 1409 | 1414 | 5 | 1 | Yes | 12 | ✅ PASS |
| API | 1409 | 1414 | 5 | 1 | Yes | 5 | ✅ PASS |

**CLI Error:**
```
error: unrecognized subcommand 'remove-blank'

Usage: paperpilot-cli [OPTIONS] <COMMAND>

For more information, try '--help'.

```

#### Examples

**CLI:**
```bash
paperpilot-cli remove-blank --input /app/tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/nonblank.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_blank",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_nonblank.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_remove_blank \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_nonblank.pdf"}'
```

---

### Page Numbers (`pdf_page_numbers`)

**Synopsis:** Adds page numbers to a PDF document.

#### Metrics Table

| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |
|---|---|---|---|---|---|---|---|
| CLI | 1409 | 0 | -1409 | 0 | No | 10 | ❌ FAIL |
| MCP | 1409 | 2116 | 707 | 1 | Yes | 12 | ✅ PASS |
| API | 1409 | 2116 | 707 | 1 | Yes | 5 | ✅ PASS |

**CLI Error:**
```
error: unrecognized subcommand 'page-numbers'

Usage: paperpilot-cli [OPTIONS] <COMMAND>

For more information, try '--help'.

```

#### Examples

**CLI:**
```bash
paperpilot-cli page-numbers --input /app/tests/e2e_fixtures/multi_page.pdf --position bottom-center --output /app/tests/e2e_fixtures/out/tri_e2e/numbered.pdf
```

**MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_page_numbers",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/multi_page.pdf",
      "position": "bottom-center",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/mcp_numbered.pdf"
    }
  }
}
```

**API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_page_numbers \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/multi_page.pdf", "position": "bottom-center", "output": "/app/tests/e2e_fixtures/out/tri_e2e/api_numbered.pdf"}'
```

---
