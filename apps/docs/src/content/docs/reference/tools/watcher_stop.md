---
title: "watcher_stop"
description: "Stop and remove a local folder watcher after its current PDF operation finishes."
---

### What it does

Cancels a real watcher, waits for any in-flight PDF operation and worker thread to finish, then removes its registry entry. After success no further files are processed by that watcher. Unknown IDs return an error.

The desktop Stop action retains the stopped configuration and final statistics so it can be restarted. Desktop Remove and MCP/REST stop remove the runtime entry. Removing from another local interface is reflected in the desktop panel by its local status refresh. These APIs apply to watchers in the same process.

### CLI

```bash
# Press Ctrl+C in the terminal running `paperpilot watch` to stop it gracefully.
# `paperpilot watch --once ...` stops automatically after the initial sweep.
```

### MCP JSON-RPC

```json
{
  "name": "watcher_stop",
  "arguments": { "watcher_id": "WATCHER_ID_FROM_START" }
}
```

Returns `{"success":true}` only after the worker has stopped and been removed.

### REST API

```bash
curl -i -X DELETE http://127.0.0.1:7823/v1/automation/watchers/WATCHER_ID_FROM_START
```

Returns HTTP 204 after stopping/removing, or HTTP 404 for an unknown watcher.
