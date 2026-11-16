# Spec 149: Universal Per-Request Zero-Trace & Dynamic Logging Suppression Engine (Penta-Interface)

## Metadata
- **Spec Number**: 149
- **Title**: Universal Per-Request Zero-Trace & Dynamic Logging Suppression Engine (Penta-Interface)
- **Phase**: Cross-Cutting Architecture / Enterprise Privacy & Zero Data Retention (ZDR)
- **Status**: 📋 In Progress / Ready for Implementation
- **Impacted Interfaces**: All 5 Surfaces (💻 CLI, 🌐 REST API Gateway, 🤖 MCP, ⚡ WASM Web, 🖥️ Desktop UI)
- **Impacted Crates & Packages**: `paperpilot-gateway`, `apps/edge`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-wasm`, `apps/desktop`
- **Dependencies**: `tracing`, `tracing-subscriber`, `axum`, `tower-http`

---

## 1. Executive Summary & Problem Statement

Spec 146 established structured, leveled logging across PaperPilot (`json`, `compact`, `pretty`, `text`). However, in enterprise environments (GDPR Article 25 - Data Protection by Design, HIPAA Security Rule, and SOC 2 Type II), enterprise clients require **dynamic client-driven Zero Data Retention (ZDR)**.

When organizations process sensitive files (mergers & acquisitions, litigation discovery, medical records, financial tax filings), their compliance policies strictly forbid storing or emitting document metadata (file names, sizes, page counts, route paths, client IPs) into application logs, centralized log sinks (Datadog, Splunk, Vector), edge collectors (Cloudflare Logpush), or local desktop log files.

Currently, log suppression is inconsistent:
- The CLI supports `--log-level off` at command invocation.
- But the **REST API Gateway**, **Cloudflare Edge**, **MCP Server**, **WASM JS Wrapper**, and **Desktop UI** unconditionally emit request, console, or file logs. Clients cannot programmatically enforce: *"Process this specific document across any of the 5 interfaces, but emit zero log traces"*.

This specification establishes a **Universal Per-Request Zero-Trace & Dynamic Logging Suppression Engine** across all **5 PaperPilot Interfaces**.

---

## 2. The 5-Interface Zero-Trace Specification

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                 PENTA-INTERFACE ZERO-TRACE ARCHITECTURE                     │
├─────────────────┬───────────────────────────────────────────────────────────┤
│ 1. 💻 CLI       │ paperpilot --log-level off (or NO_LOG=1) -> 0 bytes stderr │
│ 2. 🌐 REST API  │ Header: X-Zero-Trace: true / X-Log-Level: off             │
│                 │ Query:  ?zero_trace=true                                  │
│                 │ Resp:   X-Zero-Trace-Enforced: true                       │
│ 3. 🤖 MCP       │ Tool Arg: {"_zero_trace": true, "log_level": "off"}        │
│ 4. ⚡ WASM Web  │ JS Call: engine.run(file, { zeroTrace: true })            │
│ 5. 🖥️ Desktop   │ Settings > Privacy > "Zero-Trace Processing" Toggle       │
└─────────────────┴───────────────────────────────────────────────────────────┘
```

---

### Interface 1: 💻 CLI (`paperpilot-cli`)

1. **Global Flag**:
   ```bash
   paperpilot --log-level off convert --input sensitive.pdf --format docx --output out.docx
   ```
2. **Environment Variable**:
   ```bash
   NO_LOG=1 paperpilot ...
   # or
   RUST_LOG=off paperpilot ...
   ```
3. **Execution Contract**:
   - `stderr`: Strictly `0` bytes emitted. No spans, no metadata, no timestamps.
   - `stdout`: Pure valid output bytes (binary document or structured `--json`).
   - Exit code: Preserved normally (`0` on success).

---

### Interface 2: 🌐 REST API Gateway & Cloudflare Edge (`paperpilot-gateway` & `apps/edge`)

1. **Supported Request Headers**:
   - `X-Log-Level: <trace|debug|info|warn|error|off>` (Case-insensitive per-request log level).
   - `X-Zero-Trace: <true|1>` (Contractual zero-retention alias for `X-Log-Level: off`).
2. **Supported Query Parameters**:
   - `?zero_trace=true` or `?log_level=off` (For GET/download streaming endpoints).
