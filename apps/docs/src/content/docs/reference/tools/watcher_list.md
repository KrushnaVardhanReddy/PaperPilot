---
title: "watcher_list"
description: "Reference, CLI, MCP, REST API, and Desktop specifications for watcher_list."
sidebar:
  order: 93
---

**Status**: ✅ Verified  |  **Format**: Automation / Hot Folder

### What it does
**Inspects active folder watchers, runtime lifecycle states, and live processing statistics.**  
Queries the in-memory `WatcherManager` registry to return comprehensive telemetry on all registered watchers, including total items processed, success/failure counts, current lifecycle state (`watching`, `stopping`, `stopped`, `error`), and paths to the most recent output and receipt files.

---

### Key Information Reported
- `watcher_id`: Unique identifier assigned during `watcher_start`.
- `status`: Current daemon lifecycle state (`watching`, `stopping`, `stopped`, or `error`).
- `dir`: Absolute path to the monitored inbox directory.
- `recipe`: Active recipe identifier or file path.
- `total_processed`: Cumulative count of processed documents (`succeeded + failed`).
- `succeeded` / `failed`: Granular counts of successful pipeline runs vs. failed runs.
- `last_processed_file`: Filename of the most recently processed PDF.
- `last_processed_at`: UTC timestamp of the latest event.
- `last_output` / `last_receipt`: Absolute paths to the latest produced document and receipt.
- `last_error`: Diagnostic message of the last failure encountered, if any.

---

### CLI
The PaperPilot CLI runs watchers in foreground daemon mode with real-time log output directly in the console:

```bash
# Live status and file transitions stream to stdout/stderr in the terminal:
paperpilot watch --directory ./inbox --recipe preset-legal-discovery
```

To query programmatic status across daemon workers, use the MCP JSON-RPC or REST API endpoints against the running PaperPilot Gateway or Desktop service.

---

### MCP JSON-RPC
Query active watchers via Model Context Protocol (MCP):

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "watcher_list",
    "arguments": {}
  },
  "id": 3
}
```

#### MCP Output Response
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"watchers\":[{\"dir\":\"/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_in\",\"failed\":0,\"last_error\":null,\"last_output\":\"/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_out/doc1.pdf\",\"last_processed_at\":\"2026-11-16T15:58:08.524142935+00:00\",\"last_processed_file\":\"doc1.pdf\",\"last_receipt\":\"/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_out/doc1.pdf.receipt.json\",\"recipe\":\"preset-legal-discovery\",\"status\":\"watching\",\"succeeded\":1,\"total_processed\":1,\"watcher_id\":\"watcher_c9b8f2d87e1445bfbb5bcf112a912803\"}]}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- The `watcher_list` tool takes an empty arguments object `{}`.

---

### REST API
List all active watchers via PaperPilot Gateway:

```bash
curl -s -X GET http://127.0.0.1:7823/v1/automation/watchers
```

#### Response (`HTTP 200 OK`)
```json
{
  "watchers": [
    {
      "watcher_id": "watcher_c9b8f2d87e1445bfbb5bcf112a912803",
      "status": "watching",
      "dir": "/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_in",
      "recipe": "preset-legal-discovery",
      "total_processed": 1,
      "succeeded": 1,
      "failed": 0,
      "last_processed_file": "doc1.pdf",
      "last_processed_at": "2026-11-16T15:58:08.524142935Z",
      "last_output": "/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_out/doc1.pdf",
      "last_receipt": "/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_out/doc1.pdf.receipt.json",
      "last_error": null
    }
  ]
}
```

---

### Known Limitations & Troubleshooting
- **Process Memory Lifetime**: Watcher registry records are maintained in process memory. If the host application or server restarts, previous watchers must be re-registered via `watcher_start` (or reloaded from desktop project preferences).
- **WASM Support**: Not applicable to browser WebAssembly environments.
