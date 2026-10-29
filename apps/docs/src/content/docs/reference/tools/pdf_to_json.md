---
title: "pdf_to_json"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_json."
sidebar:
  order: 51
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Exports full multimodal PDF layout AST to structured JSON.**  
Extracts text blocks, bounding boxes, font metrics, structural hierarchy, and embedded image metadata into machine-readable JSON for downstream AI workflows.

### CLI
```bash
paperpilot convert --input document.pdf --format json --output document.json --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_json",
  "arguments": {
    "input": "document.pdf",
    "output": "document.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-json -H 'Content-Type: application/json' -d '{"input": "document.pdf", "output": "document.json"}'
```
