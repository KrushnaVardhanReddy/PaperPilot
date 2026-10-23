# Spec 096: Booklet Imposition Engine (`pdf_booklet`)

## 1. Background & Executive Summary
In commercial printing, office publishing, and document binding, multipage documents cannot simply be printed in sequential page order if they are to be folded and stapled down the spine as a booklet. 

**Booklet Imposition** is the process of reordering and arranging document pages onto 2-sided sheets (signatures) such that when folded down the center, the pages appear in the correct numerical reading sequence (1, 2, 3, 4, ...).

**Spec 096** specifies the pure-Rust implementation of `pdf_booklet` (Task 5.3.1) across all five deployment surfaces (Core Rust Engine, CLI, MCP Server, REST Gateway, WASM, and Cloudflare Edge).

---

## 2. Technical Architecture & Imposition Algorithm

### 2.1 Saddle-Stitch 4-Page Signature Mathematics
A standard booklet sheet has 2 sides (Front and Back), with 2 page logical halves per side, giving **4 booklet pages per physical sheet**:
- Let $N$ be the input page count.
- If $N$ is not a multiple of 4, the document must be padded with blank pages at the end until $N_{padded} \pmod 4 = 0$.
- For total sheets $S = N_{padded} / 4$:
  - For sheet index $i \in [0, S - 1]$:
    - **Front side of sheet**: Left = Page $N_{padded} - 2i$, Right = Page $2i + 1$
    - **Back side of sheet**: Left = Page $2i + 2$, Right = Page $N_{padded} - 2i - 1$

#### Example: 8-Page Booklet
- Sheet 1:
  - Front: [Page 8, Page 1]
  - Back:  [Page 2, Page 7]
- Sheet 2:
  - Front: [Page 6, Page 3]
  - Back:  [Page 4, Page 5]

When Sheet 1 and Sheet 2 are printed duplex, stacked, and folded down the middle, the reader flips through pages in exact 1 ➔ 2 ➔ 3 ➔ 4 ➔ 5 ➔ 6 ➔ 7 ➔ 8 order.

### 2.2 Reordering vs. 2-Up Sheet Transformation
`pdf_booklet` supports two imposition modes:
1. **Reorder-Only Mode (`create_sheets: false`)**:
   Reorders the original single pages in booklet folding order with blank filler pages inserted as needed. The printer driver handles 2-up scaling.
2. **2-Up Impose Mode (`create_sheets: true` - Default)**:
   Places two adjacent booklet pages side-by-side onto an expanded landscape sheet (e.g. two Letter/A4 portrait pages side-by-side on an A3/Tabloid or landscape sheet).

---

## 3. Data Model

In `paperpilot-pdf/src/operations/pdf_booklet.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfBookletOptions {
    /// If true, combines pairs of pages onto 2-up landscape sheets.
    /// If false, reorders single pages into booklet sequence.
    #[serde(default = "default_true")]
    pub create_sheets: bool,

    /// Add a subtle fold line down the center of each sheet.
    #[serde(default)]
    pub center_fold_line: bool,

    /// Margin between side-by-side pages on a sheet in points (default: 0.0).
    #[serde(default)]
    pub gutter_pt: f32,
}

fn default_true() -> bool {
    true
}

impl Default for PdfBookletOptions {
    fn default() -> Self {
        Self {
            create_sheets: true,
            center_fold_line: false,
            gutter_pt: 0.0,
        }
    }
}
```

---

## 4. Verification & Testing
1. **Single-page document**: Padded to 4 pages, imposed cleanly.
2. **8-page document**: Imposed onto 2 duplex sheets (4 logical pages per sheet).
3. **Empty document**: Fails gracefully with `PdfError::InvalidInput` or `ParseError`.
4. **All 5 Surfaces**: CLI, MCP, REST Gateway, WASM, and E2E conformance pass with zero warnings.
