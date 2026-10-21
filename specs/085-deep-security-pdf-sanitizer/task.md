# Task Checklist: Spec 085 — Privacy & Deep Security Sanitizer (`pdf_sanitize`)

## Phase 1: Pure-Rust Core Engine (`paperpilot-pdf`)
- [ ] Define `SanitizeOptions` and `SanitizeReport` structs in `paperpilot-pdf/src/operations/sanitize.rs`
- [ ] Implement `sanitize_pdf(input_bytes: &[u8], options: &SanitizeOptions) -> Result<(Vec<u8>, SanitizeReport), anyhow::Error>`
  - [ ] Strip Document Information Dictionary (`/Info` dict)
  - [ ] Strip XML `/Metadata` streams from catalog and page objects
  - [ ] Purge `/EmbeddedFiles` from document `/Names` dict
  - [ ] Remove `/JS`, `/JavaScript`, `/OpenAction`, and `/AA` action dictionaries
  - [ ] Remove `/PieceInfo` and `/Thumb` objects
  - [ ] Full xref rebuild and compact rewrite
- [ ] Unit tests in `paperpilot-pdf` verifying stripped items
- [ ] Register in `paperpilot-pdf/src/operations/mod.rs` and workflow runner

## Phase 2: The 5 Thin Wrappers
- [ ] **CLI**: Add `Sanitize` command to `paperpilot-cli/src/cli.rs` and `paperpilot-cli/src/commands/sanitize.rs`
- [ ] **MCP**: Add `pdf_sanitize` tool declaration and executor in `paperpilot-mcp/src/server.rs`
- [ ] **REST Gateway**: Add `/api/v1/pdf/sanitize` endpoint in `paperpilot-gateway/src/handlers.rs` & `server.rs`
- [ ] **WASM**: Expose `WasmPdfEngine.sanitize` in `paperpilot-wasm/src/operations.rs` & `lib.rs`
- [ ] **Edge**: Expose `POST /api/v1/sanitize` in `apps/edge/src/index.ts`

## Phase 3: Desktop UI & Pipeline Integration
- [ ] Add `pdf_sanitize` tool definition to `apps/desktop/src/lib/types/toolList.ts` (category: `security`, icon: `🛡️`)
- [ ] Add inspector parameters & execution handler in `apps/desktop/src/lib/components/layout/OperationsPanel.svelte`
- [ ] Add step definition in `apps/desktop/src/lib/state/pipeline.svelte.ts`
- [ ] Update `ExportCodeModal.svelte` and `snippetGenerator.ts` to support `pdf_sanitize` snippets

## Phase 4: E2E Verification & Audit
- [ ] Add unmocked test case in `tools/penta-interface-e2e`
- [ ] Run `cargo test -p paperpilot-pdf -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway -j 4`
- [ ] Update `PHASES.md` and `HANDOFF.md`
