# Documentation RAG (Retrieval-Augmented Generation)

## Overview
PaperPilot implements an embedded, zero-cloud Documentation RAG engine to answer user queries about its operations natively via the Chat Panel. This allows users to ask questions like "How do I encrypt a PDF with password?" and receive accurate CLI commands, REST cURL requests, and MCP JSON-RPC payload snippets instantly.

## Architecture
- **In-Memory Search (`paperpilot-nlp/src/rag.rs`)**: Uses an embedded TF-IDF vectorization and cosine similarity approach for matching user questions to the indexed tool documentation chunks.
- **Indexed Chunks**: Pre-seeds 44+ PDF operations' metadata, tool syntax, and CLI/API examples based on the `TRI_INTERFACE_E2E_AND_DOCS.md` report.
- **Tauri IPC Bridge (`apps/desktop/src-tauri/src/lib.rs`)**: Exposes `query_documentation_rag` which serializes the result `RagAnswer` back to the frontend.
- **Frontend Integration (`apps/desktop/src/lib/components/PdfChatPanel.svelte`)**: Intercepts chat messages and proactively checks the RAG engine before defaulting to NLP tool mapping. Valid hits render a `docs-answer-card` UI.

## Performance
- The embedded index allows `< 5ms` query latency on the CPU without requiring heavy LLMs, external cloud providers, or dedicated vector databases.
