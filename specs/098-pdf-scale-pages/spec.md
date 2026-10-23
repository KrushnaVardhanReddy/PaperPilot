# Spec 098: Page Dimensional Scaling Engine (`pdf_scale_pages`)

## 1. Background & Executive Summary
In global document workflows, documents formatted for North American **US Letter** (8.5 $\times$ 11 in / 612 $\times$ 792 pt) must frequently be converted to international standard **ISO A4** (210 $\times$ 297 mm / 595.28 $\times$ 841.89 pt), or scaled down for legal filings and archival standards.

**Spec 098** implements `pdf_scale_pages` (Task 5.3.3) across all 5 deployment surfaces, supporting standard target paper sizes (A3, A4, A5, Letter, Legal, Tabloid), arbitrary percentage scaling, aspect-ratio preservation, and centered positioning.

---

## 2. Technical Architecture & Transformation

### 2.1 Supported Sizing Modes
1. **Target Standard Paper Size**:
   - `A4`: [0, 0, 595.28, 841.89]
   - `Letter`: [0, 0, 612.0, 792.0]
   - `Legal`: [0, 0, 612.0, 1008.0]
   - `A3`: [0, 0, 841.89, 1190.55]
   - `A5`: [0, 0, 419.53, 595.28]
   - `Tabloid`: [0, 0, 792.0, 1224.0]
2. **Scale Factor / Percentage**:
   - e.g. `scale_x = 0.8`, `scale_y = 0.8` (80% scaling).
3. **Aspect Ratio Preservation**:
   - When scaling to a target box with a different aspect ratio, fit within bounds preserving aspect ratio (`scale = min(s_x, s_y)`) and center horizontally and vertically.

### 2.2 Content Stream Transformation
Update page `/MediaBox`, `/CropBox`, and wrap the page's `/Contents` stream with:
```text
q
sx 0 0 sy tx ty cm
... original stream ...
Q
```

---

## 3. Data Model

In `paperpilot-pdf/src/operations/pdf_scale_pages.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfScalePagesOptions {
    /// Target size: "A4", "Letter", "Legal", "A3", "A5", "Tabloid", or "Custom"
    #[serde(default = "default_target_size")]
    pub target_size: String,

    /// Scale percentage factor (e.g. 1.0 = 100%, 0.8 = 80%)
    #[serde(default = "default_scale_factor")]
    pub scale_factor: f32,

    /// Maintain original aspect ratio when fitting into target dimensions
    #[serde(default = "default_true")]
    pub maintain_aspect_ratio: bool,

    /// Center scaled content on the new page boundaries
    #[serde(default = "default_true")]
    pub center_content: bool,
}

fn default_target_size() -> String { "A4".to_string() }
fn default_scale_factor() -> f32 { 1.0 }
fn default_true() -> bool { true }

impl Default for PdfScalePagesOptions {
    fn default() -> Self {
        Self {
            target_size: default_target_size(),
            scale_factor: 1.0,
            maintain_aspect_ratio: true,
            center_content: true,
        }
    }
}
```

---

## 4. Verification & Testing
1. Scale Letter PDF to A4: output pages have `/MediaBox` equal to `[0, 0, 595.28, 841.89]`.
2. Scale by percentage (50%): verifies stream transformation matrix.
3. Empty document error handling.
