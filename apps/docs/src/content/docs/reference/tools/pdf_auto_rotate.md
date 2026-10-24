---
title: "pdf_auto_rotate"
description: "Automatically corrects page orientations (portrait or landscape) based on their dimensions."
---

# pdf_auto_rotate

The `pdf_auto_rotate` tool automatically detects and corrects the orientation of pages in a PDF document based on their physical dimensions. It rotates landscape-oriented pages to portrait or vice versa, ensuring consistent page orientations throughout the document.

## Description

This operation parses a PDF document, analyzes the `MediaBox` and existing `/Rotate` properties of each page, and rotates pages by 90 degrees if they don't match the target orientation.

## Options

*   **`target_orientation`** (`string`, default: `"portrait"`): The desired page orientation. Valid options are `"portrait"`, `"landscape"`, or `"auto"`. Currently `"auto"` behaves the same as `"portrait"`.
*   **`pages`** (`number[]`, optional): Specific 1-based page numbers to apply rotation to. If not provided, all pages are analyzed and rotated if necessary.

## Examples

### CLI

```bash
paperpilot pdf-auto-rotate input.pdf output.pdf --target-orientation portrait
```

### MCP (JSON-RPC)

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "pdf_auto_rotate",
  "params": {
    "file_path": "input.pdf",
    "output_path": "output.pdf",
    "target_orientation": "portrait"
  }
}
```

### REST API

```bash
curl -X POST http://localhost:8080/tools/pdf_auto_rotate \
  -H "Content-Type: application/json" \
  -d '{"file_path": "input.pdf", "output_path": "output.pdf", "target_orientation": "portrait"}'
```
