# Framework & AI Agent Integrations

Connect PaperPilot's zero-copy Rust PDF manipulation and extraction engine directly into your AI workflows, agent frameworks, and self-hosted automation platforms.

---

## Available Integration Guides

| Platform / Framework | Type | Primary Capabilities | Guide |
|---|---|---|---|
| **Open-WebUI** | Web UI / Drop-in Tika | Transparent document upload ingestion, 10x faster than Apache Tika, zero memory JVM overhead | [Open-WebUI Integration Guide](open_webui.md) |
| **LangChain & LangGraph** | Python AI SDK / State Agents | Semantic PDF Chunking (`pdf_chunk`), dynamic tool-calling ReAct agents, MCP stdio | [LangChain & LangGraph Guide](langchain_and_langgraph.md) |
| **n8n Automation** | Workflow Automation | Zero-code HTTP Request nodes for PDF chunking, OCR, table extraction, and Vector DB pipelines | [n8n Automation Guide](n8n.md) |
| **Dify & Flowise** | Low-Code AI App Builders | OpenAPI custom tools and JavaScript tool nodes for document RAG applications | [Dify & Flowise Guide](dify_and_flowise.md) |

---

## Universal Protocol Support

PaperPilot supports three universal protocols out of the box:

1. **REST Microservice Gateway** (`http://127.0.0.1:7823`):
   - Interactive OpenAPI/Swagger documentation available at `http://127.0.0.1:7823/swagger-ui`.
   - OpenAPI 3.0 schema at `http://127.0.0.1:7823/api-docs/openapi.json`.
2. **Model Context Protocol (MCP)**:
   - Full STDIO Model Context Protocol server exposing all 91 tools to Claude Desktop, Cursor, Zed, Cline, and custom agent loops via `paperpilot-mcp`.
3. **Apache Tika Wire Protocol Compatibility**:
   - Native emulation of `PUT /tika/text`, `/tika/json/md`, and `/rmeta/text` for transparent integration with any software expecting an Apache Tika server.
