---
title: "pdf_compare"
description: "Reference, CLI, MCP, and REST API commands for pdf_compare."
sidebar:
  order: 12
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Compares two PDF documents and writes a JSON difference report.**
The report includes `diff_detected` and affected page numbers. In a workflow, later PDF operations continue reading the source PDF; the report is a separate artifact.

### CLI
```bash
paperpilot compare --input tests/e2e_fixtures/real/merge_a.pdf --input-b tests/e2e_fixtures/real/merge_b.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_compare",
  "arguments": {
    "input_a": "tests/e2e_fixtures/real/merge_a.pdf",
    "input_b": "tests/e2e_fixtures/real/merge_b.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/comparison.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compare -H 'Content-Type: application/json' -d '{"file1": "tests/e2e_fixtures/real/merge_a.pdf", "file2": "tests/e2e_fixtures/real/merge_b.pdf"}'
```
