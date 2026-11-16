---
title: "execution_receipt_get"
description: "Read one durable execution receipt, including actual stage or row outcomes."
sidebar:
  order: 9
---

**Status**: ✅ Verified  |  **Format**: Storage Receipt Log $\to$ Complete JSON Execution Record

### What it does

Reads one durable execution receipt by its exact filename (`receipt_id`) in an explicitly specified directory. Returns normalized metadata, cryptographic SHA-256 hashes, start/finish timestamps, durations, error information, and the complete execution report under `details`.

- **Comprehensive Stage / Row Audit**: Unpacks the full stage tree for workflow recipes (including completed, skipped, and failed stages with error reasons) or per-row outcomes for batch merge runs.
- **Path Isolation**: Rejects path traversal attempts (`../`), symlinks, and files outside the designated receipt directory.
- **Legacy Compatibility**: Reads workflow receipts (`run-*.json`), watcher receipts (`*.receipt.json`), and batch mail-merge receipts (`execution_receipt.json`) without modifying or rewriting original files on disk.

---

### CLI

Query a specific receipt using `paperpilot workflow history`:

```bash
paperpilot workflow history \
  --directory ./out/.paperpilot-receipts \
  --id run-HrUGVW.json \
  --json
```

**Output**:
```json
{
  "success": true,
  "operation": "Workflow",
  "error": null,
  "workflow": {
    "receipt_id": "run-HrUGVW.json",
    "job_id": "run-HrUGVW",
    "source_schema": "workflow/v1",
    "started_at": "2026-11-16T15:28:21.381763727+00:00",
    "duration_ms": 3,
    "success": true,
    "input_sha256": "f20cc181ad1f7b364e75f0f49d392228cac8293210d01ef8237108e445da935a",
    "output_sha256": "5ccfae9ea7911625362cb29199c8486dc6a817f56f8545dd0dbba92205deb84b",
    "failed_stage": null,
    "details": {
      "schema_version": 1,
      "job_id": "run-HrUGVW",
      "operation": "workflow",
      "started_at": "2026-11-16T15:28:21.381763727+00:00",
      "finished_at": "2026-11-16T15:28:21.385654735+00:00",
      "duration_ms": 3,
      "success": true,
      "input": "input.pdf",
      "input_sha256": "f20cc181ad1f7b364e75f0f49d392228cac8293210d01ef8237108e445da935a",
      "requested_output": "out.pdf",
      "output": "out.pdf",
      "output_sha256": "5ccfae9ea7911625362cb29199c8486dc6a817f56f8545dd0dbba92205deb84b",
      "error": null,
      "report": {
        "workflow_id": "Audit & Watermark Pipeline",
        "workflow_name": "Audit & Watermark Pipeline",
        "steps_total": 2,
        "steps_completed": 2,
        "steps_skipped": 0,
        "output_bytes_len": 1860,
        "duration_ms": 2,
        "archived_original": false,
        "error": null,
        "stages": [
          {
            "id": "step_1",
            "operation_id": "pdf_watermark",
            "status": "completed",
            "reason": null,
            "duration_ms": 0
          },
          {
            "id": "step_2",
            "operation_id": "pdf_page_numbers",
            "status": "completed",
            "reason": null,
            "duration_ms": 0
          }
        ]
      }
    }
  }
}
```

#### CLI Parameters
| Flag | Required | Description |
| --- | --- | --- |
| `--directory` | Yes | Path to the directory containing execution receipts (e.g. `out/.paperpilot-receipts`) |
| `--id` | Yes | Exact filename of the target receipt (e.g. `run-HrUGVW.json` or `execution_receipt.json`) |
| `--json` | No | Format output as structured JSON |

---

### MCP JSON-RPC

```json
{
  "name": "execution_receipt_get",
  "arguments": {
    "directory": "/local/out/.paperpilot-receipts",
    "receipt_id": "run-HrUGVW.json"
  }
}
```

**MCP Response**:
```json
{
  "receipt_id": "run-HrUGVW.json",
  "job_id": "run-HrUGVW",
  "source_schema": "workflow/v1",
  "started_at": "2026-11-16T15:28:21.381763727+00:00",
  "duration_ms": 3,
  "success": true,
  "input_sha256": "f20cc181ad1f7b364e75f0f49d392228cac8293210d01ef8237108e445da935a",
  "output_sha256": "5ccfae9ea7911625362cb29199c8486dc6a817f56f8545dd0dbba92205deb84b",
  "failed_stage": null,
  "details": {
    "schema_version": 1,
    "job_id": "run-HrUGVW",
    "operation": "workflow",
    "started_at": "2026-11-16T15:28:21.381763727+00:00",
    "finished_at": "2026-11-16T15:28:21.385654735+00:00",
    "duration_ms": 3,
    "success": true,
    "input": "input.pdf",
    "output": "out.pdf",
    "report": {
      "workflow_name": "Audit & Watermark Pipeline",
      "steps_total": 2,
      "steps_completed": 2,
      "stages": [
        {
          "id": "step_1",
          "operation_id": "pdf_watermark",
          "status": "completed"
        },
        {
          "id": "step_2",
          "operation_id": "pdf_page_numbers",
          "status": "completed"
        }
      ]
    }
  }
}
```

---

### REST API

#### Endpoint
`GET /v1/automation/receipts/{id}`

#### Request
```bash
curl -G http://127.0.0.1:7823/v1/automation/receipts/run-HrUGVW.json \
  --data-urlencode 'directory=/local/out/.paperpilot-receipts'
```

#### Response
Returns the full JSON representation containing normalized summary fields and full `details` record.

---

### Known Limitations & Troubleshooting

- **Receipt Not Found**: Ensure `receipt_id` includes the exact filename (including extension, e.g., `run-HrUGVW.json`), as returned by `execution_receipt_list`.
- **Directory Bounds**: Attempting to pass sub-paths or relative path references in `receipt_id` (e.g. `../receipt.json`) is rejected with an invalid parameter error.
- **File Size Limit**: Receipt files exceeding 16 MiB are rejected to prevent denial-of-service via corrupted disk files.
