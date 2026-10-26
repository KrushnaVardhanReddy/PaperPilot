# Spec 127: PDF-to-EPUB Semantic Engine via Intermediate AST

## 1. Overview & Problem Statement
E-reader owners and Calibre users (prompted by user `Shorel` on Hacker News) struggle with fragmented line wraps, broken paragraphs, and missing chapter headings when converting complex PDFs to EPUB format.

## 2. Requirements & Objectives
1. **Core EPUB 3.0 Generation**:
   - In `paperpilot-pdf/src/operations/convert/epub.rs`, implement `PdfToEpubOperation`:
     - Extract structured AST using `pdf_to_markdown` / `pdf_to_html` heuristics.
     - Detect document outline hierarchy (`/Outlines`) or heading sizes for table-of-contents (`nav.xhtml` / `toc.ncx`).
     - Package content into valid EPUB 3.0 container with `mimetype`, `META-INF/container.xml`, and spine documents.
2. **CLI & MCP Tool**:
   - Add CLI command `paperpilot convert --format epub` and MCP tool `pdf_to_epub`.
3. **Zero-Mock Unit Tests**:
   - Add unit tests verifying valid EPUB 3.0 ZIP magic headers (`PK\x03\x04`), uncompressed `mimetype` first entry, and OPF manifest validation.
