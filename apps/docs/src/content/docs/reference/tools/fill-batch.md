---
title: "fill-batch"
description: "Batch fills AcroForm PDF templates using CSV or JSON data."
---

# fill-batch

Batch fills AcroForm PDF templates using CSV or JSON data.

## Description

The `fill-batch` tool enables CSV/JSON-driven mail merge on AcroForm PDF templates. It generates one output PDF per data row, with customizable filename token patterns.

## Examples

### CLI

```bash
paperpilot fill-batch \
  --template template.pdf \
  --data data.csv \
  --output-dir ./output \
  --name-pattern "{Name}_{_date}.pdf"
```

### MCP JSON-RPC

```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "fill-batch",
    "arguments": {
      "template": "template.pdf",
      "data": "data.csv",
      "output_dir": "./output",
      "name_pattern": "{Name}_{_date}.pdf"
    }
  },
  "id": 1
}
```

### REST API

```bash
curl -X POST http://localhost:8080/api/v1/tools/fill-batch \
  -H "Content-Type: application/json" \
  -d '{
    "template": "template.pdf",
    "data": "data.csv",
    "output_dir": "./output",
    "name_pattern": "{Name}_{_date}.pdf"
  }'
```
