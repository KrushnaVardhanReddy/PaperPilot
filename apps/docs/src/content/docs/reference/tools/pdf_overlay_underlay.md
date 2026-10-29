---
title: "pdf_overlay_underlay"
description: "Reference, CLI, MCP, and REST API commands for pdf_overlay_underlay."
sidebar:
  order: 56
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Applies letterheads, watermarked stationary, and templates as background or foreground.**  
Leverages Form XObjects to efficiently embed stationary templates once and reuse them across multiple pages with custom opacity and positioning.

### Options

* **template_path** (Optional String): Path to the template PDF to overlay or underlay.
* **template_data** (Optional String): Inline base64 template PDF data.
* **mode** (Optional String): Mode of application: "overlay" (foreground) or "underlay" (background / watermark behind text). Defaults to `"underlay"`.
* **pages** (Optional String): Target pages to apply the template: `"all"`, `"1"`, `"1,3,5"`, `"first"`, `"last"`. Defaults to `"all"`.
* **opacity** (Optional Float): Opacity between 0.0 and 1.0. Defaults to `1.0`.

### CLI
```bash
paperpilot pdf-overlay-underlay --input input.pdf --output output.pdf --template-path template.pdf --mode underlay --pages all
```

### MCP JSON-RPC
```json
{
  "name": "pdf_overlay_underlay",
  "arguments": {
    "input": "/tmp/input.pdf",
    "output": "/tmp/output.pdf",
    "template_path": "/tmp/template.pdf",
    "mode": "underlay",
    "pages": "all"
  }
}
```

### REST API
```bash
curl -X POST http://localhost:7823/api/v1/pdf/overlay-underlay -H "Content-Type: application/json" -d '{"input": "/tmp/input.pdf", "output": "/tmp/output.pdf", "template_path": "/tmp/template.pdf", "mode": "underlay", "pages": "all"}'
```