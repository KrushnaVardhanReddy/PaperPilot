---
title: "PDF Posterize"
description: "Tile oversized PDF pages across a grid of standard printable sheets with overlap margins and cut guides."
---

# PDF Posterize

The `pdf_posterize` tool takes a single-page or multi-page PDF with large dimensions and tiles each page across a grid of smaller standard sheets (e.g., 2x2, 3x3, or auto-calculated). It allows you to print oversized documents (like maps, schematics, or posters) on standard home or office printers.

## Features
- Overlap margin (glue gutter) to easily align and assemble the printed pieces.
- Corner trim guide lines to slice off unprintable margins.
- Auto-calculation of grid dimensions based on target paper size and scale.

## Usage Examples

### CLI

```bash
paperpilot-cli pdf-posterize --input large-map.pdf --output poster-tiles.pdf --target-sheet a4 --overlap 15 --scale 1.5
```

### MCP JSON-RPC

```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "pdf_posterize",
    "arguments": {
      "input": "/path/to/large-map.pdf",
      "output": "/path/to/poster-tiles.pdf",
      "target_sheet": "a4",
      "overlap_pt": 15.0,
      "scale": 1.5,
      "include_guides": true
    }
  },
  "id": 1
}
```

### REST API

```bash
curl -X POST http://localhost:8080/api/v1/pdf/posterize \\
  -H "Content-Type: application/json" \\
  -d '{
    "input": "/path/to/large-map.pdf",
    "output": "/path/to/poster-tiles.pdf",
    "target_sheet": "a4",
    "overlap_pt": 15.0,
    "scale": 1.5
  }'
```
