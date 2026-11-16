# PaperPilot Jules Rust Engineering & Quality Master Standards

All Jules tasks that modify or implement Rust code across PaperPilot (`paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`, `paperpilot-wasm`, `paperpilot-core`, or `apps/desktop/src-tauri`) MUST strictly adhere to the following principles.

---

### 1. Zero Panic Guarantee (Error Handling)
- **NEVER** use `.unwrap()` or `.expect()` in non-test production code.
- Always propagate errors via `Result<T, E>` using `?` or handle them gracefully with `match` / `if let`.
- Use `thiserror` for library error definitions (`OperationError`, `GatewayError`, `McpError`) or `anyhow` for top-level CLI error handling.
- When an operation encounters an invalid parameter, return a descriptive error variant instead of crashing.

---

### 2. Ownership, Borrowing & Buffer Performance
- Prefer passing immutable byte slices (`&[u8]`) over owned `Vec<u8>` or cloning data buffers.
- Large PDF documents (tens of megabytes) are processed in memory; minimize deep clones of PDF streams, dictionaries, or page arrays.
- Avoid unnecessary nested references that complicate the borrow checker. If shared read-only state across threads is required, use `std::sync::Arc`.

---

### 3. Pure Core Engine vs. Thin Wrappers
- **Pure Synchronous Core (`paperpilot-pdf`)**:
  - The core engine must be pure, deterministic, CPU-bound Rust with **zero async runtimes**, no `tokio`, and no networking.
  - Signature pattern:
    ```rust
    pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
    ```
- **Async Wrappers (`paperpilot-gateway`, `paperpilot-mcp`)**:
  - Integrate cleanly with existing `axum` and `tokio` asynchronous handlers.
  - Do not introduce blocking operations directly on the async runtime thread; use `tokio::task::spawn_blocking` when invoking CPU-heavy PDF transformations.
- **Strict Zero-Cloud Policy**:
  - Never add external network calls, analytics, telemetry, or remote dependencies to tools. PaperPilot is 100% offline and air-gapped.

---

### 4. Strong Domain Modeling & Type Safety
- Represent tool options and parameters using strongly typed `struct` and `enum` types instead of raw strings, numbers, or unstructured `serde_json::Value`.
- Derive standard idioms on all parameter structs and options:
  ```rust
  #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
  ```
- Use `#[serde(rename_all = "snake_case")]` or explicit serde field tags to ensure wire compatibility across CLI, MCP, Gateway, and WASM.

---

### 5. Automated Pre-Submission Verification (Mandatory in Container)
Before opening the Pull Request, run the following verification steps in the Jules environment:
1. `cargo check -p paperpilot-pdf -p paperpilot-gateway -p paperpilot-mcp -p paperpilot-cli -j 2` — verify targeted crates compile cleanly.
2. `cargo fmt --check` — ensure code strictly follows Rust formatting standards.
3. `cargo clippy -p <target-crates> --all-targets -- -D warnings` — ensure zero clippy warnings on modified files.
4. `cargo test -p <target-crate> -j 2` — zero-mock testing using synthesized byte buffers or real disk fixtures.
5. **Security & Supply Chain Gate (`make audit` or targeted scan)**:
   - Run `semgrep scan --config auto` (or `make audit`) to verify zero high/critical vulnerabilities or insecure patterns (e.g. no `curl | sh`, no shell injection, no secrets).
   - If `.github/` workflows or configurations are modified, verify they pass `semgrep scan --config auto .github/` with 0 warnings.

---

### 6. Documentation & Tools Handbook Synchronization
- Whenever a tool is implemented, extended, or modified:
  1. Ensure its reference doc exists or is updated under `apps/docs/src/content/docs/reference/tools/<tool_name>.md` with CLI, MCP JSON-RPC, and REST API snippets.
  2. Run the handbook generator to update `tools-handbook.md`:
     ```bash
     python3 apps/docs/src/content/docs/reference/generate_handbook.py
     ```
  3. Ensure `npm --prefix apps/docs run build` completes cleanly.

---

### 7. Pull Request Documentation & Explanation
- Provide a clean, structured PR description summarizing:
  - Architecture decisions and why specific traits or lifetime boundaries were selected.
  - Testing verification results (exact commands and pass count).
  - List of modified crates and files.

---

### 8. The "Engine First" Audit Rule (Never Trust Existing Helpers Blindly)
- When implementing a user-facing tool (CLI/MCP/REST), **DO NOT simply write a thin wrapper around existing helper functions without inspecting them**.
- Audit the underlying engine implementation in `paperpilot-pdf`:
  - Does the core function generate valid PDF appearance streams (`/AP`), or does it only set dictionary keys (`/V`)?
  - Does flattening bake appearance streams into page content, or does it just drop the `/AcroForm` catalog?
  - Can the operation be batched without re-parsing the document from disk on every iteration (e.g. parse once in RAM and clone per record)?
- If the core engine function is naive, inadequate, or unsafe, **upgrade the engine in `paperpilot-pdf` first**, then expose it via the CLI/MCP.

---

### 9. Edge-Case, Unicode & Security Matrix
Every PR that touches documents, input data, or filenames MUST explicitly test and guard against:
1. **Directory Traversal / Token Injection**: File naming tokens (e.g., `{row.name}`) must be sanitized to prevent directory traversal (`../`).
2. **Partial Failures & Execution Receipts**: A single malformed record must NOT abort the whole batch. Output a machine-readable execution receipt (`verification.json`) detailing per-item status, duration, error reason, and SHA256 hashes.
3. **Unicode & Non-ASCII**: Include explicit test cases for UTF-8/UTF-16 text, CJK (Chinese/Japanese/Korean) characters, and quoted multi-line values.
4. **Scale & Performance**: When adding batch or iterative tools, include a high-volume scale test (e.g., 500–1,000 items) to prove memory stability and efficiency.

---

### 10. Inspectable Evidence Requirement (Reports Directory)
- Do NOT just write code and unit tests.
- When introducing a new workflow or major operation, generate inspectable evidence under `reports/<task-name>-validation/`:
  - `verification.json`: Machine-readable results and timing.
  - Sample generated PDF output files.
  - Rendered preview image (PNG) verifying visual layout.
- Include a summary report in `reports/<TASK_NAME>_REPORT.md` documenting performance, test coverage, and limitations.

