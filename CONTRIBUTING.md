# Contributing to PaperPilot

Thank you for your interest in contributing to PaperPilot! 🦀

PaperPilot is a 100% private, air-gapped, zero-cloud PDF and document intelligence toolkit written in pure Rust. Whether you're squashing bugs, adding new PDF operations, improving documentation, or optimizing the MCP surface, your contributions are deeply appreciated.

---

## 🧭 Core Principles

Every pull request must uphold PaperPilot's architectural guarantees:
1. **Local-First & Air-Gapped**: Zero cloud calls, zero external analytics, zero network phoning. Everything executes on the user's CPU, GPU, or browser tab.
2. **Memory Safety & Zero Panics**: PDF parsing must handle malformed, corrupted, circular, or weaponized streams gracefully (`Result::Err`). Never `unwrap()` or panic on user-supplied PDF bytes.
3. **Penta-Surface Parity**: When adding or changing a PDF operation, keep it accessible across all 5 surfaces where applicable:
   - **Core Engine** (`paperpilot-pdf`)
   - **MCP Server** (`paperpilot-mcp` for Claude, Codex, Cursor, Pi)
   - **CLI** (`paperpilot-cli`)
   - **Local REST Gateway** (`paperpilot-gateway` / port 7823)
   - **Browser WASM** (`paperpilot-wasm`)

---

## 🛠️ Development Setup

### Prerequisites
- **Rust**: 1.75+ (stable toolchain)
- **Node.js**: v18+ & **pnpm**: v9+ (for Desktop Tauri app & web packages)
- **Make**: for project orchestration

### Getting Started

```bash
# 1. Clone the repository
git clone https://github.com/KrushnaVardhanReddy/PaperPilot.git
cd PaperPilot

# 2. Checkout the active development branch (all PRs target feature/dev)
git checkout feature/dev

# 3. Create your feature branch
git checkout -b feature/my-cool-tool
```

### Building & Testing

Always test with bounded thread counts (`-j 2` or `-j 4`) to prevent memory exhaustion when compiling large dependencies:

```bash
# Build core CLI & MCP server
cargo build -p paperpilot-cli -p paperpilot-mcp -j 2

# Run workspace unit and integration tests
cargo test --workspace -j 2

# Run malformed PDF security regression tests
cargo test -p paperpilot-pdf --test malformed_regression -j 2

# Run the 5-interface end-to-end integration test
cargo run -p penta-interface-e2e -j 2
```

### Security & Lints Check

Before opening a pull request, run the local security audit:

```bash
# Runs Semgrep SAST, secret checks, and cargo audit
make audit
```

---

## 🌿 Branching Strategy & PR Workflow

- **Base Branch**: All contributions should target `feature/dev`. Direct pushes to `main` are restricted.
- **Commit Messages**: Follow standard Conventional Commits:
  - `feat(pdf): add lossless stream deduplication`
  - `fix(mcp): resolve stdio buffer truncation on large schema output`
  - `docs(readme): clarify Claude Desktop setup instructions`
  - `test(fuzz): add corrupted cross-reference table test fixture`

### Opening a Pull Request
1. Ensure your branch compiles with zero warnings: `cargo clippy --workspace --all-targets -j 2`.
2. Ensure automated tests pass: `cargo test --workspace -j 2`.
3. Include an explanation of the change, test coverage added, and any relevant issue number.
4. If modifying existing architectural patterns, run `graphify update .` to keep the codebase knowledge graph synchronized.

---

## 🙋 Getting Help & Discussions

- **GitHub Discussions**: Share ideas, workflows, or showcase client setups.
- **Issue Tracker**: Use our [Bug Report](https://github.com/KrushnaVardhanReddy/PaperPilot/issues/new?template=bug_report.yml) or [Feature Request](https://github.com/KrushnaVardhanReddy/PaperPilot/issues/new?template=feature_request.yml) templates.
- **Code of Conduct**: Be respectful, constructive, and kind to fellow builders.
