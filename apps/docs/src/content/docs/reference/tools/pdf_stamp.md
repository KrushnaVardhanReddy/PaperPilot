---
title: "pdf_stamp"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_stamp."
sidebar:
  order: 61
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ PDF (`.pdf`)

### What it does
**Places coordinate-exact images, logos, badges, and signatures onto PDF pages with configurable transparency.**  
Injects image XObjects (PNG or JPEG) into PDF page content streams and `/Resources` dictionaries at precise point offsets `(x, y)` with custom bounding dimensions `(width, height)` and opacity levels.

---

### CLI
Stamp an image onto a PDF document using `paperpilot pdf-stamp`:

```bash
paperpilot pdf-stamp \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/stamped.pdf \
  --image tests/e2e_fixtures/img1.png \
  --x 50 \
  --y 50 \
  --width 120 \
  --height 80 \
  --opacity 0.85 \
  --pages "all" \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the source PDF document.
- `--output <OUTPUT>` *(string, required)*: Destination file path for the stamped PDF document.
- `--image <IMAGE>` *(string, optional)*: File path to the stamp image (`.png` or `.jpg` / `.jpeg`).
- `--x <X>` *(float, optional, default: `50.0`)*: Horizontal offset in points from the bottom-left corner origin.
- `--y <Y>` *(float, optional, default: `50.0`)*: Vertical offset in points from the bottom-left corner origin.
- `--width <WIDTH>` *(float, optional, default: `150.0`)*: Rendered image width in points.
- `--height <HEIGHT>` *(float, optional, default: `50.0`)*: Rendered image height in points.
- `--opacity <OPACITY>` *(float, optional, default: `1.0`)*: Transparency alpha factor from `0.0` (fully transparent) to `1.0` (fully opaque).
- `--pages <PAGES>` *(string, optional, default: `"all"`)*: Target page selector: `"all"`, `"last"`, or comma-separated 1-based page indices such as `"1,3,5"`.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke `pdf_stamp`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_stamp",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/stamped.pdf",
      "image_path": "tests/e2e_fixtures/img1.png",
      "x": 72.0,
      "y": 72.0,
      "width": 144.0,
      "height": 72.0,
      "opacity": 0.9,
      "pages": "1"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Path where the output PDF file will be saved.
- `image_path` or `image` *(string, optional)*: Path to the image file on disk.
- `image_data` *(string, optional)*: Inline Base64-encoded image binary (used when image file is not on disk).
- `x` *(number, optional, default: `50.0`)*: X coordinate in points from bottom-left origin.
- `y` *(number, optional, default: `50.0`)*: Y coordinate in points from bottom-left origin.
- `width` *(number, optional, default: `150.0`)*: Rendered width in points.
- `height` *(number, optional, default: `50.0`)*: Rendered height in points.
- `opacity` *(number, optional, default: `1.0`)*: Opacity factor from `0.0` to `1.0`.
- `pages` or `page` *(string or integer, optional, default: `"all"`)*: Target pages (`"all"`, `"last"`, or `"1,2"`).

---

### REST API
Execute image stamping via PaperPilot Gateway:

#### 1. Direct Endpoint (`/api/v1/pdf/stamp`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/stamp \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/stamped_api.pdf",
    "image_path": "tests/e2e_fixtures/img1.png",
    "x": 100.0,
    "y": 100.0,
    "width": 150.0,
    "height": 50.0,
    "opacity": 0.8,
    "pages": "1"
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_stamp`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_stamp \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/stamped_canonical.pdf",
    "image_path": "tests/e2e_fixtures/img1.png",
    "x": 50.0,
    "y": 50.0,
    "opacity": 1.0
  }'
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const stampOptions = JSON.stringify({
  image_data: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==",
  x: 72.0,
  y: 72.0,
  width: 100.0,
  height: 50.0,
  opacity: 0.85,
  pages: "all"
});

// Returns Uint8Array of stamped PDF bytes
const stampedPdfBytes = WasmPdfEngine.pdf_stamp(inputPdfBytes, stampOptions);
```

---

### Known Limitations & Troubleshooting
- **Coordinate Origin**: Coordinates start at the bottom-left of each page $(0, 0)$. Ensure your $Y$ coordinate accounts for page height (e.g. standard A4 height is $\sim 842$ pt, US Letter is $792$ pt).
- **Supported Formats**: Supports PNG and JPEG images. For vector SVG branding, convert the SVG to high-resolution PNG or use `pdf_overlay_underlay`.
- **In-Memory / Base64 Support**: In headless WASM or isolated microservices where the filesystem is absent, pass Base64 raw image bytes via `image_data` instead of `image_path`.
