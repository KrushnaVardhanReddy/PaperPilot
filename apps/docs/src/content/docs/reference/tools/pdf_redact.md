---
title: "pdf_redact"
description: "Reference, CLI, MCP, and REST API commands for pdf_redact."
sidebar:
  order: 37
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Permanently blackouts and scrubs sensitive text, regular expressions, or coordinate bounding boxes from a PDF.**  
Unlike cosmetic black highlight overlays, PaperPilot's redaction engine sanitizes underlying PDF content streams, glyph positions, and font descriptors to guarantee that redacted characters cannot be extracted or recovered.

---

### CLI
Redact content using coordinate rectangles, exact keywords, regular expressions, or structured rule datasets:

#### 1. Keyword Redaction with Custom Overlay
```bash
paperpilot redact \
  --input ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/redact_cli_keyword.pdf \
  --query "Customer" \
  --overlay-text "[CONFIDENTIAL]" \
  --occurrence all \
  --json
```

#### 2. Regex Pattern Redaction (SSN, Credit Cards, PII)
```bash
paperpilot redact \
  --input ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/redact_cli_regex.pdf \
  --regex "\d{3}-\d{2}-\d{4}" \
  --overlay-text "[SSN REDACTED]" \
  --json
```

#### 3. Coordinate Box Redaction
```bash
paperpilot redact \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/redact_cli_rect.pdf \
  --pages 1 \
  --rect "0,0,100,100" \
  --json
```

#### 4. Structured JSON Redaction Dataset (`--data`)
```bash
paperpilot redact \
  --input ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/redact_cli_data.pdf \
  --data '[{"query":"Customer","overlay_text":"[REDACTED]"}]' \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--output` *(string, required)*: Path to save redacted PDF.
- `--query` *(string, optional)*: Exact string match to scrub across document pages.
- `--regex` *(string, optional)*: Regular expression pattern for automated PII identification and redaction.
- `--data` *(string, optional)*: Inline JSON string or path to JSON file specifying redaction rules.
- `--overlay-text` *(string, optional)*: Text displayed over redaction blackout boxes (e.g., `"[REDACTED]"`).
- `--occurrence` *(string, optional, default: `"all"`)*: Match occurrence filter: `"all"`, `"first"`, `"last"`, or index lists.
- `--pages` *(string, optional, default: `"1"`)*: Target page number (1-based) or page list when redacting by coordinates.
- `--rect` *(string, optional, default: `"0,0,100,100"`)*: Bounding box coordinates formatted as `"x,y,width,height"`.
- `--json` *(flag, optional)*: Emit structured execution status in JSON format.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke `pdf_redact`:

#### Query / Regex Rule Call
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_redact",
    "arguments": {
      "input": "ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/redacted_pattern.pdf",
      "query": "Customer",
      "overlay_text": "[REDACTED]",
      "occurrence": "all"
    }
  },
  "id": 1
}
```

#### Coordinate Bounding Area Call
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_redact",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/redacted_rect.pdf",
      "page": 1,
      "rect": [50.0, 50.0, 200.0, 50.0]
    }
  },
  "id": 2
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the PDF to redact.
- `output` *(string, required)*: Path to save the redacted PDF.
- `query` *(string, optional)*: Search keyword or text pattern to redact permanently.
- `regex` *(string, optional)*: Regular expression to match and scrub.
- `overlay_text` *(string, optional)*: Optional overlay text centered over blackout blocks.
- `occurrence` *(string, optional, default: `"all"`)*: Filter matches (`"all"`, `"first"`, `"last"`).
- `page` *(integer, optional, default: `1`)*: Target page number (1-based) for coordinate redaction.
- `rect` *(array of 4 numbers or comma-separated string, optional)*: Spatial box `[x, y, width, height]`.
- `rules` *(array of rule objects, optional)*: Advanced multi-criteria redaction rule set.

---

### REST API
Execute redactions via PaperPilot Gateway:

```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_redact \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/redacted.pdf",
    "query": "Customer",
    "overlay_text": "[CONFIDENTIAL]"
  }'
```

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_redact \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/redacted.pdf",
    "page": 1,
    "rect": [50, 50, 200, 50]
  }'
```

---

### Known Limitations & Troubleshooting
- **Scanned Bitmaps / Flattened Images**: Text redaction operates on digital vector text streams. Scanned PDFs with flattened raster bitmaps require prior OCR processing (`paperpilot ocr`) or explicit bounding box coordinates (`--rect`) to obscure graphical text.
- **Complex Font Encodings**: If a document uses non-standard font encoding schemes lacking valid `/ToUnicode` CMaps, regex/string matching may fail to identify text tokens. Coordinate-based redactions (`--rect`) bypass font encodings directly.
- **WASM Support**: Standalone buffer operations for `pdf_redact` are not bundled in `paperpilot-wasm` (use CLI, MCP, or REST API).
