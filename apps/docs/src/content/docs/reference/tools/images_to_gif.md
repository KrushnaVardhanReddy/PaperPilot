---
title: "images_to_gif"
description: "Reference, CLI, MCP, and REST API commands for images_to_gif."
sidebar:
  order: 65
---

**Status**: ✅ Verified  |  **Format**: Multiple Images (`.png`, `.jpeg`, `.webp`) to Animated GIF (`.gif`)

### What it does
**Combines an ordered sequence of images into a single animated looping GIF.**  
Takes a sequence of raster image files (PNG, JPEG, WebP) and stitches them into a looping GIF file with custom frame delays, frame rates, and consistent width scaling. Designed for time-lapse sequences, stop-motion previews, and slide-deck animations.

---

### CLI
Stitch a series of images into an animated GIF via the PaperPilot CLI `images-to-gif` subcommand:

```bash
paperpilot images-to-gif \
  --input tests/e2e_fixtures/real/img1.png,tests/e2e_fixtures/real/img2.png \
  --output tests/e2e_fixtures/out/penta_e2e/slideshow.gif \
  --fps 4 \
  --width 800 \
  --json
```

#### CLI Parameters
- `--input` *(string array / comma-separated, required)*: Paths to the input image files (e.g. `--input img1.png,img2.png` or `--input img1.png --input img2.png`).
- `--inputs` / `--images` *(string array, optional)*: Alternative alias for input image files.
- `--output` *(string, required)*: Destination file path for the animated `.gif`.
- `--fps` *(integer, optional, default: `4`)*: Frames per second.
- `--delay` *(integer, optional, default: `25`)*: Delay between frames in centiseconds (1/100s).
- `--width` *(integer, optional)*: Target width in pixels to normalize all frames to. Height scales proportionally.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `images_to_gif`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "images_to_gif",
    "arguments": {
      "images": [
        "tests/e2e_fixtures/real/img1.png",
        "tests/e2e_fixtures/real/img2.png"
      ],
      "output": "tests/e2e_fixtures/out/penta_e2e/slideshow.gif",
      "fps": 4,
      "width": 800
    }
  },
  "id": 1
}
```

#### MCP Parameters
- `images` / `inputs` / `input` *(array of strings or comma-separated string, required)*: Paths to image files in display order.
- `output` *(string, required)*: Target path for the generated GIF file.
- `fps` *(number, optional)*: Playback frame rate in frames per second.
- `delay` *(number, optional)*: Frame duration in centiseconds.
- `width` *(number, optional)*: Optional frame width normalization in pixels.

---

### REST API
Invoke the primary REST API endpoint `/api/v1/image/to-gif` (or legacy `/api/v1/pdf/images-to-gif` for backward compatibility):

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/image/to-gif \
  -H 'Content-Type: application/json' \
  -d '{
    "images": [
      "tests/e2e_fixtures/real/img1.png",
      "tests/e2e_fixtures/real/img2.png"
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/slideshow.gif",
    "fps": 4,
    "width": 800
  }'
```

> **Note**: `/api/v1/pdf/images-to-gif` remains fully supported as a backward-compatible alias.

#### Response Example
```json
{
  "success": true,
  "request_id": "d3b5b67d-5678-90ab-cdef-1234567890ab",
  "message": "Combine Images into Animated GIF executed successfully",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Combine Images into Animated GIF executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/slideshow.gif\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/slideshow.gif\"}}"
      }
    ]
  }
}
```

---

### Real-World Use Cases
- **Stop-Motion & Scan Sequences:** Assemble a sequence of captured camera snapshots or scanner bed captures into a fluid animated inspection loop.
- **Diagram Evolution & Infographics:** Combine progressive software architecture diagrams or wireframes to illustrate workflow changes over time.
- **AI Agent Visual Reports:** Agent workflows that render step-by-step UI actions or document transformations can stitch intermediate frame snapshots into an audit playback GIF.
