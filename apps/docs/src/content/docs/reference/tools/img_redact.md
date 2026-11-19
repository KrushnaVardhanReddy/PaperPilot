---
title: "img_redact"
description: "Reference, CLI, MCP, and REST API commands for img_redact."
sidebar:
  order: 68
---

**Status**: ✅ Verified  |  **Added in**: `v0.8.0+`  |  **Format**: Image (`.png`, `.jpeg`, `.webp`, `.tiff`) to Image

### What it does
**Applies permanent, irreversible pixel-level redaction over sensitive image regions.**  
Overwrites pixel data within specified rectangular bounding boxes (`x,y,width,height`) with solid color fills (black, white, red, or hex `#RRGGBB`). Unlike visual overlay layers, the original underlying pixel values are completely destroyed and unrecoverable from metadata or hidden alpha channels.

---

### CLI
Redact sensitive regions on an image via the PaperPilot CLI `img-redact` subcommand:

```bash
paperpilot img-redact \
  --input tests/e2e_fixtures/real/license.png \
  --output tests/e2e_fixtures/out/penta_e2e/redacted.png \
  --rect "120,450,300,50" \
  --color "#000000" \
  --json
```

#### Multiple Redaction Boxes
```bash
paperpilot img-redact \
  --input tests/e2e_fixtures/real/license.png \
  --output tests/e2e_fixtures/out/penta_e2e/redacted.png \
  --rect "50,100,200,40" \
  --rect "320,400,180,60" \
  --color black \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input image file.
- `--output` *(string, required)*: Destination file path for the redacted image.
- `--rect` *(string / array of strings, required)*: Bounding box coordinates formatted as `x,y,w,h` (e.g. `120,450,300,50`). Can be specified multiple times for multiple redaction regions.
- `--color` *(string, optional, default: `"#000000"`)*: Solid fill color. Supports names (`black`, `white`, `red`) and hex codes (`#000000`, `#FFFFFF`).
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `img_redact`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_redact",
    "arguments": {
      "input": "tests/e2e_fixtures/real/license.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/redacted.png",
      "rect": [
        "120,450,300,50",
        "320,400,180,60"
      ],
      "color": "#000000"
    }
  },
  "id": 1
}
```

#### MCP Parameters
- `input` *(string, required)*: Path to the input image.
- `output` *(string, required)*: Path to write redacted output.
- `rect` / `rects` *(array of strings or bounding objects, required)*: Target coordinates to redact (`x,y,w,h` or `{"x": 10, "y": 20, "width": 100, "height": 30}`).
- `color` *(string, optional, default: `"#000000"`)*: Hex or named color for solid fill.

---

### REST API
Invoke the primary REST API endpoint `/api/v1/image/redact` (or legacy `/api/v1/pdf/img-redact` for backward compatibility):

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/image/redact \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/license.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/redacted.png",
    "rect": ["120,450,300,50"],
    "color": "#000000"
  }'
```

> **Note**: `/api/v1/pdf/img-redact` remains fully supported as a backward-compatible alias.

#### Response Example
```json
{
  "success": true,
  "request_id": "62378a67-3333-4444-5555-666677778888",
  "message": "Redact Sensitive Image Regions executed successfully",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Redact Sensitive Image Regions executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/redacted.png\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/redacted.png\"}}"
      }
    ]
  }
}
```

---

### Real-World Use Cases
- **Government ID & PII Masking:** Permanently black out Social Security Numbers, driver license numbers, dates of birth, and home addresses from customer onboarding photo uploads.
- **Financial Compliance:** Sanitize credit card numbers and banking routing codes on payment receipt images before cloud archiving.
- **Healthcare Records (HIPAA):** Obscure patient names, medical record numbers (MRNs), and facial images in medical record photography.
