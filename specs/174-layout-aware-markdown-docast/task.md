# Task 174: Implement Layout-Aware Markdown Extractor (DocAST)

## Overview
Implement Spec 174 by engineering the DocAST parser and layout reconstructor in `paperpilot-pdf`, bridging PDF spatial geometry with structured Markdown output.

---

## Subtasks & Checklist

- [x] **Subtask 1: DocAST Engine (`paperpilot-pdf/src/operations/docast.rs`)**
  - [x] Implement glyph bounding box extraction and spatial line grouping.
  - [x] Implement multi-column gutter detection and reading-order sort.
  - [x] Implement semantic node classification (Headings, Paragraphs, Tables, Lists).
  - [x] Implement Markdown serialization from AST nodes.

- [x] **Subtask 2: Conversion Operation Integration (`paperpilot-pdf/src/operations/conversion.rs`)**
  - [x] Connect `pdf_to_markdown` operation with DocAST layout engine.
  - [x] Support zero-copy mmap streaming inputs.

- [x] **Subtask 3: Unit and Integration Tests**
  - [x] Verify extraction across single-column, multi-column, and tabular test fixtures.
