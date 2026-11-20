---
title: "img_convert"
description: "Reference, CLI, MCP, and REST API commands for img_convert."
sidebar:
  order: 70
---

**Status**: ✅ Verified  |  **Added in**: `v0.8.0+`  |  **Format**: Image (`PNG`, `JPEG`, `WebP`, `TIFF`, `AVIF`) Transcoding

### What it does
**Performs pure-Rust transcoding between image formats without system-level image libraries.**  
Converts images seamlessly between PNG, JPEG, WebP, TIFF, and AVIF formats with configurable compression quality. Operates with zero external dependencies (no libpng, libjpeg, or ImageMagick required) for cross-platform and sandboxed environments.

---

### CLI
Convert image formats via the PaperPilot CLI `img-convert` subcommand:

```bash
paperpilot img-convert \
  --input tests/e2e_fixtures/real/document.png \
  --output tests/e2e_fixtures/out/penta_e2e/document.webp \
  --format webp \
  --quality 90 \
  --json
```

#### Inferred Format from Output Extension
```bash
paperpilot img-convert \
  --input tests/e2e_fixtures/real/scan.tiff \
  --output tests/e2e_fixtures/out/penta_e2e/scan.jpeg \
  --quality 85 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input image file.
- `--output` *(string, required)*: Destination file path for the converted image.
- `--format` *(string, optional)*: Explicit target format (`png`, `jpeg`/`jpg`, `webp`, `tiff`/`tif`, `avif`). If omitted, inferred automatically from the `--output` file extension.
- `--quality` *(integer, optional, default: `90`)*: Compression quality factor (`1`–`100`) for lossy formats such as JPEG and WebP.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `img_convert`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_convert",
    "arguments": {
      "input": "tests/e2e_fixtures/real/document.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.webp",
      "format": "webp",
      "quality": 90
    }
  },
  "id": 1
}
```

#### MCP Parameters
- `input` *(string, required)*: Path to the input image file.
- `output` *(string, required)*: Destination file path.
- `format` *(string, optional)*: Target image format (`png`, `jpeg`, `webp`, `tiff`, `avif`).
- `quality` *(number, optional, default: `90`)*: Quality level for lossy encoding.

---

### REST API
Invoke the primary REST API endpoint `/api/v1/image/convert` (or legacy `/api/v1/pdf/img-convert` for backward compatibility):

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/image/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/document.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.webp",
    "format": "webp",
    "quality": 90
  }'
```

> **Note**: `/api/v1/pdf/img-convert` remains fully supported as a backward-compatible alias.

#### Response Example
```json
{
  "success": true,
  "request_id": "62378a67-5555-6666-7777-888899990000",
  "message": "Convert Image Format executed successfully",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Convert Image Format executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.webp\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.webp\"}}"
      }
    ]
  }
}
```

---

### Real-World Use Cases
- **Web Modernization:** Transcode legacy heavyweight TIFF and uncompressed BMP scans into modern WebP/AVIF formats for 70%+ bandwidth savings on document portals.
- **Cross-Platform Compatibility:** Convert unstandardized camera image formats into universally readable PNG and JPEG assets for PDF generation and embedding.
- **Embedded & Serverless Workflows:** Execute image format conversions inside lightweight containers or WASM runtimes without needing bulky ImageMagick or native C library bindings.
