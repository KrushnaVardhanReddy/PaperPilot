# `pdf_oxide` Engine Migration

## Overview
PaperPilot has migrated key text extraction, markup generation, and search indexing operations to `pdf_oxide` (v0.3.78) for high-performance and exact spatial reading order extraction.

## New Architecture

### `original_bytes` Retention
Because `pdf_oxide::api::Pdf::from_bytes` requires the unmodified, original bitstream of the document to parse cross-reference tables effectively, `LopdfDocument` now natively captures the buffer in `pub original_bytes: Option<Vec<u8>>` at `load()` time.

### Extraction Upgrades
* **`PdfToMarkdownOperation`**: Uses `pdf.to_markdown` for semantic headings (`#`) and tables.
* **`PdfToHtmlOperation`**: Uses `pdf.to_html` to output `<article>` blocks, with injected CSS ensuring mobile responsiveness.
* **`ExtractTextOperation`**: Uses `pdf.extract_spans` directly, retaining spaces and paragraph boundaries far more reliably than line-by-line char appends.
* **`SearchOperation`**: Leverages `pdf.search` to return fully quantified bounding box arrays (`SearchResultBbox`), drastically improving the UX for viewer highlighting and RAG annotations.

### RAG Chunking (`PdfRagChunksOperation`)
We implemented a zero-Python semantic text chunker using the Markdown layout. By exporting to Markdown through `pdf_oxide` and splitting text clusters around heading markers (`#`), we produce structured `RagChunk` outputs (page number, section heading, content) designed for vector ingestion.

## Limitations
Currently `pdf_oxide` requires `#![no_std]` compromises, meaning memory mapping isn't directly compatible across all surfaces. Cloudflare Workers will load the `original_bytes` directly from memory instead.
