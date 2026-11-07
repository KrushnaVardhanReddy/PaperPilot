---
title: "cli_ask"
description: "Reference, CLI, MCP, and REST API commands for cli_ask."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: Query / Markdown

### What it does
**Search and ask questions across the workspace.**
This tool allows searching and querying the local file workspace using semantic search and local AI.

### CLI
```bash
paperpilot ask "What is in my workspace?"
```

### MCP JSON-RPC
```json
{
  "name": "cli_ask",
  "arguments": {
    "query": "What is in my workspace?"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/workspace/ask \
  -H 'Content-Type: application/json' \
  -d '{"query": "What is in my workspace?"}'
```
