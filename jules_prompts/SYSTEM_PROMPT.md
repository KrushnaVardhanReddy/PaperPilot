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

### 6. Pull Request Documentation & Explanation
- Provide a clean, structured PR description summarizing:
  - Architecture decisions and why specific traits or lifetime boundaries were selected.
  - Testing verification results (exact commands and pass count).
  - List of modified crates and files.
