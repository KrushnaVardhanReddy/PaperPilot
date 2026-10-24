# Task 5.3.10: Automated Page Orientation & Uprighting Engine (`pdf_auto_rotate`)

## Status
- Spec: [Spec 102](file:///home/krushna/Project/PaperPilot/specs/102-pdf-auto-rotate/spec.md)
- Target: Wave 2 (`v0.7.1`) Core Operations Sprint
- Status: ⏳ In-Flight Jules Submission

---

## Objectives & Work Items

### 1. Scaffolding Parity
- [x] Pre-scaffolded across all 5 surfaces (CLI, MCP, Gateway, WASM, Desktop Svelte, E2E harness).
- [x] Verify clean compilation across workspace.

### 2. Core Uprighting Engine (`paperpilot-pdf/src/operations/pdf_auto_rotate.rs`)
- [ ] Define `PdfAutoRotateOptions` with `target_orientation: String` ("portrait" default, "landscape", "auto") and `pages: Option<Vec<u32>>`.
- [ ] Implement `pdf_auto_rotate_bytes(input_bytes: &[u8], options: &PdfAutoRotateOptions) -> Result<Vec<u8>, PdfError>`.
- [ ] Implement `PdfAutoRotateOperation` implementing `PdfOperation`.
- [ ] Traverse document pages:
  - Extract `/MediaBox` dimension (`[llx, lly, urx, ury]`).
  - Read existing `/Rotate` dictionary entry.
  - Calculate effective width $W_{\text{eff}}$ and height $H_{\text{eff}}$.
  - If target is portrait and $W_{\text{eff}} > H_{\text{eff}}$, adjust rotation `(current_rotation + 90) % 360`.
  - If target is landscape and $H_{\text{eff}} > W_{\text{eff}}$, adjust rotation `(current_rotation + 90) % 360`.
- [ ] Save modified document bytes.

### 3. Unit Tests & Conformance
- [ ] Unit tests in `paperpilot-pdf/src/operations/pdf_auto_rotate.rs`:
  - `test_auto_rotate_landscape_to_portrait`: Converts landscape page to upright.
  - `test_auto_rotate_already_portrait`: Leaves portrait page unchanged.
  - `test_auto_rotate_empty_document`: Errors gracefully on empty buffer.
- [ ] Run unit tests: `cargo test -p paperpilot-pdf --lib operations::pdf_auto_rotate -j 2`
- [ ] Run workspace compilation check: `cargo check --workspace -j 2`
