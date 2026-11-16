---
title: "pdf_merge"
description: "Reference, CLI, MCP, and REST API commands for pdf_merge."
sidebar:
  order: 31
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Combines multiple separate PDF documents into a single continuous file.**  
Appends pages in order, renumbers indirect objects to resolve ID collisions, consolidates document catalogs, and preserves font resources and page hierarchies into one master document.

---

### CLI
Combine two or more PDF files via the PaperPilot CLI:

```bash
paperpilot merge \
  --input tests/e2e_fixtures/real/merge_a.pdf tests/e2e_fixtures/real/merge_b.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/merge_cli_simple.pdf \
  --json
```

#### CLI Parameters
- `--input` *(array of strings, required)*: Space-separated paths to 2 or more PDF documents to merge in order.
- `--output` *(string, required)*: Destination file path for the consolidated output PDF.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_merge`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_merge",
    "arguments": {
      "inputs": [
        "tests/e2e_fixtures/real/merge_a.pdf",
        "tests/e2e_fixtures/real/merge_b.pdf"
      ],
      "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `inputs` *(array of strings, required)*: Array of source file paths to merge in sequential order (also accepts comma-separated string via `input` fallback).
- `output` *(string, required)*: Destination file path for the merged output document.

---

### REST API
Execute merging operations via PaperPilot Gateway:

#### 1. Dedicated Merge Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/merge \
  -H 'Content-Type: application/json' \
  -d '{
    "inputs": [
      "tests/e2e_fixtures/real/merge_a.pdf",
      "tests/e2e_fixtures/real/merge_b.pdf"
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"
  }'
```

#### 2. Exact Tool Route (`pdf_merge`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_merge \
  -H 'Content-Type: application/json' \
  -d '{
    "inputs": [
      "tests/e2e_fixtures/real/merge_a.pdf",
      "tests/e2e_fixtures/real/merge_b.pdf"
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "7a17fa26-0c06-4e33-8704-77fe3799bb75",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"PDFs merged successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf\",\"data\":null}"
      }
    ]
  }
}
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

// Accepts an Array of Uint8Array buffers and returns a single merged Uint8Array
const mergedPdfBytes = WasmPdfEngine.merge([pdfBufferA, pdfBufferB]);
```

---

### Known Limitations & Troubleshooting
- **Minimum Input Requirement:** At least 2 PDF documents must be supplied to `--input` / `inputs`. Passing an empty list or a single document will fail validation.
- **Form Flattening Recommendation:** When merging multiple PDFs containing interactive AcroForms with overlapping field names, consider running `pdf_flatten` beforehand to avoid field key conflicts across documents.
- **Proxy Environments:** In environments where corporate HTTP proxies are enabled, use `--noproxy '*'` or configure `NO_PROXY=127.0.0.1` when executing curl requests against the gateway.
