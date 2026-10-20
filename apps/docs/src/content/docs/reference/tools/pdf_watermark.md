---
title: "pdf_watermark"
description: "Reference, CLI, MCP, and REST API commands for pdf_watermark."
sidebar:
  order: 52
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Stamps semi-transparent text or image watermarks across PDF pages.**  
Adds diagonal or centered branding (e.g., 'CONFIDENTIAL', 'DRAFT') with custom opacity, font size, and rotation.

### CLI
```bash
paperpilot watermark --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/watermark_cli_Simple.pdf --text WATERMARK --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_watermark",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "text": "CONFIDENTIAL",
    "output": "tests/e2e_fixtures/out/penta_e2e/watermarked.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/watermark -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "text": "CONFIDENTIAL", "output": "tests/e2e_fixtures/out/penta_e2e/watermarked.pdf"}'
```
