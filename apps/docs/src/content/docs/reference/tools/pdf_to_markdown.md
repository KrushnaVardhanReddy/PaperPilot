---
title: "pdf_to_markdown"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_markdown."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF / Markdown

### What it does
**Convert PDFs to Markdown.**
This tool extracts text and semantic formatting from a PDF document and converts it into standard Markdown format.

### CLI
```bash
paperpilot convert --format markdown --input tests/e2e_fixtures/real/test.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_to_markdown_result.md
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_markdown",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_to_markdown_result.md"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-markdown \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/test.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_to_markdown_result.md"}'
```
