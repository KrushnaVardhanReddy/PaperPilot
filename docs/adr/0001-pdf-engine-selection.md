# 1. PDF Engine Selection

Date: 2024-05-15

## Status

Accepted

## Context

PaperPilot is a document automation platform. The PDF engine is the most critical technical decision. We need to choose the appropriate Rust PDF library or combination of libraries to handle PDF parsing, manipulation, rendering, and text extraction.

We have evaluated the following libraries:

1.  **`lopdf`**
    *   **Strengths:** Excellent for structural operations (merge, split, page manipulation). Good low-level access to PDF objects. Written in pure Rust.
    *   **Weaknesses:** Not designed for rendering or complex layout calculations. Limited built-in text extraction capabilities.
2.  **`pdfium-render`**
    *   **Strengths:** Outstanding rendering and text extraction. Backed by Google's robust PDFium library (used in Chrome). Very accurate for visual representation.
    *   **Weaknesses:** Requires a C++ binary dependency (PDFium). Heavier footprint compared to pure Rust solutions. Not ideal as a primary tool for low-level structural manipulation without re-rendering or converting.
3.  **`printpdf`**
    *   **Strengths:** Good for creating new PDFs from scratch.
    *   **Weaknesses:** Designed primarily for PDF generation, not parsing or modifying existing complex PDFs. Not suitable for our general-purpose needs.
4.  **`qpdf`** (via bindings)
    *   **Strengths:** Powerful C++ library for structural transformations.
    *   **Weaknesses:** Requires C++ dependencies. The Rust bindings might be less idiomatic or less actively maintained than native Rust solutions like `lopdf` or officially supported bindings like `pdfium-render`.

## Decision

We will use a hybrid approach, combining the strengths of two libraries:

*   **`pdfium-render`** will be used for rendering PDFs (e.g., generating thumbnails or page views) and for extracting text. Its accuracy in these areas is unmatched due to the underlying PDFium engine.
*   **`lopdf`** will be used for structural operations, such as merging multiple PDFs, splitting PDFs, and manipulating pages. Its pure Rust nature and focus on the PDF object model make it ideal for these tasks.

**Crucially, both libraries must be abstracted behind PaperPilot traits.** This architectural decision ensures that our core application logic is not tightly coupled to these specific libraries. If a better pure Rust alternative emerges in the future, or if we need to switch engines for different platforms, the traits will allow us to swap the underlying implementations with minimal friction.

## Consequences

*   **Positive:** We get the best of both worlds: robust structural manipulation (lopdf) and highly accurate rendering/extraction (pdfium-render).
*   **Positive:** Abstracting behind traits future-proofs the codebase and enforces clean architectural boundaries.
*   **Negative:** We introduce a C/C++ dependency (PDFium) into our build process, which may complicate cross-compilation or distribution slightly compared to a pure Rust stack.
*   **Negative:** Managing two separate libraries for PDF operations increases the complexity of the PDF engine implementation layer.
