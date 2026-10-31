# Task 149: Implement Universal Per-Request Zero-Trace & Dynamic Logging Suppression Engine (Penta-Interface)

## Overview
Implement Spec 149 across all 5 PaperPilot interfaces: `paperpilot-gateway` (REST), `apps/edge` (Cloudflare Workers), `paperpilot-cli` (CLI), `paperpilot-mcp` (MCP), `paperpilot-wasm` (WASM), and `apps/desktop` (Desktop UI). Enable per-request log suppression, return the compliance confirmation header `X-Zero-Trace-Enforced: true`, and guarantee zero document metadata leakage to log sinks.

---

## Subtasks & Checklist

- [ ] **Subtask 1: Gateway Request Tracing Middleware & Header Contract (`paperpilot-gateway`)**
  - [ ] Define `ZeroTrace(pub bool)` marker in `paperpilot-gateway/src/server.rs`.
  - [ ] In `request_id_and_tracing_middleware`:
    - Parse incoming headers for `x-log-level` and `x-zero-trace`.
    - Parse query string for `log_level=off` and `zero_trace=true`.
    - If zero-trace is enabled:
      - Skip creating request `tracing::info_span!`.
      - Skip emitting `tracing::debug!("Incoming request started")`.
      - Skip emitting `tracing::info!("Request completed")`.
      - Attach outbound response header `X-Zero-Trace-Enforced: true`.
  - [ ] In `paperpilot-gateway/src/handlers.rs`:
    - Check `req.extensions().get::<ZeroTrace>()`.
    - If `ZeroTrace(true)`, suppress emitting document file names, sizes, or error stack traces to server logs.
    - Return standard client HTTP error JSON (`{"success": false, "error": "...", "code": "...", "request_id": "..."}`).

- [ ] **Subtask 2: Cloudflare Edge Zero-Trace Implementation (`apps/edge`)**
  - [ ] In the edge worker request pipeline:
    - Inspect incoming request headers for `X-Zero-Trace: true` or `X-Log-Level: off`.
    - Suppress `console.log`, `console.info`, and analytics engine events containing document metadata.
    - Attach `X-Zero-Trace-Enforced: true` to the outbound Response.

- [ ] **Subtask 3: CLI Environment & Flag Verification (`paperpilot-cli`)**
  - [ ] In `paperpilot-cli/src/lib.rs`:
    - Support `NO_LOG=1` environment variable alongside `RUST_LOG=off`.
    - 🚨 **CRITICAL GUARD**: Keep `args.len() > 1 && args[1] == "watch"` check BEFORE calling `Cli::parse()`.
    - Ensure `--log-level off` guarantees exactly 0 bytes written to `stderr`.

- [ ] **Subtask 4: MCP Protocol Zero-Trace Parameter (`paperpilot-mcp`)**
  - [ ] In `paperpilot-mcp/src/server.rs` (`execute_call_tool`):
    - Parse tool call input parameters for `_zero_trace: true` or `log_level: "off"`.
    - When present, suppress diagnostic `tracing::debug!` and `tracing::error!` output to `stderr`.
    - Preserve clean JSON-RPC envelopes on `stdout`.

- [ ] **Subtask 5: Client-Side WASM & Desktop UI Integration (`paperpilot-wasm` & `apps/desktop`)**
  - [ ] In `apps/desktop/src/lib/wasm/index.ts`:
    - Accept optional `{ zeroTrace?: boolean }` parameter on WASM engine methods.
    - Suppress browser console debug/info logs when `zeroTrace: true`.
  - [ ] In `apps/desktop/src/lib/components/layout/SettingsPanel.svelte`:
    - Add "Zero-Trace Processing" toggle in Privacy & Security settings.
    - When toggled, automatically append `X-Zero-Trace: true` to all background sidecar HTTP calls.

- [ ] **Subtask 6: Comprehensive Automated Test Suite**
  - [ ] Add `paperpilot-gateway/tests/test_zero_trace.rs`:
    - `test_gateway_x_zero_trace_header`
    - `test_gateway_x_log_level_off_header`
    - `test_gateway_query_param_zero_trace`
    - `test_gateway_normal_request_retains_logging`
  - [ ] Add `paperpilot-cli/tests/test_zero_trace.rs`:
    - `test_cli_log_level_off_emits_zero_stderr`
    - `test_cli_no_log_env_emits_zero_stderr`
  - [ ] Add `paperpilot-mcp/tests/test_zero_trace.rs`:
    - `test_mcp_zero_trace_argument_suppresses_stderr`
  - [ ] Verify test suite:
    - `cargo test -p paperpilot-gateway --test test_zero_trace -j 4`
    - `cargo test -p paperpilot-cli --test test_zero_trace -j 4`
    - `cargo test -p paperpilot-mcp -j 4`
    - `cargo test -p paperpilot-gateway -j 4`
