---
title: pdf_stamp
description: Add an image stamp or watermark to a PDF document.
---

# `pdf_stamp`

Places an image (such as a logo, signature, or watermark) onto specified pages of a PDF document at specific coordinates, with optional transparency.

## Options

- **`image_path`** (optional): The path to the image file (PNG or JPEG).
- **`image_data`** (optional): Base64 encoded image data if `image_path` is not provided.
- **`x`**: X coordinate in points (bottom-left origin). Default is 50.0.
- **`y`**: Y coordinate in points (bottom-left origin). Default is 50.0.
- **`width`**: Rendered width in points. Default is 150.0.
- **`height`**: Rendered height in points. Default is 50.0.
- **`opacity`**: Value between 0.0 (transparent) and 1.0 (opaque). Default is 1.0.
- **`pages`**: Target pages to stamp. Can be "all", "last", or a comma-separated list like "1,3,5". Default is "all".

## Examples

### CLI

```bash
penta pdf-stamp input.pdf output.pdf --image-path stamp.png --x 100 --y 100 --width 200 --height 50 --opacity 0.5 --pages all
```

### REST API

```bash
curl -X POST http://localhost:8080/api/v1/pdf/stamp \
  -F "file=@input.pdf" \
  -F 'options={"image_path":"stamp.png","x":100.0,"y":100.0,"width":200.0,"height":50.0,"opacity":0.5,"pages":"all"}' \
  -o output.pdf
```

### MCP (JSON-RPC)

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "call_tool",
  "params": {
    "name": "pdf_stamp",
    "arguments": {
      "file_path": "/path/to/input.pdf",
      "output_path": "/path/to/output.pdf",
      "image_path": "stamp.png",
      "x": 100.0,
      "y": 100.0,
      "width": 200.0,
      "height": 50.0,
      "opacity": 0.5,
      "pages": "all"
    }
  }
}
```
