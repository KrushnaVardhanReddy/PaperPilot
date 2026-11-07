---
title: "watcher_list"
description: "Reference, CLI, MCP, and REST API commands for watcher_list."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: Query / JSON List

### What it does
**List configured directory watchers.**
This tool lists the active directory watchers configured to automatically trigger workflows when new files arrive.

### CLI
```bash
paperpilot watch list
```

### MCP JSON-RPC
```json
{
  "name": "watcher_list",
  "arguments": {}
}
```

### REST API
```bash
curl -s -X GET http://127.0.0.1:7823/api/v1/automation/watchers
```
