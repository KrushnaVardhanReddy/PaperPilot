# Task 112A: Implement Scanned Dual-Page Book Center Spine Split (`pdf_split_scanned_book`)

## Metadata
- **Spec**: [Spec 112: Scanned Dual-Page Book Center Spine Split (`pdf_split_scanned_book`)](file:///home/krushna/Project/PaperPilot/specs/112-pdf-split-scanned-book/spec.md)
- **Phase**: Phase 5.3 / Task 5.3.16 & 5.4.7
- **Primary Language**: Rust 2021
- **Impacted Crates**: `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`

---

## Detailed Requirements

### 1. Core Logic (`paperpilot-pdf/src/operations/pdf_split_scanned_book.rs`)
- Expand `PdfSplitScannedBookOptions`:
  - `pub reading_direction: Option<String>` (`"ltr"` default, `"rtl"`)
  - `pub gutter_overlap: Option<f32>` (points of overlap at center gutter, default 0.0)
  - `pub split_horizontal: Option<bool>` (default false)
- Implement `pdf_split_scanned_book_bytes(&[u8], &PdfSplitScannedBookOptions) -> Result<Vec<u8>, PdfError>`:
  - Load document bytes with `lopdf::Document::load_mem`.
  - Iterate through document pages.
  - For each page, read `/MediaBox` (or `/CropBox`).
  - Calculate left and right halves (or top and bottom if `split_horizontal == true`), adjusting with `gutter_overlap`.
  - Create two page entries in new document page tree referencing the source page contents and resources, with respective adjusted `/MediaBox` and `/CropBox`.
  - Order output pages based on `reading_direction` (`"ltr"` => left then right; `"rtl"` => right then left).
  - Save and return output PDF bytes.

### 2. Unit Tests
- `test_split_landscape_spread_ltr`: Verify 1 landscape 2-up page splits into 2 portrait pages in LTR order with correct media boxes.
- `test_split_landscape_spread_rtl`: Verify RTL ordering places right page before left page.
- `test_split_with_gutter_overlap`: Verify overlap is added to both boundary edges.

### 3. Interface Integration
- CLI: `paperpilot-cli/src/commands/pdf_split_scanned_book.rs`
- MCP: `paperpilot-mcp/src/server.rs`
- Documentation:
  - `wiki/112-Pdf-Split-Scanned-Book.md`
  - `apps/docs/src/content/docs/reference/tools/pdf_split_scanned_book.md`

### 4. Verification
- `cargo test -p paperpilot-pdf --lib operations::pdf_split_scanned_book -j 2`
- `cargo check --workspace -j 2`
