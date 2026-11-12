# Gemini-CLI Agent Delegation Prompt: Agent DX, Scalability & Project Maturity (Specs 159-163)

You are tasked with implementing the high-impact Developer Experience (DX), AI Agent, and Large-Scale Memory enhancements defined in Specs 159 through 163. Each feature is designed with modular boundaries to ensure zero merge conflicts across parallel execution branches.

---

## 🎯 Scope of Work

### Wave 1: Semantic Chunking Engine (Spec 159)
- **Crates affected**: `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`, `apps/desktop`
- **Core task**:
  1. Implement `pdf_chunk` in `crates/paperpilot-pdf/src/operations/pdf_chunk.rs` using the structures defined in `specs/159-agent-semantic-chunking/spec.md`.
  2. Implement heuristic token estimation (~4 chars/token) and structure-aware boundary detection (`#`, `##`, `\n\n`, table boundaries `|...|`).
  3. Support `--strip-boilerplate` to remove recurring running headers/footers across consecutive pages.
  4. Expose CLI command `paperpilot pdf-chunk`, MCP tool `pdf_chunk`, and Gateway route `POST /api/v1/pdf/chunk`.
  5. Register tool in `apps/desktop/src/lib/types/toolList.ts`.
  6. Add unit tests in `crates/paperpilot-pdf/tests/test_pdf_chunk.rs`.

---

### Wave 2: System Diagnostics Engine (Spec 160)
- **Crates affected**: `paperpilot-core`, `paperpilot-cli`, `paperpilot-gateway`, `apps/desktop`
- **Core task**:
  1. Implement `crates/paperpilot-core/src/diagnostics.rs` with probes for:
     - Gateway port 7823 availability
     - Temp directory write/delete permissions
     - Local model directory (`~/.cache/paperpilot/models`) existence & permissions
     - Disk space availability
  2. Implement CLI subcommand `paperpilot doctor` with clean ANSI terminal output.
  3. Add `GET /health` and `GET /api/v1/doctor` to `paperpilot-gateway`.
  4. Expose Tauri command `run_system_doctor` in `apps/desktop/src-tauri/src/lib.rs` and add a button in `SettingsPanel.svelte`.

---

### Wave 3: Universal Installers & Packaging (Spec 161)
- **Files affected**: `scripts/install.sh`, `scripts/install.ps1`, `packaging/homebrew/paperpilot.rb`
- **Core task**:
  1. Create POSIX `scripts/install.sh` that detects Darwin/Linux and x86_64/aarch64, downloads the latest release archive, validates checksums, and installs the binary.
  2. Create PowerShell `scripts/install.ps1` for Windows.
  3. Create Homebrew formula `packaging/homebrew/paperpilot.rb`.

---

### Wave 4: Framework Integration Recipes (Spec 162)
- **Files affected**: `recipes/` directory (isolated from core code to guarantee 0 merge conflicts)
- **Core task**:
  1. Create `recipes/langchain/paperpilot_loader.py` implementing `BaseLoader` via `http://localhost:7823/api/v1/pdf/chunk`.
  2. Create `recipes/llamaindex/paperpilot_reader.py` implementing `BaseReader`.
  3. Create `recipes/n8n/invoice-auto-ocr-pipeline.json` showing end-to-end hot folder invoice parsing.
  4. Create `recipes/mcp-presets/claude_desktop_config.json` and `cursor_mcp_config.json`.
  5. Create `recipes/README.md` cataloging all recipes.

---

### Wave 5: Zero-Copy Memory-Mapped Document Handling (Spec 163)
- **Crates affected**: `paperpilot-core`, `paperpilot-pdf`
- **Core task**:
  1. Implement `DocumentSource` in `crates/paperpilot-core/src/io/document_source.rs` using `memmap2` for files $\ge$ 10 MB on non-WASM targets.
  2. Strictly isolate with `#[cfg(not(target_arch = "wasm32"))]` so `paperpilot-wasm` remains 100% compliant.
  3. Integrate into read-only pipelines (`pdf_chunk`, `pdf_search`, `document_pii_scan`, `pdf_extract_patterns`).
  4. Verify flat resident memory usage (< 100 MB) when scanning large test files.

---

## 🔒 Strict Engineering Rules & Constraints
1. **Compilation Verification**: Local cargo checks must ALWAYS use `-j 4`:
   ```bash
   cargo check --workspace -j 4
   cargo test -p paperpilot-pdf -j 4
   ```
2. **Zero-Panic Mandate**: All PDF parsing must gracefully return `Result<T, PaperPilotError>` and never call `.unwrap()` or `.expect()` on untrusted document bytes.
3. **Purity**: Core operations in `paperpilot-pdf` must remain pure Rust with no external dynamic C++ dependencies.
4. **Svelte 5 Runes**: Any Desktop UI updates in `apps/desktop` must strictly use Svelte 5 runes (`$state`, `$derived`, `$props`) and never use legacy `export let`.
