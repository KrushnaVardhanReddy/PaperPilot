# Spec 020: Pure-Rust WASM 22-Tools Suite Expansion

## Status: ACTIVE (Phase 5.5.1)

## 1. Context & Motivation
Following the successful migrations to 100% pure-Rust rendering (`hayro` + `tiny-skia`, PR #142) and neural OCR (`ocrs` via `rten` SIMD, PR #141), plus the Phase 5.1.1 15-tool WASM suite (PR #143), PaperPilot is now capable of executing advanced PDF rasterization, text extraction, OCR, and document manipulation directly in WebAssembly without OS-level C++ binaries or external services.

This specification expands the client-side WebAssembly engine (`paperpilot-wasm`), the Web demo (`apps/web/`), and the Cloudflare Edge microservice (`apps/edge/`) from 15 tools to **22 tools**.

---

## 2. Tool Architecture & Scope

The 22 tools are divided into three functional categories:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       PaperPilot WASM 22-Tool Suite                         │
├──────────────────────────┬──────────────────────────┬───────────────────────┤
│    Original 12 Tools     │    Phase 5.1.1 (3)       │    Phase 5.5.1 (7)    │
│    (Core Layout/Struct)  │    (Image & Hash)        │    (Render/OCR/Info)  │
├──────────────────────────┼──────────────────────────┼───────────────────────┤
│ 1.  merge                │ 13. images_to_pdf        │ 16. render_page       │
│ 2.  split                │ 14. extract_images       │ 17. extract_text      │
│ 3.  rotate               │ 15. pdf_hash             │ 18. decrypt           │
│ 4.  compress             │                          │ 19. page_numbers      │
│ 5.  encrypt              │                          │ 20. header_footer     │
│ 6.  watermark            │                          │ 21. pdf_info          │
│ 7.  delete_pages         │                          │ 22. ocr               │
│ 8.  extract_pages        │                          │                       │
│ 9.  reorder_pages        │                          │                       │
│ 10. crop                 │                          │                       │
│ 11. flatten              │                          │                       │
│ 12. set_metadata         │                          │                       │
└──────────────────────────┴──────────────────────────┴───────────────────────┘
```

---

## 3. Rust API Signatures (`paperpilot-wasm`)

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl WasmPdfEngine {
    // --- 7 New Operations for 22 Tools Suite ---

    /// 16. Render PDF page to PNG image buffer via pure-Rust hayro + tiny-skia
    #[wasm_bindgen]
    pub fn render_page(input: &[u8], page_index: u32, scale: f32) -> Result<Vec<u8>, JsValue>;

    /// 17. Extract all textual content from document streams
    #[wasm_bindgen]
    pub fn extract_text(input: &[u8]) -> Result<String, JsValue>;

    /// 18. Decrypt password-protected PDF document
    #[wasm_bindgen]
    pub fn decrypt(input: &[u8], password: &str) -> Result<Vec<u8>, JsValue>;

    /// 19. Stamp dynamic page numbers ("Page X of Y")
    #[wasm_bindgen]
    pub fn page_numbers(input: &[u8], format: &str, position: &str) -> Result<Vec<u8>, JsValue>;

    /// 20. Add custom headers and footers to document pages
    #[wasm_bindgen]
    pub fn header_footer(input: &[u8], header: &str, footer: &str) -> Result<Vec<u8>, JsValue>;

    /// 21. Inspect PDF structure (page count, version, encryption, metadata)
    #[wasm_bindgen]
    pub fn pdf_info(input: &[u8]) -> Result<String, JsValue>;

    /// 22. Perform client-side neural OCR via ocrs and rten SIMD
    #[wasm_bindgen]
    pub fn ocr(input: &[u8]) -> Result<String, JsValue>;
}
```

---

## 4. Web Worker & Frontend Integration

### `apps/web/src/lib/wasm/index.ts`
All 7 new tools are mapped to asynchronous client calls that dispatch through the background Web Worker (`pdfWorker.ts`), preventing any freezing of the UI thread during rasterization or OCR computation.

### `apps/web/src/views/OperationsView.svelte`
- Provide dedicated card components for each new tool.
- Dynamic input controls (page slider for `render_page`, password field for `decrypt`, position dropdown for `page_numbers`, text preview box for `extract_text` & `ocr`, JSON viewer for `pdf_info`).

---

## 5. Verification & Quality Gate
1. **Cargo Unit Tests**: 100% pass on `cargo test -p paperpilot-wasm` across all 22 operations.
2. **Playwright E2E Parity**: `apps/web/tests/e2e_wasm_22_tools.spec.ts` executing end-to-end browser operations against production preview build.
3. **Artifacts**:
   - `reports/WASM_22_TOOLS_E2E_SCORECARD.md`
   - `reports/WASM_22_TOOLS_EXPANSION_REPORT.md`
   - `wiki/13-Wasm-22-Tools-Suite.md`
