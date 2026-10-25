---
title: "Stitch Continuous Canvas"
description: "Combine multiple pages vertically or horizontally into a continuous single-page canvas."
---

# Stitch Continuous Canvas

Combine multiple pages of a PDF document into a single continuous canvas, either vertically or horizontally.

This is useful for creating scrolling viewports, endless-scroll web assets, or panoramic print documents.

## Options

- **direction** (`string`): The direction to stitch the pages. Can be `vertical` or `horizontal`. (Default: `vertical`)
- **margin_pt** (`number`): The margin around the outer edge of the combined canvas in points. (Default: `0.0`)
- **spacing_pt** (`number`): The spacing between the pages in points. (Default: `0.0`)

## Examples

### CLI
```bash
paperpilot pdf-stitch-canvas --input input.pdf --output stitched.pdf --direction horizontal --margin 10 --spacing 5
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "pdf_stitch_canvas",
    "arguments": {
      "input": "input.pdf",
      "output": "stitched.pdf",
      "direction": "vertical",
      "margin_pt": 20,
      "spacing_pt": 0
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -X POST http://localhost:8000/api/v1/pdf/stitch-canvas \
  -H "Content-Type: multipart/form-data" \
  -F "file=@input.pdf" \
  -F "direction=vertical" \
  -F "margin_pt=10" \
  -F "spacing_pt=5"
```
