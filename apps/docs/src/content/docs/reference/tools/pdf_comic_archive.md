---
title: "pdf_comic_archive"
description: "Reference, CLI, MCP, and REST API commands for pdf_comic_archive."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF / CBR/CBZ

### What it does
**Convert PDFs to Comic Archives.**
This tool converts a standard PDF document containing images into a comic book archive format (CBZ or CBR).

### CLI
```bash
paperpilot convert --format cbr --input tests/e2e_fixtures/real/test.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_comic_archive_result.cbr
```

### MCP JSON-RPC
```json
{
  "name": "pdf_comic_archive",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_comic_archive_result.cbr"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/comic-archive \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/test.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_comic_archive_result.cbr"}'
```
