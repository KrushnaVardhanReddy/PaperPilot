# Spec 070: PDF-to-DOCX Conversion via `unpdf` Recursive XY-Cut Layout Analysis

## Status: PROPOSED (3rd Backup Track for High-Fidelity Flow Layout)
## Author: Krushna Vardhan Reddy / Antigravity AI
## Date: 2026-10-18
## Target Crates: `paperpilot-pdf`

---

## 1. Problem Statement & Motivation
Complex brochure and multi-column marketing PDFs (such as `fixtures/Get_Started_With_Smallpdf.pdf`) mix 2-column card layouts, irregular column gutters, and vector illustrations. If token-level heuristics fail or collapse columns out of order, recursive XY-Cut layout segmentation reliably partitions page geometry into coherent visual blocks.

The pure-Rust crate **`unpdf`** (MIT license) specializes in:
1. **Recursive XY-Cut layout analysis** to separate multi-column flows.
2. **Table and heading extraction** with markdown/HTML rendering (`unpdf::to_markdown` / `unpdf::render::to_markdown`).
3. **Clean typographic grouping** preserving document hierarchy.

## 2. Architecture: `unpdf` Layout Extraction Pipeline
```
[ Input PDF ]
     │
     ▼
[ unpdf::Unpdf Builder ] ──► Recursive XY-Cut spatial segmentation
     │
     ▼
[ Structured Markdown / Document Model ]
     │
     ▼
[ pulldown-cmark AST Parser ]
     │
     ▼
[ OpenXML Document Builder (docx-rs) ]
  ├── 0.5 in Margins (720 dxa)
  ├── Heading 1 (Title): Centered 22pt bold (#1F242E)
  ├── Heading 3 (Subtitle): Centered 11pt (#757E8A)
  ├── Structured Cards: 2-column borderless OpenXML tables (<w:tbl>)
  └── Strictly 1-page fit in LibreOffice & Word
```

## 3. Deliverables & Acceptance Criteria
1. Add optional/direct `unpdf` support in `paperpilot-pdf/Cargo.toml` (`unpdf = { version = "0.46.0", default-features = false }`).
2. Integrate `unpdf` layout parser into `PdfToDocxOperation` in `paperpilot-pdf/src/operations/conversion.rs`.
3. Multi-column cards in `fixtures/Get_Started_With_Smallpdf.pdf` render cleanly as borderless tables without text squashing.
4. Pass `cargo check -p paperpilot-pdf -j 4` and `cargo test -p paperpilot-pdf --lib test_pdf_to_docx -j 4`.
