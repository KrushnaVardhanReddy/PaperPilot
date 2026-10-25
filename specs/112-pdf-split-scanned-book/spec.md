# Spec 112: Scanned Dual-Page Book Center Spine Split (`pdf_split_scanned_book`)

## 1. Overview
When books or bound reports are scanned on a flatbed or overhead scanner, two facing pages (e.g., pages 2 & 3) are frequently captured as a single landscape sheet. `pdf_split_scanned_book` automates the separation of two-up scanned spreads into two distinct single portrait pages.

---

## 2. Technical Architecture

### 2.1 Geometry & Splitting Logic
For each page in the input document:
1. Obtain the effective bounding box from `/MediaBox` or `/CropBox`: `[x0, y0, x1, y1]`.
2. Determine orientation:
   - Width $W = x_1 - x_0$, Height $H = y_1 - y_0$.
   - If $W > H$ (landscape spread), center splitting occurs along the vertical midline $X_{mid} = x_0 + W / 2.0$.
3. Gutter / Creep Overlap:
   - A configurable `gutter_overlap` (e.g. 0 to 10 points) allows preserving text that curves into the spine.
   - Left Page Rect: `[x0, y0, X_mid + gutter_overlap, y1]`
   - Right Page Rect: `[X_mid - gutter_overlap, y0, x1, y1]`
4. Reading Order:
   - Standard LTR: Page 1 = Left, Page 2 = Right.
   - RTL (Manga, Arabic, Hebrew): Page 1 = Right, Page 2 = Left.
5. In pure-Rust `lopdf`, each physical input page dictionary can be duplicated or re-referenced with its own adjusted `/MediaBox` and `/CropBox`, preserving 100% of underlying vector graphics, OCR text layers, and embedded images without re-encoding.

### 2.2 Parameters (`PdfSplitScannedBookOptions`)
```rust
pub struct PdfSplitScannedBookOptions {
    pub reading_direction: Option<String>, // "ltr" (default) or "rtl"
    pub gutter_overlap: Option<f32>,       // points of overlap at spine (default 0.0)
    pub split_horizontal: Option<bool>,    // split top/bottom instead of left/right (default false)
}
```

---

## 3. Penta-Interface Parity
- **Core Engine**: `paperpilot-pdf/src/operations/pdf_split_scanned_book.rs`
- **CLI**: `paperpilot pdf-split-scanned-book --input in.pdf --output out.pdf --direction ltr --gutter-overlap 5.0`
- **MCP Server**: `pdf_split_scanned_book`
- **REST Gateway**: `POST /api/v1/pdf/split-scanned-book`
- **Docs & Wiki**: `wiki/112-Pdf-Split-Scanned-Book.md`, `apps/docs/src/content/docs/reference/tools/pdf_split_scanned_book.md`
