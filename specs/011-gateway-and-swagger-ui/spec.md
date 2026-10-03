# Spec: Phase 2.5 — PaperPilot Gateway, OpenAPI 3.1 & Interactive Swagger UI

## Objective
Provide Stirling-PDF API parity by embedding an interactive Swagger UI at `/swagger-ui` and serving a standard OpenAPI 3.1 specification at `/api-docs/openapi.json`. This allows developers, homelab self-hosters, and automated scripts to explore, test, and generate client SDKs for all 14+ REST endpoints and 44 MCP tools.

## Architecture Decision (Locked)
- **Framework**: `axum` (current gateway framework).
- **OpenAPI Generator**: `utoipa = { version = "5", features = ["axum", "uuid"] }`.
- **Embedded Swagger UI**: `utoipa-swagger-ui = { version = "8", features = ["axum"] }`.

## API Surface & Endpoints

### 1. Dedicated Operation Endpoints
- `POST /api/v1/pdf/merge`: Takes `{ "inputs": string[], "output": string }`, returns `{ "success": true, "output_path": string }`.
- `POST /api/v1/pdf/split`: Takes `{ "input": string, "ranges": string, "output_pattern": string }`.
- `POST /api/v1/pdf/compress`: Takes `{ "input": string, "output": string, "level": "low"|"medium"|"high" }`.
- `POST /api/v1/pdf/extract-text`: Takes `{ "input": string, "output": string }`.
- `POST /api/v1/pdf/convert`: Converts Markdown / HTML / Images to styled PDF with CSS presets.
- `POST /api/v1/pdf/watermark`: Stamps text or branding onto all pages.
- `POST /api/v1/pdf/info`: Returns metadata, page count, and structural metrics.
- `POST /api/v1/pdf/compare`: Compares two documents and flags detected revisions.
- `POST /api/v1/pdf/render-page`: Headless rasterization of a specific page to PNG image bytes.

### 2. Universal Tool Router
- `POST /api/v1/pdf/tools/{tool_name}`: Universal JSON dispatcher accepting `{ "arguments": { ... } }` and routing to any of the 44 MCP tools.

### 3. MCP Protocol Endpoints
- `GET /mcp/sse`: Server-Sent Events endpoint for persistent MCP agent connections.
- `POST /mcp/messages`: Message posting endpoint for JSON-RPC MCP frames.

## Swagger UI Access
- Interactive UI: `http://localhost:7823/swagger-ui`
- OpenAPI JSON Spec: `http://localhost:7823/api-docs/openapi.json`
- Static Export: `docs/api/openapi.json`
