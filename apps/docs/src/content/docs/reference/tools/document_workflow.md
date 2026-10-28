---
title: "document_workflow"
description: "Execute local TOML or JSON document recipes with runtime conditions and per-stage outcomes."
sidebar:
  order: 3
---

### What it does

Runs a version-1 recipe through one shared in-memory PDF engine. Existing JSON
`SavedWorkflow` files and presets remain accepted. TOML `.ppflow` recipes add
runtime conditions, explicit dependencies and no-match handling. Execution is
local; the runner never downloads models, interpolates environment variables,
or sends document bytes to a service.

### CLI

```bash
paperpilot run examples/workflows/conditional-invoice.ppflow invoice.pdf -o processed.pdf --json
paperpilot workflow run --recipe invoice.ppflow --input invoice.pdf --output processed.pdf --json
paperpilot workflow inspect --recipe invoice.ppflow --json
paperpilot workflow list --json
```

Specify exactly one of `--preset` or `--recipe` with `workflow`. The `run`
shortcut takes the recipe and input as positional arguments. Input and output
must be different files. Successful CLI output is published atomically; a
recipe failure does not replace an existing output. `--json` prints exactly one
object; its `workflow` field contains the execution report. Failures exit nonzero.
Inspection lists parameter names without exposing their values.

### MCP JSON-RPC
```json
{
  "name": "document_workflow",
  "arguments": {
    "input": "invoice.pdf",
    "output": "processed.pdf",
    "recipe": "examples/workflows/conditional-invoice.ppflow"
  }
}
```

### REST API
```bash
curl -X POST http://127.0.0.1:7823/api/v1/workflow/run-json \
  -H 'Content-Type: application/json' \
  -d '{"input":"invoice.pdf","output":"processed.pdf","recipe":{"name":"Conditional invoice","steps":[{"id":"large","operationId":"pdf_compress","condition":{"if_page_count_gt":50},"params":{"quality":"medium"}}]}}'
```

### Recipe format

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

`[[step]]` / `tool` are aliases for `[[stage]]` / `op`. JSON accepts `steps`
and `operationId` (also `operation_id`). Existing IDs, `updatedAt`, labels,
icons, `isPreset`, and `teamId` remain compatible. A missing workflow ID defaults
to its name when parsed. Version defaults to `"1"`. Stage IDs must be unique and
nonempty; recipes contain 1–256 stages. Unknown schema fields are rejected.

### Conditions

Conditions inspect the **actual PDF entering that stage**, after earlier changes
or the explicitly selected dependency. Supplied tests are combined with AND.

| Field | Meaning |
| --- | --- |
| `if_scanned = true` | At least one page paints an image and has no extractable non-whitespace text. |
| `if_scanned = false` | Inverts that heuristic, enabling a complementary branch. |
| `if_page_count_gt = 50` | Page count is strictly greater than 50. |
| `comparator = "Greater", value = 3` | Page-count comparison; also `GreaterOrEqual`, `Equal`, `Less`, `LessOrEqual`. |

Scan detection includes image XObjects, nested Form XObjects, inherited page
resources and mixed text/image documents. Blank pages and unused images in
resources do not count as scans. This heuristic is not a classifier: an OCRed
scan with a text layer normally evaluates false. Invalid PDF structures or text
inspection failures return errors instead of guessing. Inspection bounds Form
depth to 32, inherited resource depth to 64, and content to 16 MiB per inspected
page/Form stream.

| `on_no_match` | Result |
| --- | --- |
| `skip` (default) | Record `skipped`; pass the stage input unchanged to dependants. |
| `archive_original` | Stop the workflow and return the exact original input bytes to the requested output. Remaining stages are `not_run`; no archive directory is created. |
| `error` | Record `failed`; abort without publishing a partial PDF. |

Empty conditions, incomplete comparator/value pairs and `on_no_match` without a
condition are invalid.

### Dependencies and output

Without `depends_on`, a stage consumes the previous **declared** stage (the
first consumes the input). `depends_on = ["stage-id"]` consumes that stage's
artifact, enabling fan-out and stable topological execution. Missing IDs and
cycles fail before execution. The final declared stage selects the output.
Multiple dependencies/fan-in require an explicit artifact merge and are rejected
in this version. Conditions on separate stages are evaluated independently;
they are not a paired IF/ELSE UI node. Use the same dependency when both tests
must inspect the same source artifact.

### Supported operations and parameters

The `pdf_` prefix is optional for the operations below.

| Operation | Accepted parameters |
| --- | --- |
| `watermark` | `text`, `opacity` (0–1), `angle` (−360–360) |
| `compress` | `quality` or `level`; low/screen, medium/ebook, high/prepress, integer 1–100; aggressive maps to low |
| `rotate` | `angle`: 0, 90, 180, 270 |
| `page_numbers` | `position` (top/bottom left/center/right), `format` |
| `bates` | `prefix`, `start` (u32), `padding` (0–32) |
| `remove_blank` | `sensitivity` (0–100) |
| `flatten`, `linearize` | None |
| `sanitize` | Boolean `remove_metadata`, `remove_attachments`, `remove_scripts`, `remove_thumbnails`, `remove_private_data` |
| `encrypt` | Nonempty `password` or `user_password`; optional `owner_password`; final declared stage only |
| `ocr` | None; native OCR-enabled build with locally installed models required |
| `filter_page_count` | No parameters; requires a condition; passes matching input through |

Unknown operations and parameter names/types fail before any stage executes,
even if their conditions would skip them. Redaction is explicitly unsupported
by this workflow runner: the old silent pass-through has been removed. The
`redact-protect` preset needs explicit supported redaction/password configuration
and now fails rather than claiming protection. Encryption really encrypts the
output and must be terminal, with no dependent stages. Reports exclude parameter values/passwords.

OCR uses the existing native operation. Missing models, unavailable build
capabilities, or no recognized text produce a failure. There is no automatic
model download. Scan detection can recognize structures the current OCR engine
cannot process; those inputs fail explicitly. WASM builds can skip OCR stages on
text PDFs, but matching OCR stages require a native OCR-enabled build.

### Other interfaces and reports

- MCP `document_workflow` / `pdf_document_workflow`: `input`, `output`, and exactly
  one `preset` or `recipe` (local recipe path, raw JSON, or multiline TOML).
  Success returns the report in `data`; execution errors include it in error data.
- REST `POST /api/v1/workflow/run-json`: `input`, `output`, and one `preset` or
  `recipe` (JSON object or TOML/JSON string). Success/error JSON includes `report`.
- REST multipart `POST /api/v1/workflow/run`: recipe accepts TOML or JSON;
  successful PDF responses include executed/skipped counts and duration headers.
- WASM `WasmPdfEngine.workflow(bytes, recipe)`: uses the same parser and runner,
  returning bytes or an error. It also accepts preset IDs.

Reports contain total/completed/skipped counts, output byte length, elapsed time,
`archived_original`, an optional error and per-stage `completed`, `skipped`,
`failed` or `not_run` statuses with reasons and timings. Output length is zero
on engine failure. CLI `--json` is a machine-readable result; generic persistent
recipe audit receipts remain a separate task.

Folder/schedule/webhook triggers, `[output]` routing, retries, secret interpolation,
multi-artifact joins and desktop IF/ELSE editing are separate tasks. Their schema
fields are rejected instead of silently ignored. Existing standalone tools
continue to expose their own broader parameter sets.
