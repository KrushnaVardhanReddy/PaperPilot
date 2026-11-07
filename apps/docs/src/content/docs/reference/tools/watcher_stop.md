---
title: "watcher_stop"
description: "Reference, CLI, MCP, and REST API commands for watcher_stop."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: ID / Status

### What it does
**Stop an active directory watcher daemon.**
This tool safely halts an active directory watcher, completing any in-progress workflows before terminating.

### CLI
```bash
paperpilot watch stop --id watcher-123
```

### MCP JSON-RPC
```json
{
  "name": "watcher_stop",
  "arguments": {
    "id": "watcher-123"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/automation/watchers/watcher-123/stop
```
