# Spec 030: Web & Embed Batch Multi-File Processing & Interactive Tool Parameter Unit Tests

## Status
Approved

## Context
In Phase 5, the PaperPilot web distribution layers were significantly enhanced:
1. `apps/embed/src/Widget.svelte` (Embedded Widget / `embed.js`):
   - Added interactive parameter configuration panels for:
     - `extract`: custom page numbers and ranges (e.g. `1`, `3-5`, `2, 4`).
     - `delete`: custom pages to delete (e.g. `1`, `2-4`) executed in reverse index order.
     - `split`: page carve-outs (e.g. `1-2`, `3-5`).
     - `rotate`: custom rotation angle (`90°`, `180°`, `270°`) and target pages (`all` or specific pages).
     - `watermark`: custom watermark text stamp (`CONFIDENTIAL`, etc.).
   - Added **batch multi-file processing**: non-merge operations (`extract`, `delete`, `rotate`, `watermark`, `split`, `reorder`, `compress`) now process every uploaded document individually and generate dedicated download cards for each output file.
   - Built standalone bundle at `apps/embed/dist/embed.js` and synced to `apps/web/public/embed.js`.

2. `apps/web/src/views/OperationsView.svelte` (PaperPilot WASM Engine Playground):
   - Migrated single-file drop limitations (`multiple={false}`) to multi-file batch queues (`multiple={true}`) across `split`, `rotate`, `compress`, `encrypt`, `watermark`, `delete`, `extract`, and `reorder`.
   - Updated client handlers to iterate over all files in the batch, invoke `wasmPdfClient`, and trigger staggered file downloads with file counts in the action buttons.

## Objectives
1. Add dedicated Playwright component/E2E unit tests in `apps/embed/tests/` covering:
   - Tool selection and interactive config panel opening (`extract`, `delete`, `rotate`, `watermark`, `split`).
   - Custom page range inputs and validation.
   - Batch multi-file upload & download cards generation for non-merge tools.
   - Proper single-output aggregation for `merge`.
2. Add dedicated Playwright unit tests in `apps/web/tests/` covering:
   - Multi-file dropzones in `OperationsView.svelte`.
   - Batch execution for `compress`, `rotate`, `extract`, `delete`, and `watermark`.
   - Multi-file removal controls (`✕`) and button label reactivity.
3. Verify that 100% of tests pass cleanly in both `apps/embed` and `apps/web`.
4. Output a verified test report at `reports/WEB_AND_EMBED_BATCH_UNIT_TESTS_REPORT.md`.

## Deliverables
- `apps/embed/tests/widget_batch_and_config.spec.ts`
- `apps/web/tests/operations_batch_processing.spec.ts`
- `reports/WEB_AND_EMBED_BATCH_UNIT_TESTS_REPORT.md`
- `wiki/24-Web-And-Embed-Batch-Testing.md`
