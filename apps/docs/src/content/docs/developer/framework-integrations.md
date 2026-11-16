---
title: AI Framework Integrations (LangChain, n8n, Open-WebUI)
description: Connect PaperPilot to LangChain, LangGraph, n8n, Open-WebUI, Dify, and Flowise.
---

PaperPilot provides zero-copy document processing, semantic chunking (`pdf_chunk`), and neural OCR directly to modern AI orchestration frameworks and automation platforms.

---

## Supported Frameworks & Guides

### 1. LangChain & LangGraph (Python)
Use PaperPilot Gateway as a high-speed replacement for `PyPDFLoader` and `RecursiveCharacterTextSplitter`.

- **Semantic Chunker**: Call `POST /api/v1/pdf/chunk` with `target_tokens` to extract clean chunks with preserved section headings and markdown tables.
- **Agent Tools**: Wrap PaperPilot endpoints (`pdf_ocr`, `pdf_extract_text`, `pdf_merge`) as LangChain `@tool` functions for LangGraph ReAct agents.
- **Reference**: See full Python code samples in [docs/integrations/langchain_and_langgraph.md](https://github.com/KrushnaVardhanReddy/PaperPilot/blob/feature/dev/docs/integrations/langchain_and_langgraph.md).

---

### 2. Open-WebUI (Drop-in Apache Tika Replacement)
Replace heavy Apache Tika JVM instances with PaperPilot:

```env
CONTENT_EXTRACTION_ENGINE=tika
TIKA_SERVER_URL=http://127.0.0.1:7823
TIKA_SERVER_VERSION=3
```

- Uploading PDFs in chat transparently sends binary streams to `/tika/text`.
- Zero JVM memory overhead, ~10x to 50x faster extraction.
- **Reference**: See configuration in [docs/integrations/open_webui.md](https://github.com/KrushnaVardhanReddy/PaperPilot/blob/feature/dev/docs/integrations/open_webui.md).

---

### 3. n8n Workflow Automation
Use n8n's **HTTP Request** node to process PDF invoices, extract tables, and send semantic chunks to vector databases (Qdrant, Pinecone):

- **Chunk Endpoint**: `POST http://host.docker.internal:7823/api/v1/pdf/chunk`
- **OCR Endpoint**: `POST http://host.docker.internal:7823/api/v1/pdf/tools/pdf_ocr`
- **Reference**: Copy sample workflow JSON from [docs/integrations/n8n.md](https://github.com/KrushnaVardhanReddy/PaperPilot/blob/feature/dev/docs/integrations/n8n.md).

---

### 4. Dify & Flowise (No-Code AI Apps)
Import the PaperPilot OpenAPI 3.0 specification (`/api-docs/openapi.json`) into Dify or configure a custom JavaScript tool in Flowise to enable instant document understanding.

- **Reference**: See [docs/integrations/dify_and_flowise.md](https://github.com/KrushnaVardhanReddy/PaperPilot/blob/feature/dev/docs/integrations/dify_and_flowise.md).