3. **Gateway Middleware Behavior (`paperpilot-gateway/src/server.rs`)**:
   - If zero-trace is enabled:
     - Suppress entering request-level `tracing::info_span!`.
     - Suppress `tracing::debug!("Incoming request started")` and `tracing::info!("Request completed")`.
     - Handlers check `req.extensions().get::<ZeroTrace>()` and suppress emitting file names or parameter dumps on errors.
   - Always return verification response header:
     ```http
     X-Zero-Trace-Enforced: true
     ```
4. **Cloudflare Edge (`apps/edge`)**:
   - Suppresses `console.log` / `console.info` / analytics logging and returns `X-Zero-Trace-Enforced: true`.

---

### Interface 3: 🤖 MCP Server (`paperpilot-mcp`)

1. **Tool Parameter Contract**:
   All 89 MCP tool schemas accept optional zero-trace arguments:
   ```json
   {
     "_zero_trace": true,
     "log_level": "off"
   }
   ```
2. **Execution Contract**:
   - When present, suppresses all `tracing::debug!` and `tracing::error!` lines on `stderr`.
   - `stdout` strictly emits valid JSON-RPC envelopes without logging interference.

---

### Interface 4: ⚡ Client-Side WebAssembly (`paperpilot-wasm` & JS Wrapper)

1. **Zero-Trace WASM Wrapper**:
   - In `apps/desktop/src/lib/wasm/index.ts` and web embed packages:
   - Methods accept optional `{ zeroTrace?: boolean }`:
     ```typescript
     await WasmPdfEngine.pdf_to_docx(fileBytes, { zeroTrace: true });
     ```
2. **Execution Contract**:
   - Completely suppresses browser `console.debug`, `console.info`, or timing metrics in Web Worker or main thread.
   - Zero sensitive document names or telemetry exposed in browser developer tools.

---

### Interface 5: 🖥️ Desktop UI (`apps/desktop`)

1. **UI Privacy Controls**:
   - In **Settings > Privacy & Security**: Add toggle:
     `[x] Zero-Trace Processing (Suppress all local operation logs and telemetry)`
2. **Execution Contract**:
   - When enabled, all internal IPC calls to the Tauri Rust core and background gateway sidecar automatically attach `X-Zero-Trace: true` and `{ zeroTrace: true }`.
   - No temporary log files or document paths are written to local disk logs (`~/.config/paperpilot/logs/`).

---

## 3. Compliance & Security Verification

| Standard / Policy | Requirement | PaperPilot Penta-Interface Implementation |
| :--- | :--- | :--- |
| **GDPR Article 25** | Data protection by design & by default; zero metadata retention | Zero-trace suppresses document metadata across CLI, REST, MCP, WASM, and Desktop. |
| **HIPAA Security Rule** | Protection of electronic Protected Health Information (ePHI) in logs | Prevents patient identifiers, file names, or routes from being written to server/edge sinks. |
| **SOC 2 Type II** | Confidentiality controls for processing customer data | Confirmed receipt via `X-Zero-Trace-Enforced: true` response header. |

---

## 4. Acceptance Criteria & Automated Test Suite

1. **Gateway Tests (`paperpilot-gateway/tests/test_zero_trace.rs`)**:
   - Test `X-Zero-Trace: true` returns 200, header `X-Zero-Trace-Enforced: true`, and zero log output.
   - Test `X-Log-Level: off` behaves identically.
   - Test query param `?zero_trace=true` returns `X-Zero-Trace-Enforced: true`.
   - Test normal requests without headers retain standard `X-Request-ID` and logging.
2. **CLI Tests (`paperpilot-cli/tests/test_zero_trace.rs`)**:
   - Test `paperpilot --log-level off` emits exactly 0 bytes on stderr.
   - Test `NO_LOG=1` emits exactly 0 bytes on stderr.
3. **MCP Tests (`paperpilot-mcp/tests/test_zero_trace.rs`)**:
   - Test calling tools with `_zero_trace: true` emits 0 bytes on stderr.
4. **WASM & Desktop Checks**:
   - Verify WASM TypeScript bindings accept `{ zeroTrace: true }`.
   - Verify Desktop Settings toggle persists zero-trace state.
