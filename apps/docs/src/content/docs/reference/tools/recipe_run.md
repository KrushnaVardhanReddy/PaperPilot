---
title: "recipe_run"
description: "Reference, CLI, MCP, and REST API commands for recipe_run declarative recipe execution."
sidebar:
  order: 4
---

**Status**: ✅ Verified  |  **Format**: PDF + Recipe (.ppflow / TOML / JSON / Preset) $\to$ PDF + Execution Receipt

### What it does

Executes a validated declarative document recipe (`.ppflow`, TOML, JSON) or preset against a local PDF.
Uses the unified multi-step workflow engine and writes a durable terminal receipt (`.paperpilot-receipts/run-*.json`) beside the output file.

- **Unified Runner**: Executes the same underlying pipeline stages as [`document_workflow`](/reference/tools/document_workflow/), supporting runtime conditions, dependency branching, and skip heuristics.
- **Durable Receipts**: On completion, generates a cryptographic execution receipt including `job_id`, `receipt_path`, input/output SHA-256 hashes, and per-stage timings.
- **Dynamic Output Naming**: Supports opt-in pattern templating via `--name-template` / `name_template` (e.g. `{input_stem}-{hash:8}.pdf`). When used, `output` specifies the target directory.
- **Failure Preservation**: Existing output documents are never overwritten or corrupted if recipe execution fails midway. Execution failures include the partial stage report and receipt location in error data.

---

### CLI

Execute a recipe via `paperpilot run`:

```bash
# Execute recipe and output JSON report
paperpilot run \
  ManaulTestingFixtures/document_workflow/Input/audit_pipeline.ppflow \
  input.pdf \
  --output out/result.pdf \
  --json
```

**Output**:
```json
{
  "success": true,
  "operation": "Run",
  "error": null,
  "workflow": {
    "workflow_id": "Audit & Watermark Pipeline",
    "workflow_name": "Audit & Watermark Pipeline",
    "job_id": "run-w8jBIp",
    "output": "out/result.pdf",
    "receipt_path": "out/.paperpilot-receipts/run-w8jBIp.json",
    "output_bytes_len": 1829,
    "duration_ms": 1,
    "steps_total": 2,
    "steps_completed": 2,
    "steps_skipped": 0,
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
```

#### Dynamic Output Naming (Directory Mode)
```bash
paperpilot run \
  audit_pipeline.ppflow \
  input.pdf \
  --output out/ \
  --name-template "{input_stem}-{hash:8}.pdf" \
  --json
```

#### CLI Parameters
| Argument / Flag | Description |
| --- | --- |
| `<RECIPE>` | Path to a local TOML `.ppflow` recipe, JSON recipe file, or preset name |
| `<INPUT>` | Local input PDF document |
| `-o, --output <OUTPUT>` | Output destination file path (or destination directory when `--name-template` is set) |
| `--name-template <TEMPLATE>` | Output filename template (e.g., `{input_stem}-{date}.pdf`); preserves existing files |
| `--json` | Format result report as structured JSON |

---

### MCP JSON-RPC

```json
{
  "name": "recipe_run",
  "arguments": {
    "input": "/local/input.pdf",
    "recipe": "preset-legal-discovery",
    "output": "/local/out/result.pdf"
  }
}
```

**MCP Response**:
```json
{
  "success": true,
  "output": "/local/out/result.pdf",
  "receipt": {
    "workflow_id": "preset-legal-discovery",
    "job_id": "run-abc123",
    "receipt_path": "/local/out/.paperpilot-receipts/run-abc123.json",
    "steps_total": 3,
    "steps_completed": 3,
    "steps_skipped": 0,
    "duration_ms": 12,
    "stages": [
      {"id": "clean", "operation_id": "pdf_remove_blank", "status": "completed", "duration_ms": 4},
      {"id": "bates", "operation_id": "pdf_bates", "status": "completed", "duration_ms": 5},
      {"id": "flatten", "operation_id": "pdf_flatten", "status": "completed", "duration_ms": 3}
    ]
  }
}
```

#### MCP Dynamic Naming
Pass `name_template` and set `output` to an existing directory:
```json
{
  "name": "recipe_run",
  "arguments": {
    "input": "/local/input.pdf",
    "recipe": "/local/recipes/clean.ppflow",
    "output": "/local/out",
    "name_template": "{input_stem}-{date}.pdf"
  }
}
```

---

### REST API

#### Endpoint
`POST /api/v1/workflow/run-json`

#### Request
```bash
curl -X POST http://127.0.0.1:7823/api/v1/workflow/run-json \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "/local/input.pdf",
    "output": "/local/out/result.pdf",
    "preset": "preset-legal-discovery"
  }'
```

#### Response
```json
{
  "success": true,
  "output": "/local/out/result.pdf",
  "report": {
    "workflow_id": "preset-legal-discovery",
    "job_id": "run-X9yZ12",
    "receipt_path": "/local/out/.paperpilot-receipts/run-X9yZ12.json",
    "steps_total": 3,
    "steps_completed": 3,
    "steps_skipped": 0,
    "duration_ms": 14,
    "stages": [...]
  }
}
```

#### Multipart Byte Streaming
For memory-only processing without writing files or receipts to disk, use the multipart endpoint:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/workflow/run \
  -F "file=@input.pdf" \
  -F "recipe=@flow.ppflow" \
  -o processed.pdf
```

---

### Recipe Syntax Example

```toml
name = "Invoice Processing"
version = "1"

[[stage]]
id = "watermark-draft"
op = "watermark"
condition = { if_scanned = false }
[stage.params]
text = "PROCESSED"
opacity = 0.2

[[stage]]
id = "compress-if-large"
op = "compress"
condition = { if_page_count_gt = 5 }
[stage.params]
level = "medium"
```

---

### Known Limitations & Troubleshooting

- **Identical Input & Output Paths**: In-place replacement (`input == output`) is strictly forbidden to guarantee atomicity and guard against accidental document destruction.
- **Terminal Receipts**: Receipts are saved exclusively for disk-based runs (`run`, `workflow run`, `POST /api/v1/workflow/run-json`, MCP `recipe_run`). Streaming multipart endpoints do not persist filesystem receipts.
- **Audit History Retrieval**: Use [`execution_receipt_list`](/reference/tools/execution_receipt_list/) and [`execution_receipt_get`](/reference/tools/execution_receipt_get/) to inspect historical runs after process restarts.
