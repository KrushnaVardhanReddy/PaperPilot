# Spec 149: Universal Per-Request Zero-Trace & Dynamic Logging Suppression Engine

## Metadata
- **Spec Number**: 149
- **Title**: Universal Per-Request Zero-Trace & Dynamic Logging Suppression Engine
- **Phase**: Cross-Cutting Architecture / Enterprise Privacy & Zero Data Retention (ZDR)
- **Status**: 📋 In Progress / Ready for Implementation
- **Impacted Crates & Packages**: `paperpilot-gateway`, `apps/edge`, `paperpilot-cli`, `paperpilot-mcp`, `apps/desktop`
- **Dependencies**: `tracing`, `tracing-subscriber`, `axum`, `tower-http`

---

## 1. Executive Summary & Problem Statement

Spec 146 established structured, leveled logging across all PaperPilot interfaces (`json`, `compact`, `pretty`, `text`). However, in enterprise, legal discovery, healthcare (HIPAA), and European privacy (GDPR Article 25 - Data Protection by Design) environments, enterprise clients require **dynamic client-driven Zero Data Retention (ZDR)**.

When organizations process sensitive files (merger acquisitions, litigation discovery, medical charts, tax returns), their compliance policies strictly forbid storing or emitting document metadata (file names, sizes, page counts, route paths, client IPs) into application logs, centralized log aggregators (Datadog, Splunk, Vector), or edge log collectors (Cloudflare Logpush).

Currently:
- The CLI supports `--log-level off` at command invocation.
- But the **REST API Gateway**, **Cloudflare Edge**, and **MCP** server unconditionally emit request and diagnostic logs. Clients cannot programmatically instruct the server: *"Process this specific document, but emit zero log traces"*.

This specification establishes a **Universal Per-Request Zero-Trace & Dynamic Logging Suppression Engine** across all PaperPilot interfaces.

---

## 2. Universal Interface Specifications

### 2.1 REST API Gateway (`paperpilot-gateway`)

Every REST API endpoint must support per-request log level overrides and zero-trace processing via HTTP headers and query parameters.

#### A. Supported Request Headers
Clients can send either of the following headers on any request:
* **`X-Log-Level: <trace|debug|info|warn|error|off>`**
  - Case-insensitive override of the server-wide default log level for the lifetime of that specific HTTP request.
  - Setting `X-Log-Level: off` disables all logging for that request.
* **`X-Zero-Trace: <true|1>`**
  - Syntactic sugar for `X-Log-Level: off`. Specifically designed for compliance and legal audit clarity.

#### B. Supported Query Parameters
For GET endpoints (e.g. downloads, document streams, health checks):
* `?log_level=off` or `?zero_trace=true`

#### C. Gateway Middleware Execution Behavior
1. In `paperpilot-gateway/src/server.rs`, `request_id_and_tracing_middleware` inspects headers and query parameters for log suppression flags.
2. If `X-Log-Level: off` or `X-Zero-Trace: true` is present:
   - Suppresses the creation of the request-level `tracing::info_span!`.
   - Bypasses all `tracing::info!` ("Incoming request started") and `tracing::info!` ("Request completed") statements.
   - Handlers running within the request scope suppress operation-level tracing output.
3. **Response Header Confirmation**:
   - The server must return the response header:
     ```http
     X-Zero-Trace-Enforced: true
     ```
   - This allows automated compliance suites and client API SDKs to verify that zero-trace processing was contractually honored.

---

### 2.2 Cloudflare Edge Microservice (`apps/edge`)

On Cloudflare Workers / Edge deployments:
1. When `X-Zero-Trace: true` or `X-Log-Level: off` is detected on an incoming request:
   - Suppress `console.log`, `console.info`, and `console.debug`.
   - Prevent emitting analytics metrics containing file or document metadata to Cloudflare Logpush / Analytics Engine.
   - Return `X-Zero-Trace-Enforced: true` in the HTTP response.

---

### 2.3 Command Line Interface (`paperpilot-cli`)

1. Verify that `paperpilot --log-level off <command>` guarantees:
   - `stderr`: Exactly 0 bytes emitted (no spans, no levels, no metadata).
   - `stdout`: Emits strictly pure output bytes (clean binary PDF/DOCX stream or structured `--json` payload).
2. Support `NO_LOG=1` environment variable alongside `RUST_LOG=off`.

---

### 2.4 Model Context Protocol (`paperpilot-mcp`)

1. All MCP tool schemas support an optional metadata parameter:
   ```json
   {
     "log_level": "off"
   }
   ```
2. When `log_level == "off"`:
   - Suppresses tool execution debug and info logs to stderr.
   - Keeps stderr completely silent, maintaining clean stdio isolation.

---

### 2.5 Desktop UI (`apps/desktop`)

1. Add a **"Zero-Trace Processing"** toggle in **Settings > Privacy & Telemetry**:
   - When enabled, all background API gateway calls and local worker invocations automatically attach `X-Zero-Trace: true`.
   - Local temporary debug logs for operations are completely bypassed.

---

## 3. Compliance & Security Verification

| Standard / Policy | Requirement | PaperPilot Implementation |
| :--- | :--- | :--- |
| **GDPR Article 25** | Data protection by design & by default; minimize metadata leakage | `X-Zero-Trace: true` header ensures document metadata never reaches persistent log files. |
| **HIPAA Security Rule** | Protection of electronic Protected Health Information (ePHI) in transit and logs | Zero-trace ensures patient identifiers in file names or routes are not captured in log sinks. |
| **SOC 2 Type II** | Confidentiality & privacy controls for processing customer data | Confirmed receipt via `X-Zero-Trace-Enforced: true` response header. |

---

## 4. Acceptance Criteria & Test Matrix

1. **Gateway Zero-Trace Test**:
   - Send `POST /api/v1/pdf/convert` with header `X-Zero-Trace: true`.
   - Assert response status is `200 OK`.
   - Assert response header `X-Zero-Trace-Enforced: true` is present.
   - Assert zero lines were written to tracing subscriber or terminal stderr.
2. **Gateway Dynamic Level Test**:
   - Send request with `X-Log-Level: debug`.
   - Assert debug-level telemetry is emitted for that request without altering the global server default for other concurrent requests.
3. **CLI Verification**:
   - Run `paperpilot --log-level off convert ...`.
   - Assert `stderr` is 0 bytes.
4. **Non-Regression**:
   - All existing 146 structured logging tests and Wave 1-8 penta-interface tests continue passing.
