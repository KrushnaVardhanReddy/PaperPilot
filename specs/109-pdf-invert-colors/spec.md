# Spec 109: Invert PDF Colors & High-Contrast Dark Mode (`pdf_invert_colors`)

## 1. Background & Executive Summary
Reading PDFs in low-light environments, late at night, or on high-contrast assistive displays is a frequent user need. Most existing web or desktop tools accomplish "dark mode" either by rasterizing pages into inverted bitmap images (which destroys text searchability, selectable text, and blows up file size) or by relying on client-side CSS filters (which cannot be saved or exported).

**Spec 109** implements a pure-Rust vector and content stream color inversion engine:
- Inverts color operands directly within PDF `/Contents` streams (`rg`, `RG`, `k`, `K`, `g`, `G`, `sc`, `SC`).
- Supports **Dark Mode** (white text on dark charcoal/black background `#181818`), **High-Contrast Monochrome**, and **Sepia** tones.
- Injects a background fill rectangle matching page `/MediaBox`.
- **100% Vector Integrity**: Preserves crystal-sharp vector typography, embedded fonts, and selectable text.
- **Penta-Interface Parity**: Exposes `pdf_invert_colors` across CLI (`paperpilot invert-colors`), MCP (`pdf_invert_colors`), REST Gateway (`POST /api/v1/pdf/invert-colors`), WASM, and Desktop UI.

---

## 2. Technical Architecture & Algorithm

### 2.1 PDF Operator Inversion Logic
In PDF content streams, colors are manipulated by specific operators:
- Non-stroking RGB: `r g b rg` $\to$ `(1-r) (1-g) (1-b) rg`
- Stroking RGB: `r g b RG` $\to$ `(1-r) (1-g) (1-b) RG`
- Grayscale: `gray g` $\to$ `(1-gray) g`, `gray G` $\to$ `(1-gray) G`
- CMYK: `c m y k k` $\to$ `(1-c) (1-m) (1-y) (1-k) k`

### 2.2 Background Plane Injection
Since standard PDFs have implicit transparent/white page backgrounds:
1. Parse the page's `/MediaBox` rectangle `[x, y, width, height]`.
2. Inject a background drawing instruction block at the very beginning of the page's `/Contents` stream:
   ```text
   q
   0.094 0.094 0.094 rg
   x y width height re f
   Q
   ```
   (where `0.094` corresponds to `#181818` dark background).
3. If inverted mode is selected, default black text `0 0 0 rg` automatically becomes `1 1 1 rg` (clean white text).

---

## 3. Data Types & API Specification

### 3.1 Mode Options
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvertMode {
    DarkMode,       // White text on #181818 dark gray background
    TrueInvert,     // Mathematical 1.0 - color for all vector channels
    HighContrast,   // Pure black & pure white
    Sepia,          // Warm sepia tone background & text
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvertColorsOptions {
    pub mode: Option<InvertMode>,
    pub pages: Option<String>, // e.g. "all", "1,3-5"
}
```

### 3.2 Penta-Interface Endpoints
1. **CLI**:
   ```bash
   paperpilot invert-colors --input doc.pdf --output dark.pdf --mode dark_mode
   ```
2. **MCP Tool**:
   - `pdf_invert_colors`: arguments `input`, `output`, `mode` (`dark_mode`, `true_invert`, `high_contrast`, `sepia`), `pages`.
3. **REST Gateway**:
   - `POST /api/v1/pdf/invert-colors` (JSON payload or multipart with `mode`).
4. **WASM / Desktop**:
   - Integrated into `OperationsPanel.svelte` under `🎨 Edit & Enhance`.
