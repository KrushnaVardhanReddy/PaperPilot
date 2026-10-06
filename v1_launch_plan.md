# PaperPilot v1.0 Launch Plan — Pending Free / Community Tier Tasks

> **Focus:** Remaining tasks required to finalize the **100% Free / Community Tier** ($0 Forever, 100% Local Processing, Zero Cloud Dependency).

---

## 📋 Pending Tasks Summary

| # | Task | Area | Description / Exit Criteria | Status |
|---|---|---|---|---|
| **4.8.1** | `paperpilot-wasm` Crate & Web Worker Bridge | Client-Side WASM | `paperpilot-wasm` crate with `wasm-bindgen`, in-memory buffer adapters for 6 core ops (`merge`, `split`, `rotate`, `compress`, `encrypt`, `watermark`), and TypeScript Web Worker bridge (`pdfWorker.ts`). | ✅ **Completed** (PR #135) |
| **4.8.2** | Zero-Install Web App Demo | Viral Growth Hook | Web-based drag-and-drop tool suite deployed to Cloudflare Pages / GitHub Pages. Executes 100% in browser memory with download CTA funneling to Desktop. | ✅ **Completed** (PR #137) |
| **4.8.4** | Expand Web Suite to 12 Pure-Rust Tools | Web App Expansion | Add 6 additional in-browser tools (`delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `flatten`, `set_metadata`) to `paperpilot-wasm` and `apps/web/`. | ✅ **Completed** (PR #138) |
| **4.8.5** | WASM 12 Tools Playwright E2E Suite | In-Browser E2E Parity | Data-driven Playwright test suite for all 12 WASM tools, pure-Rust memory encryption, and scorecard verification. | ✅ **Completed** (PR #139) |
| **4.8.3** | Cloudflare Workers Edge Microservice | Edge Serverless | Deploy `paperpilot-wasm` to Cloudflare Workers for sub-10ms, memory-only edge processing with 0ms cold start and zero disk. | ✅ **Completed** (PR #140) |
| **5.2.1** | Pure-Rust OCR Engine Migration (`ocrs`) | Zero-C++ OCR | Replace external C++ Tesseract/Leptonica with Robert Knight's pure-Rust `ocrs` and `rten` SIMD engine with graceful fallback. | ✅ **Completed** (PR #141) |
| **5.3.1** | Pure-Rust PDF Rasterization & Rendering | Zero-C++ Rendering | Replace Google PDFium (`libpdfium.so`) with pure-Rust `hayro` vector interpreter and `tiny-skia` 2D rasterizer in `paperpilot-pdf/src/operations/render.rs`. | ✅ **Completed** (PR #142) |
| **5.1.1** | Pure-Rust Image & Hash WASM Expansion (15 Tools) | Client-Side WASM | Expand `paperpilot-wasm` and `apps/web/` with in-memory `images_to_pdf`, `extract_images`, and `pdf_hash` via `image` and `sha2`. | ✅ **Completed** (PR #143) |
| **5.5.1** | Re-Export WASM Engine Post-Migration (22 Tools) | WASM & Edge Expansion | Re-export pure-Rust operations (OCR, Render, Hash, Images, Text Extraction, Decrypt, Numbering) into `paperpilot-wasm`, rebuild, and sync to `apps/web/` & `apps/edge/` (expanding to 22 in-browser tools). | ✅ **Completed** (Session 8898129496130614008) |
| **4.9.1** | `embed.js` Universal Drop-In CDN Script | Embedded Web Distribution | Lightweight JS loader (< 5KB gzip) served via Cloudflare CDN for embedding PaperPilot into any website via 2 lines of HTML. | ⏳ **In Flight** (Session 11292841270028301362) |
| **4.9.2** | Brandable Svelte 5 Embed Widget (`apps/embed/`) | Embedded Web Distribution | Configurable, responsive, iframe-safe Svelte 5 embed UI with Shadow DOM isolation, dynamic theme tokens, and viral badge. | ⏳ **In Flight** (Session 11292841270028301362) |
| **4.9.9** | Embed E2E Playwright Suite | Embedded Web Distribution | Playwright tests asserting `embed.js` mounting, Shadow DOM style isolation, and end-to-end PDF processing in fixture HTML. | ⏳ **In Flight** (Session 11292841270028301362) |
| **4.9.11** | `usepaperpilot.com` Official Web Portal | Web Distribution & Growth | Official portal on Cloudflare Pages with live Hero Embed Playground, no-code Embed Generator, and Swagger/OpenAPI explorer. | ⏳ **In Flight** (Session 6987262726422123273) |
| **4.9.12** | `docs.usepaperpilot.com` Official Docs Hub | Developer Documentation | 4-pillar documentation portal: Getting Started, 44-tool Tri-Interface reference, `embed.js` integration, and Swagger / Claude Desktop MCP setup. | 📋 **Pending** (v1.0 Co-Launch) |
| **4.9.13** | Certificate & Template Studio | Growth & Bulk Generation | Vector template engine, live browser customizer, and client-side CSV bulk generator (Spec 023) with viral verification watermark. | 📋 **Planned** (Spec 023) |
| **4.1.8** | Embedded Documentation RAG (`sqlite-vec`) | Local AI Assistant | In-memory/embedded SQLite vector store indexing `TRI_INTERFACE_E2E_AND_DOCS.md` and user guides. Answers user questions directly in chat with exact copy-pasteable CLI/API snippets and 0 hallucinations. | ✅ **Completed** (PR #136) |
| **3.1.7** | iOS Target Build & Packaging | Mobile Port | Compile and test desktop frontend for iOS / iPadOS via Tauri mobile bindings. | 📋 **Pending** (Requires macOS / Xcode) |
| **R2.E4** | System Performance & Latency Re-Benchmark | Final Verification | End-to-end benchmark measuring canvas viewer page render latency, memory footprint across 50+ open tabs, and operation execution times. | 📋 **Pending** (Pre-release gate) |

---

## 🎯 Recommended Execution Sequence

1. **Execute Task 5.5.1 (WASM Re-Export & Edge Sync)**: Now that all pure-Rust migrations have landed (OCR PR #141, Render PR #142, and 15 Tools WASM PR #143), re-export OCR & Render into `paperpilot-wasm`, expanding to 20+ in-browser and edge tools.
2. **Execute Phase 4.9 Core (Embedded Web: 4.9.1 + 4.9.2 + 4.9.9)**: Build `embed.js` CDN loader and `apps/embed/` Svelte 5 widget for simultaneous v1.0 release alongside Free Desktop and Cloudflare Edge.
3. **Execute Task R2.E4 (System Performance & Latency Re-Benchmark)**: Measure final latencies, memory usage across all 44 tools, and canvas viewer rendering speed before v1.0 binary release.
4. **Trigger Task 3.1.7 (iOS Target Build & Packaging)**: Package desktop frontend for iOS / iPadOS via Tauri mobile bindings once macOS/Xcode environment is available.


