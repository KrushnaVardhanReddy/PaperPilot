---
title: "watcher_stop"
description: "Reference, CLI, MCP, REST API, and Desktop specifications for watcher_stop."
sidebar:
  order: 92
---

**Status**: ✅ Verified  |  **Format**: Automation / Hot Folder

### What it does
**Gracefully cancels a running folder watcher after its active PDF operation completes and unregisters it.**  
Signals the worker thread to stop, waits for any in-flight PDF pipeline step and audit receipt generation to finish safely, and removes the watcher instance from the shared manager registry.

---

### Key Behaviors
- **Graceful Drain**: In-flight file processing is never forcibly killed mid-operation; pending writes and `.receipt.json` outputs complete cleanly.
- **Error Handling**: Supplying an invalid or unknown `watcher_id` returns an error response rather than failing silently.
- **Registry Removal**: Successful stoppage unregisters the daemon so it stops consuming system resources and thread pools.

---

### CLI
Stop running folder watchers from the command line:

#### 1. Graceful Terminal Interrupt
When running `paperpilot watch` in foreground daemon mode, send an interrupt signal:
```bash
# Press Ctrl+C in the active terminal running paperpilot watch
# The watcher completes the current file, flushes receipts, and exits cleanly.
```

#### 2. Automatic Termination with `--once`
Batch sweep runs terminate automatically once all existing directory files have been processed:
```bash
paperpilot watch --directory ./inbox --recipe preset-legal-discovery --once
```

---

### MCP JSON-RPC
Stop and unregister an active watcher via Model Context Protocol (MCP):

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "watcher_stop",
    "arguments": {
      "watcher_id": "watcher_c9b8f2d87e1445bfbb5bcf112a912803"
    }
  },
  "id": 2
}
```

#### MCP Output Response
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `watcher_id` *(string, required)*: The unique identifier of the active watcher to cancel and unregister.

---

### REST API
Terminate and unregister a watcher via PaperPilot Gateway:

```bash
curl -i -X DELETE http://127.0.0.1:7823/v1/automation/watchers/watcher_c9b8f2d87e1445bfbb5bcf112a912803
```

#### Response Status
- `HTTP 204 No Content`: The watcher was stopped and removed from the registry.
- `HTTP 404 Not Found`: No active watcher exists with the provided ID.

---

### Known Limitations & Troubleshooting
- **Lifecycle Scope**: MCP and REST termination commands only apply to watchers hosted within that specific server process. Standalone CLI processes running `paperpilot watch` in other terminal sessions must be stopped via their respective terminal process signals (`SIGINT` / `Ctrl+C`).
- **WASM Support**: Not applicable to browser WebAssembly environments.
