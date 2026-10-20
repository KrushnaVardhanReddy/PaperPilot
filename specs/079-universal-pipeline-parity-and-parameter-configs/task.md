# Task 5.9.68: Universal Pipeline Parity & Comprehensive Parameter Configuration

## Overview
Polish and expand the multi-step automation pipeline across Desktop (`apps/desktop`) and Web Embed (`apps/embed`).

## Checklist
- [ ] In `apps/desktop/src/lib/state/pipeline.svelte.ts`:
  - Review all tools in `OPERATION_REGISTRY` to ensure complete, user-friendly `paramSchema` fields for every tool (`pdf_crop`, `pdf_bates`, `pdf_header_footer`, `pdf_redact`, `pdf_encrypt`, `pdf_watermark`, `pdf_split`, `pdf_extract_pages`, etc.).
  - Add missing tool definitions if any are missing from the 20 core desktop tools.
- [ ] In `apps/desktop/src/lib/components/pipeline/PipelineRunner.svelte`:
  - Handle intermediate directory outputs safely if split/extract tools are chained into single-file tools (pass first valid `.pdf` part).
  - Provide clear progress notifications and error toasts.
- [ ] In `apps/embed/src/api.ts`:
  - Enhance `runPipeline` step executor to handle all client-side operations: `rotate`, `watermark`, `extract_pages`/`extract`/`split`, `delete_pages`, `reorder_pages`, `compress`, `crop`, `flatten`, `encrypt`, `metadata`.
  - Handle parameter variations cleanly (e.g. `pages` vs `pageInput`, `angle` vs `degrees`).
- [ ] Automated tests:
  - Add Playwright E2E test in `apps/desktop/tests/e2e_ui.spec.ts` testing pipeline configuration panel interaction and pipeline execution.
  - Verify `npm --prefix apps/embed test` and `npm --prefix apps/desktop run check` pass 100%.
