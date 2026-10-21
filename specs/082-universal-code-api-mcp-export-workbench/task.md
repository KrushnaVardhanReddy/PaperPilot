# Task Checklist: Spec 082 — Universal "Export as Code / API / MCP Prompt" Workbench

## Phase 1: Core Engine & Pipeline Flow Integration (Phase 5.9.69D) - [COMPLETED]
- [x] Create `apps/desktop/src/lib/components/pipeline/ExportCodeModal.svelte`
- [x] Implement tabbed view (cURL, CLI, MCP Prompt, Embed JS)
- [x] Implement 1-click copy-to-clipboard button with visual feedback
- [x] Implement drawer backdrop, transitions, and close handler
- [x] Add `</> Code / API` button in `apps/desktop/src/lib/components/pipeline/FlowSelector.svelte`
- [x] Add Playwright test in `tests/e2e_pipeline_flow_management.spec.ts`

## Phase 2: Universal Individual Tool Integration (Phase 5.9.71)
- [ ] Implement headless snippet generator in `apps/desktop/src/lib/utils/snippetGenerator.ts`:
  - [ ] CLI command generator for tools (e.g. `paperpilot rotate --input in.pdf --angle 90 --output out.pdf`, `paperpilot bates`, `paperpilot compress`, `paperpilot watermark`, `paperpilot redact`, `paperpilot merge`, `paperpilot split`, etc.)
  - [ ] cURL generator targeting Gateway REST endpoints (e.g. `curl -X POST http://localhost:8080/api/v1/tools/rotate -F "file=@in.pdf" -F "angle=90"`)
  - [ ] MCP JSON-RPC & Agent Prompt generator (natural language instructions + tool name/arguments)
  - [ ] Embed JS generator (`await window.PaperPilot.executeTool(...)`)
- [ ] Refactor / enhance `ExportCodeModal.svelte`:
  - [ ] Accept optional `target?: SnippetTarget` prop alongside fallback to `pipelineState`
  - [ ] Generate snippets using `generateSnippets(target)` when single-tool target is passed
- [ ] Add `</> Code / API` button to `apps/desktop/src/lib/components/EditableActionCard.svelte`:
  - [ ] Place next to "Execute Action" button (or in card header)
  - [ ] Dynamically compute `SnippetTarget` from active form state (angle, password, bates prefix, watermark text, output path, input document)
  - [ ] Open `ExportCodeModal` when clicked
- [ ] Unit test suite in `apps/desktop/tests/snippetGenerator.test.ts`:
  - [ ] Test CLI snippet generation for at least 6 tools (Rotate, Bates, Watermark, Compress, Encrypt, Split)
  - [ ] Test cURL multipart generation
  - [ ] Test MCP tool call and natural language prompt generation
  - [ ] Test Embed JS snippet generation
- [ ] Playwright E2E verification:
  - [ ] In `apps/desktop/tests/e2e_action_card_code_export.spec.ts` (or integrated into existing E2E suite), assert clicking `</> Code / API` on an action card opens the modal and renders the correct tool's CLI/cURL/MCP commands
- [ ] Build & Typecheck:
  - [ ] `npm run check` passes with 0 errors in `apps/desktop`
  - [ ] `npm run build` passes in `apps/desktop`
