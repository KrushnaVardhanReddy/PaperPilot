---
title: "pdf_to_pptx"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_pptx."
sidebar:
  order: 49
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Converts PDF pages into an editable Microsoft PowerPoint presentation (.pptx).**  
Exports each PDF page's extracted text into a presentation slide. The PPTX includes the package relationships, presentation manifest, slide relationships, and content types needed by Office readers. Its output can be passed to `office_extract_text` as a real `.pptx` intermediate.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format pptx --output tests/e2e_fixtures/out/penta_e2e/out.pptx --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_pptx",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.pptx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-pptx -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pptx"}'
```
