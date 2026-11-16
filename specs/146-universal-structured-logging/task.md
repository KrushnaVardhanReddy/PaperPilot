# Task 146: Implement Universal Leveled & Structured Logging Engine

## Overview
Implement Spec 146 across `paperpilot-gateway`, `paperpilot-cli`, `paperpilot-mcp`, and `paperpilot-pdf`. Provide multi-format logging (`json`, `compact`, `pretty`, `text`), granular log-level visibility, dual error logging (log root cause on server + return safe structured error to client), and stdio isolation.

---

## Subtasks & Checklist

- [x] **Subtask 1: Gateway Tracing Middleware & Request ID Correlation (`paperpilot-gateway`)**
  - [x] Ensure `paperpilot-gateway/Cargo.toml` has `tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }`.
  - [x] Implement `LOG_FORMAT` env variable handling (`json`, `compact`, `pretty`, `text`) in `paperpilot-gateway/src/main.rs`.
  - [x] Add `x-request-id` header extractor / UUID v4 generator in Axum middleware (`paperpilot-gateway/src/server.rs`).
  - [x] Emit `tracing::info!` on request completion with duration in ms, status code, route, client IP, and `request_id`.
  - [x] Emit `tracing::debug!` on request start with input length, tool ID, and headers.
  - [x] Emit `tracing::error!` on handler failure with full error cause before returning HTTP error response.
  - [x] Include `request_id` in error response bodies (`{"error": "...", "code": "...", "request_id": "..."}`).

- [x] **Subtask 2: CLI Global Flags & Stderr Routing (`paperpilot-cli`)**
  - [x] Add `--log-format <text|json|compact|pretty>` and `--log-level <trace|debug|info|warn|error>` flags to root `Cli` struct in `paperpilot-cli/src/cli.rs`.
  - [x] Initialize `tracing_subscriber` in `paperpilot-cli/src/lib.rs` directed explicitly to `std::io::stderr`.
  - [x] 🚨 **CRITICAL CLI GUARD**: In `paperpilot-cli/src/lib.rs`, do NOT run `Cli::parse()` before checking `if args.len() > 1 && args[1] == "watch"`. The hot-folder daemon intercepts raw `watch` arguments. Calling `Cli::parse()` first causes Clap to reject `watch` and breaks `test_dynamic_naming`.
  - [x] Emit `tracing::debug!` on subcommand dispatch with arguments (passwords redacted).
  - [x] Emit `tracing::error!` on subcommand failure before returning exit code.
  - [x] Verify stdout receives purely valid operation binary bytes or structured `--json` payload without log pollution.

- [x] **Subtask 3: MCP Protocol Stdio Isolation (`paperpilot-mcp`)**
  - [x] Actually modify `paperpilot-mcp/src/main.rs` to initialize `tracing_subscriber` routing strictly to `std::io::stderr`. (Do not just add dependencies to Cargo.toml!).
  - [x] In `paperpilot-mcp/src/server.rs` (`execute_call_tool`), emit `tracing::debug!` when tool calls are dispatched and `tracing::error!` when tools return an error.
  - [x] Verify stdout contains exclusively valid JSON-RPC envelopes.

- [x] **Subtask 4: Core Engine Operation Tracing (`paperpilot-pdf`)**
  - [x] Actually modify `paperpilot-pdf/src/operations/` (`compress.rs`, `ocr.rs`, `merge.rs`, `decrypt.rs`) to add `tracing::debug!` spans and metrics. (Do not just add dependencies to Cargo.toml!).
  - [x] Ensure password parameters are redacted (`password="[REDACTED]"`).
  - [x] Emit `tracing::warn!` on recoverable edge cases (e.g. malformed xref auto-repair).

- [x] **Subtask 5: Comprehensive Automated Test Suite**
  - [x] Create `paperpilot-gateway/tests/test_logging.rs`:
    - Test `LOG_FORMAT=json` outputs valid JSON lines.
    - Test that `x-request-id` is returned and logged.
    - Test that error responses include `request_id` and trigger `tracing::error!`.
  - [x] Create `paperpilot-cli/tests/test_logging.rs`:
    - Test `--log-format json` writes valid NDJSON to stderr while keeping stdout clean.
    - Test `--log-level debug` outputs detailed step diagnostics.
  - [x] Verify `cargo test -p paperpilot-gateway -j 4`.
  - [x] Verify `cargo test -p paperpilot-cli -j 4`.
  - [x] Verify `cargo test -p paperpilot-mcp -j 4`.
