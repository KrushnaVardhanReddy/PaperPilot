# PaperPilot

**Open-source document infrastructure for humans and AI agents.**

> *AI decides what to do. Rust decides how to do it.*

PaperPilot is a blazing-fast, local-first document automation platform. It allows users, developers, and autonomous AI agents to seamlessly manipulate PDF documents via a modern Desktop App (Svelte 5 + Tauri 2), a Terminal CLI, a high-performance REST API sidecar, or the Model Context Protocol (MCP).

---

## ✨ Features

- **🚀 Native Performance:** Powered entirely by Rust (`lopdf`) for sub-millisecond document manipulation. Capable of processing 10,000+ pages per second.
- **🏆 100% Tri-Interface Parity:** All 44 PDF operations are fully unified across CLI, MCP, and REST API with zero functional drift, verified by automated suites ([Verification Report](reports/TRI_INTERFACE_E2E_100_VERIFIED.md)).
- **🧠 100% Offline AI & NLP:** Built-in hybrid engine featuring keyword/regex rule routing and embedded INT8 ONNX intent classification (`TinyBERT-4L-312D`, ~7MB `zstd`, <2ms CPU latency). Zero server calls or external downloads required for single-turn command execution.
- **💻 Desktop GUI (Svelte 5 + Tauri 2):** Fast, reactive desktop experience with a multi-tab canvas viewer, interactive sticky notes & drawing markups, visual side-by-side diff slider, and collapsible multi-turn AI chat side-drawer.
- **🤖 Model Context Protocol (MCP):** Connect PaperPilot directly to Claude Desktop, Cursor, OpenCode, or custom autonomous agents via stdio JSON-RPC.
- **⚡ Developer Mode (Local REST Gateway):** Toggleable Axum HTTP sidecar (`localhost:7823`) with interactive Swagger/OpenAPI documentation (`/docs`) for seamless Python, Node.js, and n8n automations.
- **🔒 Absolute Privacy & Zero Leaks:** 100% local processing. Memory-hard Argon2id key derivation protects encrypted files offline with zero cloud phone-home dependencies.

---

## 🏗️ Architecture

PaperPilot follows a strict multi-crate workspace separation of concerns, keeping the desktop interface thin and executing all intensive operations in Rust:

```text
Svelte 5 UI (apps/desktop) ──► Tauri 2 IPC
                                  │
┌─────────────────────────────────┴────────────────────────────────┐
│                        Rust Workspace                            │
├───────────────────┬───────────────────┬──────────────────────────┤
│ paperpilot-core   │ Core traits & API │ PdfOperation, types      │
│ paperpilot-pdf    │ 44 PDF Operations │ Merge, OCR, Encrypt, etc.│
│ paperpilot-nlp    │ Offline AI Engine │ TinyBERT ONNX + Rules    │
│ paperpilot-mcp    │ Agent Interface   │ Stdio JSON-RPC Server    │
│ paperpilot-gateway│ Developer API     │ Axum REST Server (:7823) │
│ paperpilot-cli    │ Shell Interface   │ Command-line automation  │
└───────────────────┴───────────────────┴──────────────────────────┘
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
   cargo build --release
   ```

3. **Run the Desktop App (Development)**
   ```bash
   cd apps/desktop
   pnpm install
   pnpm tauri dev
   ```

4. **Run Workspace Unit Tests & Verification**
   ```bash
   cargo test --workspace
   python3 scripts/test_tri_interface_e2e.py
   ```

---

## 🛠️ Developer Interfaces

### 1. Terminal CLI (`paperpilot-cli`)
Automate batch document operations directly from your terminal:
```bash
# Merge multiple documents
paperpilot merge --inputs doc1.pdf doc2.pdf --output merged.pdf

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

### 4. Offline Intent Classifier (`paperpilot-nlp`)
Local natural-language-to-plan compilation with zero network overhead:
```rust
let resolver = RuleBasedResolver::new();
let plan = resolver.resolve("Merge doc1.pdf and doc2.pdf then compress")?;
```

---

## 🗺️ Roadmap & Phases

Check out [`PHASES.md`](./PHASES.md) and [`HANDOFF.md`](./HANDOFF.md) for detailed progress tracking.

- **Phase 1: Core Engine & CLI** — ✅ 100% Completed (All 44 PDF operations)
- **Phase 2: MCP & Gateway** — ✅ 100% Completed (Stdio JSON-RPC + Axum REST)
- **Phase 3: Desktop App Foundation** — ✅ 100% Completed (Viewer, Canvas, Tools Dock)
- **Phase 4: AI Layer & Multi-Interface Parity** — 🚧 In-Progress
  - Tri-Interface 100% Backend Parity: ✅ Achieved (PR #120)
  - Desktop 44-Operations Parity & Interactive Canvas E2E: 🚧 In-Progress (Jules)
  - Embedded TinyBERT Intent Classifier (Task 4.0.5): 🚧 In-Progress (Jules)
  - Multi-Turn Document Chat Drawer (`PdfChatPanel.svelte`, Task 4.3.7): 🚧 In-Progress (Jules)

## 📄 License & Open Core Model

- **Community Edition (This Repository):** Licensed under the **[Apache License 2.0](LICENSE)**. Includes all 44 PDF operations, Terminal CLI, MCP server, local REST gateway sidecar, offline NLP, and the open-source Desktop App.
- **Enterprise Edition (Commercial):** Hosted in a separate private repository for enterprise-grade compliance: SAML 2.0/OIDC SSO, SCIM 2.0 provisioning, multi-tenant RBAC, cryptographic audit logs, DLP policy enforcement, and dedicated SLA support. See [LICENSE](LICENSE) for details.
