---
title: Invert PDF Colors
description: Invert colors or apply dark mode to a vector PDF while preserving text and crispness.
---

# Invert PDF Colors

This tool processes vector PDFs and intelligently inverts colors or applies a true dark mode, all while preserving selectable vector text and embedded fonts.

## Features
* Applies vector-level color manipulation without rasterizing pages.
* Supports **dark_mode**, **true_invert**, **high_contrast**, and **sepia** modes.

## CLI Invocation
```bash
paperpilot invert-colors --input doc.pdf --output out.pdf --mode dark_mode
```

## REST API
```bash
curl -X POST http://localhost:7823/api/v1/pdf/invert-colors \
  -H "Content-Type: application/json" \
  -d '{"input":"doc.pdf","output":"out.pdf","mode":"true_invert"}'
```

## MCP API
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_invert_colors",
    "arguments": {
      "input": "doc.pdf",
      "output": "out.pdf",
      "mode": "dark_mode"
    }
  }
}
```
