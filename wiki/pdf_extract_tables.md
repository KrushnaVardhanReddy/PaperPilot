# PDF Extract Tables (`pdf_extract_tables`)

## Overview

The `pdf_extract_tables` command implements a pure-Rust engine that analyzes structural PDF drawing operators to cluster text and paths into distinct `ExtractedTable` representations. It operates entirely without headless browsers, Chromium binaries, Python environments, or Java dependencies.

## Supported Formats

- **JSON**: Fully structured spatial data including tables list, table ID, source page, bounding box `bbox`, structural matrix sizes `rows`/`cols`, detected `headers`, and a unified `data` multi-dimensional 2D matrix array.
- **CSV**: Generates strict RFC 4180 compliant Comma-Separated Values, heavily utilizing defensive double-quote escaping techniques to protect internal cell data spanning multiline strings and numeric values with internal commas.

## Single Core Runner Pattern

Following PaperPilot's universal architecture pattern, the operation logic is defined once inside `paperpilot-pdf/src/operations/table.rs`, wrapping `lopdf` primitives. It exposes the signature:

```rust
pub fn extract_tables_bytes(
    input_bytes: &[u8],
    options: &TableExtractOptions,
) -> Result<TableExtractResult, PdfError>
```

This ensures guaranteed parity across CLI (`paperpilot extract-tables`), MCP Server JSON-RPC invocations, API Gateway POST actions, Cloudflare Edge Workers, and Desktop WASM embedded targets.
