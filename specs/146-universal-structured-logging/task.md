# Task 146: Implement Universal Leveled & Structured Logging Engine

## Overview
Implement Spec 146 across `paperpilot-gateway`, `paperpilot-cli`, `paperpilot-mcp`, and `paperpilot-pdf`. Provide multi-format logging (`json`, `compact`, `pretty`, `text`), granular log-level visibility, dual error logging (log root cause on server + return safe structured error to client), and stdio isolation.

---

## Subtasks & Checklist

- [ ] **Subtask 1: Gateway Tracing Middleware & Request ID Correlation (`paperpilot-gateway`)**
  - [ ] Ensure `paperpilot-gateway/Cargo.toml` has `tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }`.
  - [ ] Implement `LOG_FORMAT` env variable handling (`json`, `compact`, `pretty`, `text`) in `paperpilot-gateway/src/main.rs`.
  - [ ] Add `x-request-id` header extractor / UUID v4 generator in Axum middleware (`paperpilot-gateway/src/server.rs`).
  - [ ] Emit `tracing::info!` on request completion with duration in ms, status code, route, client IP, and `request_id`.
  - [ ] Emit `tracing::debug!` on request start with input length, tool ID, and headers.
  - [ ] Emit `tracing::error!` on handler failure with full error cause before returning HTTP error response.
  - [ ] Include `request_id` in error response bodies (`{"error": "...", "code": "...", "request_id": "..."}`).

- [ ] **Subtask 2: CLI Global Flags & Stderr Routing (`paperpilot-cli`)**
  - [ ] Add `--log-format <text|json|compact|pretty>` and `--log-level <trace|debug|info|warn|error>` flags to root `Cli` struct in `paperpilot-cli/src/cli.rs`.
  - [ ] Initialize `tracing_subscriber` in `paperpilot-cli/src/main.rs` directed explicitly to `std::io::stderr`.
  - [ ] Emit `tracing::debug!` on subcommand dispatch with arguments (passwords redacted).
  - [ ] Emit `tracing::error!` on subcommand failure before returning exit code.
  - [ ] Verify stdout receives purely valid operation binary bytes or structured `--json` payload without log pollution.

- [ ] **Subtask 3: MCP Protocol Stdio Isolation (`paperpilot-mcp`)**
  - [ ] Ensure `tracing_subscriber` in `paperpilot-mcp/src/main.rs` routes strictly to `std::io::stderr`.
  - [ ] Emit `tracing::debug!` when tool calls are dispatched and `tracing::error!` when tools return an error.
  - [ ] Verify stdout contains only valid JSON-RPC envelopes.

- [ ] **Subtask 4: Core Engine Operation Tracing (`paperpilot-pdf`)**
  - [ ] Add `tracing::debug!` spans to core operations (`pdf_compress`, `pdf_ocr`, `pdf_merge`, `pdf_decrypt`).
  - [ ] Ensure password parameters are redacted (`password="[REDACTED]"`).
  - [ ] Emit `tracing::warn!` on recoverable edge cases (e.g. malformed xref auto-repair).

- [ ] **Subtask 5: Comprehensive Automated Test Suite**
  - [ ] Create `paperpilot-gateway/tests/test_logging.rs`:
    - Test `LOG_FORMAT=json` outputs valid JSON lines.
    - Test that `x-request-id` is returned and logged.
    - Test that error responses include `request_id` and trigger `tracing::error!`.
  - [ ] Create `paperpilot-cli/tests/test_logging.rs`:
    - Test `--log-format json` writes valid NDJSON to stderr while keeping stdout clean.
    - Test `--log-level debug` outputs detailed step diagnostics.
  - [ ] Verify `cargo test -p paperpilot-gateway -j 4`.
  - [ ] Verify `cargo test -p paperpilot-cli -j 4`.
  - [ ] Verify `cargo test -p paperpilot-mcp -j 4`.
