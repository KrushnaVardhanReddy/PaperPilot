---
title: "watcher_start"
description: "Start a real local PDF folder watcher with validated recipes and audit receipts."
---

### What it does

Registers an operating-system folder watcher, processes existing and arriving PDFs with the shared workflow engine, and reports actual progress. Invalid paths and unsupported recipes fail before a watcher is reported as running.

The recipe accepts a preset ID, a local TOML/JSON `.ppflow` path, or serialized workflow content. Outputs default to `INPUT/processed`, failure receipts to `INPUT/.error`. Input and output folders must differ. Sources stay in place unless the CLI/REST caller explicitly requests `move_original`. Existing files are never overwritten; collisions receive a numeric suffix. Watching is nonrecursive. Each start sweeps existing files again.

`settle_delay_ms` defaults to 500 and accepts 50–60000. The worker waits for a stable file size and ignores unchanged duplicate events during that run. A processing or write failure increments `failed`, emits an error, and retains the source; later PDFs can still succeed. Success receipts contain input/output SHA-256 hashes and the workflow report. Failure receipts contain the error and input hash; `.error` stores receipts, not relocated PDFs.

Watchers live in the owning process. Desktop commands, its local REST gateway, and its MCP endpoint share one manager; separate processes have separate managers. Saved desktop configurations require an explicit start after relaunch. No network request is made unless a CLI caller supplies `--webhook`.

Explicit CLI webhooks retain the completion fields `success`, `operation`, `file`, and `error`, adding `output` and `receipt` paths. Delivery has a 30-second timeout; HTTP/network failures are logged separately from PDF processing results.

Optional `name_template` enables [dynamic naming](/reference/output-naming/) from input names, UTC dates, SHA-256, run indexes and PDF Info fields. CLI uses `--name-template`. Receipts reference the final collision-resolved output path.

### CLI

```bash
paperpilot watch --directory /local/inbox --recipe preset-legal-discovery --settle-delay-ms 500
# Add --once to process existing PDFs and exit; any failed file yields a nonzero exit code.
```

### MCP JSON-RPC

```json
{
  "name": "watcher_start",
  "arguments": {
    "dir": "/local/inbox",
    "recipe": "preset-legal-discovery",
    "settle_delay_ms": 500
  }
}
```

Returns `watcher_id`, `status`, and an actual runtime snapshot. Keep `watcher_id` for listing/stopping.

### REST API

```bash
curl -s -X POST http://127.0.0.1:7823/v1/automation/watchers \
  -H 'Content-Type: application/json' \
  -d '{"dir":"/local/inbox","recipe":"preset-legal-discovery","settle_delay_ms":500}'
```

Returns HTTP 201 after registration, HTTP 400 for invalid configuration. Optional REST fields are `output_dir`, `error_dir`, and `move_original`.

### Dynamic filenames

CLI accepts `--name-template`; MCP and REST accept optional `name_template`, for example `{input_stem}-{date:%Y%m%d}-{hash:8}-{index:04}.pdf`. Templates are validated before registration. Input-specific metadata errors retain the source and write a failure receipt; later files continue. Indexes count attempts within the current watcher run. Existing output files receive numeric collision suffixes, and receipt names track the actual published filename. Explicitly archived originals retain their original names. See [all supported tokens and constraints](/reference/output-naming/).
