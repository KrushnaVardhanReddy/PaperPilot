# Conditional Recipe Execution Engine — Task 4.7.6 / 8.2.3

## Problem and outcome

The existing workflow runner executes a JSON list unconditionally. Unsupported
operations (including redaction/encryption) return the original PDF while being
counted as completed, and WASM duplicates a different dispatch loop. Users need
reusable offline recipes that test actual document properties before each stage
and report what happened accurately.

The implementation extends `SavedWorkflow`, adds a pure TOML/JSON parser and
graph validation, and uses one `paperpilot-pdf` runner across CLI, MCP, REST and
WASM. It preserves legacy JSON and usable presets, adds the `paperpilot run`
shortcut, and exposes stage-level outcomes.

## Acceptance criteria

- Version-1 `.ppflow` TOML, JSON and legacy `SavedWorkflow` parse without side
  effects. Stage/tool aliases bridge Phase 4 and Phase 8 terminology.
- Every supplied condition is evaluated against the stage's actual input:
  `if_scanned`, `if_page_count_gt`, and typed page-count comparators. Multiple
  predicates are ANDed. Complementary conditions can inspect a common dependency.
- The scan heuristic requires a painted raster image without extractable text
  on at least one page; blank documents and unused resources are not scans.
- `on_no_match` has explicit skip, archive-original and error semantics.
- Stable dependency order, source-artifact selection, missing ID/cycle checks,
  duplicate ID checks, nonempty stages and version checks run before operations.
- Unsupported operations and invalid parameters fail instead of passing through.
  Real terminal encryption requires a password. Redaction remains unsupported
  in this runner and fails explicitly.
- Reports preserve completed/skipped/failed/not-run stages, reasons and timing.
  Failed CLI execution does not publish a partial PDF or replace an existing one.
- Real generated PDF fixtures verify output pages, rotations, extracted text,
  scan/blank/mixed cases, encryption/decryption, file preservation and interface
  compatibility. No mocked operation results or placeholder PDF bytes.
- Tool reference, CLI guide, handbook, examples and phase trackers stay in sync.

## Defined scope

Omitted dependencies retain sequential behavior. Explicit single dependencies
enable fan-out and topological ordering; the final declared stage selects the
output. Multi-artifact joins require a future merge contract and are rejected.
`archive_original` returns the exact original bytes to the requested output and
stops; it does not create an archive directory. Independent conditions are not a
paired desktop IF/ELSE node.

OCR delegates to the existing native engine and requires local models. A build
without OCR returns an explicit capability error only if the OCR stage matches.
No models are downloaded; no secret interpolation or unexpected network requests
are introduced. An empty OCR result is a failure, avoiding fallback success.

Folder/schedule/webhook triggers, retries, output-directory routing, environment
secret interpolation, generic persisted audit receipts and the desktop branching
editor are separate tasks. Unknown recipe fields fail rather than implying those
features exist. Task 3.4.11 follows after this engine is reviewed.

See `apps/docs/src/content/docs/reference/tools/document_workflow.md` for the
complete schema and operation parameter contract, and
`reports/CONDITIONAL_WORKFLOW_REPORT.md` for verification evidence.
