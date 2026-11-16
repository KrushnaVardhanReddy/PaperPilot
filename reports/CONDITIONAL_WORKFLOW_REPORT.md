# Conditional Workflow Engine — Task 4.7.6 / 8.2.3

Validation date: October 28, 2026 (Asia/Hong_Kong).
Branch: `feat/conditional-pipeline-engine-task-4-7-6`.
Base: `origin/feature/dev` at `0f26149`, after PRs #292 and #293 were merged.

## Result and implementation

Added version-1 TOML `.ppflow` parsing alongside legacy JSON, runtime scan and
page-count predicates, explicit no-match behavior, deterministic dependency
execution and stage-level outcome reports. The CLI shortcut is:

```bash
paperpilot run examples/workflows/conditional-invoice.ppflow invoice.pdf -o processed.pdf --json
```

The existing `workflow run`, direct `workflow --input ...`, preset listing and
inspection remain available. JSON mode emits one response, including a workflow
report on execution failure. Output publication is atomic on the CLI, preserving
existing outputs on failure; input/output aliases are rejected. Inspection shows
parameter names without exposing values.

Core types and the pure parser live in `paperpilot-core`. The only execution
implementation lives in `paperpilot-pdf/src/operations/workflow.rs`; CLI, MCP,
REST and WASM call it. WASM's divergent dispatch loop was removed. Timing uses
`web-time` so the shared runner does not call the unsupported browser
`std::time::Instant` implementation. Artifacts are released when their last
consumer finishes, retaining only live dependencies and the final output.

Unknown operations and parameters now fail before execution, including when a
condition would skip them. Workflow redaction explicitly fails rather than
reporting success with unchanged bytes. Encryption requires a nonempty password,
really encrypts, and must be the terminal declared stage without dependants. OCR delegates to the
existing native local-model operation; missing capabilities/models and empty
recognition results return errors instead of fallback success.

## Automated verification

All Cargo commands use `-j 2`. Tests use genuine dynamically generated PDFs with
complete page trees, text fonts and raster image data; no mocked responses or
placeholder PDF bytes are used.

Final verification results are recorded below after the last source change:

| Target | Result |
| --- | --- |
| `cargo test -p paperpilot-core --lib workflow:: -j 2` | 6 passed |
| PDF conditional workflow integration | 12 passed |
| CLI conditional workflow integration | 7 passed |
| MCP conditional workflow integration | 2 passed |
| REST conditional workflow integration | 2 passed |
| WASM wrapper integration on the native host | 2 passed |
| Existing CLI batch-fill regression (including 1,000 PDFs) | 10 passed |

All 41 targeted tests passed. Coverage includes page-count boundaries, complementary predicates, scan/text/
blank/mixed documents, nested Form images and inherited resources, conditions
after prior operations, explicit dependency artifacts and forward order,
archive-original byte equality, failed/not-run reports, unsupported operations,
invalid parameters, malformed/encrypted input, actual encryption/password
verification, usable legacy presets, legacy CLI/JSON compatibility, single JSON
stdout and existing-output preservation. The cyclic Form inspection regression
ensures malformed content returns an error without unbounded recursion.

MCP tests call the public tool handler; REST tests invoke the actual JSON
handlers directly. They do not claim network transport or browser testing.

Reproduce the integration suites:

```bash
cargo test -p paperpilot-pdf -p paperpilot-mcp -p paperpilot-gateway --test conditional_workflow -j 2 -- --test-threads=2
cargo test -p paperpilot-cli --test test_conditional_workflow -j 2 -- --test-threads=2
cargo test -p paperpilot-wasm --test conditional_workflow -j 2
```

## Independent real OCR and PDF evidence

Generated a public synthetic invoice using ReportLab, rasterized it with
PyMuPDF, and embedded the raster as a JPEG image into a complete scan PDF.
Executed the example recipe using the existing locally installed native PP-OCR
models. No model download or document upload occurred.

- Text input: OCR and large-document compression skipped; watermark completed.
- JPEG scan: OCR and watermark completed; compression skipped (one-page input).
  The local example OCR stage took approximately 10.5 seconds; this is an example
  run, not a benchmark or performance guarantee.
- Both outputs were independently reopened by pypdf and PyMuPDF. Each retained
  one page, the invoice number/total and the `PROCESSED` watermark. OCR output
  extracted `INVOICE`, `12345`, `TOTAL`, `100`, and `DOLLARS`.
- Rendered outputs were visually inspected; scan content and watermark remained
  visible and correctly placed.
- An ASCII85/Flate image scan exposed a limitation in the existing OCR image
  decoder: no text was recognized. The workflow correctly returned failure,
  marked following stages `not_run`, and created no output PDF. This limitation
  is not represented as an OCR pass.

Inputs, outputs, independent SHA-256/text checks and execution JSON are in
[`conditional-workflow-validation/`](conditional-workflow-validation/).
[`verification.json`](conditional-workflow-validation/verification.json) records
the exact output hashes. All documents are synthetic and contain no private data.
The unsupported-scan failure is recorded as `scan-result.json`.

## Documentation and remaining boundaries

- Dedicated tool reference, CLI guide, regenerated handbook (77 entries),
  example recipes, Spec 136, HANDOFF and Phase 4/8 trackers updated.
- Documentation site built successfully: 90 pages, 89 indexed. Astro telemetry
  was disabled for the build.
- Graph refresh was attempted with `graphify-rs build .`; the executable is not
  installed and no local graph exists. Sources and tests were inspected directly.
- Rust formatting / whitespace checks passed.

This implements per-stage conditions and single-artifact dependencies, with
fan-out and deterministic ordering. Multiple-dependency joins, trigger daemons,
retries, output-folder routing, secret interpolation and generic persisted
recipe audit receipts remain separate tasks; unsupported fields are rejected.
`archive_original` returns original bytes to the requested output; it does not
create an archive folder. Conditions are independent runtime tests, not paired
desktop IF/ELSE nodes. The final declared stage selects the output.

Scan detection is an explicit local heuristic, not a learned classifier. The
existing OCR decoder does not handle every image encoding or layout; empty
results now fail. This report does not claim an exhaustive OCR regression,
browser execution, cross-compilation, full workspace/desktop test run or remote
CI pass. Task 3.4.11 adds the desktop branching UI after this engine is reviewed.
