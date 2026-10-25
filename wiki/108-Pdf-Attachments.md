# PDF Portfolio Attachments Engine

The `pdf_attachments` operation provides comprehensive support for inspecting, extracting, embedding, and removing file attachments from PDF Portfolios. This feature leverages the `/EmbeddedFiles` name tree within the PDF document's root catalog, conforming strictly to the PDF 1.7 standard (ISO 32000-1).

## Capabilities

The operation is pure Rust and integrates seamlessly across PaperPilot interfaces, supporting four core actions:
1. **List (`list`)**: Traverses the embedded files catalog, reads descriptions and mime-types, and decompresses stream lengths to produce a structured `AttachmentsReport` in JSON.
2. **Embed (`add`)**: Accepts raw file bytes, compresses them using `FlateDecode` algorithm, generates a standard `/Filespec` entry, and registers the stream securely into the catalog's `/Names` tree.
3. **Extract (`extract`)**: Given a specific portfolio filename, locates and directly decompresses the targeted internal byte stream for output.
4. **Remove (`remove`)**: Removes the specified attachment specification from the `/EmbeddedFiles` index, rewriting the structural catalog mapping.

## E2E Architecture Integration
- **CLI**: Standardizes raw file paths for embedding and extraction (`--file`, `--file-name`).
- **REST Gateway**: Adapts action execution dynamically via query params, setting `application/json`, `application/pdf`, and `application/octet-stream` accordingly.
- **MCP Server**: Safely wraps Base64 strings for robust cross-border RPC interaction and JSON logging.
- **Svelte UI**: Implements an interactive multi-action panel in `OperationsPanel.svelte` for robust graphical management.

## Unit Testing
Comprehensive testing guarantees robust memory management during `Base64` processing, proper byte assertions for decompressed data, and exact structural tracking to gracefully parse empty portfolios.
