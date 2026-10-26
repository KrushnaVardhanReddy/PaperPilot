# 117 Alternating PDF Merge & Duplex Collation

## Overview

The `pdf_merge_alternate` tool provides a way to interleave pages from two PDF documents. This is especially useful for manual duplex scanning scenarios where:
- Document A contains the front pages (e.g. 1, 3, 5, 7)
- Document B contains the back pages (e.g. 2, 4, 6, 8 or in reverse order if scanned upside down).

The tool collates these pages into a single properly sequenced document.

## Features

- **Alternating Interleave:** Merge two documents by alternating pages.
- **Variable Steps:** Configure the number of pages taken from each document per cycle using `step` (for Doc A) and `step_b` (for Doc B).
- **Reverse Mode:** Reverse the sequence of pages from either document (`reverse_a`, `reverse_b`), crucial for correcting backwards duplex scanner output.
- **Safe Object Renumbering:** Uses `lopdf` internal renumbering to safely migrate and interleave page objects without ID collisions.

## Configuration Options

```rust
pub struct PdfMergeAlternateOptions {
    pub step: usize,           // Default: 1
    pub step_b: usize,         // Default: 1
    pub reverse_a: bool,       // Default: false
    pub reverse_b: bool,       // Default: false
}
```

## Architecture

- **Core Engine:** Located at `paperpilot-pdf/src/operations/pdf_merge_alternate.rs`. Iteratively collates `ObjectId`s from both `lopdf::Document` parsed files.
- **CLI Wrapper:** Uses `--input-a` and `--input-b` to read both files and map the CLI arguments to the core engine.
- **MCP Server:** Exposes the `pdf_merge_alternate` tool accepting `input_a`, `input_b`, and optional parameters.
- **REST Gateway:** Endpoint `/api/v1/pdf/merge-alternate` consuming `PdfMergeAlternateJsonRequest`.
- **WASM / Edge Layer:** Accepts an array of buffers, delegating directly to the pure-Rust logic for browser execution.
