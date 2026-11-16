# PaperPilot AI Framework Integration Recipes

This directory contains turnkey integrations, workflows, and configurations to seamlessly connect PaperPilot's offline capabilities with popular AI frameworks and automation tools.

## Supported Integrations

- [LangChain](#langchain)
- [LlamaIndex](#llamaindex)
- [MCP (Model Context Protocol) Presets](#mcp-model-context-protocol-presets)
- [n8n Automation](#n8n-automation)

---

## LangChain

Connect PaperPilot's document processing to your LangChain pipelines for RAG (Retrieval-Augmented Generation).

### Setup
1. Copy `langchain/paperpilot_loader.py` to your project.
2. Install dependencies: `pip install -r langchain/requirements.txt`

### Usage
```python
from paperpilot_loader import PaperPilotLoader

# Connects to your local PaperPilot Gateway to chunk and extract text
loader = PaperPilotLoader(file_path="financial_report_2023.pdf")
docs = list(loader.lazy_load())

for doc in docs:
    print(f"Page {doc.metadata['page']} - {doc.metadata['tokens']} tokens")
    print(doc.page_content)
```

---

## LlamaIndex

Use PaperPilot as a robust local reader for your LlamaIndex knowledge bases.

### Setup
1. Copy `llamaindex/paperpilot_reader.py` to your project.
2. Install dependencies: `pip install -r llamaindex/requirements.txt`

### Usage
```python
from paperpilot_reader import PaperPilotReader

reader = PaperPilotReader()
docs = reader.load_data(file_path="user_manual.pdf")

# Use docs with a LlamaIndex VectorStoreIndex
# from llama_index.core import VectorStoreIndex
# index = VectorStoreIndex.from_documents(docs)
```

---

## MCP (Model Context Protocol) Presets

Connect PaperPilot natively to AI assistants that support MCP, allowing them to autonomously read, edit, and analyze PDFs locally.

### Claude Desktop
Add the contents of `mcp-presets/claude_desktop_config.json` to your Claude Desktop configuration file (typically located at `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS).

### Cursor IDE
Configure Cursor to use the `paperpilot-mcp` command as an MCP server using the structure in `mcp-presets/cursor_mcp_config.json`.

---

## n8n Automation

Automate document workflows visually using n8n and the local PaperPilot REST API.

### Hot-Folder OCR Pipeline
The `n8n/invoice-auto-ocr-pipeline.json` provides a pre-built workflow that:
1. Watches a designated folder for new PDF files.
2. Sends them to the local PaperPilot Gateway for offline OCR.
3. Saves the extracted structured JSON data back to disk.

**To Import:**
1. Open your n8n workspace.
2. Go to Workflows -> Import from File.
3. Select `invoice-auto-ocr-pipeline.json`.
