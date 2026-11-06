---
title: "pdf_split_scanned_book"
description: "Reference, CLI, MCP, and REST API commands for pdf_split_scanned_book."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Detects center binding gutter spine and splits two-up page spreads.**

### CLI
```bash
paperpilot pdf-split-scanned-book --input scan_spread.pdf --output split_pages.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_split_scanned_book",
  "arguments": {
    "input": "scan_spread.pdf",
    "output": "split_pages.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split-scanned-book -H 'Content-Type: application/json' -d '{"input": "scan_spread.pdf", "output": "split_pages.pdf"}'
```
