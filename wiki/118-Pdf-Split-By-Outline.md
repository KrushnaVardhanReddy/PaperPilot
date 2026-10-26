# Spec 118: Split PDF by Outline / Bookmarks (`pdf_split_by_outline`)

## Overview
Enables automatically splitting a multi-chapter PDF document (e.g., textbook, technical manual, legal brief) by its top-level bookmark hierarchy (`/Outlines`).

Rather than requiring users to manually lookup and enter page ranges, this tool reads the PDF `/Outlines` dictionary, identifies each bookmark's title and destination page, and extracts each section into a zip archive named after the bookmark title (e.g. `Chapter 1.pdf`).

## Architecture
The core engine (`paperpilot-pdf`) traverses the PDF `/Outlines` dictionary up to `max_depth` (default 1) and resolves `/Dest` / `/GoTo` nodes to their associated page numbers. It creates distinct PDF byte streams for each bounded section and returns them compressed together in a standard `Vec<u8>` Zip archive layout.

## Capabilities
- Extract named sections directly based on outline structure.
- Handles standard `/Dest` as well as explicit `/Action` with `/GoTo` resolving.
- Output sanitized filenames.
- Support `max_depth` limiting (only split top-level chapters).
