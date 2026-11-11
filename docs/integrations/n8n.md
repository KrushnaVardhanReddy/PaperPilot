# n8n Automation & Workflow Integration Guide

Integrate PaperPilot into **n8n** self-hosted workflow automation to transform invoices, run high-precision OCR, merge documents, redact sensitive PII, and generate semantic embeddings for AI agent workflows.

---

## 1. Quick Setup

Make sure your PaperPilot Gateway is running and reachable by your n8n container or process:

```bash
paperpilot serve --port 7823
```

- **If n8n is running locally**: Target `http://127.0.0.1:7823`.
- **If n8n is running inside Docker**: Target `http://host.docker.internal:7823`.

---

## 2. n8n HTTP Request Node Patterns

You can invoke any PaperPilot capability using n8n's standard **HTTP Request Node**.

### Pattern A: Extract & Chunk PDF for Vector Databases (Qdrant / Pinecone)

Connect an **HTTP Request Node** to extract structured markdown text and semantic chunks directly:

- **Method**: `POST`
- **URL**: `http://host.docker.internal:7823/api/v1/pdf/chunk`
- **Send Body**: `JSON`
- **Body Parameters**:
  ```json
  {
    "input": "={{ $json.filePath }}",
    "target_tokens": 500
  }
  ```
- **Output Data**:
  ```json
  {
    "total_chunks": 4,
    "total_tokens": 1240,
    "chunks": [
      {
        "chunk_index": 0,
        "page_start": 1,
        "page_end": 2,
        "estimated_tokens": 310,
        "text": "# Invoice Statement...",
        "headings_context": ["Invoice Details"]
      }
    ]
  }
  ```
- *Next Node in n8n*: Pipe `$json.chunks` into an **Item Lists (Split Out Items)** node, then send each chunk straight to the **OpenAI Embeddings** or **Qdrant Vector Store** node!

---

### Pattern B: OCR Scanned Document Ingestion

Convert scanned invoices received via email/webhook into clean, searchable PDFs:

- **Method**: `POST`
- **URL**: `http://host.docker.internal:7823/api/v1/pdf/tools/pdf_ocr`
- **Send Body**: `JSON`
- **Body Parameters**:
  ```json
  {
    "input": "={{ $json.attachmentPath }}",
    "output": "/tmp/ocr_verified_{{ $json.attachmentId }}.pdf"
  }
  ```

---

### Pattern C: Tika Ingestion for n8n AI Chat Document Loaders

If you use n8n's **AI Agent** or **Information Extractor** nodes, you can set the **Custom Document Loader** or HTTP Request node to point to PaperPilot's native Tika endpoint:

- **Method**: `PUT`
- **URL**: `http://host.docker.internal:7823/tika/text`
- **Send Binary Data**: Yes (`data`)
- **Headers**:
  - `Content-Type`: `application/pdf`
- **Response**:
  ```json
  {
    "X-TIKA:content": "Full extracted document text...",
    "xmpTPg:NPages": "5"
  }
  ```

---

## 3. Sample n8n Workflow JSON

Import this sample snippet directly into n8n via **Import from Clipboard**:

```json
{
  "nodes": [
    {
      "parameters": {
        "method": "POST",
        "url": "http://host.docker.internal:7823/api/v1/pdf/chunk",
        "sendBody": true,
        "specifyBody": "json",
        "jsonBody": "{\n  \"input\": \"/data/sample.pdf\",\n  \"target_tokens\": 500\n}",
        "options": {}
      },
      "id": "e2e92c20-745a-497b-9231-1b29fae5e6f1",
      "name": "PaperPilot Semantic Chunker",
      "type": "n8n-nodes-base.httpRequest",
      "typeVersion": 4.2,
      "position": [
        460,
        300
      ]
    }
  ],
  "connections": {}
}
```
