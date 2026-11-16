---
title: "pdf_rotate"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_rotate."
sidebar:
  order: 42
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Rotates PDF pages by 90, 180, or 270 degrees clockwise.**  
Modifies page `/Rotate` dictionary attributes in place to correct upside-down scans or landscape documents without re-encoding page content or degrading image clarity.

---

### CLI
Rotate document pages from the command line:

#### 1. Rotate Specific Pages
```bash
paperpilot rotate \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --degrees 90 \
  --pages 1 \
  --output tests/e2e_fixtures/out/batch3_test/rotated.pdf \
  --json
```

#### 2. Rotate Entire Document
Rotate all pages by 180 degrees:
```bash
paperpilot rotate \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --degrees 180 \
  --output tests/e2e_fixtures/out/batch3_test/rotated_all.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--degrees` *(integer, required)*: Rotation angle in degrees (must be a positive multiple of 90: `90`, `180`, `270`).
- `--output` *(string, required)*: Path to write the rotated PDF.
- `--pages` *(string, optional)*: Specific 1-based page indices or ranges (e.g. `"1"`, `"1,3"`, `"1-3"`). When omitted, all pages are rotated.
- `--json` *(flag, optional)*: Emit structured execution status in JSON format.

---

### MCP JSON-RPC
Invoke `pdf_rotate` through Model Context Protocol (MCP) clients:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_rotate",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "angle": 90,
      "pages": "1",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_rotated.pdf"
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
        "text": "{\"success\":true,\"message\":\"Pages rotated successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch3_test/mcp_rotated.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `angle` *(number, required)*: The angle to rotate by, typically `90`, `180`, or `270`.
- `pages` *(string, required)*: The pages to rotate, either `"all"` or specific pages like `"1,3"`.
- `output` *(string, required)*: File path for the rotated output.

---

### REST API
Execute page rotation via PaperPilot Gateway endpoints:

#### Dedicated Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_rotate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "angle": 90,
    "pages": "1",
    "output": "tests/e2e_fixtures/out/batch3_test/api_rotated.pdf"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_rotate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "degrees": 90,
    "pages": "all",
    "output": "tests/e2e_fixtures/out/batch3_test/api_rotated.pdf"
  }'
```

---

### WASM (Web & Edge)
Rotate PDF pages in memory with `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

// Rotate pages in memory
const rotatedBytes: Uint8Array = WasmPdfEngine.rotate(
  inputBytes, // Uint8Array of input PDF
  90,         // Angle: 90 | 180 | 270
  "all"       // Target pages: "all" | "1" | "1,2"
);
```

---

### Known Limitations & Troubleshooting
- **Angle Constraints**: Rotation angles must be multiples of 90 degrees (`90`, `180`, `270`). Arbitrary geometric skew angles (e.g. 15° deskew) cannot be set via page dictionary `/Rotate` flags; use `pdf_auto_rotate` for heuristic deskew and upright orientation.
- **Existing Page Rotation Accumulation**: The rotation angle specified is applied as an absolute clockwise transformation.
