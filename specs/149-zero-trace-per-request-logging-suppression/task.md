# Task 149: Implement Universal Per-Request Zero-Trace & Dynamic Logging Suppression Engine

## Overview
Implement Spec 149 across `paperpilot-gateway`, `apps/edge`, `paperpilot-cli`, and `paperpilot-mcp`. Enable per-request log suppression (`X-Log-Level: off`, `X-Zero-Trace: true`, `?log_level=off`, `--log-level off`), return the confirmation header `X-Zero-Trace-Enforced: true`, and ensure zero metadata leakage to log sinks.

---

## Subtasks & Checklist

- [ ] **Subtask 1: Gateway Request Tracing Middleware Suppression (`paperpilot-gateway`)**
  - [ ] In `paperpilot-gateway/src/server.rs` (`request_id_and_tracing_middleware`):
    - Parse incoming headers for `x-log-level` and `x-zero-trace`.
    - Check query parameters for `log_level=off` and `zero_trace=true`.
    - If zero-trace is enabled:
      - Skip emitting `tracing::debug!("Incoming request started")`.
      - Skip emitting `tracing::info!("Request completed")`.
      - Attach response header `X-Zero-Trace-Enforced: true`.
    - If a specific level is requested (e.g. `x-log-level: debug`), configure a request-scoped span filter.
  - [ ] Ensure handler errors in zero-trace mode still return clean client HTTP errors with `request_id`, but suppress logging sensitive document metadata to server logs.

- [ ] **Subtask 2: Cloudflare Edge Zero-Trace Implementation (`apps/edge`)**
  - [ ] In the edge worker request handler:
    - Inspect request headers for `X-Zero-Trace: true` or `X-Log-Level: off`.
    - Wrap console logging statements to skip output when zero-trace is active.
    - Attach `X-Zero-Trace-Enforced: true` to the outbound Response.

- [ ] **Subtask 3: CLI Environment & Flag Verification (`paperpilot-cli`)**
  - [ ] Add support for `NO_LOG=1` environment variable alongside `RUST_LOG=off`.
  - [ ] Ensure that running with `--log-level off` guarantees strict 0-byte output on `stderr`.

- [ ] **Subtask 4: MCP Protocol Zero-Trace Parameter (`paperpilot-mcp`)**
  - [ ] Add optional `_zero_trace` / `_log_level` parameter to tool dispatch handler in `paperpilot-mcp/src/server.rs`.
  - [ ] Suppress stderr diagnostic logging during tool execution when `_zero_trace` is true.

- [ ] **Subtask 5: Comprehensive Automated Test Suite**
  - [ ] Add tests in `paperpilot-gateway/tests/test_zero_trace.rs`:
    - Send request with `X-Zero-Trace: true`, verify status 200, verify header `X-Zero-Trace-Enforced: true`, and assert no logs emitted.
    - Send request with `X-Log-Level: off`, verify same zero-trace behavior.
    - Send request with query param `?zero_trace=true`, verify zero-trace behavior.
    - Send normal request (without headers), verify normal `tracing::info!` logging is retained.
  - [ ] Add CLI test in `paperpilot-cli/tests/test_zero_trace.rs`:
    - Test `paperpilot --log-level off` emits 0 bytes on stderr.
    - Test `NO_LOG=1 paperpilot ...` emits 0 bytes on stderr.
  - [ ] Run `cargo test -p paperpilot-gateway -j 4`.
  - [ ] Run `cargo test -p paperpilot-cli -j 4`.
  - [ ] Run `cargo test -p paperpilot-mcp -j 4`.
