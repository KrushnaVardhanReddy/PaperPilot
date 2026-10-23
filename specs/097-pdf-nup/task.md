# Task 5.3.2: Multi-Page N-Up Imposition Engine (`pdf_nup`)

## Status
- Spec: [Spec 097](file:///home/krushna/Project/PaperPilot/specs/097-pdf-nup/spec.md)
- Target: Wave 2 (`v0.7.1`) Core Operations Sprint
- Status: ⏳ Ready for Jules Submission

---

## Objectives & Work Items

### 1. Scaffolding
- [ ] Run `cargo run -j 2 -p penta-scaffold -- --name pdf_nup --title "N-Up Page Imposition" --category organize --icon "📑" --input-type pdf --output-type pdf --description "Combine multiple pages per sheet in a 2, 4, 9, or 16-up grid layout."`
- [ ] Verify wiring compiles cleanly across CLI, MCP, REST Gateway, WASM, Desktop, and E2E harness.

### 2. Core Imposition Algorithm (`paperpilot-pdf/src/operations/pdf_nup.rs`)
- [ ] Define `PdfNupOptions` (`nup`, `ordering`, `margin_pt`, `spacing_pt`, `draw_borders`).
- [ ] Load document via `lopdf::Document::load_mem(input_bytes)`.
- [ ] Compute grid rows and columns based on `nup`:
  - 2 $\rightarrow$ 1x2 or 2x1
  - 4 $\rightarrow$ 2x2
  - 9 $\rightarrow$ 3x3
  - 16 $\rightarrow$ 4x4
- [ ] Scale and position logical pages into sheet cells using transformation matrix `cm`.
- [ ] Unit tests:
  - `test_nup_2_up`: 4 pages $\rightarrow$ 2 output sheets.
  - `test_nup_4_up`: 4 pages $\rightarrow$ 1 output sheet.
  - `test_nup_empty`: Empty document returns error.

### 3. Verification & Conformance
- [ ] `cargo test -p paperpilot-pdf --lib operations::pdf_nup -j 2`
- [ ] `cargo run -j 2 -p penta-interface-e2e -- --group pdf_nup`
- [ ] `cargo check --workspace -j 2`
