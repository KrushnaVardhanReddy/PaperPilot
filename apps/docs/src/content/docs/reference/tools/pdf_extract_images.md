---
title: "pdf_extract_images"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_extract_images."
sidebar:
  order: 22
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Image files (`.png` / `.bin` / `.jpg`)

### What it does
**Extracts embedded photos, raster graphics, and figures from inside a PDF file.**  
Dumps embedded image streams directly without lossy re-compression to an output directory, allowing recovery of full-resolution graphics, logos, and illustrations.

---

### CLI
Extract embedded images via the PaperPilot CLI using `extract-images`:

```bash
paperpilot extract-images \
  --input tests/e2e_fixtures/real/image_doc.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/images \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the input PDF document.
- `--output <OUTPUT>` *(string, required)*: Destination directory path where extracted images will be stored.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_extract_images`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_images",
    "arguments": {
      "input": "tests/e2e_fixtures/real/image_doc.pdf",
      "output_dir": "tests/e2e_fixtures/out/penta_e2e/images"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output_dir` *(string, required)*: The directory where images will be saved.

---

### REST API
Extract images via PaperPilot Gateway:

#### 1. Universal Tool Route (`/api/v1/pdf/tools/pdf_extract_images`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_images \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/image_doc.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/api_images"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "69eb4313-1e1e-4794-81e6-d785bf723e64",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Images extracted successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/api_images\",\"data\":null}"
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

// Returns an array of Uint8Array image buffers
const imageBuffers = WasmPdfEngine.extract_images(pdfBytes);
imageBuffers.forEach((buf, idx) => {
  console.log(`Extracted image ${idx + 1}: ${buf.byteLength} bytes`);
});
```

---

### Known Limitations & Troubleshooting
- **Parameter Name Variations**: The CLI uses `--output <OUTPUT>` (which points to an output folder), whereas the MCP schema strictly defines `output_dir` (though the server runtime accepts either `output_dir` or `output`). Always specify `output_dir` for MCP and REST API calls.
- **Vector Graphics vs. Raster Images**: This tool targets embedded XObject image streams (JPEG, PNG, raw pixel buffers). Vector drawings, geometric curves, and page annotations are not raster images and cannot be dumped with `pdf_extract_images` (render the page with `pdf_render` instead).
- **Corrupt Stream Fallback**: When an image stream filter is malformed, PaperPilot logs a warning and attempts a raw deflate extraction fallback before skipping unrecoverable objects.
