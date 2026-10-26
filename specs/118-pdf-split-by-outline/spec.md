# Spec 118: Split PDF by Outline / Bookmarks (`pdf_split_by_outline`)

## 1. Overview
Enables automatically splitting a multi-chapter PDF document (e.g., textbook, technical manual, legal brief) by its top-level bookmark hierarchy (`/Outlines`). 

Rather than requiring users to manually lookup and enter page ranges, this tool reads the PDF `/Outlines` dictionary, identifies each bookmark's title and destination page, and extracts each section into a standalone PDF or zip archive named after the bookmark title (e.g. `Chapter 1 - Introduction.pdf`).

---

## 2. Technical Architecture

### 2.1 Configuration Options
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdfSplitByOutlineOptions {
    pub max_depth: Option<usize>,      // Maximum outline depth to split at (default: 1, top-level chapters)
    pub sanitize_names: bool,          // Sanitize bookmark titles into safe filenames (default: true)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OutlineSectionResult {
    pub title: String,
    pub start_page: usize,
    pub end_page: usize,
    pub pdf_bytes: Vec<u8>,
}
```

### 2.2 Algorithm & Semantics
1. Parse the input PDF using `lopdf::Document::load_mem`.
2. Traverse the `/Outlines` dictionary from the document catalog:
   - Follow `/First` and `/Next` references up to `max_depth`.
   - Resolve destination (`/Dest` or `/A` with `/GoTo`) to a target page object and compute 1-indexed page numbers.
3. Compute the start and end page for each chapter:
   - Chapter $i$ spans from `chapter[i].start_page` to `chapter[i+1].start_page - 1` (or total page count for the final chapter).
4. For each section, slice the page range into a standalone PDF.
5. Return the list of section PDFs and their metadata, or a zip archive when requested.

---

## 3. Interfaces
- **Core Engine**: `paperpilot-pdf/src/operations/pdf_split_by_outline.rs`
- **CLI**: `paperpilot split-by-outline --input book.pdf --output-dir ./chapters [--max-depth 1]`
- **MCP Server**: `pdf_split_by_outline` tool
- **REST Gateway**: `POST /api/v1/pdf/split-by-outline`
- **Docs**: `wiki/118-Pdf-Split-By-Outline.md`, `apps/docs/src/content/docs/reference/tools/pdf_split_by_outline.md`
