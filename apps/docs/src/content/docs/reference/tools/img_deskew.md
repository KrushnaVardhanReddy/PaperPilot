---
title: "img_deskew"
description: "Reference, CLI, MCP, and REST API commands for img_deskew."
sidebar:
  order: 66
---

**Status**: ✅ Verified  |  **Format**: Image (`.png`, `.jpeg`, `.webp`, `.tiff`) to Image

### What it does
**Detects text baseline skew angle and straightens tilted document scans.**  
Applies pure-Rust projection profile analysis to calculate skew angles between -45° and +45°, automatically rotating the canvas and filling exposed borders with a clean background color. Essential for digitizing legacy paperwork, camera photo captures, and preparing scans for high-accuracy OCR.

---

### CLI
Deskew a tilted document image via the PaperPilot CLI `img-deskew` subcommand:

```bash
paperpilot img-deskew \
  --input tests/e2e_fixtures/real/tilted_scan.png \
  --output tests/e2e_fixtures/out/penta_e2e/deskewed.png \
  --json
```

#### Manual Angle Override
```bash
paperpilot img-deskew \
  --input tests/e2e_fixtures/real/tilted_scan.png \
  --output tests/e2e_fixtures/out/penta_e2e/deskewed.png \
  --angle -2.5 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input image file.
- `--output` *(string, required)*: Destination path for the deskewed image.
- `--angle` *(float, optional)*: Explicit manual rotation angle in degrees (positive = counter-clockwise, negative = clockwise). When omitted, automatic detection is enabled.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `img_deskew`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_deskew",
    "arguments": {
      "input": "tests/e2e_fixtures/real/tilted_scan.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/deskewed.png"
    }
  },
  "id": 1
}
```

#### MCP Parameters
- `input` *(string, required)*: Path to the tilted document image.
- `output` *(string, required)*: Destination path for the corrected image.
- `angle` *(number, optional)*: Explicit rotation angle in degrees.
- `auto_detect` *(boolean, optional, default: `true`)*: Whether to auto-detect skew angle using projection profiles.
- `background_color` *(string, optional, default: `"white"`)*: Fill color for corner borders exposed during rotation (`"white"`, `"black"`, `"transparent"`, or hex `#FFFFFF`).

---

### REST API
Invoke the REST API endpoint `/api/v1/pdf/img-deskew`:

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/img-deskew \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/tilted_scan.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/deskewed.png"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "af02eca6-1111-2222-3333-444455556666",
  "message": "Deskew Document Image executed successfully",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Deskew Document Image executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/deskewed.png\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/deskewed.png\"}}"
      }
    ]
  }
}
```

---

### Real-World Use Cases
- **Scan Ingestion Pipeline:** Straighten crooked documents from physical flatbed or feeder scanners prior to running `pdf_ocr` or table extraction.
- **Mobile Camera Scans:** Correct perspective tilt when users photograph invoices, ID cards, and receipts on smartphones.
- **Archive Preservation:** Normalize alignment across thousands of historical records, legal pleadings, and medical paperwork for consistent presentation.
