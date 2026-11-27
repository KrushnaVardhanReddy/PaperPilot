# Architecture

This document tracks the main architectural decisions for PaperPilot.

## Architecture Decision Records (ADRs)

*   [ADR 0001: PDF Engine Selection](../docs/adr/0001-pdf-engine-selection.md)
*   [ADR 0002: On-Device AI Engine Selection & Embedded Local RAG](../docs/adr/0002-on-device-ai-engine-and-rag.md)

## System Components

1. **`paperpilot-core` / `paperpilot-pdf`**: Core domain logic, typed operations (`PdfOperation`), lopdf/pdfium abstractions, DocAST layout parser, zero-copy memory-mapped streaming (`memmap2`), and in-memory self-healing parser.
2. **`paperpilot-cli`**: Clap-based native CLI binary with JSON streaming support and natural language interface (`cli_ask`).
3. **`paperpilot-mcp`**: Standard Model Context Protocol (stdio/HTTP) exposing all 101 PDF operations as agent tools with Glama TDQS Grade A+ schemas.
4. **`paperpilot-gateway`**: Axum REST API with OpenAPI 3.0 / Swagger UI, configurable payload body limits, and OWASP baseline security headers.
5. **`paperpilot-wasm`**: Pure-Rust WebAssembly client engine powering client-side execution in browsers, mobile web, and Cloudflare Edge microservices.
6. **`paperpilot-storage`**: Hexagonal persistence adapter (`DatabasePort`) embedding Turso / libSQL for offline-first jobs, audit logs, and vector embeddings.
7. **`paperpilot-nlp` / `paperpilot-ai`**: On-device AI stack (pure-Rust `rten` inference, TinyBERT, SmolLM, and GGUF quantization).
8. **`apps/desktop`**: Tauri 2 + Svelte 5 lightweight native desktop application with visual workflow pipeline builder.
9. **`apps/web` / `apps/embed`**: Official web portal and embeddable white-label widget studio.

## Architectural Deep-Dives

*   [Tool 101: In-Place PDF Text Replacement (`pdf_edit_text`)](101-Pdf-Edit-Text.md)
*   [Engine Architecture: Zero-Copy Memory Mapping & Hexagonal Storage](Engine-Performance-And-Storage.md)
*   [Security Architecture: Dynamic & Static Analysis, Leak Scanning, and Container Hardening](Security-Audits-And-Scanners.md)
*   [Penta-Interface Mandate and End-to-End Protocol](25-Penta-Interface-Mandate-And-E2E-Protocol.md)
*   [DocAST Layout Analysis](PDF-Oxide-Layout-Analysis.md)
*   [Universal Structured Logging Engine](146-Universal-Structured-Logging.md)

