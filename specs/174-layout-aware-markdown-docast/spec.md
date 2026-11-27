# Spec 174: Layout-Aware Structured Markdown Extractor (DocAST)

## Metadata
- **Spec Number**: 174
- **Title**: Layout-Aware Structured Markdown Extractor (DocAST)
- **Phase**: Document Understanding & Semantic Ingestion
- **Status**: ✅ Completed
- **Impacted Systems**:
  - `paperpilot-pdf`: Document AST parser and layout reconstructor (`src/operations/docast.rs`, `src/operations/conversion.rs`)
  - `paperpilot-cli`: Markdown extraction flags and DocAST pipeline
  - `paperpilot-gateway`: Structured markdown conversion endpoints

---

## 1. Executive Summary & Problem Statement

Standard PDF-to-Markdown converters flatten all textual elements linearly in stream order. When reading complex multi-column reports, headers, footers, callout boxes, and tabular figures, linear stream reading scrambles sentences across column gutters, mingles repeated page running headers into prose, and fragments tables into incomprehensible lines.

**Spec 174** introduces **DocAST (Document Abstract Syntax Tree)**: a layout-aware pure-Rust document structure parser in `paperpilot-pdf/src/operations/docast.rs`.

DocAST extracts geometric bounding boxes and font attributes from PDF content streams, groups text fragments into words and lines, clusters lines into semantic blocks (Headings, Paragraphs, Lists, Tables, Code Blocks, and Footers), and outputs cleanly structured GitHub-Flavored Markdown.

---

## 2. Technical Architecture & Parsing Pipeline

```
[ PDF Document ]
       │
       ├──► 1. Glyph & Text Fragment Extraction (with Bounding Boxes & Font Sizes)
       │
       ├──► 2. Spatial Clustering & Line Grouping
       │      • Horizontal overlap & baseline threshold grouping
       │      • Dynamic word spacing estimation
       │
       ├──► 3. Column Detection & Reading Order Traversal
       │      • X-projection profile analysis to locate column gutters
       │      • Top-down, left-to-right reading order sorting per column
       │
       ├──► 4. Semantic Element Classification (DocAST Nodes)
       │      • Heading levels (H1-H6) derived from relative font height & weight
       │      • Tabular rows detected via grid alignment & vertical separators
       │      • Bullet & numbered lists detected via prefix pattern recognition
       │      • Running headers & footers filtered via vertical page boundaries
       │
       └──► 5. Markdown Synthesis
              • Clean GitHub-Flavored Markdown generation
              • Preservation of formatting, code fences, and pipe tables
```

### 2.1 AST Data Structures (`paperpilot-pdf/src/operations/docast.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocAstNode {
    Heading { level: u8, text: String },
    Paragraph { text: String },
    List { ordered: bool, items: Vec<String> },
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    CodeBlock { language: Option<String>, code: String },
    Blockquote { text: String },
    PageBreak { page_number: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocAstDocument {
    pub total_pages: u32,
    pub nodes: Vec<DocAstNode>,
    pub metadata: DocumentMetadata,
}
```

---

## 3. Verification & Acceptance Criteria
1. Multi-column PDF documents convert to Markdown following natural column reading order without gutter interleaving.
2. Tables with aligned columns convert to Markdown pipe tables (`| Col 1 | Col 2 |`).
3. Running headers and footers are cleanly isolated or suppressed to avoid cluttering LLM context windows.
4. Core unit and layout tests pass cleanly in `cargo test -p paperpilot-pdf --lib operations::docast -j 4`.
