---
title: PDF Portfolio Attachments
description: Inspect, extract, or embed file attachments into PDF portfolios.
sidebar:
  order: 52
---

# PDF Portfolio Attachments (`pdf_attachments`)

The **PDF Portfolio Attachments** tool allows users to manage embedded files within PDF documents. A "portfolio" document in PDF can contain arbitrary file types encapsulated securely inside it. This tool supports four distinct operations: list, add, extract, and remove.

## 🚀 Features
- **Inspect Portfolios**: Rapidly read all embedded attachment filenames, descriptions, and sizes.
- **Embed Content**: Losslessly embed any arbitrary file (`.txt`, `.docx`, `.png`, etc.) directly into a host PDF.
- **Extract Data**: Dump embedded file contents safely back to disk.
- **Sanitize**: Remove unneeded attachments to shrink document overhead.

---

## 🛠️ CLI Usage

List all files attached to a document:
```bash
paperpilot pdf-attachments --input portfolio.pdf --action list --output report.json
```

Embed a text file into the portfolio:
```bash
paperpilot pdf-attachments --input base.pdf --action add --file-name secret.txt --file ./secret.txt --output updated.pdf
```

Extract a specific embedded file:
```bash
paperpilot pdf-attachments --input portfolio.pdf --action extract --file-name secret.txt --output extracted_secret.txt
```

---

## 🌐 REST API Usage

### Extract an Attachment
```bash
curl -X POST "http://localhost:7823/api/v1/pdf/attachments?action=extract&file_name=secret.txt" \
  -H "Content-Type: application/json" \
  -d '{"input": "portfolio.pdf", "output": "local_extracted.txt"}'
```

*(Note: In REST, the backend natively interprets the query string to map to the `action` and `file_name` parameters, determining the correct binary HTTP response.)*

---

## 🤖 MCP JSON-RPC

To add an attachment using the Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "pdf_attachments",
    "arguments": {
      "input": "base.pdf",
      "output": "updated.pdf",
      "action": "add",
      "file_name": "data.csv",
      "description": "Monthly financial data",
      "file_data_base64": "SGVsbG8sIFdvcmxkIQ=="
    }
  },
  "id": 1
}
```
