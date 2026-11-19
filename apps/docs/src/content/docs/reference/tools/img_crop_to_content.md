---
title: "img_crop_to_content"
description: "Reference, CLI, MCP, and REST API commands for img_crop_to_content."
sidebar:
  order: 67
---

**Status**: ✅ Verified  |  **Format**: Image (`.png`, `.jpeg`, `.webp`, `.tiff`) to Image

### What it does
**Detects document boundaries and automatically trims scanner bed and background margins.**  
Samples outer image borders and corner regions to determine background luminance, finds the minimum enclosing bounding box of the document content that deviates by more than a configurable luminance threshold, and crops with an optional protective padding margin.

---

### CLI
Auto-crop background borders from a scanned document image via the PaperPilot CLI `img-crop-to-content` subcommand:

```bash
paperpilot img-crop-to-content \
  --input tests/e2e_fixtures/real/scanner_scan.png \
  --output tests/e2e_fixtures/out/penta_e2e/cropped.png \
  --threshold 0.1 \
  --padding 10 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input image file.
- `--output` *(string, required)*: Destination file path for the cropped output image.
- `--threshold` *(float, optional, default: `0.1`)*: Background difference luminance threshold (`0.0` to `1.0`). Pixels differing from the sampled background by more than this ratio are classified as document content.
- `--padding` *(integer, optional, default: `10`)*: Margin padding in pixels retained around the detected content boundary.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `img_crop_to_content`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_crop_to_content",
    "arguments": {
      "input": "tests/e2e_fixtures/real/scanner_scan.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/cropped.png",
      "threshold": 0.1,
      "padding": 10
    }
  },
  "id": 1
}
```

#### MCP Parameters
- `input` *(string, required)*: Path to the input document image.
- `output` *(string, required)*: Path for the cropped result.
- `threshold` *(number, optional, default: `0.1`)*: Content detection luminance threshold.
- `padding` *(number, optional, default: `10`)*: Padding around bounding box in pixels.

---

### REST API
Invoke the primary REST API endpoint `/api/v1/image/crop-to-content` (or legacy `/api/v1/pdf/img-crop-to-content` for backward compatibility):

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/image/crop-to-content \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/scanner_scan.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/cropped.png",
    "threshold": 0.1,
    "padding": 10
  }'
```

> **Note**: `/api/v1/pdf/img-crop-to-content` remains fully supported as a backward-compatible alias.

#### Response Example
```json
{
  "success": true,
  "request_id": "62378a67-2222-3333-4444-555566667777",
  "message": "Auto-Crop Image to Content executed successfully",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Auto-Crop Image to Content executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/cropped.png\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/cropped.png\"}}"
      }
    ]
  }
}
```

---

### Real-World Use Cases
- **Scanner Bed Margin Removal:** Eliminate black edges, scanner cover borders, and oversized white canvas margins around scanned receipts, checks, and ID cards.
- **Form & Document Standardization:** Produce uniform aspect ratios and trimmed boundaries across batches of mixed-size physical documents.
- **Preparation for Vision Models:** Remove uninformative padding pixels before passing images to multimodal models, saving tokens and improving model focus.
