---
title: "pdf_auto_rotate"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_auto_rotate."
sidebar:
  order: 52
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Heuristically detects page aspect ratios and orientations to automatically upright pages.**  
Analyzes page dimensions (`MediaBox`) and existing `/Rotate` dictionary properties, rotating landscape-oriented pages to portrait (or vice versa based on target orientation) by 90-degree increments to ensure consistent reading orientation without re-encoding content streams.

---

### CLI
Automatically upright or rotate pages to target orientation from the command line:

#### 1. Auto-Rotate to Upright Portrait Orientation
```bash
paperpilot pdf-auto-rotate \
  --input tests/e2e_fixtures/rotated.pdf \
  --output tests/e2e_fixtures/out/batch18/auto_rotated.pdf \
  --json
```

#### 2. Standard Upright Invocation
```bash
paperpilot pdf-auto-rotate \
  --input input.pdf \
  --output output_upright.pdf
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--output` *(string, required)*: Path to write the re-oriented PDF document.
- `--json` *(flag, optional)*: Emit structured JSON status report (`{"success":true,"operation":"PdfAutoRotate","error":null}`).
- `--log-format` *(string, optional)*: Output format for logs (`text`, `json`, `compact`, `pretty`).
- `--log-level` *(string, optional)*: Log level verbosity (`trace`, `debug`, `info`, `warn`, `error`).
- `--webhook` *(string, optional)*: Webhook URL to deliver execution results.

---

### MCP JSON-RPC
Invoke `pdf_auto_rotate` through Model Context Protocol (MCP) clients:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_auto_rotate",
    "arguments": {
      "input": "tests/e2e_fixtures/rotated.pdf",
      "output": "tests/e2e_fixtures/out/batch18/auto_rotated_mcp.pdf"
    }
  },
  "id": 1
}
```

#### MCP Output Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Auto Upright Orientation executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/batch18/auto_rotated_mcp.pdf\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/batch18/auto_rotated_mcp.pdf\"}}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Path where the oriented PDF will be written.

---

### REST API
Execute auto-orientation through PaperPilot Gateway endpoints:

#### Dedicated Endpoint (`/api/v1/pdf/auto-rotate` or `/api/v1/pdf/pdf_auto_rotate`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/auto-rotate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/rotated.pdf",
    "output": "tests/e2e_fixtures/out/batch18/auto_rotated_api.pdf"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_auto_rotate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/rotated.pdf",
    "output": "tests/e2e_fixtures/out/batch18/auto_rotated_tool.pdf"
  }'
```

---

### WASM (Web & Edge)
Run auto-orientation in client-side or browser runtimes via `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

// Auto-rotate pages in-memory to default upright portrait orientation
const options = JSON.stringify({
  target_orientation: "portrait", // "portrait" | "landscape" | "auto"
  pages: null                     // null for all pages, or [1, 2]
});

const rotatedBytes: Uint8Array = WasmPdfEngine.pdf_auto_rotate(
  inputBytes,
  options
);
```

---

### Known Limitations & Troubleshooting
- **Text Baseline vs Physical Aspect Ratio**: The orientation detection primarily checks the effective dimensions (`width` vs `height`) derived from page `MediaBox` and existing `/Rotate` degrees. Documents where landscape pages contain rotated landscape text might require fine-grained page inspection or manual rotation with `pdf_rotate`.
- **Target Orientation Options**: While the underlying library supports `target_orientation` (`"portrait"`, `"landscape"`, `"auto"`) and `pages` selection, the default CLI and MCP interfaces automatically standardize to upright portrait (`"portrait"`). Custom orientations are directly accessible via the WASM engine and core library API.
