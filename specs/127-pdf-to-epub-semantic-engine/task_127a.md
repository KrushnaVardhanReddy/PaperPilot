# Task 127A: PDF-to-EPUB 3.0 Semantic Reflow Engine Implementation

## Status
- Branch: `feat/pdf-to-epub-spec-127`
- Spec: `specs/127-pdf-to-epub-semantic-engine/spec.md`
- Priority: P1 / High Demand

---

## Action Checklist

### 1. Core Engine (`paperpilot-pdf`)
- [ ] Create `paperpilot-pdf/src/operations/pdf_to_epub.rs`:
  - Implement `PdfToEpubConfig` struct (title, author, cover_image, language).
  - Implement text extraction & paragraph unwrapping.
  - Implement chapter segmentation via outline hierarchy or font heading heuristics.
  - Implement EPUB 3.0 packaging with `zip` crate:
    - First entry `mimetype` with `CompressionMethod::Stored`.
    - `META-INF/container.xml`.
    - `OEBPS/content.opf` (metadata, manifest, spine).
    - `OEBPS/toc.xhtml` (EPUB 3 Nav) + `OEBPS/toc.ncx` (EPUB 2 compatibility).
    - `OEBPS/styles.css` (clean serif typography).
    - `OEBPS/chapter_*.xhtml` (valid XML/XHTML).
  - Expose `run_pdf_to_epub_bytes(input_bytes: &[u8], config: &PdfToEpubConfig) -> Result<Vec<u8>, OperationError>`.
- [ ] Export `pdf_to_epub` module in `paperpilot-pdf/src/operations/mod.rs`.
- [ ] Write unmocked unit tests in `pdf_to_epub.rs`:
  - Test valid EPUB ZIP magic bytes (`PK\x03\x04`).
  - Test `mimetype` uncompressed requirement.
  - Test `content.opf` and XML well-formedness.

### 2. Multi-Surface Integrations
- [ ] **CLI** (`paperpilot-cli`):
  - Add `PdfToEpub` subcommand with `-i`, `-o`, `--title`, `--author`.
- [ ] **MCP** (`paperpilot-mcp`):
  - Register `pdf_to_epub` tool with parameters.
- [ ] **REST Gateway** (`paperpilot-gateway`):
  - Add `POST /api/v1/pdf/to-epub` endpoint.
- [ ] **Desktop UI** (`apps/desktop`):
  - Add `pdf_to_epub` in `toolList.ts` under `category: 'convert'`.

### 3. Documentation & Verification
- [ ] Create `wiki/127-Pdf-To-Epub.md`.
- [ ] Create `apps/docs/src/content/docs/reference/tools/pdf_to_epub.md`.
- [ ] Run `python3 apps/docs/src/content/docs/reference/generate_handbook.py`.
- [ ] Verify `cargo test -p paperpilot-pdf -j 2` and `cargo check --workspace -j 2`.
