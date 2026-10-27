## Phase 10 — WebAssembly (WASM) Target

**Goal:** Compile `paperpilot-core` and `paperpilot-pdf` to WASM so PDF processing can run entirely in-browser — enabling a zero-install web app, powerful browser extensions, and embeddable SDKs for third-party websites.

**Why WASM matters for PaperPilot:**
- Our Rust core is already platform-agnostic — WASM is a natural compilation target.
- Enables a `paperpilot.app` web version that processes files 100% locally in the browser (zero upload = perfect privacy).
- The Browser Extension (Phase 8.8) can run the full PDF engine in-page without needing the native app installed.
- Third-party developers can embed `paperpilot-wasm` in their own web apps as a drop-in library.

**[BLOCKED BY Phase 3 / Phase 2 MCP]**

### 10.1 — WASM Core Build
| # | Task | Notes |
|---|---|---|
| 10.1.1 | Audit `paperpilot-core` for WASM compatibility | Replace any `std::fs` / `std::path` calls with WASM-safe equivalents (e.g., in-memory byte slices) |
| 10.1.2 | Add `wasm32-unknown-unknown` target | Add to workspace `Cargo.toml` and CI matrix |
| 10.1.3 | Create `paperpilot-wasm` crate | Thin `wasm-bindgen` wrapper that exposes core operations as JS-callable functions |
| 10.1.4 | Build WASM bundle | `wasm-pack build --target web` — outputs `paperpilot_wasm.js` + `paperpilot_wasm_bg.wasm` |
| 10.1.5 | Publish to npm as `@paperpilot/wasm` | Versioned npm package for easy consumption by web projects |

### 10.2 — Web App (paperpilot.app)
| # | Task | Notes |
|---|---|---|
| 10.2.1 | New SvelteKit web app | `apps/web` — same component library as the desktop, but without Tauri IPC |
| 10.2.2 | File drag-and-drop via browser File API | Accept PDF uploads into in-memory `Uint8Array` |
| 10.2.3 | WASM-based operations | All PDF processing via `@paperpilot/wasm` — zero server upload |
| 10.2.4 | Progressive Web App (PWA) | Add service worker + manifest so users can "install" paperpilot.app to their desktop |
| 10.2.5 | File System Access API integration | Directly read/write to local disk on Chrome/Edge without downloading |

### 10.3 — WASM CI & Testing
| # | Task | Notes |
|---|---|---|
| 10.3.1 | WASM unit tests | `wasm-pack test --headless --chrome` for all core operations |
| 10.3.2 | Playwright WASM E2E | Spin up the web app and run the same Playwright tests as the desktop |
| 10.3.3 | Bundle size tracking | Fail CI if WASM bundle exceeds 5 MB (keep it fast to load) |

### 10.4 — In-Browser SLM Model Runtime & WebGPU Acceleration `[Spec 130]`
| # | Task | Notes |
|---|---|---|
| 10.4.1 | INT4 ONNX Quantization Pipeline | Export fine-tuned `PaperPilot-Qwen-0.5B` to a compact ~300 MB INT4 ONNX binary optimized for browser WebGPU and WASM SIMD. |
| 10.4.2 | Persistent Browser Cache Storage (`ModelCacheStorage.ts`) | Streaming download with visual progress bar; stores weights in browser `caches` API so users only download once. |
| 10.4.3 | WebGPU / WASM Hybrid Inference Engine | Connects `onnxruntime-web` with automatic fallback: WebGPU (sub-50ms) ➔ CPU WASM SIMD. |
| 10.4.4 | "Try in Web ➔ Power-up in Desktop" Conversion Funnel | Subtle banner and CTA encouraging users needing uncapped file sizes, full directory watching, and Coding Agent (MCP) support to install the Desktop app. |

---

## Architecture Improvements — Hexagonal / Ports & Adapters

**Goal:** Harden the Hexagonal Architecture foundations already present in PaperPilot so the codebase stays infrastructure-agnostic as we scale to cloud and WASM.

**Strategy:** These are incremental, non-breaking additions to `paperpilot-core`. No operation code needs to change. Adapters are added alongside the existing direct-write pattern.

---

### A.1 — StoragePort (Output Adapter)

| # | Task | Notes |
|---|---|---|
| A.1.1 | `StoragePort` trait ✅ | Defined in `paperpilot-core/src/ports.rs` — `write`, `read`, `exists` |
| A.1.2 | `LocalFileStorage` adapter ✅ | Default impl — wraps `std::fs`; used in CLI and desktop |
| A.1.3 | `MemoryStorage` adapter ✅ | In-memory map — use in unit tests, eliminates all temp file setup |
| A.1.4 | Wire `StoragePort` into CLI commands | Replace `std::fs::write` calls in `paperpilot-cli` with `LocalFileStorage` |
| A.1.5 | Wire `StoragePort` into MCP execute arms | Replace direct file writes in `paperpilot-mcp/src/server.rs` with injected port |
| A.1.6 | `S3Storage` adapter | Implements `StoragePort` via `aws-sdk-s3`; used in Cloud phase (Phase 9) |
| A.1.7 | Refactor all operation tests to use `MemoryStorage` | Eliminates all `tempfile` usage from tests |

---

### A.2 — PdfBackend Port (Input Adapter)

| # | Task | Notes |
|---|---|---|
| A.2.1 | `PdfBackend` trait ✅ | Defined in `paperpilot-core/src/ports.rs` — `load_from_bytes`, `load_from_path` |
| A.2.2 | `LopdfBackend` adapter | Implement `PdfBackend` for `lopdf` in `paperpilot-pdf` — wraps `LopdfDocument::load` |
| A.2.3 | Wire `PdfBackend` into MCP server | Replace `LopdfDocument::load(...)` hardcoded calls with injected `PdfBackend` trait object |
| A.2.4 | `PdfiumBackend` adapter (Future) | Swap in `pdfium-render` for higher-fidelity rendering, no operation changes needed |
| A.2.5 | `MuPdfBackend` adapter (Future) | Use MuPDF for OCR-heavy workflows via FFI |

---

### A.3 — Dependency Inversion in CLI & MCP

| # | Task | Notes |
|---|---|---|
| A.3.1 | `AppContext` struct | Single struct holding `Box<dyn StoragePort>` + `Box<dyn PdfBackend>` — injected at startup |
| A.3.2 | CLI uses `AppContext` | `paperpilot-cli/src/main.rs` builds `AppContext` and passes to all command handlers |
| A.3.3 | MCP uses `AppContext` | `PaperPilotMcpServer` holds `Arc<AppContext>` — all tools use it instead of hardcoded paths |
| A.3.4 | Integration test harness | Boot the full app with `MemoryStorage` + `LopdfBackend` — true end-to-end, no file I/O |

---

