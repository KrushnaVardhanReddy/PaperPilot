# Task Checklist: Spec 082 — Universal "Export as Code / API / MCP Prompt" Workbench

## Phase 1: Core Engine & Pipeline Flow Integration (Phase 5.9.69D)
- [ ] Implement `apps/desktop/src/lib/utils/snippetGenerator.ts`
  - [ ] CLI argument formatter (`paperpilot workflow run ...`)
  - [ ] cURL generator (`curl -X POST http://localhost:8080/api/v1/workflow/run ...`)
  - [ ] MCP JSON RPC & agent prompt generator (`document_workflow`)
  - [ ] Embed JS snippet generator (`window.PaperPilot.runPipeline`)
- [ ] Create `apps/desktop/src/lib/components/ui/ExportCodeModal.svelte`
  - [ ] Tabbed view (cURL, CLI, MCP Prompt, Embed JS)
  - [ ] 1-click copy-to-clipboard button with visual confirmation
  - [ ] Clean modal backdrop, escape key, and outside-click dismissal
- [ ] Add `</> Code / API` button in `apps/desktop/src/lib/components/pipeline/FlowSelector.svelte`
- [ ] Add Playwright test in `tests/e2e_pipeline_flow_management.spec.ts`

## Phase 2: Universal Individual Tool Integration (Phase 5.9.71)
- [ ] Extend `snippetGenerator.ts` to support all individual tool IDs (e.g. `pdf_bates`, `pdf_compress`, `pdf_redact`, `pdf_watermark`, `pdf_to_docx`, `images_to_pdf`, `pdf_remove_blank`, etc.)
- [ ] Add `</> Code` button in `apps/desktop/src/lib/components/EditableActionCard.svelte` next to "Execute"
- [ ] Connect reactive parameter changes from action card inputs to `ExportCodeModal`
- [ ] Add unit test suite `apps/desktop/src/lib/utils/snippetGenerator.test.ts` with 100% test coverage
- [ ] Add Playwright E2E test verifying code generation on at least 3 distinct tools (e.g., Bates, Compress, Rotate)
