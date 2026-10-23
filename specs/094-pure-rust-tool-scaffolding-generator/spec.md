# Spec 094: Pure-Rust Penta-Interface Tool Scaffolding Generator (`penta-scaffold`)

## 1. Background & Executive Summary
PaperPilot delivers unified PDF processing capabilities across a 5-interface surface model:
1. **Core Rust Engine** (`paperpilot-pdf`)
2. **Native CLI** (`paperpilot-cli`)
3. **Model Context Protocol (MCP)** Server (`paperpilot-mcp`)
4. **REST Gateway with Swagger UI & OpenAPI** (`paperpilot-gateway`)
5. **In-Browser WebAssembly Engine** (`paperpilot-wasm`)
6. **Cloudflare Edge Worker** (`apps/edge`)
7. **Desktop & Embed UI** (`apps/desktop` & `apps/embed`)
8. **Automated E2E Suite** (`tools/penta-interface-e2e`)

Currently, adding any new tool requires manually creating and wiring boilerplate code across 8 separate directories. This repetitive plumbing causes high developer overhead, slows down AI subagents (such as Jules), and leads to typing inconsistencies and merge conflicts.

**Spec 094** introduces `tools/penta-scaffold`: a standalone, pure-Rust CLI tool in the PaperPilot workspace that automatically generates end-to-end scaffolding across all 5 surfaces from a single command or specification file.

---

## 2. Technical Architecture & Design

### 2.1 Tool Invocation & CLI Parameters
The binary `penta-scaffold` will accept:
```bash
cargo run -p penta-scaffold -- \
  --name pdf_validate_signatures \
  --title "Validate Digital Signatures" \
  --category "security" \
  --icon "🔏" \
  --input-format "pdf" \
  --output-format "json" \
  --description "Cryptographic verification of digital signatures and certificate chains."
```
Or via Makefile shortcut:
```bash
make new-tool NAME=pdf_validate_signatures TITLE="Validate Digital Signatures" CATEGORY=security ICON="🔏" IN=pdf OUT=json DESC="..."
```

### 2.2 Generated Artifacts Matrix

| Surface / File | Generated Content |
|---|---|
| `paperpilot-pdf/src/operations/<name>.rs` | `struct <Name>Operation`, `PdfOperation` implementation, error handling, parameter struct, and `#[cfg(test)]` unit test scaffold. |
| `paperpilot-pdf/src/operations/mod.rs` | Adds `pub mod <name>;` and exports the operation. |
| `paperpilot-cli/src/cli.rs` | Adds clap `Commands::<Name>` variant with typed `--input`, `--output`, and optional flags. |
| `paperpilot-cli/src/commands/<name>.rs` | Subcommand execution handler invoking `paperpilot-pdf`. |
| `paperpilot-cli/src/commands/mod.rs` | Dispatches `Commands::<Name>` to `commands::<name>::execute()`. |
| `paperpilot-mcp/src/server.rs` | Injects tool declaration in `list_tools()` with JSON Schema properties, and adds branch in `call_tool()`. |
| `paperpilot-gateway/src/handlers.rs` | Adds Axum HTTP request handler (`POST /api/v1/pdf/<endpoint>`). |
| `paperpilot-gateway/src/server.rs` | Registers router endpoint and OpenAPI path documentation. |
| `paperpilot-wasm/src/lib.rs` | Exposes `WasmPdfEngine.<name>()` and registers into `list_tools()` catalog. |
| `apps/edge/src/index.ts` | Maps endpoint route to the WASM runner. |
| `apps/desktop/src/lib/types/toolList.ts` | Appends `ToolDefinition` with id, title, icon, category, formats, and description. |
| `tools/penta-interface-e2e/src/cases/<name>.rs` | Generates 10 unmocked test assertions across all 5 surfaces. |
| `tools/penta-interface-e2e/src/cases/mod.rs` & `main.rs` | Registers the test module into the E2E test runner. |

---

## 3. Reference Implementations
To ensure high fidelity and clean patterns, the generator templates must reference our two most robust existing cleanroom implementations:
1. **`pdf_sanitize`** (`Task 5.9.59` / `PR #228`):
   - Core runner: `paperpilot-pdf/src/operations/sanitize.rs`
   - CLI: `paperpilot-cli/src/commands/sanitize.rs`
   - MCP: `paperpilot-mcp/src/server.rs` (search `"pdf_sanitize"`)
   - E2E: `tools/penta-interface-e2e/src/cases/sanitize.rs`
2. **`document_pii_scan`** (`Task 5.9.60` / `PR #232`):
   - Core runner: `paperpilot-pdf/src/operations/pii.rs`
   - CLI: `paperpilot-cli/src/commands/pii.rs`
   - MCP: `paperpilot-mcp/src/server.rs` (search `"document_pii_scan"`)
   - E2E: `tools/penta-interface-e2e/src/cases/pii.rs`

---

## 4. Verification & Validation
1. **Compilation**: `cargo check -p penta-scaffold` succeeds with zero warnings.
2. **Dry-Run Test**:
   - Run `cargo run -p penta-scaffold -- --name pdf_sample_test --title "Sample Test" --category "organize" --icon "🧪" --input-format "pdf" --output-format "pdf" --description "Sample test tool" --dry-run` and verify code output without touching filesystem.
3. **Execution Test**:
   - Run generation for a test tool, run `cargo check --workspace` to ensure all generated code compiles cleanly across all workspace crates.
