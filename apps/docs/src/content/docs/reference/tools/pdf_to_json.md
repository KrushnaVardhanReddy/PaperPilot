---
title: "pdf_to_json"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_json."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF / JSON

### What it does
**Convert PDFs to structured JSON.**
This tool extracts the text, layout, and structure of a PDF document and serializes it into a JSON format for data processing.

### CLI
```bash
paperpilot convert --format json --input tests/e2e_fixtures/real/test.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_to_json_result.json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_json",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_to_json_result.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-json \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/test.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_to_json_result.json"}'
```
