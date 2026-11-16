# Comic Archive Module (`pdf_comic_archive`)

The Comic Archive module handles the conversion of PDF documents into `.cbz` (Comic Book Zip) format.

## Overview
CBZ is simply a Zip archive containing a series of sequential image files. To perform this conversion, the system takes the input PDF document, traverses each page, and utilizes the rendering pipeline (`hayro` and `tiny_skia`) to rasterize each page into a PNG.

These PNGs are then systematically named (e.g. `page_001.png`, `page_002.png`) and zipped into a `zip::ZipWriter` buffer.

## Edge Cases

- **File Flushing:** Care must be taken to flush the zip buffer `zip.finish()` to ensure valid PK zip headers are produced on completion.
- **Empty Documents:** Handled gracefully by returning an explicit `PdfError::ParseError("Document has no pages")` before allocating memory or processing frames.
