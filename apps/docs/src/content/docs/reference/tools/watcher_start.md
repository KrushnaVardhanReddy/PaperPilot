---
title: "watcher_start"
description: "Reference, CLI, MCP, and REST API commands for watcher_start."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: Config / Process ID

### What it does
**Start a directory watcher daemon.**
This tool starts a background watcher process on a specific directory, executing workflows automatically when files match predefined criteria.

### CLI
```bash
paperpilot watch start --directory input/ --workflow ingest.json
```

### MCP JSON-RPC
```json
{
  "name": "watcher_start",
  "arguments": {
    "directory": "input/",
    "workflow": "ingest.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/automation/watchers/start \
  -H 'Content-Type: application/json' \
  -d '{"directory": "input/", "workflow": "ingest.json"}'
```
