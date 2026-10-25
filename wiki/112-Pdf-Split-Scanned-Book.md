# Scanned Dual-Page Book Center Spine Split (`pdf_split_scanned_book`)

This module implements a pure-Rust mechanism for splitting 2-up scanned book spreads (two pages on a single sheet) into individual portrait pages, by splitting the document horizontally or vertically at the center binding gutter.

## Features
- **Orientation**: Split either left/right (vertical cut) or top/bottom (horizontal cut).
- **Reading Direction**: Dictates the sequence of the output pages. LTR for left-to-right (Left then Right page) or RTL for right-to-left languages (Right then Left page).
- **Gutter Overlap**: To avoid losing text in the spine curve distortion, allows adding overlap points directly to the center crop bounds.

## Engine
Uses `lopdf` pure-Rust engine in `paperpilot-pdf`. Splits without rasterizing vectors or degrading text by dividing bounding boxes (`/MediaBox`, `/CropBox`) and mapping one input page twice in the output tree with appropriate cropping.

## Wrappers
Fully integrated into PaperPilot core:
- **CLI**: `pdf-split-scanned-book --input <in> --output <out> --reading-direction rtl --gutter-overlap 5.0`
- **MCP**: Tool `pdf_split_scanned_book` via JSON-RPC.
- **REST**: Available via `/api/v1/pdf/split-scanned-book` in PaperPilot Gateway.
