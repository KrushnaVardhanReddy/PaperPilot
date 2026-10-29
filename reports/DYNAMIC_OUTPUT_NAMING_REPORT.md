# Task 4.7.5: dynamic recipe and watcher output names

One shared `OutputNameTemplate` renderer supports input basename/stem/extension, UTC date formatting, source SHA-256 prefixes, padded attempt indexes and six PDF Info fields. It rejects unsafe basenames, reserved Windows names, invalid date formats and unsupported tokens. PDF metadata is read from the exact source buffer, with PDF string decoding and explicit errors for missing fields or encrypted inputs requiring metadata.

CLI recipes and watchers, MCP `recipe_run` / `document_workflow` / `watcher_start`, and REST JSON workflows/watchers accept the optional template. Named recipe output uses a directory and never overwrites files. Standard explicit-output behavior is retained without a template. All PDF processing still uses the existing shared workflow engine.

The naming-aware receipt runner reads source bytes once for naming and processing, stores the actual published output and captures collisions as failures. Watchers retain numeric collision suffixes, now name receipts after the actual published file, and keep original archive filenames. Indexes reset per watcher run and include successful and failed processing attempts. Existing public `WatchConfig`, `start` and `handle_watch` callers remain supported.

This branch is based on receipt PR #310 to reuse its persisted publication wrapper. The naming feature is a separate commit/PR; #310 should merge first. Batch interface PR #311 is independent.

Validation uses complete synthetic PDF fixtures and actual PDF Info dictionaries, CLI subprocesses, real managed watchers, MCP handlers and REST routers. Coverage includes text preservation, hash/output/receipt agreement, UTF-16 metadata, missing-metadata continuation, invalid patterns, collisions, source retention and archive naming.

59 tests across 15 suites passed on Windows GNU:

```text
cargo test -p paperpilot-pdf -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway --test dynamic_output_naming --test test_dynamic_naming --test dynamic_naming --test desktop_automation --test test_watch --test watcher_lifecycle --test execution_receipts --test test_receipt_history --test conditional_workflow --test test_conditional_workflow -j 2 --offline -- --test-threads=2
```

The opt-in webhook regression uses a real HTTP listener on 127.0.0.1; it passed when local loopback connections were allowed (the restricted sandbox initially timed out). The final documentation build passed: 97 pages, 83 regenerated handbook tools. `git diff --check` passed. Windows path comparisons in new tests account for platform separators and canonical paths.

Compilation of all CLI, MCP and REST targets passed with `cargo check -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway --all-targets -j 2 --offline`.

The optional `graphify-rs build .` command was attempted; the executable is unavailable. No desktop UI redesign, XMP/form-field naming, persistent watcher index, release packaging or whole-workspace validation is included.
