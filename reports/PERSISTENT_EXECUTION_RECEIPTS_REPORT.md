# Task 4.7.7 — Persistent execution receipts

Implemented on `feat/persistent-execution-receipts-task-4-7-7` for independent review into `feature/dev`. This work does not include the parked Spec 143/144 UI integration.

One native wrapper in `paperpilot-pdf::operations::receipts` delegates PDF processing to the existing pure workflow runner, publishes the output atomically and records terminal success/failure in a unique, synced JSON file. History storage is reserved before processing; persistence errors are surfaced. Records include job ID, UTC start/end, duration, actual source/output hashes and stage outcomes. Failed stages and failed output writes retain prior output files.

CLI `run` / `workflow run`, MCP `recipe_run` / `document_workflow`, and REST JSON workflows share this wrapper. Existing stage reports retain their shape with additive provenance. CLI `workflow history`, two MCP receipt query tools and local REST GET routes share one query implementation. Existing batch and watcher receipts are read through normalized projections, without migration. New CLI processes and new server instances can read prior results.

Queries require an explicit directory, bound list results (1–1000), omit large details from list responses and expose details through get. Corrupt receipts fail visibly; symlink receipts and files above 16 MiB are rejected. Raw PDF bytes and recipe parameter maps are not stored. Reports can contain stage inspection reasons. No network traffic, global registry or database migration is introduced.

Scope: parsed file-based workflow executions and existing batch/watcher terminal receipts. Byte-only/WASM/multipart processing remains unchanged. Interrupted jobs without a completed receipt are not represented as completed history. This does not implement scheduling, retry policy or durable in-flight job resumption.

Validation: **33 real-PDF integration/regression tests passed** across core (18), CLI (9), MCP (3) and REST (3). `cargo check -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway -j 2 --offline` passed. Handbook regenerated (83 tool pages), documentation build passed (96 pages), and `git diff --check` passed. All Cargo commands use `-j 2`; fixtures are complete synthetic PDFs or existing genuine synthetic form fixtures. Core tests exercise parsed PDFs, hashes, stage failure, filesystem write failure, corrupt history, password exclusion and a real watcher receipt. Wrapper tests exercise actual CLI processes, editable batch form values, MCP execution and REST routes.

```text
cargo test -p paperpilot-pdf -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway --test execution_receipts --test test_receipt_history --test conditional_workflow --test test_conditional_workflow -j 2 --offline -- --test-threads=2
```

The local `graphify-rs` executable is unavailable. No new full-workspace or release-packaging validation is claimed.
