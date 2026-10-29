---
title: Split Scanned Book (Spine)
description: Detect center binding gutter spine and split two-up page scans into single pages.
---

# Split Scanned Book (Spine)

Detect center binding gutter spine and split two-up page scans into single pages without modifying underlying vector streams, avoiding rasterization blur.

## Features
- Supports LTR and RTL reading directions for sequence output.
- Split horizontally (for top/bottom bindings) or vertically.
- Adjustable gutter overlap to avoid inner margin text loss.

## Interfaces

### CLI Usage
```bash
paperpilot pdf-split-scanned-book \
    --input scan_spread.pdf \
    --output split_pages.pdf \
    --reading-direction ltr \
    --gutter-overlap 5.0 \
    --split-horizontal
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_split_scanned_book",
    "arguments": {
      "input": "scan_spread.pdf",
      "output": "split_pages.pdf",
      "reading_direction": "ltr",
      "gutter_overlap": 5.0,
      "split_horizontal": false
    }
  }
}
```

### REST API
```bash
curl -X POST http://localhost:7823/api/v1/pdf/split-scanned-book \
  -H "Content-Type: application/json" \
  -d '{"input": "scan_spread.pdf", "output": "split_pages.pdf", "reading_direction": "ltr", "gutter_overlap": 5.0, "split_horizontal": false}'
```
