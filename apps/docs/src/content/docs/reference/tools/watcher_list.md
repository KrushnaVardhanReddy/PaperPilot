---
title: "watcher_list"
description: "Read actual folder watcher lifecycle states and processing statistics."
---

### What it does

Lists runtime entries from the shared local manager with `watcher_id`, `status`, `dir`, `recipe`, `last_processed_file`, `last_processed_at`, `total_processed`, `succeeded`, `failed`, `last_error`, `last_output`, and `last_receipt`. Output/receipt paths remain available when the desktop panel is reopened. `total_processed` counts successes plus failures, rather than successful writes alone.

Statuses are `watching`, `stopping`, `stopped`, or `error`. A per-file processing failure increments `failed` while watching continues; fatal filesystem errors end the worker with `error`. Persisted desktop preferences are never reported as active runtime entries. Desktop-stopped watchers remain listed until removal, while MCP/REST stop removes the entry.

### CLI

```bash
# The CLI watcher runs in its own process and logs events in its terminal.
# Use MCP/REST to list workers belonging to their server process.
```

### MCP JSON-RPC

```json
{ "name": "watcher_list", "arguments": {} }
```

### REST API

```bash
curl -s http://127.0.0.1:7823/v1/automation/watchers
```

Both interfaces return `{"watchers":[...]}`; an empty list means there are no entries in that process.
