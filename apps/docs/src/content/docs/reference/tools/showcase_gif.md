---
title: "showcase_gif"
description: "Reference, CLI, MCP, and REST API commands for showcase_gif."
sidebar:
  order: 64
---

**Status**: ✅ Verified  |  **Added in**: `v0.8.0+`  |  **Format**: PDF (`.pdf`) to Showcase Animated GIF (`.gif`)

### What it does
**Generates an animated showcase mockup card highlighting document content with elegant transitions.**  
Renders PDF document pages into a polished presentation mockup card featuring clean drop shadows, smooth page transitions, and progressive scanning animations. Designed for product marketing, documentation showcases, portfolio highlights, and software demo reels.

---

### CLI
Generate an animated showcase GIF via the PaperPilot CLI `showcase-gif` subcommand:

```bash
paperpilot showcase-gif \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/showcase.gif \
  --fps 4 \
  --width 1200 \
  --transition-frames 10 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF document.
- `--output` *(string, required)*: Destination file path for the showcase `.gif`.
- `--fps` *(integer, optional, default: `4`)*: Playback frame rate in frames per second.
- `--delay` *(integer, optional, default: `25`)*: Delay between frames in centiseconds (1/100s, e.g. `25` for 0.25 seconds).
- `--width` *(integer, optional, default: `1200`)*: Target canvas width in pixels. Aspect ratio scales proportionally.
- `--transition-frames` *(integer, optional, default: `10`)*: Number of interpolation animation frames per transition cycle.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `showcase_gif`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "showcase_gif",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/showcase.gif",
      "fps": 4,
      "width": 1200,
      "transition_frames": 10
    }
  },
  "id": 1
}
```

#### MCP Parameters
- `input` *(string, required)*: Path to the input PDF document.
- `output` *(string, required)*: Path to the destination GIF file.
- `fps` *(number, optional)*: Frames per second.
- `delay` *(number, optional)*: Delay between frames in centiseconds.
- `width` *(number, optional)*: Showcase canvas width in pixels.
- `transition_frames` *(number, optional)*: Number of transition animation frames.

---

### REST API
Invoke the REST API endpoint `/api/v1/pdf/showcase-gif`:

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/showcase-gif \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/showcase.gif",
    "fps": 4,
    "width": 1200
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "784dcc9b-1234-4567-89ab-cdef01234567",
  "message": "Generate Showcase GIF executed successfully",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Generate Showcase GIF executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/showcase.gif\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/showcase.gif\"}}"
      }
    ]
  }
}
```

---

### Real-World Use Cases
- **Product Landing Pages & GitHub READMEs:** Embed eye-catching, professional animated presentation cards of invoices, reports, certificates, or brochures directly in markdown README files and marketing websites.
- **Client Delivery Mockups:** Automatically deliver client deliverables inside a branded, animated mockup card that showcases page highlights.
- **SaaS Feature Highlights:** Demonstrate automated document generation, redaction, or form-filling results with high-impact before-and-after animated loops.
