---
title: "pdf_to_gif"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_gif."
sidebar:
  order: 63
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) to Animated GIF (`.gif`)

### What it does
**Converts multi-page PDF documents into animated looping GIF flipbooks.**  
Renders successive pages of a PDF document and compiles them into a smooth, lightweight animated GIF. Configurable frame rate (`fps`), frame delay (`delay`), target image width, and maximum page count allow creating interactive flipbook previews, thumbnail reels, and social media shareables without external dependencies.

---

### CLI
Convert a multi-page PDF into an animated GIF via the PaperPilot CLI `pdf-to-gif` subcommand:

```bash
paperpilot pdf-to-gif \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/preview.gif \
  --fps 2 \
  --width 800 \
  --max-pages 10 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF document.
- `--output` *(string, required)*: Destination file path for the animated `.gif`.
- `--fps` *(integer, optional, default: `1`)*: Playback frame rate in frames per second. If provided, frame delay is calculated as `(100 / fps)`.
- `--delay` *(integer, optional, default: `100`)*: Delay between frames in centiseconds (1/100s, e.g. `100` for 1.0 second).
- `--max-pages` *(integer, optional)*: Maximum number of pages to render from the beginning of the PDF. If omitted, all pages are converted.
- `--width` *(integer, optional, default: `800`)*: Target horizontal width in pixels. Height scales proportionally preserving aspect ratio.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_to_gif`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_gif",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/preview.gif",
      "fps": 2,
      "width": 800,
      "max_pages": 5
    }
  },
  "id": 1
}
```

#### MCP Parameters
- `input` *(string, required)*: Path to the input PDF document.
- `output` *(string, required)*: Path to the target output GIF file.
- `fps` *(number, optional)*: Frame rate in frames per second.
- `delay` *(number, optional)*: Delay between frames in centiseconds.
- `max_pages` *(number, optional)*: Limit on the number of rendered pages.
- `width` *(number, optional)*: Frame width in pixels.

---

### REST API
Invoke the REST API endpoint `/api/v1/pdf/to-gif`:

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-gif \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/preview.gif",
    "fps": 2,
    "width": 800
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "b1a2c3d4-e5f6-7a8b-9c0d-1e2f3a4b5c6d",
  "message": "Convert PDF to Animated GIF executed successfully",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Convert PDF to Animated GIF executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/preview.gif\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/preview.gif\"}}"
      }
    ]
  }
}
```

---

### Real-World Use Cases
- **Document Previews on Web & Mobile:** Generate animated multi-page flipbook previews for document management systems, catalogs, and client portals without downloading full multi-megabyte PDFs.
- **Social Media Marketing:** Create auto-playing teasers of whitepapers, pitch decks, research papers, and eBooks for LinkedIn, X (Twitter), and newsletters.
- **E-Commerce & Digital Goods:** Showcase digital planner pages, printable templates, and sheet music with animated page flips.
