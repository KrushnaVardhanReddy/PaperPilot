---
title: "document_workflow"
description: "Reference, CLI, MCP, and REST API commands for document_workflow."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: Various / Various

### What it does
**Run automated pipelines on documents.**
This tool executes a configured document workflow pipeline on a set of input files, producing processed artifacts and execution receipts.

### CLI
```bash
paperpilot workflow run --config pipeline.json --input input_dir --output output_dir
```

### MCP JSON-RPC
```json
{
  "name": "document_workflow",
  "arguments": {
    "config": "pipeline.json",
    "input": "input_dir",
    "output": "output_dir"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/automation/workflows/run \
  -H 'Content-Type: application/json' \
  -d '{"config": "pipeline.json", "input": "input_dir", "output": "output_dir"}'
```
