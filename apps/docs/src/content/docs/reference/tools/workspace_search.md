---
title: "workspace_search"
description: "Reference, CLI, MCP, and REST API commands for workspace_search."
sidebar:
  order: 53
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot search --query 'test' --dir tests/e2e_fixtures/real
```

### MCP JSON-RPC
```json
{
  "name": "workspace_search",
  "arguments": {
    "query": "test",
    "dir": "tests/e2e_fixtures/real"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/workspace-search -H 'Content-Type: application/json' -d '{"query": "test", "dir": "tests/e2e_fixtures/real"}'
```
