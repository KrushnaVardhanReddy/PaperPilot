---
title: "execution_receipt_list"
description: "List durable local workflow, watcher, and batch execution history."
sidebar:
  order: 10
---

**Status**: ✅ Verified  |  **Format**: Storage Receipt Logs $\to$ JSON Summaries

### What it does

Lists normalized receipt summaries from an explicitly selected local receipts directory, ordered newest to oldest.
No process registry or volatile daemon cache is used; completed execution records remain persistent and queryable across process restarts.

- **Universal Receipt Aggregation**: Reads receipts produced by:
  - Workflow / Recipe runs (`.paperpilot-receipts/run-*.json`)
  - Watcher hot folder daemons (`*.receipt.json`)
  - Batch mail-merge runs (`execution_receipt.json`)
- **Bounded Resource Consumption**: List responses summarize core metadata (IDs, SHA-256 hashes, timestamps, durations, and outcomes) while omitting verbose stage trees to bound memory and response sizes. Full per-stage trees or per-row records can be retrieved individually using [`execution_receipt_get`](/reference/tools/execution_receipt_get/).
- **Safe Directory Traversal**: Rejects path traversals, symbolic links, and receipt files exceeding 16 MiB.
- **Configurable Limits**: Supports bounding return count with `limit` (default: 50; valid: 1–1000).

---

### CLI

Query receipt history using `paperpilot workflow history`:

```bash
# Query receipt history in a receipts directory
paperpilot workflow history --directory ./out/.paperpilot-receipts --limit 50 --json
```

**Output**:
```json
{
  "success": true,
  "operation": "Workflow",
  "error": null,
  "workflow": [
    {
      "receipt_id": "run-HrUGVW.json",
      "job_id": "run-HrUGVW",
      "source_schema": "workflow/v1",
      "started_at": "2026-11-16T15:28:21.381763727+00:00",
      "duration_ms": 3,
      "success": true,
      "input_sha256": "f20cc181ad1f7b364e75f0f49d392228cac8293210d01ef8237108e445da935a",
      "output_sha256": "5ccfae9ea7911625362cb29199c8486dc6a817f56f8545dd0dbba92205deb84b",
      "failed_stage": null
    }
  ]
}
```

#### CLI Parameters
| Flag | Required | Default | Description |
| --- | --- | --- | --- |
| `--directory` | Yes | — | Absolute or relative path to the directory containing execution receipts (e.g. `out/.paperpilot-receipts`) |
| `--limit` | No | 50 | Maximum number of receipt summaries to return (1–1000) |
| `--json` | No | — | Output result as structured JSON |

---

### MCP JSON-RPC

```json
{
  "name": "execution_receipt_list",
  "arguments": {
    "directory": "/local/out/.paperpilot-receipts",
    "limit": 50
  }
}
```

**MCP Response**:
```json
[
  {
    "receipt_id": "run-HrUGVW.json",
    "job_id": "run-HrUGVW",
    "source_schema": "workflow/v1",
    "started_at": "2026-11-16T15:28:21.381763727+00:00",
    "duration_ms": 3,
    "success": true,
    "input_sha256": "f20cc181ad1f7b364e75f0f49d392228cac8293210d01ef8237108e445da935a",
    "output_sha256": "5ccfae9ea7911625362cb29199c8486dc6a817f56f8545dd0dbba92205deb84b",
    "failed_stage": null
  }
]
```

---

### REST API

#### Endpoint
`GET /v1/automation/receipts`

#### Request
```bash
curl -G http://127.0.0.1:7823/v1/automation/receipts \
  --data-urlencode 'directory=/local/out/.paperpilot-receipts' \
  --data 'limit=50'
```

#### Response
```json
[
  {
    "receipt_id": "run-HrUGVW.json",
    "job_id": "run-HrUGVW",
    "source_schema": "workflow/v1",
    "started_at": "2026-11-16T15:28:21.381763727+00:00",
    "duration_ms": 3,
    "success": true,
    "input_sha256": "f20cc181ad1f7b364e75f0f49d392228cac8293210d01ef8237108e445da935a",
    "output_sha256": "5ccfae9ea7911625362cb29199c8486dc6a817f56f8545dd0dbba92205deb84b",
    "failed_stage": null
  }
]
```

---

### Known Limitations & Troubleshooting

- **Filesystem-Free Invocations**: Byte-only runners (e.g. WebAssembly `WasmPdfEngine.workflow` or streaming multipart REST calls) do not generate on-disk receipts and will not appear in this listing.
- **Corrupt Receipts**: If a receipt JSON file is corrupted or contains unrecognized non-receipt JSON within the scanned directory, an error is returned to prevent incomplete audit conclusions.
- **Symlink Protection**: Symlinked receipt files are explicitly rejected to protect against arbitrary read vulnerabilities.
