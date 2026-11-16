---
title: "pdf_render"
description: "Reference, CLI, MCP, and REST API commands for pdf_render."
sidebar:
  order: 39
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) to Image (`.png`, `.jpeg`, `.webp`)

### What it does
**Renders PDF document pages into high-resolution raster images.**  
Rasterizes vector paths, text glyphs, font shapes, and embedded graphics at a target DPI resolution for web previews, thumbnail generation, or image processing pipelines.

---

### CLI
Render a target page from a PDF file using the `render` subcommand:

```bash
paperpilot render \
  --input tests/e2e_fixtures/single_page.pdf \
  --output tests/e2e_fixtures/out/batch14_test/cli_render.png \
  --page 1 \
  --dpi 150 \
  --format png \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file.
- `--output` *(string, required)*: Destination file path for the raster image.
- `--page` *(number, optional)*: 1-based page number to render. Defaults to `1`.
- `--dpi` *(number, optional)*: Resolution in dots per inch. Defaults to `150`.
- `--format` *(string, optional)*: Output image format (`png`, `jpeg`/`jpg`, `webp`). If omitted, inferred from `--output` file extension or defaults to `png`.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_render`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_render",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_render.png",
      "page": 1,
      "dpi": 150,
      "format": "png"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Destination path for the rendered image file.
- `page` *(number, optional)*: 1-based page number to render (default: `1`).
- `dpi` *(number, optional)*: Render resolution DPI (default: `150`).
- `format` *(string, optional)*: Target image format: `png`, `jpeg`/`jpg`, `webp` (default: inferred from output path or `png`).

---

### REST API
Render PDF pages using PaperPilot Gateway:

#### 1. Dedicated Tool Route (`/api/v1/pdf/pdf_render`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_render \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/batch14_test/rest_render.png",
    "page": 1,
    "dpi": 150
  }'
```

#### 2. Streaming Binary Endpoint (`/api/v1/pdf/render-page`)
When calling `/api/v1/pdf/render-page` without an `output` field, the gateway directly streams raw image bytes (`image/png`):

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/render-page \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "page": 1
  }' --output rendered_stream.png
```

#### Response Example (Dedicated Route)
```json
{
  "success": true,
  "request_id": "076ddebe-8ce3-48cb-bc6d-e3cd6702cb65",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"PDF rendered successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch14_test/rest_render.png\",\"data\":null}"
      }
    ]
  }
}
```

---

### WebAssembly (WASM)
In browser or worker contexts using `paperpilot-wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

// Render 0-based page index with scaling factor (1.0 = 72 DPI, 2.0 = 144 DPI)
// Returns Uint8Array of PNG image bytes
const pageIndex = 0;
const scale = 2.0;
const pngBytes = WasmPdfEngine.render_page(pdfUint8Array, pageIndex, scale);
```

---

### Known Limitations & Troubleshooting
- **Page Numbering Conventions:** CLI, MCP, and REST API routes use **1-based** page numbers (`--page 1` is page 1). Attempting to request page `0` or a page number exceeding total pages returns an out-of-bounds error. The WASM API uses **0-based** page index (`0` is page 1).
- **Backend Rendering Engine:** Requires native raster rendering support (`pdfium` library). If the dynamic library is not pre-installed or located in system search paths, configure `PDFIUM_PATH` in your environment.
