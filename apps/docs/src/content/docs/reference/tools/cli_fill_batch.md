---
title: "cli_fill_batch"
description: "Reference, CLI, MCP, and REST API commands for cli_fill_batch."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF + CSV/JSON / PDF List

### What it does
**Batch fill PDF forms.**
This tool takes a fillable PDF form and a CSV or JSON dataset, mapping columns/keys to form fields to generate multiple filled PDF documents.

### CLI
```bash
paperpilot form batch-fill --template form.pdf --data dataset.csv --output-dir out/
```

### MCP JSON-RPC
```json
{
  "name": "cli_fill_batch",
  "arguments": {
    "template": "form.pdf",
    "data": "dataset.csv",
    "output_dir": "out/"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/batch-fill \
  -H 'Content-Type: application/json' \
  -d '{"template": "form.pdf", "data": "dataset.csv", "output_dir": "out/"}'
```
