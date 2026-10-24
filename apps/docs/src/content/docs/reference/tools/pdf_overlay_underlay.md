---
title: "pdf_overlay_underlay"
description: "Overlay or underlay a template PDF (e.g., stationery letterhead or watermark) onto a target document."
sidebar:
  order: 98
---

<div class="flex gap-2 mt-4 mb-8">
  <span class="badge badge-success">Production Ready</span>
  <span class="badge badge-info">Format: PDF ➔ PDF</span>
</div>

The `pdf_overlay_underlay` tool applies a template PDF (such as a company letterhead or a watermark) either as a foreground overlay or a background underlay across specified pages of an input document.

By default, it functions as an underlay, placing the template behind the document's original content, effectively working as a watermark or stationery template. It leverages form XObjects to efficiently embed the template once and reuse it across multiple pages.

### Options

*   **template_path** (Optional String): Path to the template PDF to overlay or underlay.
*   **template_data** (Optional String): Inline base64 template PDF data.
*   **mode** (Optional String): Mode of application: "overlay" (foreground) or "underlay" (background / watermark behind text). Defaults to `"underlay"`.
*   **pages** (Optional String): Target pages to apply the template: `"all"`, `"1"`, `"1,3,5"`, `"first"`, `"last"`. Defaults to `"all"`.
*   **opacity** (Optional Float): Opacity between 0.0 and 1.0. Defaults to `1.0`.

---

## ⚡ Examples

### 1. CLI Usage

Using the PaperPilot CLI, you can apply a template as an underlay to a document:

```bash
paperpilot-cli pdf-overlay-underlay input.pdf output.pdf \
    --template-path template.pdf \
    --mode underlay \
    --pages all
```

### 2. MCP JSON-RPC Payload

For AI agents or direct MCP integrations, you can apply an overlay via a JSON-RPC request:

```json
{
  "jsonrpc": "2.0",
  "id": "req-1234",
  "method": "tools/call",
  "params": {
    "name": "pdf_overlay_underlay",
    "arguments": {
      "input_file": "/tmp/input.pdf",
      "output_file": "/tmp/output.pdf",
      "template_path": "/tmp/template.pdf",
      "mode": "overlay",
      "pages": "first"
    }
  }
}
```

### 3. REST API Gateway

Using `curl` or any HTTP client to interact with the REST gateway for overlaying a template:

```bash
curl -X POST http://localhost:8080/api/v1/pdf/overlay-underlay \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/path/to/input.pdf",
    "output_path": "/path/to/output.pdf",
    "options": {
      "template_path": "/path/to/template.pdf",
      "mode": "underlay",
      "pages": "all"
    }
  }'
```