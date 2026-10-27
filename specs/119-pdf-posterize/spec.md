# Spec 119: Multi-Sheet Poster Tile Slicing Engine (`pdf_posterize`)

## 1. Overview
Large documents (architectural blueprints, engineering schematics, banners, high-resolution artwork, maps) often exceed the print dimensions of standard household and office printers (Letter / A4).
`pdf_posterize` takes a single-page or multi-page PDF with large dimensions and tiles each page across a grid of smaller standard sheets (e.g., 2x2, 3x3, or auto-calculated based on target sheet size and scale factor).

To make physical assembly seamless, each poster tile must include:
1. **Configurable Overlap Margin (Glue Gutter)**: E.g., 5mm–15mm overlap so adjacent sheets can be glued together accurately.
2. **Cut Marks / Trim Guides**: Corner alignment ticks showing where to slice off printer margins.
3. **Tile Coordinate Watermark**: Subtle corner numbering (e.g., `Tile (1,1) of (2,2) - Page 1`) to guide manual assembly.

---

## 2. Core Architecture (`paperpilot-pdf`)

### Input Parameters (`PdfPosterizeConfig`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfPosterizeConfig {
    /// Target paper size: "a4", "letter", "legal", or "custom" (default: "a4")
    pub target_sheet: String,
    /// Explicit grid dimensions: columns (optional, default auto-calculated from scale or sheet count)
    pub grid_cols: Option<u32>,
    /// Explicit grid dimensions: rows (optional, default auto-calculated)
    pub grid_rows: Option<u32>,
    /// Overlap margin in points (72 points = 1 inch, default: 14.17 pt ~= 5mm)
    pub overlap_pt: f32,
    /// Scale multiplier (e.g. 2.0 = 200% size, 4.0 = 400% size, default: 1.0)
    pub scale: f32,
    /// Include corner cut trim marks and tile coordinates (default: true)
    pub include_guides: bool,
}
```

### Core Execution
- Read source PDF via `lopdf::Document`.
- For each page:
  - Determine source `MediaBox` width and height.
  - Calculate target grid ($R \times C$) based on target sheet dimensions ($W_{sheet}, H_{sheet}$), scale, and overlap.
  - For each tile $(r, c)$:
    - Clone the page object tree.
    - Set the tile page `MediaBox` to the target sheet dimensions.
    - Transform the page `/Contents` stream using a PDF matrix operator (`cm`):
      $x' = x \cdot scale - c \cdot (W_{tile} - overlap)$, $y' = y \cdot scale - r \cdot (H_{tile} - overlap)$.
    - If `include_guides` is true, append trim tick marks and coordinate text (`Tile R,C`) in the margin area.
- Output a multi-page PDF where each page corresponds to one printable tile.

---

## 3. Penta-Interface Surface Plan

1. **Core**: `paperpilot-pdf/src/operations/pdf_posterize.rs` with `run_operation_bytes`.
2. **CLI**: `paperpilot-cli posterize --input large.pdf --output poster.pdf --sheet a4 --overlap 15 --grid 2x2`
3. **MCP Server**: Tool `pdf_posterize` with schema matching `PdfPosterizeConfig`.
4. **REST Gateway**: `POST /api/v1/pdf/posterize` handling multipart and JSON parameters.
5. **WASM / Edge**: Exported via `WasmPdfEngine::posterize` in `paperpilot-wasm` and handled in `apps/edge/src/index.ts`.
6. **Desktop & Documentation**: Add tool definition to `apps/desktop/src/lib/types/toolList.ts`, create `wiki/119-Pdf-Posterize.md`, and docs reference page.
