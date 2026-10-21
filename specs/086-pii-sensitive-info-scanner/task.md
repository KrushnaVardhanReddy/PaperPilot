# Task Checklist: Spec 086 — Automated PII & Sensitive Information Scanner (`document_pii_scan`)

## Phase 1: Pure-Rust Core Engine (`paperpilot-pdf` & `paperpilot-core`)
- [ ] Define `PiiScanOptions`, `PiiFinding`, and `PiiScanReport` structs in `paperpilot-pdf/src/operations/pii.rs`
- [ ] Implement robust regex patterns & checksum validators:
  - [ ] SSN regex validator
  - [ ] Credit card detector with Luhn algorithm validation
  - [ ] Email regex pattern detector
  - [ ] International & US phone number detector
  - [ ] IBAN detector with checksum validation
- [ ] Implement text extraction + token matching per page with text bounding box estimation
- [ ] Implement `scan_pii(input_bytes: &[u8], options: &PiiScanOptions) -> Result<PiiScanReport, PdfError>`
- [ ] Add unit test suite in `paperpilot-pdf` covering clean documents, multi-PII documents, and invalid buffers
- [ ] Register in `paperpilot-pdf/src/operations/mod.rs` and workflow runner

## Phase 2: The 5 Thin Wrappers
- [ ] **CLI**: Add `PiiScan` command in `paperpilot-cli/src/cli.rs` and handler in `paperpilot-cli/src/commands/pii.rs` & `mod.rs`
- [ ] **MCP**: Register `document_pii_scan` tool schema and dispatch in `paperpilot-mcp/src/server.rs`
- [ ] **REST Gateway**: Add `/api/v1/pdf/pii-scan` route in `paperpilot-gateway/src/server.rs` & `handlers.rs`
- [ ] **WASM**: Implement in-memory scanning in `paperpilot-wasm/src/operations.rs` & export in `lib.rs` (without `paperpilot-pdf` dependency)
- [ ] **Edge**: Expose `/api/v1/pii-scan` in `apps/edge/src/index.ts`

## Phase 3: Desktop UI & Pipeline Integration
- [ ] Add `document_pii_scan` tool definition to `apps/desktop/src/lib/types/toolList.ts` (category: `security`, icon: `🔍`)
- [ ] Add step definition in `apps/desktop/src/lib/state/pipeline.svelte.ts`
- [ ] Add code snippet templates in `apps/desktop/src/lib/utils/snippetGenerator.ts`
- [ ] Update `wiki/document_pii_scan.md`

## Phase 4: E2E Verification & Audit
- [ ] Add unmocked test case in `tools/penta-interface-e2e/src/cases/pii.rs`
- [ ] Wire `--group pii` in `tools/penta-interface-e2e/src/main.rs`
- [ ] Verify 10/10 tests pass and output report in `reports/PENTA_INTERFACE_E2E_PII_SCAN_REPORT.md`
- [ ] Update `PHASES.md` and `HANDOFF.md`
