---
title: "pdf_watermark"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_watermark."
sidebar:
  order: 52
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Stamps semi-transparent text watermarks across PDF pages.**  
Applies diagonal, centered, or custom angled branding (e.g. `CONFIDENTIAL`, `DRAFT`, `COPYRIGHT`) with customizable font sizes, opacity, rotation angle, and primary color palettes across all document pages.

---

### CLI
Stamp text watermarks across PDF documents from the command line:

#### 1. Standard Angle & Opacity
```bash
paperpilot watermark \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --text "CONFIDENTIAL" \
  --angle 45 \
  --opacity 0.3 \
  --output tests/e2e_fixtures/out/batch3_test/watermarked.pdf \
  --json
```

#### 2. Custom Color & Font Size
```bash
paperpilot watermark \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --text "INTERNAL REVIEW ONLY" \
  --angle 30 \
  --opacity 0.25 \
  --color red \
  --font-size 48 \
  --output tests/e2e_fixtures/out/batch3_test/watermark_red.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--text` *(string, required)*: Text string to stamp as watermark.
- `--output` *(string, required)*: Path to write the watermarked PDF.
- `--angle` *(number, optional, default: `45`)*: Counterclockwise rotation angle in degrees.
- `--opacity` *(number, optional, default: `0.2`)*: Opacity factor from `0.0` (fully transparent) to `1.0` (fully opaque).
- `--color` *(string, optional, default: `"gray"`)*: Named color palette (`gray`, `red`, `blue`, `green`, `black`, `white`).
- `--font-size` *(number, optional)*: Explicit font size in points. When omitted, dynamically scales to fit page geometry.
- `--json` *(flag, optional)*: Emit structured execution status in JSON format.

---

### MCP JSON-RPC
Invoke `pdf_watermark` through Model Context Protocol (MCP) clients:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_watermark",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "text": "CONFIDENTIAL",
      "angle": 45,
      "opacity": 0.3,
      "color": "gray",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_watermarked.pdf"
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
        "text": "{\"success\":true,\"message\":\"Watermark applied successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch3_test/mcp_watermarked.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `text` *(string, required)*: The text of the watermark.
- `output` *(string, required)*: The output PDF file path.
- `angle` *(number, optional, default: `45`)*: Angle of the watermark.
- `opacity` *(number, optional, default: `0.2`)*: Opacity factor between `0.0` and `1.0`.
- `color` *(string, optional)*: Named color string (`"gray"`, `"red"`, `"blue"`, `"green"`, `"black"`, `"white"`).
- `font_size` *(number, optional)*: Custom font size in points.

---

### REST API
Execute watermarking via PaperPilot Gateway endpoints:

#### Dedicated Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/watermark \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "text": "CONFIDENTIAL",
    "angle": 45.0,
    "opacity": 0.3,
    "output": "tests/e2e_fixtures/out/batch3_test/api_watermarked.pdf"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_watermark \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "text": "DO NOT DISTRIBUTE",
    "output": "tests/e2e_fixtures/out/batch3_test/api_watermarked.pdf"
  }'
```

---

### WASM (Web & Edge)
Apply watermarks in memory with `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

// Stamp watermark onto in-memory PDF buffer
const watermarkedBytes: Uint8Array = WasmPdfEngine.watermark(
  inputBytes,      // Uint8Array of input PDF
  "CONFIDENTIAL",  // Watermark text
  45.0,            // Angle (optional, default: 45)
  0.25             // Opacity (optional, default: 0.2)
);
```

---

### Known Limitations & Troubleshooting
- **Standard Base14 Fonts**: Text watermarks utilize built-in standard Type1 Helvetica fonts. Non-Latin scripts or emojis are not supported directly by the base watermark generator; for localized logos or graphical stamps, use `pdf_stamp` or `pdf_overlay_underlay`.
- **Existing Page Stacking Order**: Watermark layers are appended to page content streams with transparent alpha state dicts. Documents containing opaque vector fills or full-page background images may visually obscure watermarks placed behind them.
