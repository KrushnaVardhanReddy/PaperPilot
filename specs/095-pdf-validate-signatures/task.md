# Task 5.9.61: Digital Signature Verification Engine (`pdf_validate_signatures`)

## Status
- Spec: [Spec 095](file:///home/krushna/Project/PaperPilot/specs/095-pdf-validate-signatures/spec.md)
- Target: Wave 2 (`v0.7.1`) Core Operations Sprint
- Status: ✅ Complete (Merged PR #238, commit [`a6e68c6`](https://github.com/KrushnaVardhanReddy/PaperPilot/commit/a6e68c6))

---

## Objectives & Work Items

### 1. Core Logic in `paperpilot-pdf` (`src/operations/pdf_validate_signatures.rs`)
- [ ] Define `DigitalSignatureInfo` and `SignatureValidationReport` structs with `Serialize`, `Deserialize`.
- [ ] Implement `pdf_validate_signatures_bytes(input_bytes: &[u8], options: &PdfValidateSignaturesOptions) -> Result<Vec<u8>, PdfError>`:
  - Load document via `lopdf::Document::load_mem(input_bytes)`.
  - Scan `/AcroForm` and page annotations for `/Type /Sig` or `/FT /Sig`.
  - Parse `/ByteRange`, `/SubFilter`, `/Name`, `/M` (Date), and `/Contents`.
  - Detect post-signing modifications (bytes after ByteRange or invalid xref offsets).
  - Return JSON-serialized `SignatureValidationReport`.
- [ ] Wire `PdfValidateSignaturesOperation::execute` to parse bytes and populate report.
- [ ] Add unit tests:
  - `test_validate_unsigned_pdf`: returns 0 signatures without error.
  - `test_validate_signed_pdf_fixture`: detects signature field, subfilter, and byte range.

### 2. Penta-Interface Integration
- [ ] **CLI**: Ensure `paperpilot validate-signatures input.pdf --output report.json` writes valid JSON report.
- [ ] **MCP**: Verify `pdf_validate_signatures` tool schema and execution in `paperpilot-mcp/src/server.rs`.
- [ ] **Gateway**: Verify `POST /api/v1/pdf/validate-signatures` returns HTTP 200 with JSON report.
- [ ] **WASM**: Verify in-browser execution via `WasmPdfEngine.pdf_validate_signatures`.
- [ ] **E2E**: Verify 10/10 assertions pass in `tools/penta-interface-e2e/src/cases/pdf_validate_signatures.rs`.

### 3. Verification & CI
- [ ] `cargo test -p paperpilot-pdf --lib -j 2`
- [ ] `cargo check --workspace`
- [ ] Update [PHASES.md](file:///home/krushna/Project/PaperPilot/PHASES.md) and [ROADMAP.md](file:///home/krushna/Project/PaperPilot/ROADMAP.md).
