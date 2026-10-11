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
