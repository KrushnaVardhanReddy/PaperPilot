# PaperPilot

**Open-source document infrastructure for humans and AI agents.**

> *AI decides what to do. Rust decides how to do it.*

PaperPilot is a blazing-fast, local-first document automation platform. It allows users, developers, and autonomous AI agents to seamlessly manipulate PDF documents via a modern Desktop App (Svelte 5 + Tauri 2), a Terminal CLI, a high-performance REST API sidecar, or the Model Context Protocol (MCP).

---

## ✨ Features

- **🚀 Native Performance & Zero-Chrome:** Powered entirely by pure-Rust (`lopdf`, `fulgur`, `hayro`, `ocrs`, `rten`) for sub-millisecond document manipulation and sub-60ms office conversions. Zero external Chromium or C++ runtime dependencies.
- **🏆 100% Multi-Surface Parity:** All 44 PDF operations unified across CLI, MCP, and REST API, with 22 pure-Rust operations running directly client-side in the browser via WASM and serverless on Cloudflare Edge ([Verification Report](reports/TRI_INTERFACE_E2E_100_VERIFIED.md)).
- **🧠 100% Offline AI & NLP:** Built-in hybrid engine featuring keyword/regex rule routing and embedded on-device intent classification (`TinyBERT-4L-312D` via pure-Rust SIMD tensor engine, <2ms latency). Zero server calls or external downloads required for command execution.
- **💻 Desktop GUI (Svelte 5 + Tauri 2):** Fast, reactive desktop experience with a multi-tab canvas viewer, interactive sticky notes & drawing markups, visual side-by-side diff slider, and collapsible multi-turn AI chat side-drawer.
- **🌐 Client-Side Browser WASM & Edge Microservice:** Zero-install web suite (`apps/web`) executing 100% in browser memory with zero server upload, plus a Cloudflare Workers edge microservice (`apps/edge`) delivering sub-10ms serverless PDF processing.
- **📦 Drop-In Embeddable Widget (`embed.js`):** Embed a full-featured, branded PDF processing portal into any website with just two lines of HTML.
- **🤖 Model Context Protocol (MCP):** Connect PaperPilot directly to Claude Desktop, Cursor, OpenCode, or custom autonomous agents via stdio JSON-RPC.
- **⚡ Developer Mode (Local REST Gateway):** Toggleable Axum HTTP sidecar (`localhost:7823`) with interactive Swagger/OpenAPI documentation (`/docs`) for seamless Python, Node.js, and n8n automations.
- **🔒 Absolute Privacy & Zero Leaks:** 100% local processing. Memory-hard Argon2id key derivation protects encrypted files offline with zero cloud phone-home dependencies. Verified by 1-hour sustained soak tests ([Benchmark Report](reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md)).

---

## 🏗️ Architecture

PaperPilot follows a strict multi-crate workspace separation of concerns, keeping interfaces thin while executing all compute-intensive operations in pure Rust:

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                                   Developer Surfaces                                   │
├─────────────────────┬────────────────────┬─────────────────────┬───────────────────────┤
│ Desktop App (Tauri) │ Terminal CLI (Rust)│ Gateway REST (Axum) │ Agent MCP (JSON-RPC)  │
│ Svelte 5 + Canvas   │ `paperpilot-cli`   │ `localhost:7823`    │ Claude / Cursor / Stdio│
└──────────┬──────────┴─────────┬──────────┴──────────┬──────────┴───────────┬───────────┘
           │                    │                     │                      │
┌──────────▼────────────────────▼─────────────────────▼──────────────────────▼───────────┐
│                                 Rust Core Workspace                                    │
├───────────────────┬──────────────────────────────────────────┬─────────────────────────┤
│ paperpilot-core   │ Shared traits, data models & abstractions│ PdfOperation, types     │
│ paperpilot-pdf    │ 44 PDF operations (Pure-Rust engine)     │ lopdf, fulgur, hayro    │
│ paperpilot-nlp    │ Offline AI intent & entity engine        │ TinyBERT, rten, rules   │
│ paperpilot-gateway│ Axum HTTP REST server & OpenAPI engine   │ Swagger UI, healthcheck │
│ paperpilot-mcp    │ Model Context Protocol server            │ Stdio JSON-RPC protocol │
│ paperpilot-wasm   │ In-memory WebAssembly bindings           │ Browser & Cloudflare    │
└───────────────────┴──────────────────────────────────────────┴─────────────────────────┘
           │                                                                 │
           ▼                                                                 ▼
