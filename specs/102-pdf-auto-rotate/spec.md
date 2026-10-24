# Spec 102: Automated Page Orientation & Uprighting Engine (`pdf_auto_rotate`)

## 1. Background & Executive Summary
Scanned PDFs, mobile captures, multi-feed office scanners, and mixed orientation packets (e.g. landscape spreadsheets mixed into portrait legal contracts) often result in pages rotated sideways (90° / 270°) or upside-down (180°). Manually inspecting and rotating each page in large multi-page documents is tedious and error-prone.

**Spec 102** implements `pdf_auto_rotate` (Task 5.3.10) across all 5 PaperPilot surfaces. It provides a pure-Rust, deterministic page orientation engine that inspects each page's content characteristics and rotates sideways pages to upright (0°):
1. **Aspect-Ratio & Dimension Mode (Default)**: Detects landscape pages ($W > H$) and normalizes them to upright portrait ($H > W$), or allows configuring `target_orientation: "portrait" | "landscape" | "auto"`.
2. **Text Baseline Analysis**: Inspects text character stream orientation vectors when vector text elements are present.
3. **Preserves Existing Transformation and Structure**: Accurately adjusts `/Rotate` metadata and coordinate boxes (`/MediaBox`, `/CropBox`), ensuring zero degradation of vector graphics, embedded fonts, or images.

---

## 2. Technical Architecture & Transformation

### 2.1 PDF Page Rotation Semantics
In PDF 1.4 - 2.0 specifications (ISO 32000-1 §7.7.3.3):
- Each page dictionary has an optional `/Rotate` entry specifying the number of degrees by which the page should be rotated clockwise when displayed or printed.
- Value MUST be a multiple of 90: `0`, `90`, `180`, `270`.
- In addition, page bounding boxes `/MediaBox` = `[llx, lly, urx, ury]` define the physical dimensions.
  - Width: $W = urx - llx$
  - Height: $H = ury - lly$

### 2.2 Orientation Normalization
1. **Current Effective Orientation**:
   - Calculate effective width $W_{\text{eff}}$ and height $H_{\text{eff}}$ factoring in current `/Rotate`.
   - If current `/Rotate` is `90` or `270`, effective dimensions are swapped: $W_{\text{eff}} = H_{\text{orig}}$, $H_{\text{eff}} = W_{\text{orig}}$.
2. **Target Uprighting**:
   - For `target_orientation = "portrait"`: if $W_{\text{eff}} > H_{\text{eff}}$, rotate clockwise by +90° or +270° (configurable direction or +90° default) to ensure height is greater than width.
   - For `target_orientation = "landscape"`: if $H_{\text{eff}} > W_{\text{eff}}$, rotate clockwise by +90° so width is greater than height.
   - For `target_orientation = "auto"` (default): Normalizes all pages to portrait upright orientation.
3. **Selective Page Range**:
   - `pages`: Optional array of 1-based page indices (`None` means all pages).

---

## 3. Data Model

In `paperpilot-pdf/src/operations/pdf_auto_rotate.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfAutoRotateOptions {
    /// Target orientation: "portrait" (default), "landscape", or "auto"
    #[serde(default = "default_target_orientation")]
    pub target_orientation: String,

    /// Optional specific 1-based page numbers to evaluate. None = all pages.
    #[serde(default)]
    pub pages: Option<Vec<u32>>,
}

fn default_target_orientation() -> String {
    "portrait".to_string()
}

impl Default for PdfAutoRotateOptions {
    fn default() -> Self {
        Self {
            target_orientation: default_target_orientation(),
            pages: None,
        }
    }
}
```

---

## 4. Verification & Testing Guardrails

1. **Unit Tests in `paperpilot-pdf`**:
   - `test_auto_rotate_landscape_to_portrait`: Landscape page ($842 \times 595$) rotated to upright ($595 \times 842$ effective).
   - `test_auto_rotate_already_portrait`: Portrait page ($612 \times 792$) remains unchanged with rotation delta 0.
   - `test_auto_rotate_empty_document`: Returns appropriate `PdfError`.
2. **Penta-Interface Parity**:
   - Verified across CLI (`paperpilot-cli pdf-auto-rotate`), MCP (`pdf_auto_rotate`), REST Gateway (`POST /api/v1/pdf/auto-rotate`), WASM, and Cloudflare Edge.
