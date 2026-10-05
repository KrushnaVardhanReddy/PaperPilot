# Architecture

This document tracks the main architectural decisions for PaperPilot.

## Architecture Decision Records (ADRs)

*   [ADR 0001: PDF Engine Selection](../docs/adr/0001-pdf-engine-selection.md)
*   [ADR 0002: On-Device AI Engine Selection & Embedded Local RAG](../docs/adr/0002-on-device-ai-engine-and-rag.md)

## System Components

1. **`paperpilot-core` / `paperpilot-pdf`**: Core domain logic, typed operations (`PdfOperation`), and lopdf/pdfium abstractions.
2. **`paperpilot-cli`**: Clap-based native CLI binary with JSON streaming support.
3. **`paperpilot-mcp`**: Standard Model Context Protocol (stdio/HTTP) exposing all 44 PDF operations as agent tools.
4. **`paperpilot-gateway`**: Axum REST API with OpenAPI/Swagger UI and conversion routes.
5. **`paperpilot-nlp` / `paperpilot-ai`**: On-device AI stack (TinyBERT-4L-312D INT8 ONNX embedded via `include_bytes!`, SmolLM-135M, and SQLite documentation RAG).
6. **`apps/desktop`**: Tauri 2 + Svelte 5 lightweight native desktop application.

