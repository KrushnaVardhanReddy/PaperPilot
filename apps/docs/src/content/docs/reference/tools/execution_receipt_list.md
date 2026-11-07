---
title: "execution_receipt_list"
description: "Reference, CLI, MCP, and REST API commands for execution_receipt_list."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: Query / JSON List

### What it does
**List workflow run receipts.**
This tool lists JSON execution receipts for automated workflow runs in the current workspace.

### CLI
```bash
paperpilot workflow history --directory .penta_runs --limit 50
```

### MCP JSON-RPC
```json
{
  "name": "execution_receipt_list",
  "arguments": {
    "limit": 50,
    "directory": ".penta_runs"
  }
}
```

### REST API
```bash
curl -s -X GET http://127.0.0.1:7823/api/v1/automation/receipts?limit=50
```
