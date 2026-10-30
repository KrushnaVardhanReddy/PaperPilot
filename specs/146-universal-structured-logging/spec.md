# Spec 146: Universal Leveled & Structured Logging Engine

## Metadata
- **Spec Number**: 146
- **Title**: Universal Leveled & Structured Logging Engine
- **Phase**: Cross-Cutting Architecture / Production Observability & Diagnostics
- **Status**: Ready for Implementation
- **Impacted Crates**: `paperpilot-gateway`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-pdf`
- **Dependencies**: `tracing`, `tracing-subscriber` (with `json` and `env-filter` features)

---

## 1. Executive Summary & Problem Statement

Historically, errors in PaperPilot were returned to callers as HTTP status codes or CLI exit codes with limited server-side context. When operations failed in headless services, background daemons, or Docker containers, debugging was hindered by silent execution paths and missing telemetry.

This specification establishes a **Universal Leveled & Structured Logging Standard** across all PaperPilot interfaces:
1. **Dual Error Handling Paradigm**: Always log the full unredacted error cause and chain on the server (`tracing::error!`), while returning a clean, client-safe error response with a matching `request_id`.
2. **Comprehensive DEBUG Observability**: Every tool dispatch, input size, parameter set, and step transition emits informative `tracing::debug!` events so running with `RUST_LOG=debug` or `--log-level debug` provides an immediate, clear timeline of what occurred.
3. **Multi-Format Support**: Dynamic formatting controlled by `LOG_FORMAT=json|compact|pretty|text`.
4. **Strict Isolation & Privacy**: Absolute redaction of passwords and raw text content; 100% stderr routing for CLI and MCP (protecting binary stdout and JSON-RPC wire protocols).

---

## 2. Configuration & Control Schema

### 2.1 Environment Variables
All executables (`paperpilot`, `paperpilot-gateway`, `paperpilot-mcp`) must recognize:

* **`LOG_FORMAT`**:
  - `text` *(default)*: Standard human-readable terminal log with timestamps, level, and targets.
  - `json`: Structured NDJSON (one valid JSON object per line) suitable for Vector, FluentBit, Datadog, Splunk.
  - `compact`: Single-line concise format omitting target prefixes for clean container logs.
  - `pretty`: Multi-line indented blocks with field highlighting for deep local debugging.
* **`RUST_LOG`**:
  - Standard Rust directive supporting module-specific overrides:
  - Example: `RUST_LOG=info,paperpilot_gateway=debug,paperpilot_pdf=debug`
* **`NO_COLOR`**:
  - When set to any value, strips ANSI color escape codes from `text`, `compact`, and `pretty` outputs.

### 2.2 CLI Arguments (Global Flags)
For `paperpilot-cli`:
```bash
paperpilot --log-format json --log-level debug <subcommand>
```
* `--log-format <text|json|compact|pretty>`: Overrides `LOG_FORMAT`.
* `--log-level <trace|debug|info|warn|error>`: Overrides default log level.

---

## 3. The Dual Error Model: Log & Return

When an error occurs during tool execution, API processing, or CLI runs:

```
Client / User Request
        │
        ▼
[ PaperPilot Gateway / CLI ]
        │
        ├──► 1. SERVER LOG (Internal): Log root cause, error source chain, & latency
        │      tracing::error!(
        │          request_id = %req_id,
        │          tool_id = %tool,
        │          error = %err,
        │          source = ?err.source(),
        │          duration_ms = elapsed,
        │          "Operation execution failed"
        │      );
        │
        └──► 2. CLIENT RETURN (External): Return a structured, clean error message
               HTTP 422 / 500:
               {
                 "error": "Failed to parse PDF trailer dictionary",
                 "code": "CORRUPTED_PDF",
                 "request_id": "req-9b1b9e6"
               }
