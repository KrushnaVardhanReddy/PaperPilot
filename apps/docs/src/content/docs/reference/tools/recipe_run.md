---
title: "recipe_run"
description: "Execute a local recipe and persist its actual result and stage report."
---

### What it does

Runs a validated local TOML/JSON recipe or preset against a local PDF. Uses the same workflow engine as all other wrappers and writes a durable terminal receipt beside the output in `.paperpilot-receipts`. MCP success keeps the legacy `receipt` stage-report shape and adds `job_id` / `receipt_path`. Execution failures include the report and saved receipt location in error data. Existing output is preserved on execution failure. Same-input/output paths are rejected.

### CLI

```bash
paperpilot run flow.ppflow input.pdf --output out/result.pdf --json
```

### MCP JSON-RPC

```json
{"name":"recipe_run","arguments":{"input":"/local/input.pdf","recipe":"preset-legal-discovery","output":"/local/out/result.pdf"}}
```

### REST API

```bash
curl -X POST http://127.0.0.1:7823/api/v1/workflow/run-json -H 'Content-Type: application/json' -d '{"input":"/local/input.pdf","output":"/local/out/result.pdf","preset":"preset-legal-discovery"}'
```

Use `execution_receipt_list` / `execution_receipt_get` to retrieve history after process restart. For byte-only processing without filesystem history, use the existing pure runner or multipart endpoint.
