# PDF Oxide Layout Analysis

This document describes the internal architecture of the advanced `pdf_to_docx` multi-column and table extraction algorithm introduced in Spec 039.

## Overview

The layout analysis pipeline processes PDF streams and translates spatial coordinate positioning into logical reading flows, overcoming the inherent "soup of spans" problem in raw PDF.

### Phase 1: Spatial Span Extraction
We define a `TextSpan` structure to represent each decoded text fragment, tracking `x` and `y` coordinates using the current transformation matrix and displacement operators (`Tm`, `Td`, `TD`, `T*`, etc.).

### Phase 2: Table Recognition
We map spans onto horizontal scanlines based on proximity along the `y` axis (±2.0 pt tolerance). A cluster of rows forming a contiguous block where multiple spans align along common `x` intervals is identified as a table grid. These are converted to `LayoutItem::Table` objects.

### Phase 3: XY-Cut Column De-weaving
For the non-table paragraph blocks, we build a 1D projection profile along the `x` axis to detect vertical gutters (whitespace $> 15\,\text{pt}$). When a continuous gutter slices the text spans vertically, we separate the spans into logical columns, allowing us to read "top-to-bottom" down Column 1 *before* crossing over to Column 2.

### Output
We emit structured OpenXML tables (`<w:tbl>`) and sequenced paragraphs directly via `docx-rs`.

### Phase 4: High-Fidelity Responsive Flow Layout & Inline Image Reconstruction

As of Spec 065, the layout engine introduces advanced capabilities for high-fidelity responsive DOCX flow.

1.  **Font Name Sanitization & Typography:** Font names are extracted and sanitized by stripping 6-letter subset prefixes, style suffixes (e.g., `-Bold`, `-Italic`), and inserting spaces in CamelCase names. Unrecognized fonts fall back gracefully to `Arial`. Text spans properly render OpenXML attributes for size, weight, and color.
2.  **Inline Image Extraction:** Images (such as those embedded in nested `Form` XObjects) are spatially located via recursive `Do` and `cm` transformation traversal using `lopdf`. Extracted pixel buffers are encoded directly to PNG and embedded as inline OpenXML pictures (`docx_rs::Pic`).
3.  **2-Column Responsive Pairing:** Disparate elements with overlapping Y-coordinates and intersecting horizontal bounding boxes (such as a feature icon matching a side-by-side title/description) are programmatically clustered into a borderless 2-column OpenXML table. This ensures the output DOCX is robust and reflowable across various word processors like Microsoft Word and LibreOffice without floating anchor breaks.
4.  **Single-Page Geometry Limits:** To prevent layout spillover into multi-page fragments, the document constraints use tighter paragraph spacing (e.g. `after: 60 dxa`) and explicit 720 dxa (0.5 inch) page margins.
