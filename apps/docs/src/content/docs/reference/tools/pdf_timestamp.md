---
title: "pdf_timestamp"
description: "Reference, CLI, MCP, and REST API commands for pdf_timestamp."
sidebar:
  order: 50
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Applies RFC 3161 cryptographically verified TSA timestamp tokens.**  
Affixes immutable timestamp tokens from trusted Time Stamping Authorities (TSA) directly into PDF document security dictionaries.

### CLI
```bash
paperpilot pdf-timestamp --input legal_document.pdf --output legal_document_stamped.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_timestamp",
  "arguments": {
    "input": "legal_document.pdf",
    "output": "legal_document_stamped.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/timestamp -H 'Content-Type: application/json' -d '{"input": "legal_document.pdf", "output": "legal_document_stamped.pdf"}'
```
