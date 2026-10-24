# PDF Stamp

The `pdf_stamp` operation allows users to place an image (PNG or JPEG) as a watermark or stamp on specified pages of a PDF document.

## Overview
This feature works by embedding the supplied raster image as a PDF `/XObject` (specifically an `/Image`) and adding drawing operations to the target pages' content streams (`/Contents`). Support for transparency/opacity is achieved by creating an `/ExtGState` dictionary.

## Input Options
- `image_path` (string, optional): The path to the image file.
- `image_data` (string, optional): Base64 encoded image data if `image_path` is not provided.
- `x` (float): X coordinate in points (bottom-left origin). Default is 50.0.
- `y` (float): Y coordinate in points (bottom-left origin). Default is 50.0.
- `width` (float): Rendered width in points. Default is 150.0.
- `height` (float): Rendered height in points. Default is 50.0.
- `opacity` (float): Value between 0.0 (transparent) and 1.0 (opaque). Default is 1.0.
- `pages` (string): Target pages to stamp. Can be "all", "last", or a comma-separated list like "1,3,5". Default is "all".

## Architecture
This operation executes purely in-memory via `pdf_stamp_bytes` function in `paperpilot-pdf`. It avoids writing intermediate files.

- Wrappers for this functionality are exposed in:
  - CLI: `pdf-stamp`
  - MCP JSON-RPC: `pdf_stamp`
  - WASM: `pdf_stamp`
  - REST Gateway: `/api/v1/pdf/stamp`
