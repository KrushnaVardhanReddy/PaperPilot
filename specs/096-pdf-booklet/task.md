# Task 5.3.1: Booklet Imposition Engine (`pdf_booklet`)

## Status
- Spec: [Spec 096](file:///home/krushna/Project/PaperPilot/specs/096-pdf-booklet/spec.md)
- Target: Wave 2 (`v0.7.1`) Core Operations Sprint
- Status: ✅ Completed (Merged PR, 10/10 unmocked Penta-Interface E2E tests passing)

---

## Objectives & Work Items

### 1. Scaffolding
- [x] Run `cargo run -j 2 -p penta-scaffold -- --name pdf_booklet --title "Booklet Imposition" --category organize --icon "📖" --input-type pdf --output-type pdf --description "Reorder and impose pages for 2-sided folding booklet printing."`
- [x] Verify clean wiring across CLI, MCP, REST Gateway, WASM, and E2E harness.

### 2. Core Imposition Algorithm (`paperpilot-pdf/src/operations/pdf_booklet.rs`)
- [x] Define `PdfBookletOptions` (`create_sheets: bool`, `center_fold_line: bool`, `gutter_pt: f32`).
- [x] Load PDF with `lopdf::Document::load_mem(input_bytes)`.
- [x] Calculate page count $N$, pad with blank pages until $N_{padded} \pmod 4 = 0$.
- [x] Generate booklet sequence:
  - For each sheet $i \in [0, S - 1]$:
    - Sheet Front: Left = $N_{padded} - 2i$, Right = $2i + 1$
    - Sheet Back: Left = $2i + 2$, Right = $N_{padded} - 2i - 1$
- [x] In 2-Up mode, place page pairs onto double-width landscape sheets or reorder logical page IDs.
- [x] Add unit tests:
  - `test_booklet_4_pages`: 4 input pages $\rightarrow$ 1 sheet duplex order `[4, 1, 2, 3]`.
  - `test_booklet_padding`: 3 input pages $\rightarrow$ padded to 4 pages.
  - `test_booklet_8_pages`: 8 input pages $\rightarrow$ 2 sheets `[8, 1, 2, 7, 6, 3, 4, 5]`.

### 3. Verification & Conformance
- [x] `cargo test -p paperpilot-pdf --lib operations::pdf_booklet -j 2`
- [x] `cargo run -j 2 -p penta-interface-e2e -- --group pdf_booklet`
- [x] `cargo check --workspace -j 2`
