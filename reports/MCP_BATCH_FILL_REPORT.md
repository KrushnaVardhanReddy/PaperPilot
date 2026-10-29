# Task 4.7.9: native batch-fill interfaces

The existing CLI mail-merge orchestration now lives in `paperpilot-pdf::operations::batch_fill`. CLI, MCP `batch_fill` and REST `POST /v1/automation/fill-batch` delegate to this runner and the existing shared form engine. No PDF implementation is duplicated.

Requests accept exactly one CSV/JSON path or inline JSON array, optional field-map path, date override, naming template and editable mode. Inline receipts identify `inline_json` and hash the serialized record bytes without persisting a temporary input path. Outputs and the batch receipt never replace existing files.

Row failures continue processing: MCP sets `isError` and REST returns HTTP 207, retaining the complete receipt with each successful output and row error. Global configuration errors fail before generating outputs. The CLI contract remains compatible.

Validated on Windows GNU with real synthesized forms and the committed desktop automation form fixture:

```text
cargo check -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway -j 2 --offline
cargo test -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway --test test_fill_batch --test batch_fill -j 2 --offline -- --test-threads=2
```

15 tests passed: 10 existing CLI cases (including 1,000 readable flattened PDFs), 3 MCP and 2 REST cases. They cover actual editable field values, flattened page drawing resources, partial failure, file/inline sources, validation, persistent receipt equality and output preservation. The documentation build passed (94 pages, 81 handbook tools). `git diff --check` passed. `graphify-rs build .` was attempted; the optional executable is unavailable.

The byte-only browser API and desktop batch UI are outside this filesystem interface task. No full workspace, packaging or release validation is claimed. Local compiler/linker helpers are not committed.
