# Spec 127: PDF-to-EPUB 3.0 Semantic Reflow Engine via Intermediate AST

## 1. Overview & Problem Statement
E-reader owners and Calibre users (prompted by user feedback on Hacker News) struggle with fragmented line wraps, broken hyphenations, missing chapter headings, and rigid column layouts when reading standard fixed-layout PDFs on e-ink devices (Kindle, Kobo, Boox) or phone readers.

This specification defines a pure-Rust, zero-cloud **PDF-to-EPUB 3.0** conversion engine (`pdf_to_epub`) that parses PDF document semantics, reconstructs continuous reflowable text with chapter boundaries, and packages the result into a fully standard EPUB 3.0 Open Container Format (OCF).

---

## 2. Technical Requirements

### 2.1 Core Conversion Pipeline (`paperpilot-pdf/src/operations/pdf_to_epub.rs`)
1. **Semantic Text & Paragraph Reflow**:
   - Extract raw text streams and lines using `lopdf`.
   - Reconstruct broken paragraphs by analyzing vertical line gaps, font size changes, and hyphenated line ends (`-\n`).
   - Eliminate running headers and footers (e.g. repeated page numbers, book titles).
2. **Chapter & Section Boundary Detection**:
   - Primary: Inspect PDF document outline bookmarks (`/Outlines` tree). If outline entries exist, split chapters at bookmarked targets.
   - Fallback: Detect major heading typographic heuristics (font size $\ge 1.4\times$ body text font size, bold weight, followed by distinct whitespace).
3. **EPUB 3.0 Open Container Packaging**:
   An EPUB file is a standard ZIP archive with strict conformance requirements:
   - **`mimetype`**: First file in archive, **stored uncompressed** (CompressionMethod::Stored / 0), content `application/epub+zip` with no extra padding or trailing newline.
   - **`META-INF/container.xml`**: XML descriptor pointing to the root package file:
     ```xml
     <?xml version="1.0" encoding="UTF-8"?>
     <container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
       <rootfiles>
         <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
       </rootfiles>
     </container>
     ```
   - **`OEBPS/content.opf`**: Package document containing metadata (title, author, UUID identifier, language), `<manifest>` listing all XHTML files/CSS, and `<spine>` defining reading order.
   - **`OEBPS/toc.xhtml` & `OEBPS/toc.ncx`**: EPUB 3 navigation document (`nav.xhtml`) and backward-compatible EPUB 2 NCX table of contents.
   - **`OEBPS/chapter_*.xhtml`**: Clean, semantic XHTML5 content pages styled with lightweight typography CSS (`OEBPS/styles.css`).

### 2.2 Interface Integrations (Single Core Runner + Thin Wrappers)
1. **Core Signature**:
   ```rust
   pub fn run_pdf_to_epub_bytes(input_bytes: &[u8], config: &PdfToEpubConfig) -> Result<Vec<u8>, OperationError>
   ```
2. **CLI**:
   `paperpilot pdf-to-epub input.pdf -o output.epub [--title "Book Title"] [--author "Author"]`
3. **MCP Tool**:
   `pdf_to_epub` with JSON schema inputs `input_path`, `output_path`, `title`, `author`.
4. **REST Gateway**:
   `POST /api/v1/pdf/to-epub` multipart endpoint returning `application/epub+zip`.
5. **Desktop UI**:
   Tool entry in `toolList.ts` under `category: 'convert'`, allowing drag-and-drop PDF to EPUB download.

---

## 3. Verification Protocol
1. **Zero-Mock Unit Tests (`#[cfg(test)]`)**:
   - Test ZIP magic header `PK\x03\x04` at offset 0.
   - Verify uncompressed `mimetype` is first entry with value `application/epub+zip`.
   - Verify `container.xml` and valid `content.opf` with spine items.
   - Verify multi-page and outline-based chapter splitting.
2. **Workspace Integrity**:
   - `cargo check --workspace -j 2`
   - `cargo test -p paperpilot-pdf --lib operations::pdf_to_epub -j 2`
   - `pnpm --dir apps/desktop test`
3. **Security Audit**:
   - Semgrep OSS and `cargo audit` clean.
