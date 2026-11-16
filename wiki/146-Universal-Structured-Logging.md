# Wiki 146: Universal Leveled & Structured Logging Engine

## Overview
Spec 146 implements a Universal Leveled & Structured Logging Engine across `paperpilot-gateway`, `paperpilot-cli`, `paperpilot-mcp`, and `paperpilot-pdf`.

## Core Capabilities
- **Multi-format Logging**: Controlled via `LOG_FORMAT=json|compact|pretty|text` or `--log-format` CLI flag.
- **Granular Log Levels**: Dynamic filtering via `RUST_LOG` environment variable or `--log-level` CLI flag.
- **Dual Error Paradigm**: Internal server-side logging of unredacted root causes and error source chains (`tracing::error!`) paired with clean, client-safe error payloads containing a matching `request_id`.
- **Stdio Isolation**: Standard error (`stderr`) channeling for `paperpilot-cli` and `paperpilot-mcp` to ensure `stdout` remains unpolluted for PDF binary streams and JSON-RPC wire messages.
- **Zero PII & Secret Redaction**: Absolute redaction of user passwords (`password="[REDACTED]"`) in log output and telemetry.

## Request ID Correlation
Incoming HTTP requests in `paperpilot-gateway` inherit or generate a UUID v4 `request_id`. This ID is bound to the request tracing span, passed in the `X-Request-ID` header, and returned in JSON error response bodies.
