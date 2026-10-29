---
title: "execution_receipt_list"
description: "List durable local workflow, watcher and batch execution history."
---

### What it does

Lists normalized receipt summaries from an explicitly selected local directory, most recently stored first. The default limit is 50; valid limits are 1–1000. Details are fetched separately. No process registry or running-state cache is used, so completed records remain queryable after restart.

File-based CLI recipes, MCP `recipe_run` / `document_workflow`, and REST JSON workflows save version-1 receipts to `OUTPUT_PARENT/.paperpilot-receipts/run-*.json`. Records contain job ID, UTC timestamps, duration, actual input/output SHA-256 hashes, outcome and stage report. Raw recipes and PDF bytes are not stored. Failed executions retain the prior output and record the failed stage; a failed output write is never a successful publication. Invalid recipes rejected before execution are not recorded. History storage errors fail explicitly.

The same query reads existing watcher `*.receipt.json` and batch `execution_receipt.json` files in their existing directories. Missing legacy metadata stays null; source files are not migrated or rewritten. Corrupt recognized receipts return an error. Symlink receipts and files larger than 16 MiB are rejected. List responses omit full details to bound memory use.

### CLI

```bash
paperpilot workflow history --directory ./out/.paperpilot-receipts --limit 50 --json
```

### MCP JSON-RPC

```json
{"name":"execution_receipt_list","arguments":{"directory":"/local/out/.paperpilot-receipts","limit":50}}
```

### REST API

```bash
curl -G http://127.0.0.1:7823/v1/automation/receipts --data-urlencode 'directory=/local/out/.paperpilot-receipts' --data 'limit=50'
```

The byte-only runner and WASM remain free of filesystem side effects. Multipart responses that only return bytes do not create local history. Completed results are durable; this is not a resumable job scheduler or a record of interrupted in-flight work.
