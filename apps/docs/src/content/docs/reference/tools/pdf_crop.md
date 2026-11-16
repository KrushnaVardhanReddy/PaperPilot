---
title: "pdf_crop"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_crop."
sidebar:
  order: 18
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Trims page margins or crops documents to a custom rectangular bounding box.**  
Adjusts PDF `/CropBox` and `/MediaBox` dictionary vectors to eliminate scanner margins, cut registration marks, or isolate target regions across all or designated pages.

---

### CLI
Crop document boundaries from the command line:

#### 1. Crop using `--rect` String (`left,bottom,right,top`)
```bash
paperpilot crop \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --rect 10,10,200,200 \
  --output tests/e2e_fixtures/out/batch3_test/cropped.pdf \
  --json
```

#### 2. Crop using Individual Dimension Flags (`--x`, `--y`, `--width`, `--height`)
```bash
paperpilot crop \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --x 20 \
  --y 20 \
  --width 400 \
  --height 500 \
  --output tests/e2e_fixtures/out/batch3_test/cropped_xy.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--output` *(string, required)*: Path to write the cropped PDF.
- `--rect` *(string, optional)*: Bounding box in `left,bottom,right,top` format (e.g. `"10,10,200,200"`).
- `--x` *(number, optional)*: Lower-left X coordinate in PDF points.
- `--y` *(number, optional)*: Lower-left Y coordinate in PDF points.
- `--width` *(number, optional)*: Crop box width in PDF points.
- `--height` *(number, optional)*: Crop box height in PDF points.
- `--pages` *(string, optional)*: Specific pages to crop (e.g. `"1"`, `"1-3"`).
- `--json` *(flag, optional)*: Emit structured execution status in JSON format.

---

### MCP JSON-RPC
Invoke `pdf_crop` through Model Context Protocol (MCP) clients:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_crop",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "box": "10,10,200,200",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_cropped.pdf"
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
        "text": "{\"success\":true,\"message\":\"Pages cropped successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch3_test/mcp_cropped.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `box` *(string, required)*: The crop box in format `'left,bottom,right,top'` (e.g. `'0,0,595,842'`).
- `output` *(string, required)*: File path for the cropped output.

---

### REST API
Execute page cropping via PaperPilot Gateway endpoints:

#### Dedicated Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_crop \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "box": "10,10,200,200",
    "output": "tests/e2e_fixtures/out/batch3_test/api_cropped.pdf"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_crop \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "rect": "10,10,200,200",
    "output": "tests/e2e_fixtures/out/batch3_test/api_cropped.pdf"
  }'
```

---

### WASM (Web & Edge)
Crop PDF documents in memory with `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

// Crop in-memory PDF buffer with explicit coordinates
const croppedBytes: Uint8Array = WasmPdfEngine.crop(
  inputBytes, // Uint8Array of input PDF
  10.0,       // Left
  10.0,       // Bottom
  200.0,      // Right
  200.0       // Top
);
```

---

### Known Limitations & Troubleshooting
- **Coordinate Conventions**: PDF points adhere to a Cartesian coordinate system with `(0, 0)` located at the **bottom-left** corner of the sheet (1 point = 1/72 inch). `left` must be strictly less than `right`, and `bottom` must be strictly less than `top`.
- **Destructive vs Viewport Cropping**: Cropping updates `/CropBox` and `/MediaBox` boundaries, instructing PDF viewers and printers to only display content within that rectangle. Underlying content streams outside the crop box remain present in the file unless flattened or sanitized. To completely strip clipped objects, use `pdf_sanitize`.
