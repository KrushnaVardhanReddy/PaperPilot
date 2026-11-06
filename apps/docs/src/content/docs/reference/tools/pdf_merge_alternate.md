---
title: "pdf_merge_alternate"
description: "Reference, CLI, MCP, and REST API commands for pdf_merge_alternate."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Alternates pages from two input PDFs (e.g., front and back scan passes).**

### CLI
```bash
paperpilot pdf-merge-alternate --input-a front.pdf --input-b back.pdf --output merged.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_merge_alternate",
  "arguments": {
    "input_a": "front.pdf",
    "input_b": "back.pdf",
    "output": "merged.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/merge-alternate -H 'Content-Type: application/json' -d '{"input_a": "front.pdf", "input_b": "back.pdf", "output": "merged.pdf"}'
```
