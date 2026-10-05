# PaperPilot v1.0 Launch Plan — Pending Free / Community Tier Tasks

> **Focus:** Remaining tasks required to finalize the **100% Free / Community Tier** ($0 Forever, 100% Local Processing, Zero Cloud Dependency).

---

## 📋 Pending Tasks Summary

| # | Task | Area | Description / Exit Criteria | Status |
|---|---|---|---|---|
| **4.8.1** | `paperpilot-wasm` Crate & Web Worker Bridge | Client-Side WASM | `paperpilot-wasm` crate with `wasm-bindgen`, in-memory buffer adapters for 6 core ops (`merge`, `split`, `rotate`, `compress`, `encrypt`, `watermark`), and TypeScript Web Worker bridge (`pdfWorker.ts`). | ✅ **Completed** (PR #135) |
| **4.8.2** | Zero-Install Web App Demo | Viral Growth Hook | Web-based drag-and-drop tool suite deployed to Cloudflare Pages / GitHub Pages. Executes 100% in browser memory with download CTA funneling to Desktop. | 📋 **Pending** (Ready to trigger) |
| **4.8.3** | Cloudflare Workers Edge Microservice | Edge Serverless | Deploy `paperpilot-wasm` to Cloudflare Workers for sub-10ms, memory-only edge processing with 0ms cold start and zero disk. | 📋 **Pending** |
| **4.1.8** | Embedded Documentation RAG (`sqlite-vec`) | Local AI Assistant | In-memory/embedded SQLite vector store indexing `TRI_INTERFACE_E2E_AND_DOCS.md` and user guides. Answers user questions directly in chat with exact copy-pasteable CLI/API snippets and 0 hallucinations. | ✅ **Completed** (PR #136) |
| **3.1.7** | iOS Target Build & Packaging | Mobile Port | Compile and test desktop frontend for iOS / iPadOS via Tauri mobile bindings. | 📋 **Pending** (Requires macOS / Xcode) |
| **R2.E4** | System Performance & Latency Re-Benchmark | Final Verification | End-to-end benchmark measuring canvas viewer page render latency, memory footprint across 50+ open tabs, and operation execution times. | 📋 **Pending** (Pre-release gate) |

---

## 🎯 Recommended Execution Sequence

1. **Finalize Task 4.8.1 (`paperpilot-wasm`)**: Review and merge Jules PR once `reports/WASM_CLIENT_ENGINE_REPORT.md` is generated.
2. **Trigger Task 4.8.2 (Zero-Install Web App Demo)**: Build the web frontend importing the compiled WASM package.
3. **Trigger Task 4.1.8 (Documentation RAG with `sqlite-vec`)**: Enable local instant help in the desktop AI chat.
4. **Run Task R2.E4 (System Performance Re-Benchmark)**: Measure final latencies and memory usage across all 44 tools and canvas viewer before v1.0 binary release.
