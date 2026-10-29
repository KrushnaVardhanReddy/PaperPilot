---
title: "pdf_attachments"
description: "Reference, CLI, MCP, and REST API commands for pdf_attachments."
sidebar:
  order: 53
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Inspects, extracts, or embeds file attachments into PDF portfolios.**  
Allows users to manage embedded files within PDF documents. A "portfolio" document in PDF can contain arbitrary file types encapsulated securely inside it. Supports list, add, extract, and remove operations.

### CLI
```bash
paperpilot pdf-attachments --input portfolio.pdf --action list --output report.json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_attachments",
  "arguments": {
    "input": "base.pdf",
    "output": "updated.pdf",
    "action": "add",
    "file_name": "data.csv",
    "description": "Monthly financial data",
    "file_data_base64": "SGVsbG8sIFdvcmxkIQ=="
  }
}
```

### REST API
```bash
curl -X POST "http://127.0.0.1:7823/api/v1/pdf/attachments?action=extract&file_name=secret.txt" -H "Content-Type: application/json" -d '{"input": "portfolio.pdf", "output": "local_extracted.txt"}'
```
