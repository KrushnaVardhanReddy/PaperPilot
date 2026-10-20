# Spec 069: Universal HTML/Markdown AST Bridge for PDF-to-DOCX Conversion

## Status: PROPOSED (Alternative 2 for High-Fidelity Flow Layout)
## Author: Krushna Vardhan Reddy / Antigravity AI
## Date: 2026-10-18
## Target Crates: `paperpilot-pdf`

---

## 1. Problem Statement
Direct low-level token extraction (`lopdf::extract_text`) strips all hierarchical document structure (headings, lists, paragraph spacing, hyperlinks, and tables), collapsing complex multi-column layouts into flat text walls. Furthermore, vector illustrations (such as PDF `/Form` XObjects) contain 0 raster image streams (`/Subtype /Image`), leaving image extractors empty.

However, PaperPilot already contains a high-quality semantic parser: `pdf_oxide` / `to_markdown` / `to_html`.
When `fixtures/Get_Started_With_Smallpdf.pdf` is parsed via the semantic engine, it correctly recovers:
- `<h1>Welcome to Smallpdf</h1>`
- `<h3>Ready to take document management to the next level?</h3>`
- Four `<h4>` section headings and `<p>` body blocks with embedded `<a>` hyperlinks.

## 2. Architecture: The AST Bridge Pipeline
Instead of writing ad-hoc PDF token parsers, `PdfToDocxOperation` converts via an intermediate Markdown / HTML AST:

```
[ Input PDF ]
     │
     ▼
[ pdf_oxide semantic engine ] ──► Extracts Headings, Paragraphs, Links, Lists
     │
     ▼
[ Markdown / HTML AST (pulldown-cmark / scraper) ]
     │
     ▼
[ OpenXML Document Builder (docx-rs) ]
  ├── Document Margins: 720 dxa (0.5 in)
  ├── <h1>: Centered, Bold 44 dxa (22pt), Color #1F242E
  ├── <h3>: Centered, 22 dxa (11pt), Color #757E8A
  ├── <h4> + <p> Blocks:
  │     Converted into responsive 2-Column Borderless Tables (<w:tbl>):
  │     - Left Cell: Graphic/Icon container (2600 dxa)
  │     - Right Cell: Title + Description (5000 dxa)
  │     - Alternating left/right arrangement
  └── Embedded Hyperlinks: Clickable Word links (<w:hyperlink>)
```

## 3. Deliverables & Acceptance Criteria
1. **Zero Hardcoded Text**: Works universally on any PDF document using AST node types (`Heading(1)`, `Heading(3)`, `Heading(4)`, `Paragraph`, `Table`, `Link`).
2. **Table Reconstruction**: Multi-column text or paired heading/paragraph blocks are rendered using clean OpenXML `docx_rs::Table` elements with `TableBorders::new().clear_all()`.
3. **Single-Page Flow**: Compact margins (`720 dxa`) and proportional line spacing ensuring 1-page PDF documents remain strictly 1 page in LibreOffice / Microsoft Word.
4. **Passing Verification**: Passes `cargo test -p paperpilot-pdf --lib test_pdf_to_docx -j 4`.
