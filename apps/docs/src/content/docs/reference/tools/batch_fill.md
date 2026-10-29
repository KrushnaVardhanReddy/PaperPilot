---
title: "batch_fill"
description: "Local CSV/JSON mail merge through MCP and REST, with per-row receipts."
---

### What it does

Fills a local AcroForm PDF once per CSV/JSON record using the same core runner as [CLI fill-batch](/reference/tools/cli_fill_batch/). Text fields and checkboxes are supported. Outputs are flattened by default; `keep_editable: true` retains interactive fields. All inputs stay local.

Supply `template`, `output_dir` and exactly one of `data` (a local `.csv`/`.json` path) or `records` (an inline JSON array). Inline values may be strings, numbers, booleans or null. Nested values fail that row. Optional `field_map` is a local JSON file mapping PDF field names to input column names. Optional `date` overrides the UTC date as `YYYY-MM-DD`.

### CLI

```bash
paperpilot fill-batch --template /local/customer-form.pdf --data /local/customers.csv --output-dir /local/batch-001
```

### MCP JSON-RPC

```json
{"name":"batch_fill","arguments":{"template":"/local/customer-form.pdf","output_dir":"/local/batch-001","records":[{"customer":"Ada"},{"customer":"Grace"}],"name_template":"{index:04}-{row.customer}.pdf"}}
```

The tool returns the complete receipt as JSON text. Any failed row sets MCP `isError: true`, while preserving successful row outputs and all row errors. Invalid configuration returns an invalid-parameters error before output generation.

### REST API

```bash
curl -X POST http://127.0.0.1:7823/v1/automation/fill-batch \
  -H 'Content-Type: application/json' \
  -d '{"template":"/local/customer-form.pdf","output_dir":"/local/batch-002","data":"/local/customers.csv"}'
```

HTTP 200 means all rows succeeded; HTTP 207 means partial or complete row failure. Both return the full receipt, including totals, per-row output paths/hashes/errors and the persisted `receipt_path`. Invalid configuration or failed receipt persistence returns HTTP 400. Inline receipts have `data_source: "inline_json"`, `data: null` and the hash of the serialized records; file inputs retain their real data path and byte hash.

This native filesystem operation is available through CLI, MCP and REST. Browser byte-only APIs do not write batch folders.
