---
title: "execution_receipt_get"
description: "Read one durable execution receipt, including actual stage or row outcomes."
---

### What it does

Reads one receipt by its exact filename (`receipt_id`) in an explicitly selected directory. Returns normalized outcome, hashes, duration and failed-stage metadata plus the source record in `details`. Paths outside that directory, receipt symlinks and unsupported schemas are rejected. No missing metadata is invented for older batch/watcher receipts.

### CLI

```bash
paperpilot workflow history --directory ./out/.paperpilot-receipts --id run-JOB_ID.json --json
```

### MCP JSON-RPC

```json
{"name":"execution_receipt_get","arguments":{"directory":"/local/out/.paperpilot-receipts","receipt_id":"run-JOB_ID.json"}}
```

### REST API

```bash
curl -G http://127.0.0.1:7823/v1/automation/receipts/run-JOB_ID.json --data-urlencode 'directory=/local/out/.paperpilot-receipts'
```

Use the filename returned by the list API or the execution's `receipt_path`; `JOB_ID` above is a placeholder. Legacy batch details retain per-row results, and watcher details retain their original workflow report and artifact paths.
