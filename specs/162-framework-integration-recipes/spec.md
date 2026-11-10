# Spec 162: AI & Automation Framework Integration Recipes (`recipes/`)

## 1. Executive Summary & Problem Statement
Developers and AI engineers want to incorporate PaperPilot into their existing pipelines without writing low-level Rust or deciphering raw HTTP specs. The most common frameworks for document processing are:
1. **LangChain & LlamaIndex (Python)**: Need custom Document Loaders that extract clean markdown, metadata, and semantic chunks directly via PaperPilot's local REST API or CLI.
2. **n8n & Zapier-like Automation**: Need local workflow JSON definitions connecting hot folders to PaperPilot's REST gateway on `localhost:7823` for automated invoice processing, OCR, and compression.
3. **Claude Desktop / Cursor MCP Configs**: Need turnkey `.json` configuration snippets to connect agents with zero manual configuration.

Spec 162 introduces an official `recipes/` directory containing tested, production-grade integration snippets and starter templates.

---

## 2. Directory Structure & Deliverables

```text
recipes/
├── langchain/
│   ├── README.md
│   ├── paperpilot_loader.py       # Custom LangChain BaseLoader utilizing PaperPilot REST / CLI
│   └── requirements.txt
├── llamaindex/
│   ├── README.md
│   ├── paperpilot_reader.py       # Custom LlamaIndex BaseReader extracting structured markdown
│   └── example.py
├── n8n/
│   ├── README.md
│   └── invoice-auto-ocr-pipeline.json  # Importable n8n workflow JSON utilizing localhost:7823
└── mcp-presets/
    ├── claude_desktop_config.json      # Ready-to-paste Claude desktop config
    └── cursor_mcp_config.json          # Ready-to-paste Cursor / Windsurf config
```

---

## 3. Implementation Code Blueprint

### 3.1 LangChain Loader (`recipes/langchain/paperpilot_loader.py`)
```python
from typing import Iterator, List, Optional
import requests
from langchain_core.document_loaders import BaseLoader
from langchain_core.documents import Document

class PaperPilotPDFLoader(BaseLoader):
    """Load PDFs using a local PaperPilot REST Gateway instance (http://localhost:7823)."""

    def __init__(self, file_path: str, gateway_url: str = "http://localhost:7823", target_tokens: int = 500):
        self.file_path = file_path
        self.gateway_url = gateway_url.rstrip("/")
        self.target_tokens = target_tokens

    def lazy_load(self) -> Iterator[Document]:
        # Invoke PaperPilot semantic chunking
        resp = requests.post(
            f"{self.gateway_url}/api/v1/pdf/chunk",
            json={"input": self.file_path, "target_tokens": self.target_tokens, "strip_boilerplate": True},
            timeout=60
        )
        resp.raise_for_status()
        data = resp.json()
        for chunk in data.get("chunks", []):
            yield Document(
                page_content=chunk["text"],
                metadata={
                    "source": self.file_path,
                    "page_start": chunk["page_start"],
                    "page_end": chunk["page_end"],
                    "headings": chunk.get("headings_context", [])
                }
            )
```

### 3.2 Claude Desktop Configuration Preset (`recipes/mcp-presets/claude_desktop_config.json`)
```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "paperpilot",
      "args": ["mcp"],
      "env": {
        "PAPERPILOT_ZERO_TRACE": "true"
      }
    }
  }
}
```
