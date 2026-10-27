# 119: PDF Posterize Engine

## Overview
The `pdf_posterize` engine in PaperPilot slices large single-page or multi-page PDFs (such as architectural blueprints, engineering schematics, banners, high-resolution artwork, and maps) and tiles them across a grid of smaller standard sheets (e.g., A4, Letter).

This operation allows oversized documents to be printed on standard household or office printers and seamlessly assembled together physically.

## Features
- **Configurable Target Sheets**: Auto-scales and slices to target sizes like `a4`, `letter`, `legal`, `a3`, `a5`.
- **Configurable Overlap Margin (Glue Gutter)**: Adjustable overlap (in points) so adjacent printed sheets can be glued together accurately. (Default is ~5mm / 14.17 pt).
- **Auto Grid Calculation**: Optionally automatically calculate the required number of grid rows and columns based on the scale and source dimensions, or provide explicit grid counts.
- **Scale Factor**: Multiplier for resizing the content before posterizing (e.g. 2.0 for 200%).
- **Cut Marks**: Optional guide lines at the corners of tiles acting as trim guides.

## Internals
- Executed by transforming the page `/Contents` stream using a PDF matrix operator (`cm`): $x' = x \\cdot scale - c \\cdot (W_{tile} - overlap)$, $y' = y \\cdot scale - r \\cdot (H_{tile} - overlap)$.
- Fully isolated in `paperpilot-pdf` without any mocked boundaries or panics.

## Interfaces
- **CLI**: `paperpilot-cli pdf-posterize --input <file> --output <file> --target-sheet a4 --overlap 15`
- **MCP Server**: `pdf_posterize` Tool
- **REST Gateway**: `POST /api/v1/pdf/posterize`
