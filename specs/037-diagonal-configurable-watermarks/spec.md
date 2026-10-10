# Spec 037 — Diagonal, Semi-Transparent, and Configurable Watermarks (`pdf_watermark`)

## 1. Overview & Objectives
Watermarking is a fundamental document security and discovery workflow in PaperPilot. Previously, `pdf_watermark` applied an opaque text string horizontally fixed near the bottom-left coordinate `(100, 100)`.

This specification modernizes `pdf_watermark` to support:
1. **Diagonal (45°) Alignment by Default**: Center-stamped diagonally across the page from bottom-left to top-right, reflecting standard legal/enterprise document conventions (e.g. `CONFIDENTIAL`, `DRAFT`, `COPY`).
2. **True PDF Alpha Transparency (`ExtGState` / `ca`)**: Defaults to **20% opacity (0.2)** so underlying document text, forms, and tables remain 100% legible.
3. **Full User Adjustability**: Configurable rotation angle (`0°` horizontal, `45°` diagonal, `90°` vertical, or custom), opacity (`0.05` to `1.0`), font size, font color (`gray`, `red`, `blue`, hex), and page selection.
4. **Strict Penta-Interface Parity**: First-class support across all 5 deployment surfaces (CLI, MCP, REST API, WASM, Edge) and the Desktop UI.

---

## 2. Parameter Specification

| Parameter | Type | Default | Description |
|---|---|---|---|
| `text` | `String` | *(Required)* | The watermark text (e.g. `"CONFIDENTIAL"`). |
| `angle` | `f32` | `45.0` | Rotation angle in degrees (`45.0` diagonal, `0.0` horizontal). |
| `opacity` | `f32` | `0.2` | Alpha transparency between `0.0` (invisible) and `1.0` (opaque). |
| `font_size` | `Option<f32>` | `None` (auto ~54pt) | Font size in points. If `None`, dynamically scaled to page width. |
| `color` | `Option<String>` | `None` (`"#888888"`) | Color name (`"gray"`, `"red"`, `"blue"`) or hex code (`"#RRGGBB"`). |
| `pages` | `Option<String>` | `None` (all) | Page selector (e.g. `"1"`, `"1-3"`, `"all"`). |

---

## 3. Core Engine Architecture (`paperpilot-pdf`)

### 3.1 Mathematical Placement & Rotation Matrix
For a page of width $W$ and height $H$ (from `MediaBox`):
- Center origin: $(x_c, y_c) = (W / 2, H / 2)$.
- Angle in radians: $\theta = \text{angle} \times \frac{\pi}{180}$.
- For a default diagonal $45^\circ$:
  $$\cos(45^\circ) \approx 0.7071, \quad \sin(45^\circ) \approx 0.7071$$
- Text measurement offset: with font size $S$ and string length $L$, approximate text half-width is $\Delta x \approx \frac{L \times S \times 0.5}{2}$.
- Centered Transformation Matrix in PDF:
  ```pdf
  q
  /GS1 gs
  /F1 {font_size} Tf
  {r} {g} {b} rg
  {cos_theta} {sin_theta} {-sin_theta} {cos_theta} {x_trans} {y_trans} Tm
  ({escaped_text}) Tj
  Q
  ```

### 3.2 Transparent Graphics State (`ExtGState`)
To ensure text underneath the watermark remains readable:
1. Inject an `/ExtGState` dictionary into the page's `/Resources` dictionary:
   ```pdf
   /ExtGState << /GS1 << /Type /ExtGState /ca {opacity} /CA {opacity} >> >>
   ```
2. Activate `/GS1 gs` inside the watermark content stream.

---

## 4. Penta-Interface Implementation Surface

### Surface 1: 💻 Terminal CLI (`paperpilot-cli`)
```bash
paperpilot watermark --input in.pdf --output out.pdf --text "CONFIDENTIAL" --angle 45 --opacity 0.2 --color gray
```
Flags:
- `--text <TEXT>`: (required)
- `--angle <DEGREES>`: default `45.0`
- `--opacity <FLOAT>`: default `0.2`
- `--color <COLOR>`: default `"gray"`

### Surface 2: 🤖 MCP Server (`paperpilot-mcp`)
Tool: `pdf_watermark`
JSON Schema updated in `server.rs` to include optional `angle`, `opacity`, `font_size`, `color`.

### Surface 3: 🌐 Local REST API & OpenAPI (`paperpilot-gateway`)
Route: `POST /api/v1/pdf/watermark` and `POST /api/v1/pdf/tools/pdf_watermark`
JSON body:
```json
{
  "input": "/path/to/doc.pdf",
  "output": "/path/to/watermarked.pdf",
  "text": "CONFIDENTIAL",
  "angle": 45.0,
  "opacity": 0.2,
  "color": "gray"
}
```

### Surface 4: ⚡ Client-Side WebAssembly (`paperpilot-wasm`)
Function: `watermark(pdf_bytes: &[u8], text: &str, angle: Option<f32>, opacity: Option<f32>) -> Result<Vec<u8>, JsValue>`

### Surface 5: ☁️ Cloudflare Workers Edge (`apps/edge`)
Route: `POST /api/v1/watermark` with form-data or JSON parameters.

---

## 5. UI Integration (`apps/desktop`)
1. **`OperationsPanel.svelte`**:
   - Add watermark options:
     - Preset buttons: **[ Diagonal (45°) | Horizontal (0°) | Custom ]**
     - Opacity slider (`10%` to `100%`, default `20%`)
     - Color selector (`Gray`, `Red Alert`, `Blue`)
2. **`EditableActionCard.svelte`**:
   - Provide diagonal/horizontal toggle chip in the action plan card.

---

## 6. Verification & Quality Gates
- **Rust Unit Tests**: `cargo test -p paperpilot-pdf -- watermark`
- **Penta-Interface Tests**: 20 E2E assertions pass in `tools/penta-interface-e2e`
- **Visual & Conformance Tests**: `scripts/verify_pdf_conformance.sh` passes with `qpdf --check`.
