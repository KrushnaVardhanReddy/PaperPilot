# Spec 015: Client-Side WebAssembly (WASM) & Edge Engine (`paperpilot-wasm`)

## Status: APPROVED & ACTIVE (Phase 4.8)

## 1. Context & Motivation
PaperPilot's desktop application provides 44 native PDF tools with complete local privacy. However, acquiring users via a 50MB desktop installer (`.deb`, `.dmg`, `.exe`) has high friction for casual users, students, Chromebook owners, and mobile/tablet users.

By leveraging PaperPilot's pure native Rust engine (`lopdf`, `pdf-writer`, etc.), we compile the core operations directly to **WebAssembly (`wasm32-unknown-unknown`)**.
This unlocks:
1. **The Viral Growth Hook (Zero-Install Client-Side Web App)**: Users drag & drop files on `paperpilot.app` and execute core PDF operations (merge, split, rotate, compress, watermark, encrypt) **100% inside their browser memory** in <10ms. No files are uploaded to any server ($0 server cost).
2. **Cloudflare Workers Edge Microservice**: The same WASM module deploys into Cloudflare V8 Isolates across 300+ edge locations for sub-10ms API processing with a physical zero-disk guarantee.
3. **Desktop Download Funnel**: After experiencing instant browser operations, users are presented with a call-to-action to download PaperPilot Desktop for unlimited offline batch pipelines and local AI chat.

---

## 2. Architecture & Crate Boundaries

```
PaperPilot/
├── paperpilot-core/        # Shared core traits & error types
├── paperpilot-pdf/         # 44 PDF operation implementations
├── paperpilot-wasm/        # wasm-bindgen bindings & in-memory stream adapters
└── apps/
    ├── desktop/            # Svelte 5 + Tauri desktop app
    └── web/                # Zero-install browser demo (Vite + Svelte 5 / TS + paperpilot-wasm)
```

### Application: `apps/web/`
- **Framework**: Vite + Svelte 5 + TypeScript + Vanilla CSS (dark theme, glassmorphic accents matching PaperPilot design language).
- **Core Engine Integration**: Imports the Web Worker bridge or compiled WASM from `paperpilot-wasm` to run computations off the main UI thread.
- **Tools Included**:
  1. **Merge PDFs**: Multiple file drop, visual reorder, instant merge & download.
  2. **Split PDF**: Split by page ranges or into individual pages.
  3. **Rotate Pages**: 90°/180°/270° clockwise rotation.
  4. **Compress PDF**: In-browser byte deflation with percentage size reduction badge.
  5. **Encrypt PDF**: Standard password protection.
  6. **Watermark PDF**: Custom textual stamps.
  7. **Delete Pages**: Remove selected pages (e.g. "2, 4" or "1-3").
  8. **Extract Pages**: Extract target page range into a standalone PDF.
  9. **Reorder Pages**: Re-sequence document pages (e.g. "3, 1, 2").
  10. **Crop Pages**: Adjust document boundary box (`left, bottom, right, top`).
  11. **Flatten PDF**: Flatten interactive AcroForm fields into permanent page content.
  12. **Metadata Editor**: Read and modify Title, Author, Subject, and Keywords.
- **Privacy Assurance Badge**: Prominent banner confirming *"100% Client-Side. Your documents never leave your browser."*
- **Desktop Download CTA**: High-converting banner/footer driving visitors to download PaperPilot Desktop for 44+ tools and AI assistant.

---

## 3. Core WASM API Surface

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPdfEngine;

#[wasm_bindgen]
impl WasmPdfEngine {
    /// Merge multiple PDF file buffers into a single PDF
    #[wasm_bindgen]
    pub fn merge(files: js_sys::Array) -> Result<js_sys::Uint8Array, JsValue>;

    /// Rotate specific or all pages in a PDF buffer
    #[wasm_bindgen]
    pub fn rotate(input_bytes: &[u8], angle: u16, pages: &str) -> Result<js_sys::Uint8Array, JsValue>;