┌──────────────────────────────────────┐                   ┌─────────────────────────────┐
│       Client-Side Web & Embed        │                   │    Cloudflare Edge Server   │
│  apps/web (In-Browser 22 WASM tools) │                   │  apps/edge (Sub-10ms REST)  │
│  apps/embed (Drop-in embed.js CDN)   │                   │  Cloudflare Workers Engine  │
└──────────────────────────────────────┘                   └─────────────────────────────┘
```

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (1.75+ stable)
- [Node.js](https://nodejs.org/) & [pnpm](https://pnpm.io/)
- Python 3.10+ (for E2E verification suites)

### Installation & Build

1. **Clone the repository**
   ```bash
   git clone https://github.com/KrushnaVardhanReddy/PaperPilot.git
   cd PaperPilot
   ```

2. **Build the Rust Workspace & CLI**
   ```bash
   cargo build --release -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway
   ```

3. **Run the Desktop App (Development)**
   ```bash
   cd apps/desktop
   pnpm install
   pnpm tauri dev
   ```

4. **Build Production Standalone Desktop App**
   ```bash
   make build-release
   ```

5. **Run Workspace Unit Tests & Verification**
   ```bash
   cargo test --workspace -- -j 2
   python3 scripts/test_tri_interface_e2e.py
   ```

---

## 🛠️ Developer Interfaces

### 1. Terminal CLI (`paperpilot-cli`)
Automate batch document operations directly from your terminal:
```bash
# Merge multiple documents
paperpilot merge --inputs doc1.pdf doc2.pdf --output merged.pdf

# High-ratio image compression with quality control
paperpilot compress --input in.pdf --quality 75 --output compressed.pdf

# Rotate specific page ranges
paperpilot rotate --input in.pdf --pages 1,3-5 --angle 90 --output out.pdf

# Run headless REST/MCP server
paperpilot serve --port 7823
```

### 2. Local REST Gateway & OpenAPI (`paperpilot-gateway`)
Run automations from Python, Bash, or CI/CD pipelines via standard HTTP:
```bash
curl -X POST http://localhost:7823/api/v1/pdf/merge \
  -H "Content-Type: application/json" \
  -d '{"inputs": ["a.pdf", "b.pdf"], "output": "out.pdf"}'
```
Interactive documentation is available at `http://localhost:7823/docs` when Developer Mode is active.

### 3. Model Context Protocol (`paperpilot-mcp`)
Add PaperPilot tools to Claude Desktop (`claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "/path/to/paperpilot-mcp"
    }
  }
}
```

### 4. Client-Side Browser WASM & Embed Script
Embed client-side PDF processing into any web page without uploading files to a server:
```html
<script src="https://cdn.usepaperpilot.com/embed.js" data-tools="merge,compress,ocr"></script>
<div id="paperpilot-widget"></div>
```

---

## 🗺️ Roadmap & Phases

Check out [`PHASES.md`](./PHASES.md), [`v1_launch_plan.md`](./v1_launch_plan.md), and [`HANDOFF.md`](./HANDOFF.md) for detailed progress tracking.

- **Phase 1: Core Engine & CLI** — ✅ 100% Completed (All 44 PDF operations)
- **Phase 2: MCP & Gateway** — ✅ 100% Completed (Stdio JSON-RPC + Axum REST)
- **Phase 3: Desktop App Foundation** — ✅ 100% Completed (Viewer, Canvas, Tools Dock)
- **Phase 4: AI Layer, Multi-Interface Parity & Web** — ✅ 100% Completed
  - Tri-Interface 100% Parity: ✅ Achieved (PR #120, PR #123)
  - Embedded Documentation RAG (`sqlite-vec`): ✅ Completed (PR #136)
  - Client-Side WASM Web App (`apps/web`): ✅ Completed (PR #137)
  - Cloudflare Edge Microservice (`apps/edge`): ✅ Completed (PR #140)
  - Universal Embed Widget (`embed.js`): ✅ Completed (PR #145)
- **Phase 5: Zero-Chrome Pure-Rust Migrations** — ✅ 100% Completed
  - Pure-Rust OCR Engine (`ocrs` via `rten` SIMD): ✅ Completed (PR #141)
  - Pure-Rust PDF Rasterizer (`hayro` + `tiny-skia`): ✅ Completed (PR #142)
  - Pure-Rust Sub-60ms Office Conversions (`fulgur`): ✅ Completed (PR #150)
  - Universal High-Ratio Image Compression: ✅ Completed (`16f8ede`)
  - 1-Hour Sustained Soak & Endurance Benchmark (350K+ ops, 0 leaks): ✅ Completed (PR #151)
- **Active v1.0 Launch Verification Gates:**
  - `4.E2E.R4`: Tri-Interface Deep Behavioral Assertions Matrix ⏳ In Flight
  - `5.9.1`: Desktop Release Binary Size Optimization (<75MB) & `rten` Inference Migration ⏳ In Flight

## 📄 License & Open Core Model

- **Community Edition (This Repository):** Licensed under the **[Apache License 2.0](LICENSE)**. Includes all 44 PDF operations, Terminal CLI, MCP server, local REST gateway sidecar, offline NLP, client-side WASM engine, and the open-source Desktop App.
- **Enterprise Edition (Commercial):** Hosted in a separate private repository for enterprise compliance: SAML 2.0/OIDC SSO, SCIM 2.0 provisioning, multi-tenant RBAC, cryptographic audit logs, DLP policy enforcement, and dedicated SLA support. See [LICENSE](LICENSE) for details.
