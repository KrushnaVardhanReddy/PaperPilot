---
title: "execution_receipt_get"
description: "Reference, CLI, MCP, and REST API commands for execution_receipt_get."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: ID / JSON

### What it does
**Retrieve workflow run receipts.**
This tool fetches a specific JSON execution receipt for an automated workflow run by its ID.

### CLI
```bash
paperpilot workflow history --directory .penta_runs --id run-JOB_ID.json
```

### MCP JSON-RPC
```json
{
  "name": "execution_receipt_get",
  "arguments": {
    "receipt_id": "run-JOB_ID.json",
    "directory": ".penta_runs"
  }
}
```

### REST API
```bash
curl -s -X GET http://127.0.0.1:7823/api/v1/automation/receipts/run-JOB_ID.json
```
