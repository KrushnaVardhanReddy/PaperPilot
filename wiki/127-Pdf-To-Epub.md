# PDF to EPUB 3.0 Semantic Reflow Engine

## Overview
This engine converts PDF files into reflowable EPUB 3.0 documents. It aims to bridge the gap between static PDF layouts and accessible, responsive e-book formats.

## Architecture

The engine uses `pdf_oxide` to parse the underlying PDF file and extract Markdown (via `to_markdown()`), extracting textual components and identifying layout items.

### EPUB 3.0 Packaging
The engine builds an EPUB 3.0 OCF archive purely in-memory:
1. `mimetype`: Uncompressed, containing `application/epub+zip`.
2. `META-INF/container.xml`: Points to `OEBPS/content.opf`.
3. `OEBPS/content.opf`: Holds standard metadata (title, creator, language), a `manifest` for all assets, and a `spine` detailing the reading sequence.
4. `OEBPS/toc.xhtml`: Provides a standard EPUB 3 navigation document structure.
5. `OEBPS/toc.ncx`: Generates an NCX manifest for backwards compatibility with EPUB 2 e-readers.
6. `OEBPS/styles.css`: Embedded styles to format the reflowed HTML cleanly.
7. `OEBPS/chapter_*.xhtml`: The raw converted Markdown files turned into validated XHTML5 elements with soft hyphens removed and paragraphs cleanly mapped.

## Edge Cases & Limitations
- If a document is encrypted and not authenticated, the engine raises an error indicating the user needs to unlock it first.
- Corrupted documents are rejected.