```

### Key Principles:
1. **Never Fail Silently**: No `Err(e)` branch may discard or map an error without emitting either `tracing::warn!` (for recoverable fallbacks) or `tracing::error!` (for unrecoverable failures).
2. **Correlation ID (`request_id`)**: Every incoming request to `paperpilot-gateway` receives a unique UUID v4 `request_id` (or inherits `X-Request-ID` from caller). This ID is attached to the request's tracing span and returned in response headers and JSON bodies.
3. **No Internal Leakage**: Never expose raw system paths, internal stack traces, or credentials in client-facing HTTP responses.

---

## 4. Granular Observability by Log Level

### `ERROR` (Unrecoverable Failures)
* Server port binding collisions.
* Corrupted file byte streams failing PDF parsing.
* Filesystem write permission errors.
* Unhandled panic catches.

### `WARN` (Recoverable Edge Cases & Fallbacks)
* OCR model weights missing; falling back to heuristic search.
* Corrupt cross-reference table auto-repaired via linear scanning.
* Non-standard font encoding mapped to replacement glyphs.
* Deprecated API endpoints or CLI argument aliases used.

### `INFO` (Lifecycle & High-Level Metrics - Default)
* Service startup, address bound, and shutdown signals.
* HTTP request completed (`method`, `route`, `status_code`, `duration_ms`, `client_ip`).
* Watcher daemon registered folder or triggered recipe execution.
* Batch operations summary (`processed=100`, `success=98`, `failed=2`).

### `DEBUG` (Operational Diagnostics - Deep Visibility)
* **Gateway**:
  - Request arrival: `tracing::debug!(request_id, tool_id, content_len, client_ip, "Incoming tool request")`
  - Successful dispatch: `tracing::debug!(request_id, tool_id, output_len, duration_ms, "Tool returned success")`
* **Engine Core (`paperpilot-pdf`)**:
  - `pdf_compress`: input bytes, output bytes, deduplicated streams count, compression ratio.
  - `pdf_ocr`: detected images count, language target, inference latency.
  - `pdf_decrypt`: detected encryption filter, key length. *(Password itself is never logged)*.
  - `pdf_merge`: input files count, combined page count.
* **MCP Server (`paperpilot-mcp`)**:
  - Dispatched tool name and sanitized argument keys.

### `TRACE` (Byte-Level Diagnostics)
* Cross-reference table offsets and byte counts.
* Individual font dictionary and glyph bounding-box coordinates.
* Detailed regex/token matching passes during text extraction.

---

## 5. Structured JSON Schema (When `LOG_FORMAT=json`)

Every log line emitted in JSON mode is a standalone, valid JSON object (NDJSON):

```json
{
  "timestamp": "2026-10-30T15:30:00.123456Z",
  "level": "INFO",
  "target": "paperpilot_gateway::middleware",
  "message": "HTTP request completed",
  "request_id": "d1c238bf-49f8-410a-b328-912f910b4b21",
  "http.method": "POST",
  "http.route": "/api/v1/tools/pdf_compress",
  "http.status_code": 200,
  "duration_ms": 38.5,
  "client.ip": "127.0.0.1",
  "tool.id": "pdf_compress",
  "bytes.processed": 1048576
}
```

---

## 6. Stdio Isolation & Security Guardrails

1. **MCP Stdio Protocol Protection**:
   `paperpilot-mcp` uses `stdout` exclusively for JSON-RPC message framing. All `tracing_subscriber` output in `paperpilot-mcp` MUST be directed to `std::io::stderr` to prevent corrupted message frames.
2. **CLI Stdio Protection**:
   `paperpilot-cli` commands must write logs exclusively to `stderr` so that stdout redirection (`paperpilot merge -i a.pdf -i b.pdf > out.pdf`) receives purely valid PDF binary bytes.
3. **Zero PII & Password Redaction**:
   Never log decrypted user passwords (`password="[REDACTED]"`), credit card numbers, SSNs, or raw extracted text buffers. Only log structural metadata (page counts, byte lengths, SHA-256 hashes).

---

## 7. Acceptance Criteria & Automated Verification

1. **Unit & Integration Tests**:
   - `paperpilot-gateway/tests/test_logging.rs`: Validate that `LOG_FORMAT=json` emits valid parseable NDJSON with required fields (`timestamp`, `level`, `request_id`, `http.status_code`).
   - `paperpilot-cli/tests/test_logging.rs`: Verify `--log-format json` and `--log-level debug` emit structured logs to `stderr` while `stdout` remains clean.
   - `paperpilot-mcp`: Verify stdout contains only valid JSON-RPC envelopes.
2. **Zero Failures on Test Suites**:
   - `cargo test -p paperpilot-gateway -j 4`
   - `cargo test -p paperpilot-cli -j 4`
   - `cargo test -p paperpilot-mcp -j 4`
