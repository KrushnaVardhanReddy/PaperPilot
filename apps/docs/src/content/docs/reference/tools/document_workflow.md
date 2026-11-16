---
title: "document_workflow"
description: "Reference, CLI, MCP, REST API, and WASM commands for document_workflow multi-step pipelines."
sidebar:
  order: 3
---

**Status**: ✅ Verified  |  **Format**: PDF + Recipe (.ppflow / TOML / JSON / Preset) $\to$ PDF + Execution Report

### What it does

Runs a declarative version-1 document workflow recipe through one unified in-memory PDF engine.
Accepts custom TOML `.ppflow` files, JSON recipe definitions, or built-in system presets (e.g. `preset-legal-discovery`, `preset-clean-scan`).

- **Unified Multi-Stage Execution**: Supports compound document transformations without intermediate on-disk tempfile writing.
- **Runtime Conditions & Branching**: Evaluates dynamic conditions (scanned checks, page counts, form fields, text matching, byte size) on the actual document entering each stage.
- **Topological Dependencies**: Supports declared stage dependencies (`depends_on = ["stage-id"]`) for fan-out execution.
- **Universal Interface Support**: Available across CLI (`paperpilot workflow run`), MCP (`document_workflow`), REST API (`POST /api/v1/workflow/run-json` and multipart `POST /api/v1/workflow/run`), and WebAssembly (`WasmPdfEngine.workflow`).
- **Audit Trails**: Disk-based executions automatically persist terminal receipts (`.paperpilot-receipts/run-*.json`) queryable via [`execution_receipt_list`](/reference/tools/execution_receipt_list/) and [`execution_receipt_get`](/reference/tools/execution_receipt_get/).

---

### CLI

#### Run Workflow
```bash
# Execute workflow using preset or custom recipe
paperpilot workflow run \
  --recipe ManaulTestingFixtures/document_workflow/Input/audit_pipeline.ppflow \
  --input ManaulTestingFixtures/document_workflow/Input/incoming_document.pdf \
  --output out/audit_pipeline_out.pdf \
  --json

# Execute using built-in preset
paperpilot workflow run \
  --preset preset-legal-discovery \
  --input document.pdf \
  --output out/legal_out.pdf \
  --json
```

**Output**:
```json
{
  "success": true,
  "operation": "Workflow",
  "error": null,
  "workflow": {
    "workflow_id": "Audit & Watermark Pipeline",
    "workflow_name": "Audit & Watermark Pipeline",
    "job_id": "run-HrUGVW",
    "output": "out/audit_pipeline_out.pdf",
    "receipt_path": "out/.paperpilot-receipts/run-HrUGVW.json",
    "output_bytes_len": 1860,
    "duration_ms": 2,
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

#### Inspect and List Workflows
```bash
# List available workflow presets
paperpilot workflow list --json

# Inspect stages and parameters of a recipe or preset
paperpilot workflow inspect --recipe audit_pipeline.ppflow --json
```

#### CLI Parameters
| Flag | Description |
| --- | --- |
| `--recipe <PATH>` | Path to custom TOML (`.ppflow`) or JSON workflow file (mutually exclusive with `--preset`) |
| `--preset <NAME>` | Name of built-in preset (e.g. `preset-legal-discovery`, `preset-clean-scan`) |
| `--input <PATH>` | Input PDF file |
| `--output <PATH>` | Output destination file path (or destination directory when `--name-template` is passed) |
| `--name-template <TEMPLATE>` | Output filename template (e.g., `{input_stem}-{hash:8}.pdf`); preserves existing files |
| `--json` | Format output as structured JSON |

---

### MCP JSON-RPC

Call tool `document_workflow` (or alias `pdf_document_workflow`):

```json
{
  "name": "document_workflow",
  "arguments": {
    "input": "incoming_document.pdf",
    "output": "processed.pdf",
    "recipe": "audit_pipeline.ppflow"
  }
}
```

**MCP Response**:
```json
{
  "success": true,
  "message": "Workflow executed successfully.",
  "output_path": "processed.pdf",
  "data": {
    "workflow_id": "Audit & Watermark Pipeline",
    "job_id": "run-abc123",
    "receipt_path": ".paperpilot-receipts/run-abc123.json",
    "steps_total": 2,
    "steps_completed": 2,
    "steps_skipped": 0,
    "output_bytes_len": 1860,
    "duration_ms": 3,
    "stages": [
      {"id": "step_1", "operation_id": "pdf_watermark", "status": "completed", "duration_ms": 1},
      {"id": "step_2", "operation_id": "pdf_page_numbers", "status": "completed", "duration_ms": 1}
    ]
  }
}
```

---

### REST API

#### 1. File-Based JSON Runner
`POST /api/v1/workflow/run-json`

```bash
curl -X POST http://127.0.0.1:7823/api/v1/workflow/run-json \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "invoice.pdf",
    "output": "processed.pdf",
    "preset": "preset-legal-discovery"
  }'
