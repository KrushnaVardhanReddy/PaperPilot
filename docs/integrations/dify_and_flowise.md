# Dify & Flowise AI Integration Guide

Integrate PaperPilot as a high-speed document processing tool and knowledge ingestion engine in **Dify** and **Flowise**.

---

## 1. Overview

Both Dify and Flowise provide low-code/no-code environments for building LLM agents and RAG applications. By connecting them to PaperPilot Gateway, you gain:
- Sub-millisecond text and tabular extraction.
- Neural OCR without third-party cloud API costs.
- Semantic document chunking preserving Markdown table formatting.

---

## 2. Dify Custom Tool Integration

In Dify, navigate to **Tools > Custom Tools > Create Custom Tool**:

### OpenAPI Specification (Paste into Dify)
```yaml
openapi: 3.0.1
info:
  title: PaperPilot PDF Tools
  version: 1.0.0
  description: High-performance PDF processing engine
servers:
  - url: http://host.docker.internal:7823
paths:
  /api/v1/pdf/chunk:
    post:
      summary: Semantic PDF Chunker
      operationId: chunkPdf
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              required:
                - input
              properties:
                input:
                  type: string
                  description: Absolute path to the PDF file
                target_tokens:
                  type: integer
                  default: 500
      responses:
        '200':
          description: Chunked PDF text with headings context
  /api/v1/pdf/tools/pdf_extract_text:
    post:
      summary: Extract Text & Markdown
      operationId: extractPdfText
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              required:
                - input
              properties:
                input:
                  type: string
                  description: Path to PDF file
      responses:
        '200':
          description: Extracted markdown content
```

Once saved, enable the tool in any **Dify Agent** or **Workflow**.

---

## 3. Flowise Custom Tool Integration

In Flowise, add a **Custom Tool** node to your canvas:

- **Tool Name**: `paperpilot_chunk`
- **Tool Description**: `Chunks a PDF document into semantic sections preserving tables and headers.`
- **Schema**:
  ```json
  {
    "type": "object",
    "properties": {
      "input": {
        "type": "string",
        "description": "Path to the PDF file on the server"
      },
      "target_tokens": {
        "type": "number",
        "description": "Target token size per chunk (e.g. 500)"
      }
    },
    "required": ["input"]
  }
  ```
- **Javascript Function**:
  ```javascript
  const fetch = require('node-fetch');

  const response = await fetch('http://localhost:7823/api/v1/pdf/chunk', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
          input: $input,
          target_tokens: $target_tokens || 500
      })
  });

  const data = await response.json();
  return JSON.stringify(data.chunks.map(c => c.text));
  ```
