# Spec 097: Multi-Page N-Up Imposition Engine (`pdf_nup`)

## 1. Background & Executive Summary
In handouts, slides, paper reduction, and printing proofs, multiple pages of a document must be placed side-by-side onto a single sheet in an $M \times N$ grid (e.g., 2-up, 4-up, 9-up, 16-up).

**Spec 097** implements `pdf_nup` (Task 5.3.2) across all 5 deployment surfaces with configurable sheet sizing, ordering (horizontal vs. vertical), margins, and optional frame borders.

---

## 2. Technical Architecture & Grid Layout

### 2.1 Supported Layout Grid Options
- `nup`: Integer representing the number of pages per sheet: `2`, `4`, `9`, or `16`.
  - `2-up`: 1 row $\times$ 2 columns (or 2 rows $\times$ 1 column depending on orientation).
  - `4-up`: 2 rows $\times$ 2 columns.
  - `9-up`: 3 rows $\times$ 3 columns.
  - `16-up`: 4 rows $\times$ 4 columns.
- `ordering`:
  - `Horizontal`: Left-to-right, then top-to-bottom (default).
  - `Vertical`: Top-to-bottom, then left-to-right.
- `draw_borders`: Draw subtle hairline boundary around each logical page.
- `margin_pt`: Outer sheet margin in points (default: 18.0 pt / 0.25 in).
- `spacing_pt`: Spacing between grid cells in points (default: 10.0 pt).

### 2.2 Transform Scaling
Each input page's `/MediaBox` is scaled and translated into its designated grid slot:
- Given sheet dimensions $W_{sheet}, H_{sheet}$ and cell dimensions $W_{cell}, H_{cell}$:
  - Calculate uniform scale factor $s = \min(W_{cell} / W_{orig}, H_{cell} / H_{orig})$.
  - Center page within cell with translation offsets $(dx, dy)$.
  - Prepend/wrap content stream with `q s 0 0 s dx dy cm ... Q`.

---

## 3. Data Model

In `paperpilot-pdf/src/operations/pdf_nup.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfNupOptions {
    /// Number of pages per sheet: 2, 4, 9, or 16 (default: 2)
    #[serde(default = "default_nup")]
    pub nup: u32,

    /// Grid flow direction: "horizontal" or "vertical"
    #[serde(default = "default_ordering")]
    pub ordering: String,

    /// Outer margin in points
    #[serde(default = "default_margin")]
    pub margin_pt: f32,

    /// Spacing between cells in points
    #[serde(default = "default_spacing")]
    pub spacing_pt: f32,

    /// Draw a thin border around each page cell
    #[serde(default)]
    pub draw_borders: bool,
}

fn default_nup() -> u32 { 2 }
fn default_ordering() -> String { "horizontal".to_string() }
fn default_margin() -> f32 { 18.0 }
fn default_spacing() -> f32 { 10.0 }

impl Default for PdfNupOptions {
    fn default() -> Self {
        Self {
            nup: 2,
            ordering: default_ordering(),
            margin_pt: 18.0,
            spacing_pt: 10.0,
            draw_borders: false,
        }
    }
}
```

---

## 4. Verification & Testing
1. 8 input pages with `nup = 2` yields exactly 4 output sheets.
2. 8 input pages with `nup = 4` yields exactly 2 output sheets.
3. 5 input pages with `nup = 4` yields 2 output sheets (second sheet partially filled).
4. All 5 interfaces pass conformance verification.