    /// Split a PDF buffer by page ranges (e.g. "1-2, 3-5")
    #[wasm_bindgen]
    pub fn split(input_bytes: &[u8], ranges: &str) -> Result<js_sys::Array, JsValue>;

    /// Compress an existing PDF buffer
    #[wasm_bindgen]
    pub fn compress(input_bytes: &[u8]) -> Result<js_sys::Uint8Array, JsValue>;

    /// Encrypt a PDF with password
    #[wasm_bindgen]
    pub fn encrypt(input_bytes: &[u8], user_password: &str) -> Result<js_sys::Uint8Array, JsValue>;

    /// Add text watermark
    #[wasm_bindgen]
    pub fn watermark(input_bytes: &[u8], text: &str) -> Result<js_sys::Uint8Array, JsValue>;

    /// Delete specific pages from a PDF buffer
    #[wasm_bindgen]
    pub fn delete_pages(input_bytes: &[u8], pages: &str) -> Result<js_sys::Uint8Array, JsValue>;

    /// Extract specific pages into a new PDF buffer
    #[wasm_bindgen]
    pub fn extract_pages(input_bytes: &[u8], pages: &str) -> Result<js_sys::Uint8Array, JsValue>;

    /// Reorder pages according to a 1-based order array (e.g. [3, 1, 2])
    #[wasm_bindgen]
    pub fn reorder_pages(input_bytes: &[u8], new_order: &[u32]) -> Result<js_sys::Uint8Array, JsValue>;

    /// Crop pages to a given bounding box
    #[wasm_bindgen]
    pub fn crop(input_bytes: &[u8], left: f32, bottom: f32, right: f32, top: f32) -> Result<js_sys::Uint8Array, JsValue>;

    /// Flatten form fields into regular PDF content
    #[wasm_bindgen]
    pub fn flatten(input_bytes: &[u8]) -> Result<js_sys::Uint8Array, JsValue>;

    /// Set PDF metadata (title, author, subject, keywords)
    #[wasm_bindgen]
    pub fn set_metadata(input_bytes: &[u8], title: Option<String>, author: Option<String>, subject: Option<String>, keywords: Option<String>) -> Result<js_sys::Uint8Array, JsValue>;
}
```

---

## 4. Phase 4.8 Task Breakdown

| # | Task | Objective | Status |
|---|---|---|---|
| **4.8.1** | `paperpilot-wasm` Complete Client-Side Engine | `paperpilot-wasm` crate with `wasm-bindgen`, 6 core in-memory operations (`merge`, `split`, `rotate`, `compress`, `encrypt`, `watermark`), and TypeScript Web Worker bridge (`pdfWorker.ts`) | ✅ **Completed** (PR #135) |
| **4.8.2** | Zero-Install Web App Demo | Web-based drag-and-drop tool suite in `apps/web/` deployed to Cloudflare Pages / GitHub Pages with desktop CTA | ✅ **Completed** (PR #137) |
| **4.8.4** | Expand WASM Engine & Web Suite to 12 Pure-Rust Tools | Implement `delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `flatten`, and `set_metadata` in `paperpilot-wasm` and add interactive UI cards in `apps/web/src/views/OperationsView.svelte` | ⏳ **In-Progress** |
| **4.8.3** | Cloudflare Workers Edge Microservice | Deploy `paperpilot-wasm` to Cloudflare Workers for sub-10ms, memory-only edge processing (0ms cold start, zero disk) | 📋 Pending |

---

## 5. Security & Privacy Guarantees
- **Client-Side Web**: 100% execution in browser sandboxed memory. Network tab inspection proves zero bytes are transmitted to any server.
- **Edge Cloudflare Worker**: V8 Isolate memory execution. Physical zero-disk architecture (no disk storage exists in worker).

## 6. Consequences & Success Metrics
- **Instant User Onboarding**: 0-second barrier to entry; users test PaperPilot before downloading.
- **Hosting Overhead**: $0.00 compute cost on client-side web; $5/mo edge cost on Cloudflare.
- **Desktop Funnel Multiplier**: Substantially higher conversion from web traffic into desktop downloads.
