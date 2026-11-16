---
title: "pdf_bates"
description: "Reference, CLI, MCP, and REST API commands for pdf_bates."
sidebar:
  order: 8
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Applies consecutive legal Bates numbering stamps across documents.**  
Stamps formatted identification numbers (e.g. `CONF-000001`, `EXHIBIT-A-000042`) onto page margins for legal discovery, audit trails, and court filings.

---

### CLI
Apply Bates numbering directly via the PaperPilot CLI:

```bash
paperpilot bates \
  --input ManaulTestingFixtures/pdf_bates/Input/legal_discovery_document.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/bates_cli_Simple.pdf \
  --prefix "CONF-" \
  --start 1 \
  --position bottom-right \
  --json
```

#### Parameters
- `--input` *(string, required)*: Path to the source PDF document.
- `--output` *(string, required)*: Target destination file path for the Bates-numbered PDF.
- `--prefix` *(string, optional, default: `"CONF-"`)*: String prefix prepended to the padded counter.
- `--start` *(integer, optional, default: `1`)*: Initial sequential integer to begin numbering from.
- `--position` *(string, optional, default: `"bottom-right"`)*: Page location where the Bates stamp will be anchored:
  - `bottom-right` (default)
  - `bottom-left`
  - `bottom-center`
  - `top-right`
  - `top-left`
  - `top-center`
- `--json` *(flag, optional)*: Emit structured execution status in JSON format.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_bates`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_bates",
    "arguments": {
      "input": "ManaulTestingFixtures/pdf_bates/Input/legal_discovery_document.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf",
      "prefix": "CONF-",
      "start_number": 1,
      "padding": 6,
      "position": "bottom-right"
    }
  },
  "id": 1
}
```

#### Schema Parameters
- `input` *(string, required)*: Source PDF file path.
- `output` *(string, required)*: Output PDF file path.
- `start_number` or `start` *(integer, optional, default: `1`)*: Starting sequence counter.
- `prefix` *(string, optional, default: `"CONF-"`)*: Prefix prepended to the number.
- `padding` *(integer, optional, default: `6`)*: Total zero-padding width (e.g. `6` produces `000001`).
- `position` *(string, optional, default: `"bottom-right"`)*: Anchor location (`bottom-right`, `bottom-left`, `bottom-center`, `top-right`, `top-left`, `top-center`).

---

### REST API
Execute Bates stamping via PaperPilot Gateway:

```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_bates \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "ManaulTestingFixtures/pdf_bates/Input/legal_discovery_document.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf",
    "prefix": "CONF-",
    "start_number": 1,
    "position": "bottom-right"
  }'
```

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bates \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "ManaulTestingFixtures/pdf_bates/Input/legal_discovery_document.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf",
    "prefix": "CONF-",
    "start_number": 1,
    "padding": 6,
    "position": "bottom-right"
  }'
```

---

### Declarative Workflow
In declarative multi-stage document pipelines (`paperpilot run` or `paperpilot workflow`):

```toml
[[stage]]
id = "bates_stamping"
op = "pdf_bates"
[stage.params]
prefix = "LEGAL-"
start = 100
padding = 6
position = "bottom-right"
```

---

### Known Limitations & Troubleshooting
- **WASM Support**: Bates numbering is currently supported in native desktop, CLI, REST API, and MCP environments, but is not exposed in `paperpilot-wasm` standalone buffer exports (only declarative pipeline workflows).
- **Page Margins**: Stamps use a fixed margin of 30 points and standard Helvetica typography. Documents with minimal margins or critical edge bleeds may overlap text; use `position: "top-center"` or `position: "bottom-center"` if margins at corners are obstructed.
- **Unrecognized Positions**: Specifying an invalid position string safely falls back to `bottom-right`.
