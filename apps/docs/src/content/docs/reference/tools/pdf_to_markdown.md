---
title: "pdf_to_markdown"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_markdown."
sidebar:
  order: 52
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Decompiles PDF documents into clean, LLM-ready Markdown.**  
Extracts semantic headings, lists, inline formatting, code blocks, tables, and media links into high-fidelity Markdown text.

### CLI
```bash
paperpilot convert --input document.pdf --format markdown --output document.md --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_markdown",
  "arguments": {
    "input": "document.pdf",
    "output": "document.md"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-markdown -H 'Content-Type: application/json' -d '{"input": "document.pdf", "output": "document.md"}'
```
