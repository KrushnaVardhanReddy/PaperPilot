# Task 5.9.79: Pure-Rust Tool Scaffolding Generator (`tools/penta-scaffold`)

## Status
- Spec: [Spec 094](file:///home/krushna/Project/PaperPilot/specs/094-pure-rust-tool-scaffolding-generator/spec.md)
- Target: Wave 3 (`v0.7.0`) Developer Ergonomics & Rapid Expansion Engine
- Status: ⏳ In-Flight (Jules Session [`8915713543148419332`](https://jules.google.com/session/8915713543148419332))

---

## Objectives & Work Items

### 1. New Crate Creation (`tools/penta-scaffold`)
- [x] Create `tools/penta-scaffold/Cargo.toml` with dependencies:
  - `clap = { version = "4", features = ["derive"] }`
  - `heck = "0.5"` (for PascalCase, camelCase, snake_case string conversions)
  - `anyhow = "1.0"`
- [x] Add `"tools/penta-scaffold"` to workspace `Cargo.toml`.

### 2. Scaffold Generator Engine (`src/main.rs`, `src/generator.rs`)
- [x] Support CLI arguments:
  - `--name <SNAKE_CASE>` (e.g. `pdf_validate_signatures`)
  - `--title <STRING>` (e.g. "Validate Digital Signatures")
  - `--category <STRING>` (e.g. "security", "organize", "conversion")
  - `--icon <EMOJI>` (e.g. "🔏")
  - `--input-format <STRING>` (e.g. "pdf")
  - `--output-format <STRING>` (e.g. "json", "pdf", "csv")
  - `--description <STRING>`
  - `--dry-run` (print diffs without editing files)
- [x] Implement code templates & file appenders for:
  1. `paperpilot-pdf/src/operations/<name>.rs`
  2. `paperpilot-pdf/src/operations/mod.rs`
  3. `paperpilot-cli/src/cli.rs` & `paperpilot-cli/src/commands/<name>.rs`
  4. `paperpilot-mcp/src/server.rs`
  5. `paperpilot-gateway/src/handlers.rs` & `paperpilot-gateway/src/server.rs`
  6. `paperpilot-wasm/src/lib.rs`
  7. `apps/edge/src/index.ts`
  8. `apps/desktop/src/lib/types/toolList.ts`
  9. `tools/penta-interface-e2e/src/cases/<name>.rs`

### 3. Makefile & Workflow Integration
- [x] Add `make new-tool` command to `Makefile`.
- [x] Document usage in `wiki/ToolScaffolding.md`.

### 4. Verification
- [x] Test generator with `--dry-run` to ensure generated syntax is valid.
- [x] Verify `cargo check --workspace` passes with no compilation or lint errors.
