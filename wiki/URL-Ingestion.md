# Universal Remote PDF URL Ingestion

## Overview
PaperPilot supports Universal Remote PDF URL Ingestion across all five of its interfaces (Penta-Interface: Desktop, Edge, WASM, CLI, MCP, and Gateway). Instead of requiring users or integrations to manually download a file and supply it locally or pass raw bytes, you can supply a remote HTTP/HTTPS URL directly to any tool or API endpoint.

## Architecture

PaperPilot utilizes a core Rust fetching utility located in `paperpilot_core::url_fetch`. This utility is responsible for securely downloading external payloads.

### Features & Security
- **Magic Byte Validation:** Validates the first 4 bytes of the downloaded stream are `%PDF`, ensuring we do not process malicious or invalid data (e.g. HTML or images).
- **SSRF Protection:** Blocks fetches to loopback (127.0.0.1) and private IP ranges (10.x.x.x, 192.168.x.x, 172.16-31.x.x), as well as cloud metadata endpoints (169.254.169.254).
- **Size Limits:** Enforces a configurable 100MB maximum buffer size to prevent memory exhaustion and DoS attacks.
- **In-memory and Temp File Handlers:** Can return raw `Vec<u8>` or stream to a temporary `PathBuf`.

## Interface Usage

### Desktop & Web UI
Users can click the "🔗 Import from URL" button in the DropZone to paste a URL. The frontend will fetch the document, validate its magic bytes, and convert it to a virtual `File` object that acts exactly like a local file drop.

### REST Gateway & Cloudflare Edge
Any API endpoint (e.g. `/api/v1/rotate`, `/api/v1/compress`) accepts a `url` parameter either in the Query string, a JSON body (`{"url": "https://..."}`), or a `multipart/form-data` field.

**Example Edge Request:**
```bash
curl "https://paperpilot-edge.worker.dev/api/v1/rotate?angle=90&url=https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf" --output result.pdf
```

### CLI
Any `input` parameter in the CLI that starts with `http://` or `https://` is automatically intercepted, downloaded to a secure temporary file, and processed transparently.

**Example CLI Request:**
```bash
paperpilot pdf_rotate --input https://example.com/test.pdf --output result.pdf
```

### MCP Server
When executing an MCP Tool Call (e.g. from an LLM Agent), any `input`, `file`, `images`, or `paths` argument starting with `http://` or `https://` is fetched automatically and swapped for a secure local temporary file path before the tool runs.