```

#### 2. Streaming Multipart Endpoint
`POST /api/v1/workflow/run`

Runs directly against uploaded document bytes without local filesystem history:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/workflow/run \
  -F "file=@invoice.pdf" \
  -F "recipe=@conditional-invoice.ppflow" \
  -o processed.pdf
```

#### 3. Preset Inspection
`GET /api/v1/workflow/presets`

Retrieves all pre-registered workflow presets and their stage metadata:
```bash
curl -s http://127.0.0.1:7823/api/v1/workflow/presets
```

---

### WebAssembly (WASM)

In browser or Edge worker environments, workflows execute using `WasmPdfEngine`:

```typescript
import init, { WasmPdfEngine } from 'paperpilot-wasm';

await init();

// Retrieve list of built-in preset definitions
const presetsJson = WasmPdfEngine.workflow_presets();

// Execute a recipe or preset against in-memory PDF Uint8Array
const inputBytes = new Uint8Array(...);
const recipeTomlOrJson = `...`; // Or pass preset string e.g. "preset-legal-discovery"

const outputBytes = WasmPdfEngine.workflow(inputBytes, recipeTomlOrJson);
```

---

### Recipe Format (.ppflow)

```toml
name = "Conditional invoice processing"
version = "1"

[[stage]]
id = "ocr-if-needed"
op = "ocr"
condition = { if_scanned = true }
on_no_match = "skip"

[[stage]]
id = "compress-large"
op = "compress"
condition = { if_page_count_gt = 50 }
[stage.params]
level = "aggressive"

[[stage]]
id = "label"
op = "watermark"
[stage.params]
text = "PROCESSED"
opacity = 0.3
```

#### Stage Directives
- `id`: Nonempty, unique stage identifier.
- `op` / `tool`: Operation name (e.g. `watermark`, `compress`, `rotate`, `page_numbers`, `bates`, `remove_blank`, `flatten`, `linearize`, `sanitize`, `encrypt`, `ocr`).
- `depends_on`: Optional list of stage IDs whose output this stage consumes (default: consumes previous declared stage).
- `on_no_match`: Action taken if condition evaluates to false:
  - `skip` (default): Record stage as skipped; pass previous artifact unchanged.
  - `archive_original`: Halt workflow and publish the original input unmodified.
  - `error`: Abort execution immediately with failure.

#### Condition Selectors
| Condition | Description |
| --- | --- |
| `text_contains = "term"` | Case-insensitive search on extractable text |
| `has_form_fields = true` | Validates presence of interactive AcroForm fields |
| `is_signed = true` | Validates digital signature presence |
| `file_size_bytes = { between = [min, max] }` | Matches file size against numeric bounds |
| `if_scanned = true` | Heuristic detecting scanned pages (image present with no extractable text) |
| `if_page_count_gt = N` | Matches if total document pages strictly exceed N |
| `all`, `any`, `not` | Logical combinators enclosing nested conditions |

---

### Supported Operations & Parameters

| Operation | Parameters |
| --- | --- |
| `watermark` | `text`, `opacity` (0.0–1.0), `angle` (-360–360) |
| `compress` | `quality` or `level` (`low`/`medium`/`high` or 1–100) |
| `rotate` | `angle`: 0, 90, 180, 270 |
| `page_numbers` | `position` (`bottom-right`, `bottom-center`, etc.), `format` |
| `bates` | `prefix`, `start` (u32), `padding` (0–32) |
| `remove_blank` | `sensitivity` (0–100) |
| `flatten`, `linearize` | None |
| `sanitize` | Boolean flags: `remove_metadata`, `remove_attachments`, `remove_scripts`, `remove_thumbnails`, `remove_private_data` |
| `encrypt` | `password` or `user_password`; optional `owner_password` (terminal stage only) |
| `ocr` | Native OCR build with local models installed |

---

### Known Limitations & Troubleshooting

- **Mutual Exclusivity**: You must provide exactly one of `recipe` or `preset`. Supplying both or neither will result in a validation error.
- **Terminal Encryption**: The `encrypt` operation must be the final declared stage in the workflow. Dependent stages cannot operate on encrypted PDF streams.
- **OCR Engine Dependencies**: OCR stages require a native binary with compiled OCR models. If models are missing or unconfigured, the stage will fail explicitly (models are never auto-downloaded over the network).
- **Redaction Unsupported in Workflows**: Redaction (`pdf_redact`) requires explicit coordinates or pattern review and is intentionally unsupported as an automated workflow step.
