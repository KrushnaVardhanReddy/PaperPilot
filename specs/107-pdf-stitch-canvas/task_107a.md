# Task 107A: Implement Continuous Canvas Stitching (`pdf_stitch_canvas`) Across All Surfaces

## Metadata
- **Spec**: [Spec 107: Continuous Canvas Single-Sheet Stitch (`pdf_stitch_canvas`)](file:///home/krushna/Project/PaperPilot/specs/107-pdf-stitch-canvas/spec.md)
- **Phase**: Phase 5, Task 5.3.11
- **Primary Crate**: `paperpilot-pdf`
- **Impacted Interfaces**:
  - `paperpilot-pdf` (core engine)
  - `paperpilot-cli` (CLI command & parameters)
  - `paperpilot-mcp` (MCP tool schema & dispatch)
  - `paperpilot-gateway` (REST endpoints & OpenAPI)
  - `paperpilot-wasm` (WASM in-memory bridge)
  - `tools/penta-interface-e2e` (E2E assertions)

---

## Detailed Requirements

### 1. Core Engine: `paperpilot-pdf/src/operations/pdf_stitch_canvas.rs`
1. **Options**:
   - `direction`: `"vertical"` (default) or `"horizontal"`
   - `margin_pt`: `f32` (default `0.0`)
   - `spacing_pt`: `f32` (default `0.0`)
2. **Implementation**:
   - Parse input PDF via `lopdf::Document::load_mem`.
   - Measure each page's `/MediaBox` dimensions $(W_i, H_i)$.
   - Calculate total canvas size:
     - Vertical: $W = \max(W_i) + 2M$, $H = \sum H_i + 2M + (N-1)S$. Center pages horizontally.
     - Horizontal: $W = \sum W_i + 2M + (N-1)S$, $H = \max(H_i) + 2M$. Center pages vertically.
   - For each page, create a `/Type /XObject /Subtype /Form` stream with `/BBox [0 0 W_i H_i]` and copy the page's `/Resources` and `/Contents`.
   - In the canvas page content stream, invoke each Form XObject using `cm` transform matrix and `Do`.
   - Create a single new `/Page` with `/MediaBox [0 0 W H]`, wire it to a single `/Pages` root with `Count = 1`, and update the catalog `/Root`.
   - Prune unreferenced objects and serialize to bytes.
3. **Unit Tests**:
   - `test_pdf_stitch_canvas_empty_document` (assert error on empty document)
   - `test_pdf_stitch_canvas_vertical` (assert single page output with exact expected $W \times H$)
   - `test_pdf_stitch_canvas_horizontal` (assert single page output with exact expected $W \times H$)

---

### 2. Interface Wrappers
1. **CLI (`paperpilot-cli`)**:
   - Add optional flags to `PdfStitchCanvas` command in `src/cli.rs`:
     - `--direction <vertical|horizontal>` (default `"vertical"`)
     - `--margin <points>` (default `0.0`)
     - `--spacing <points>` (default `0.0`)
   - Pass options in `src/commands/pdf_stitch_canvas.rs`.
2. **MCP (`paperpilot-mcp`)**:
   - Expose `direction`, `margin_pt`, and `spacing_pt` in `schema_pdf_stitch_canvas`.
   - Parse parameters in `server.rs` handler and forward to `pdf_stitch_canvas_bytes`.
3. **REST Gateway (`paperpilot-gateway`)**:
   - Ensure query parameters / multipart options (`direction`, `margin_pt`, `spacing_pt`) are deserialized and passed into `pdf_stitch_canvas_bytes`.
4. **WASM Engine (`paperpilot-wasm`)**:
   - Ensure `options_json` properly deserializes into `PdfStitchCanvasOptions`.

---

### 3. Verification & Acceptance Criteria
1. `cargo test -p paperpilot-pdf operations::pdf_stitch_canvas -j 2` passes 100%.
2. `cargo check --workspace -j 2` compiles with 0 errors.
3. Real 2-page and 3-page test fixtures stitch cleanly into 1 page with no coordinate overlap.
