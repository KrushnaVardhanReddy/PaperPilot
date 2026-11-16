---
title: "pdf_to_pptx"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_pptx."
sidebar:
  order: 49
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Microsoft PowerPoint Presentation (`.pptx`)

### What it does
**Converts PDF document pages into editable Microsoft PowerPoint presentation slides (.pptx).**  
Extracts each PDF page's textual content and structural elements into individual PowerPoint presentation slides. The resulting `.pptx` container packages standard OpenXML relationships, presentation manifest, slide layouts, and content types for full native compatibility with Microsoft PowerPoint, Apple Keynote, and Google Slides.

- **Page-to-Slide Mapping**: Generates exactly one slide per PDF page, mapping extracted page texts to positioned text frame objects.
- **OpenXML Conformance**: Implements standard Open Packaging Conventions (OPC) with slide manifests and MIME content types (`application/vnd.openxmlformats-officedocument.presentationml.slide+xml`).
- **Pipeline Intermediary**: PPTX output files can be fed directly downstream to `office_extract_text` or `office_to_markdown`.

---

### CLI

Convert PDF documents to PowerPoint presentations using the unified `convert` command:

```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format pptx \
  --output tests/e2e_fixtures/out/penta_e2e/slides.pptx \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the source PDF document.
- `--format` *(string, required)*: Target format specifier (`pptx`).
- `--output` *(string, optional, default: `"output.pptx"`)*: Destination path for the `.pptx` presentation.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

#### Example CLI Output
```json
{"success":true,"operation":"Convert","error":null}
```

---

### MCP JSON-RPC

Invoke `pdf_to_pptx` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pptx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/slides.pptx"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output PPTX presentation file path.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Converted to PPTX successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/slides.pptx\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Convert documents through PaperPilot Gateway:

```bash
# Standard Tool API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_pptx \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/slides.pptx"
  }'
```

```bash
# Convert API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "format": "pptx",
    "output": "tests/e2e_fixtures/out/penta_e2e/slides.pptx"
  }'
```

#### Example Gateway Response
```json
{
  "success": true,
  "data": null,
  "error": null
}
```

---

### WASM Support
Direct OpenXML slide assembly is currently not compiled into `paperpilot-wasm`. In web client applications, trigger PPTX conversions using the PaperPilot Gateway REST endpoint or MCP server.

---

### Known Limitations & Troubleshooting
- **CLI Subcommand Note**: Use `paperpilot convert --input <INPUT> --format pptx --output <OUTPUT>`. `pdf_to_pptx` is invoked through the unified `convert` command family in the CLI.
- **Graphic Shape Reconstruction**: Complex embedded vector diagrams and curves are treated as layout elements; slide bodies focus on text runs and page layout frames. For graphical slides, render pages to PNG with `paperpilot render` and insert as slide backgrounds.
