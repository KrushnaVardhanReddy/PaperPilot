# Task 5.9.80: Penta-Scaffold UI Automation & Universal Audit Report Visualizer

## Status
- Spec: [Spec 100](file:///home/krushna/Project/PaperPilot/specs/100-penta-scaffold-ui-automation/spec.md)
- Target: Wave 2 (`v0.7.1`) Developer Ergonomics & UI Parity
- Status: ✅ Completed (Merged PR #240, commit `5ca9e5c`)

---

## Objectives & Work Items

### 1. `penta-scaffold` Code Generation (`tools/penta-scaffold/src/generator.rs`)
- [x] Add `append_desktop_args_builder(&ScaffoldArgs, &str) -> Result<()>` in `generator.rs`:
  - Locate `switch (toolId)` in `apps/desktop/src/lib/services/operationArgsBuilder.ts`.
  - Insert `case '{name_snake}':` with proper `input` and `output` filename derived from `args.output_format`.
  - Handle dry-run mode and EOF newlines cleanly.
- [x] Add test generation in `apps/desktop/src/lib/services/operationArgsBuilder.test.ts`.
- [x] Verify `cargo test -p penta-scaffold -j 2` passes.

### 2. Universal Audit & Report Visualizer (`apps/desktop`)
- [x] Create `apps/desktop/src/lib/components/ui/AuditReportViewer.svelte`:
  - Parses JSON output from security, validation, and extraction operations.
  - Displays summary status (Valid / Detected / Warning), counts, and expandable detail rows.
  - Provides "Copy JSON" and "Open in File Explorer" actions.
- [x] Integrate into `apps/desktop/src/lib/components/layout/OperationsPanel.svelte`:
  - When `activeTool.outputFormat === '.JSON'` succeeds, render `<AuditReportViewer />`.

### 3. Retroactive Wiring for Recent Tools
- [x] Wire `pdf_validate_signatures`, `document_pii_scan`, and `pdf_extract_tables` in `operationArgsBuilder.ts`.
- [x] Add unit test assertions in `operationArgsBuilder.test.ts`.

### 4. Verification
- [x] `cargo check -p penta-scaffold -j 2`
- [x] `pnpm --dir apps/desktop test`
- [x] `pnpm --dir apps/desktop check`
