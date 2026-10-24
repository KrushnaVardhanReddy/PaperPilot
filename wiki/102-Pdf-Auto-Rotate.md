# Pdf Auto Rotate Module

## Overview

The `pdf_auto_rotate` engine automatically corrects page orientations within PDF documents. It works by analyzing the `/MediaBox` dictionary along with any existing `/Rotate` attributes to calculate the effective physical dimensions of a page, then selectively applies an additional 90-degree rotation if the page's effective orientation doesn't match the desired target orientation.

## Design

### Data Structures

*   `PdfAutoRotateOptions`: Defines user configuration.
    *   `target_orientation`: String (`"portrait"`, `"landscape"`, `"auto"`).
    *   `pages`: Optional 1-based indices to constrain which pages should be analyzed.

### Core Engine (`paperpilot-pdf/src/operations/pdf_auto_rotate.rs`)

1.  **Parse**: The document is parsed via `lopdf::Document::load_mem`.
2.  **Filter**: Iterates through all pages in the PDF page tree, filtering down to targeted pages if a `pages` slice is provided.
3.  **Dimension Extraction**:
    * Extracts `/MediaBox` array `[llx, lly, urx, ury]`.
    * Extracts existing `/Rotate` attribute.
    * Normalizes current rotation `(r % 360 + 360) % 360`.
4.  **Effective Dimension Calculation**:
    * If `normalized_rotation == 90` or `270`, effective width and height swap relative to the physical bounding box dimensions.
5.  **Orientation Evaluation**:
    * If target is `"portrait"` and effective width > effective height, apply a +90 degree rotation.
    * If target is `"landscape"` and effective height > effective width, apply a +90 degree rotation.
6.  **Apply**: Set or update the `/Rotate` attribute on the page dictionary.
7.  **Save**: Writes bytes out to a buffer.

### Architectural Conformity

Like other tools in the PaperPilot core, this module strictly implements the "Single Core Runner + 5 Thin Wrappers" design. The rust-native code implemented here acts as the single source of truth for manipulation logic, and wrappers forward to this engine.
