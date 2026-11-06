---
title: "pdf_dedup_streams"
description: "Reference, CLI, MCP, and REST API commands for pdf_dedup_streams."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Losslessly deduplicates identical streams using SHA-256 hashing.**

### CLI
```bash
paperpilot pdf-dedup-streams --input document.pdf --output deduped.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_dedup_streams",
  "arguments": {
    "input": "document.pdf",
    "output": "deduped.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/dedup-streams -H 'Content-Type: application/json' -d '{"input": "document.pdf", "output": "deduped.pdf"}'
```
