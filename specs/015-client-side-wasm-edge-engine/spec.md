# Spec 015: Client-Side WebAssembly (WASM) & Edge Engine (`paperpilot-wasm`)

## Status: PROPOSED (Target: Phase 4.5)

## 1. Context & Motivation
PaperPilot's desktop application provides 44 native PDF tools with complete local privacy. However, acquiring users via a 50MB desktop installer (`.deb`, `.dmg`, `.exe`) has high friction for casual users, students, Chromebook owners, and mobile/tablet users.

By leveraging PaperPilot's pure native Rust engine (`lopdf`, `pdf-writer`, etc.), we can compile the core operations directly to **WebAssembly (`wasm32-unknown-unknown` and `wasm32-wasi`)**.
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
├── paperpilot-wasm/        # NEW: wasm-bindgen bindings & in-memory stream adapters
└── apps/
    ├── desktop/            # Svelte 5 + Tauri desktop app
    └── web/                # Zero-install browser demo (Svelte 5 + paperpilot-wasm)
```

### Crate: `paperpilot-wasm`
- **Target**: `wasm32-unknown-unknown`
- **Dependencies**: `wasm-bindgen`, `js-sys`, `web-sys`, `paperpilot-core`, `paperpilot-pdf`
- **Memory Model**: Pure in-memory `Uint8Array` / `Vec<u8>` buffers. Zero filesystem calls.

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
}
```

---

## 4. Phase 4.8 Task Breakdown

| # | Task | Objective | Status |
|---|---|---|---|
| **4.8.1** | `paperpilot-wasm` Complete Client-Side Engine | `paperpilot-wasm` crate with `wasm-bindgen`, 6 core in-memory operations (`merge`, `split`, `rotate`, `compress`, `encrypt`, `watermark`), and TypeScript Web Worker bridge (`pdfWorker.ts`) | **In-Flight** |
| **4.8.2** | Zero-Install Web App Demo | Web-based drag-and-drop tool suite deployed to Cloudflare Pages / GitHub Pages | Next |
| **4.8.3** | Cloudflare Workers Edge Microservice | Deploy `paperpilot-wasm` to Cloudflare Workers for sub-10ms, memory-only edge processing (0ms cold start, zero disk) | Backlog |

---

## 5. Security & Privacy Guarantees
- **Client-Side Web**: 100% execution in browser sandboxed memory. Network tab inspection proves zero bytes are transmitted to any server.
- **Edge Cloudflare Worker**: V8 Isolate memory execution. Physical zero-disk architecture (no disk storage exists in worker).

## 6. Consequences & Success Metrics
- **Instant User Onboarding**: 0-second barrier to entry; users test PaperPilot before downloading.
- **Hosting Overhead**: $0.00 compute cost on client-side web; $5/mo edge cost on Cloudflare.
- **Desktop Funnel Multiplier**: Substantially higher conversion from web traffic into desktop downloads.
