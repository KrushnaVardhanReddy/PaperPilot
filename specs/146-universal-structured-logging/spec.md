# Spec 146: Universal Leveled & Structured Logging Engine

## Metadata
- **Spec Number**: 146
- **Title**: Universal Leveled & Structured Logging Engine
- **Phase**: Cross-Cutting Architecture / Production Observability
- **Status**: Draft / Ready for Implementation
- **Impacted Crates**: `paperpilot-gateway`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-pdf`
- **Dependencies**: `tracing`, `tracing-subscriber` (with `json` and `env-filter` features)

---

## 1. Executive Summary

As PaperPilot transitions into headless production services, Docker containers, Kubernetes pods, and enterprise deployments, unstructured stderr/stdout text logging becomes a bottleneck for log aggregation, observability, and compliance.

This specification unifies logging across all PaperPilot interfaces to support:
1. **Multiple Formats**: `json` (machine-readable NDJSON), `compact` (single-line low-noise), `pretty` (indented multi-line), and `text` (human-readable ANSI color).
2. **Granular Log Levels**: Full dynamic filtering across `trace`, `debug`, `info`, `warn`, and `error`.
3. **HTTP Request Tracing**: Axum middleware capturing method, path, status code, duration in milliseconds, client IP, and byte volume.
4. **Strict Safety & Stdio Isolation**: Zero logging to `stdout` in `paperpilot-mcp` (preserving the JSON-RPC transport), and absolute redaction of passwords and raw document text buffers.

---

## 2. Configuration Schema & Environment Variables

### 2.1 Environment Variables
All executables (`paperpilot`, `paperpilot-gateway`, `paperpilot-mcp`) must recognize:

* **`LOG_FORMAT`**:
  - `text` *(default)*: Standard human-readable terminal log with timestamps, level, and targets.
  - `json`: Structured NDJSON (one valid JSON object per line) suitable for Vector, FluentBit, Datadog, Splunk.
  - `compact`: Single-line concise format omitting target prefixes for clean container logs.
  - `pretty`: Multi-line indented blocks with field highlighting for deep local debugging.
* **`RUST_LOG`**:
  - Standard Rust directive supporting module-specific overrides:
  - Example: `RUST_LOG=info,paperpilot_gateway=debug,paperpilot_pdf=warn`
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

## 3. Structured JSON Schema Specification

When `LOG_FORMAT=json` is active, every log entry MUST conform to the following schema:

```json
{
  "timestamp": "2026-10-30T15:30:00.123456Z",
  "level": "INFO",
  "target": "paperpilot_gateway::middleware",
  "message": "HTTP request completed",
  "http.method": "POST",
  "http.route": "/api/v1/tools/pdf_compress",
  "http.status_code": 200,
  "duration_ms": 38.5,
  "client.ip": "127.0.0.1",
  "tool.id": "pdf_compress",
  "bytes.processed": 1048576
}
```

### Required Fields:
* `timestamp`: RFC 3339 / ISO 8601 UTC timestamp with microsecond precision.
* `level`: One of `"TRACE"`, `"DEBUG"`, `"INFO"`, `"WARN"`, `"ERROR"`.
* `target`: Module path where the event occurred.
* `message`: Human-readable summary of the event.

---

## 4. Architectural Workstreams

### Workstream 1: Gateway Request & Response Middleware (`paperpilot-gateway`)
- Add `tracing` HTTP middleware in Axum server.
- On each request: record request start timestamp, method, URI, and client IP.
- On response: compute elapsed execution time (`duration_ms`), capture status code, and emit an `INFO` level tracing span event.
- If status >= 500, emit at `ERROR` level with error message.
- If status between 400 and 499, emit at `WARN` level.

### Workstream 2: CLI Global Flags & Stderr Channeling (`paperpilot-cli`)
- Add global `--log-format` and `--log-level` flags to the root `Cli` struct in `paperpilot-cli/src/cli.rs`.
- Initialize `tracing_subscriber` directing ALL log output to `io::stderr`.
- Preserve `stdout` exclusively for raw operation binary bytes or structured `--json` command returns.

### Workstream 3: MCP Stdio Protocol Protection (`paperpilot-mcp`)
- The MCP server communicates with Claude Desktop / Cursor using JSON-RPC messages over `stdin`/`stdout`.
- `tracing_subscriber` in `paperpilot-mcp/src/main.rs` MUST be explicitly configured with `.with_writer(std::io::stderr)`.
- Under NO circumstance should non-RPC logs be written to `stdout`.

### Workstream 4: PDF Engine Tracing Spans (`paperpilot-pdf`)
- Add structured execution spans to high-latency operations:
  - `pdf_compress`: input size, output size, compression ratio.
  - `ocr`: image dimensions, language, OCR inference latency.
  - `merge`: number of input files, total page count, output size.
- Ensure all logging uses zero-cost inactive macros (`tracing::debug!`, `tracing::trace!`).

---

## 5. Security & Privacy Non-Negotiables

1. **Zero Password Leakage**:
   - `password` parameters (used in `pdf_decrypt`, `pdf_encrypt`, `pdf_change_permissions`) must NEVER be printed in log arguments or JSON fields.
   - Redact to `password="[REDACTED]"` if logged.
2. **Zero PII & Document Body Dumps**:
   - Never print extracted document text, form field values, or raw PDF streams into logs.
   - Only log metadata: document byte length, page count, SHA-256 hash, and field names.

---

## 6. Acceptance Criteria & Automated Verification

1. **Format Parity**:
   - `LOG_FORMAT=json cargo run --bin paperpilot-gateway` emits valid NDJSON parsable by `serde_json`.
   - `LOG_FORMAT=compact` emits single-line logs without target prefixes.
   - `LOG_FORMAT=text` emits colored human-readable logs.
2. **CLI Stderr Guarantee**:
   - `paperpilot --log-format json merge -i a.pdf -i b.pdf -o out.pdf 2> log.json` creates valid `out.pdf` on disk and valid JSON log on stderr.
3. **MCP Stdio Guarantee**:
   - `paperpilot-mcp` stdout contains ONLY valid JSON-RPC envelopes; all diagnostic traces route to stderr.
4. **All Tests Pass**:
   - `cargo test -p paperpilot-gateway -j 4`
   - `cargo test -p paperpilot-cli -j 4`
   - `cargo test -p paperpilot-mcp -j 4`
