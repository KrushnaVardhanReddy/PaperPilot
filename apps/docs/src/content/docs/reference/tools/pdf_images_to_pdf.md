---
title: "pdf_images_to_pdf"
description: "Reference, CLI, MCP, and REST API commands for pdf_images_to_pdf."
sidebar:
  order: 29
---

**Status**: ✅ Verified  |  **Format**: Images (PNG, JPG/JPEG, WebP) to PDF (`.pdf`)

### What it does
**Combines one or more image files (PNG, JPG, WebP) into a single PDF document.**  
Each image is placed onto its own page with original image dimensions preserved at standard print resolution (72 DPI baseline). Indirect object streams are created cleanly and embedded into a valid PDF document structure. Ideal for photo albums, scanned receipts, document compilations, or creating PDF presentations from graphic slides.

---

### CLI
Convert multiple images into a PDF document via the PaperPilot CLI subcommand `images-to-pdf`:

```bash
paperpilot images-to-pdf \
  --images tests/e2e_fixtures/img1.png tests/e2e_fixtures/img2.png \
  --output tests/e2e_fixtures/out/batch14_test/cli_images_to_pdf.pdf \
  --json
```

#### CLI Parameters
- `--images` *(array of strings, required)*: Space-separated paths to one or more image files (PNG, JPG/JPEG, WebP).
- `--output` *(string, required)*: Destination file path for the generated PDF file.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_images_to_pdf` (or its alias `img_to_pdf`):

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_images_to_pdf",
    "arguments": {
      "inputs": [
        "tests/e2e_fixtures/img1.png",
        "tests/e2e_fixtures/img2.png"
      ],
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_images_to_pdf.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `inputs` *(array of strings, required)*: List of image file paths to combine into PDF pages (also accepts a single string fallback via `input`).
- `output` *(string, required)*: Target file path where the generated PDF will be written.

---

### REST API
Execute image compilation through PaperPilot Gateway:

#### 1. Generic Tool Route (`/api/v1/pdf/tools/{tool_name}`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_images_to_pdf \
  -H 'Content-Type: application/json' \
  -d '{
    "inputs": [
      "tests/e2e_fixtures/img1.png",
      "tests/e2e_fixtures/img2.png"
    ],
    "output": "tests/e2e_fixtures/out/batch14_test/rest_images_to_pdf.pdf"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "3cafd2dd-bc99-4b8f-8a8b-bd475e81b6ef",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Images converted to PDF successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch14_test/rest_images_to_pdf.pdf\",\"data\":null}"
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

// Accepts an Array of Uint8Array image buffers (PNG/JPEG) and returns the generated PDF bytes
const imageBuffers = [imgBuffer1, imgBuffer2];
const pdfUint8Array = WasmPdfEngine.images_to_pdf(imageBuffers);
```

---

### Known Limitations & Troubleshooting
- **Supported Formats:** Currently supports PNG, JPEG/JPG, and WebP raster images. Unrecognized or corrupt image data will return an image decoding error.
- **Empty Image List:** At least one image path or buffer must be provided. Passing an empty array results in an error.
- **Orientation & Dimensions:** Page dimensions match each source image's pixel dimensions directly mapped to points (72 DPI). If uniform page sizing (such as standard US Letter or A4) is required, follow up with `pdf_scale_pages`.
