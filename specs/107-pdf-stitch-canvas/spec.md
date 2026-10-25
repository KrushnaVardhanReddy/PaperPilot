# Spec 107: Continuous Canvas Single-Sheet Stitch (`pdf_stitch_canvas`)

## 1. Background & Executive Summary
In industries like architecture, construction blueprints, long cash register / receipt archiving, medical telemetry/EKG printouts, and timeline presentations, multi-page PDFs need to be converted into a single continuous, unbroken canvas rather than discrete pages.

Standard PDF tools typically force pages onto fixed physical paper dimensions (A4, Letter) or create grid impositions (N-Up, booklet). **Spec 107** implements `pdf_stitch_canvas` (Phase 5, Task 5.3.11):
- Stitches all pages of an input PDF into **one single continuous-page PDF canvas**.
- **Vertical Mode (Default)**: Pages are stitched top-to-bottom sequentially like a long web page or roll-paper receipt.
- **Horizontal Mode**: Pages are stitched left-to-right sequentially like a banner, panoramic timeline, or architectural elevation.
- **Configurable Spacing & Margins**: Configurable padding between pages and outer border margins.
- **Form XObject Architecture**: Preserves 100% of the original vector fidelity, text streams, embedded fonts, and raster images with zero downsampling or rasterization artifacts.

---

## 2. Technical Architecture & Algorithm

### 2.1 Coordinate Space & Dimension Calculation
Given $N$ pages with individual dimensions $(W_i, H_i)$ for $i \in [1..N]$, user-defined `margin_pt` ($M \ge 0$), and `spacing_pt` ($S \ge 0$):

1. **Vertical Stitching (`direction: "vertical"`)**:
   - Total Canvas Width: $W_{\text{canvas}} = \max_{i}(W_i) + 2M$
   - Total Canvas Height: $H_{\text{canvas}} = \sum_{i=1}^N H_i + 2M + (N - 1)S$
   - For page $i$:
     - $X_i = M + \frac{(W_{\text{canvas}} - 2M - W_i)}{2}$ (horizontally centered within canvas)
     - $Y_i = H_{\text{canvas}} - M - \sum_{k=1}^i H_k - (i - 1)S$ (stacked downwards from the top)

2. **Horizontal Stitching (`direction: "horizontal"`)**:
   - Total Canvas Width: $W_{\text{canvas}} = \sum_{i=1}^N W_i + 2M + (N - 1)S$
   - Total Canvas Height: $H_{\text{canvas}} = \max_{i}(H_i) + 2M$
   - For page $i$:
     - $X_i = M + \sum_{k=1}^{i-1} W_k + (i - 1)S$
     - $Y_i = M + \frac{(H_{\text{canvas}} - 2M - H_i)}{2}$ (vertically centered within canvas)

### 2.2 Form XObject Encapsulation
To avoid content stream coordinate collisions between pages:
- Each page dictionary from the source PDF is converted into an independent `/Type /XObject /Subtype /Form` stream with `/BBox [0 0 W_i H_i]` and preserves the page's original `/Resources` dictionary.
- The new single-page canvas has a concise content stream that invokes each Form XObject using `cm` (current transformation matrix) translation at `(X_i, Y_i)` and `Do /Fm{i}`.
- All original pages are pruned from the document catalog, replaced by a single `/Pages` root with `Count = 1`.

---

## 3. Data Model

In `paperpilot-pdf/src/operations/pdf_stitch_canvas.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfStitchCanvasOptions {
    /// "vertical" (default) or "horizontal"
    #[serde(default = "default_direction")]
    pub direction: String,

    /// Outer margin around the canvas in points (default: 0.0)
    #[serde(default)]
    pub margin_pt: f32,

    /// Spacing between stitched pages in points (default: 0.0)
    #[serde(default)]
    pub spacing_pt: f32,
}

fn default_direction() -> String {
    "vertical".to_string()
}

impl Default for PdfStitchCanvasOptions {
    fn default() -> Self {
        Self {
            direction: default_direction(),
            margin_pt: 0.0,
            spacing_pt: 0.0,
        }
    }
}
```

---

## 4. Penta-Interface Surface Wireup

1. **Core Engine**: `paperpilot-pdf/src/operations/pdf_stitch_canvas.rs`:
   - `pdf_stitch_canvas_bytes(input_bytes: &[u8], options: &PdfStitchCanvasOptions) -> Result<Vec<u8>, PdfError>`
   - `PdfStitchCanvasOperation` implementing `paperpilot_core::traits::PdfOperation`.
2. **CLI**: `paperpilot-cli`:
   - `paperpilot pdf-stitch-canvas --input <IN> --output <OUT> [--direction <DIR>] [--margin <PT>] [--spacing <PT>]`
3. **MCP Tool**: `paperpilot-mcp`:
   - Tool `pdf_stitch_canvas` with properties `input`, `output`, `direction`, `margin_pt`, `spacing_pt`.
4. **REST Gateway**: `paperpilot-gateway`:
   - `POST /api/v1/pdf/stitch-canvas` with multipart file upload or JSON payload.
5. **WASM & Edge Engine**: `paperpilot-wasm` & `apps/edge`:
   - In-memory execution without temporary files.
6. **E2E Testing**: `tools/penta-interface-e2e`:
   - Comprehensive multi-page test validating output page count = 1 and calculated MediaBox dimensions.
