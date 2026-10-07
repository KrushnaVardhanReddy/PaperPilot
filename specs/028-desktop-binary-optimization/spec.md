# Spec 028: Desktop Release Binary Optimization & Pure-Rust Inference (`rten`)

## Status
Approved

## Context
When building the standalone Desktop application via `make build-release`, the output binary `target/release/desktop` measures **107 MB**. 
Two main root causes were identified:
1. **Ignored Child Release Profiles**: Cargo workspace semantics ignore `[profile.release]` defined in sub-crates like `apps/desktop/src-tauri/Cargo.toml`. As a result, the desktop release binary was built with `strip = false`, `lto = false`, and `codegen-units = 16`, keeping ~20–25 MB of unneeded debug/symbol tables and unpruned dead code.
2. **Heavy C++ ONNX Runtime**: `paperpilot-nlp` depends on `ort` (Microsoft ONNX Runtime), which compiles and statically links ~14.5 MB of C++ runtime machine code for the TinyBERT intent classifier. Meanwhile, `paperpilot-pdf` already compiles Robert Knight's pure-Rust `rten` (Rust Tensor Engine) for OCR.

## Objectives
1. Configure root workspace `[profile.release]` in `Cargo.toml` with `strip = true`, `lto = "fat"`, `opt-level = 3`, `codegen-units = 1`, and `panic = "abort"`.
2. Clean up redundant child profile in `apps/desktop/src-tauri/Cargo.toml` to eliminate Cargo warnings.
3. Migrate `paperpilot-nlp` Layer 2 inference engine from Microsoft `ort` to pure-Rust `rten` (or unify runtime with `rten`).
4. Generate a verifiable Before & After Binary Size and Latency Benchmark Report.
5. Target final standalone Desktop binary size: **< 75 MB** (a ~35-40% reduction from 107 MB) while maintaining 100% test pass across all tri-interface and UI suites.

## Decisions

### 1. Workspace-Level Profile (`Cargo.toml`)
Define the single canonical release profile at the workspace root:
```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

### 2. Pure-Rust Inference Engine (`rten`)
- Convert `tinybert_int8.onnx` to `tinybert.rten` using the `rten-convert` pipeline.
- Replace `ort` in `paperpilot-nlp/Cargo.toml` with `rten` and `rten-tensor` (already present in the workspace for OCR).
- Update `paperpilot-nlp/src/layer2.rs` to load the `.rten` model and run SIMD inference directly in Rust memory without C++ bindings.
- Preserve identical intent classification API (`OnnxClassifier` / `NlpClassifier::predict`) and <2ms latency.

### 3. Verification & Multi-Interface Quality Gate
- Measure binary size before and after using `ls -lh`, `file`, and symbol categorization.
- Execute unit & integration tests (`cargo test -p paperpilot-nlp -j 2`).
- Execute Playwright UI chat test (`apps/desktop/tests/e2e_ai_chat_real_pipeline.spec.ts`).
- **Complete Multi-Interface Battery (All 44 Tools)**:
  - CLI, MCP, REST Gateway: `python3 scripts/test_tri_interface_e2e.py` (132/132 assertions pass).
  - Browser WASM (Web): `cd apps/web && pnpm exec playwright test` (100% pass).
  - Cloudflare Edge: `cd apps/edge && pnpm test` (100% pass).
- Record official report at `reports/DESKTOP_BINARY_OPTIMIZATION_REPORT.md` and wiki at `wiki/22-Desktop-Binary-Optimization.md`.
