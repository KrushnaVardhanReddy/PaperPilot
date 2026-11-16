---
title: "pdf_annotate"
description: "Reference, CLI, MCP, and REST API commands for pdf_annotate."
sidebar:
  order: 7
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Appends interactive and visual annotations (highlights, underlines, notes, and stamps) onto PDF pages.**  
Programmatically injects standard PDF markup annotations (`/Highlight`, `/Underline`, `/Text`, `/Square`, `/FreeText`) at exact spatial coordinates or dynamically anchored to search patterns.

---

### CLI
Add annotations using JSON arrays of markup objects or file-backed descriptors:

#### 1. Coordinate-Based Highlight Annotation
```bash
paperpilot annotate \
  --input ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/annotate_cli_Simple.pdf \
  --data '[{"id":"1","type":"highlight","page":1,"x":50.0,"y":50.0,"w":100.0,"h":20.0,"color":"#ffff00","content":"Key finding"}]' \
  --json
```

#### 2. Pattern & Rule-Based Annotation via File
Pass a path to a JSON file defining multiple annotations:
```bash
paperpilot annotate \
  --input ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/annotate_cli_file.pdf \
  --data ./annotations.json \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--output` *(string, required)*: Path to write the annotated PDF.
- `--data` *(string, optional, default: `[]`)*: JSON string or path to a JSON file containing an array of annotation objects.
- `--json` *(flag, optional)*: Emit structured execution status in JSON format.

---

### MCP JSON-RPC
When invoking `pdf_annotate` via MCP:

#### Coordinate Markup List
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_annotate",
    "arguments": {
      "input": "ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf",
      "annotations": [
        {
          "id": "ann-1",
          "type": "highlight",
          "page": 1,
          "x": 72.0,
          "y": 700.0,
          "w": 250.0,
          "h": 16.0,
          "color": "#ffff00",
          "content": "Abstract overview"
        }
      ]
    }
  },
  "id": 1
}
```

#### Search Pattern Shorthand
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_annotate",
    "arguments": {
      "input": "ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/annotated_search.pdf",
      "search": "quantum algorithm",
      "type": "highlight",
      "color": "#00FF00"
    }
  },
  "id": 2
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output PDF file path.
- `annotations` *(array of objects, optional)*: List of explicit spatial annotations (`id`, `type`, `page`, `x`, `y`, `w`, `h`, `color`, `content`).
- `rules` *(array of rule objects, optional)*: List of rule-based pattern annotations.
- `search` *(string, optional)*: Keyword text to find and annotate across the document.
- `regex` *(string, optional)*: Regular expression pattern to identify and annotate.
- `type` *(string, optional, default: `"highlight"`)*: Annotation type (`"highlight"`, `"underline"`, `"note"`).
- `color` *(string, optional, default: `"#FFFF00"`)*: Hex color code (e.g. `"#FF0000"`).

---

### REST API
Execute annotation injections via PaperPilot Gateway:

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_annotate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf",
    "annotations": [
      {
        "id": "1",
        "type": "highlight",
        "page": 1,
        "x": 50.0,
        "y": 50.0,
        "w": 100.0,
        "h": 20.0,
        "color": "#ffff00",
        "content": "Verified section"
      }
    ]
  }'
```

```bash
# Gateway Tool Route with Search Match
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_annotate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf",
    "search": "PaperPilot",
    "type": "highlight",
    "color": "#ffff00"
  }'
```

---

### Known Limitations & Troubleshooting
- **Viewer Rendering Parity**: Different PDF viewers (Apple Preview, Adobe Acrobat, Chrome PDF Viewer) render `/Highlight` blending modes with slight color variations. Use standard hex values (`#ffff00`, `#ff0000`, `#00ff00`, `#0000ff`).
- **Coordinate System**: Bounding coordinates use bottom-left PDF point origin standards `(x, y, w, h)` where 72 points equal 1 inch.
- **WASM Support**: Standalone in-memory buffer annotations are not currently exported in `paperpilot-wasm`. Use CLI, REST API, or MCP for document annotation workflows.
