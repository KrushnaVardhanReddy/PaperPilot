# Spec 117: Alternating PDF Merge & Duplex Collation (`pdf_merge_alternate`)

## 1. Overview
Enables interleaving/collating pages from two PDF documents in alternating order. This is a critical operation for manual duplex scanning, where:
- Document A contains the front pages (1, 3, 5, 7...).
- Document B contains the back pages in reverse scan order (8, 6, 4, 2...) or forward order (2, 4, 6, 8...).

The operation merges these into a single correctly sequenced document (`1, 2, 3, 4, 5, 6, 7, 8`).

---

## 2. Technical Architecture

### 2.1 Configuration Options
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdfMergeAlternateOptions {
    pub step: usize,           // Number of pages to take from Doc A per turn (default: 1)
    pub step_b: usize,         // Number of pages to take from Doc B per turn (default: 1)
    pub reverse_b: bool,       // Whether to reverse the order of pages in Doc B (default: false)
    pub reverse_a: bool,       // Whether to reverse the order of pages in Doc A (default: false)
}
```

### 2.2 Algorithm & Semantics
1. Parse `doc_a` and `doc_b` into `lopdf::Document`.
2. Extract page IDs from both documents:
   - Apply `reverse_a` to Doc A page IDs if requested.
   - Apply `reverse_b` to Doc B page IDs if requested.
3. Collate pages into a new target document:
   - Take `step` pages from Doc A, then `step_b` pages from Doc B, continuing until all pages from both documents are consumed.
   - Re-index object numbers to avoid ID collision.
4. Serialize and return the resulting PDF bytes.

---

## 3. Interfaces
- **Core Engine**: `paperpilot-pdf/src/operations/pdf_merge_alternate.rs`
- **CLI**: `paperpilot merge-alternate --input-a doc1.pdf --input-b doc2.pdf --output out.pdf [--reverse-b] [--step 1]`
- **MCP Server**: `pdf_merge_alternate` tool
- **REST Gateway**: `POST /api/v1/pdf/merge-alternate`
- **Docs**: `wiki/117-Pdf-Merge-Alternate.md`, `apps/docs/src/content/docs/reference/tools/pdf_merge_alternate.md`
