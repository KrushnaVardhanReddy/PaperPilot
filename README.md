# PaperPilot

**Open-source document infrastructure for humans and AI agents.**

> *AI decides what to do. Rust decides how to do it.*

PaperPilot is a blazing-fast, local-first document automation platform. It allows users and AI agents to seamlessly manipulate PDF documents via a beautiful Desktop App, a Terminal CLI, natural language processing (NLP), or the Model Context Protocol (MCP).

---

## ✨ Features

- **🚀 Native Performance:** Powered entirely by Rust (`lopdf`) for sub-millisecond document manipulation. Capable of processing 10,000+ pages per second.
- **🧠 Offline NLP Engine:** Built-in ultra-compact ONNX embedding model (NeuML's `bert-hash-nano-embeddings`, <1M parameters, ~1.1MB ONNX) and Aho-Corasick rule engine. Type *"Merge these PDFs and rotate page 1"* and the local engine instantly executes the task without needing an API key.
- **💻 Svelte 5 + Tauri 2 Desktop App:** A sleek, modern GUI for managing your document workflows locally.
- **🤖 MCP Server:** First-class support for the Model Context Protocol. Expose all 30+ PDF operations securely to external AI agents (like Claude Desktop).
- **⚙️ Terminal CLI:** Fully featured command-line interface for scripting and headless environments.
- **🔒 Privacy First:** 100% local processing. Your documents never leave your machine unless you explicitly connect a cloud AI provider.

## 🏗️ Architecture

PaperPilot follows a strict separation of concerns, keeping the frontend thin and pushing all heavy lifting to the Rust core.

```text
Svelte 5 UI (Frontend)
    │
    ▼
Tauri 2 (IPC)
    │
    ▼
Rust Workspace (Backend)
    ├── paperpilot-core     # Core traits (PdfOperation)
    ├── paperpilot-pdf      # 45+ PDF Operations
    ├── paperpilot-nlp      # Offline Intent Classification
    ├── paperpilot-mcp      # JSON-RPC Server
    └── paperpilot-cli      # Terminal Interface
```

## 🚀 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) & [pnpm](https://pnpm.io/)
- Python 3.10+ (for NLP training scripts)

### Installation & Build

1. **Clone the repository**
   ```bash
   git clone https://github.com/KrushnaVardhanReddy/PaperPilot.git
   cd PaperPilot
   ```

2. **Run the Desktop App (Development)**
   ```bash
   make dev
   ```

3. **Run the Test Suite**
   ```bash
   make test-all
   ```

## 🛠️ Components

### 1. The CLI (`paperpilot-cli`)
Automate workflows from your terminal:
```bash
paperpilot merge --inputs doc1.pdf doc2.pdf --output merged.pdf
paperpilot rotate --input in.pdf --pages 1,3-5 --angle 90 --output out.pdf
```

### 2. The MCP Server (`paperpilot-mcp`)
Connect PaperPilot to Claude or other MCP-compatible clients. Provides a stdio JSON-RPC server exposing tools like `pdf_merge`, `pdf_extract_text`, `pdf_redact`, etc.

### 3. The NLP Engine (`paperpilot-nlp`)
A local-first hybrid intent classifier that maps natural language directly to Rust executable plans:
```rust
let resolver = OfflineNlpResolver::new(rule_engine, onnx_model);
let plan = resolver.resolve("Please decrypt secret.pdf using password123").unwrap();
```

## 🗺️ Roadmap & Phases

Check out [`PHASES.md`](./PHASES.md) for our detailed development roadmap. 
Currently in **Phase 4** — transitioning from the core PDF engine to advanced Desktop UI features (Native OS Menus, PDF Viewers, and Annotations).

## 📄 License
MIT License. See [LICENSE](LICENSE) for details.
