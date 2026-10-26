# Task 122A: Implement Lossless Stream Deduplication (`pdf_dedup_streams`) & CMYK Color Space Safety

## Status
Planned (Phase 5.9 / Presse Architecture Pattern).

## Description
Incorporate Presse-inspired optimization passes into PaperPilot's PDF engine:
1. Hash-based stream deduplication pass in `paperpilot-pdf/src/operations/compress.rs` (collapsing duplicate images, fonts, and ICC profiles across pages).
2. CMYK color-space guards preventing destructive RGB conversion on prepress print assets.
3. Unit tests verifying multi-page stream deduplication and CMYK preservation.

## Deliverables
1. **Engine Optimization**: `paperpilot-pdf/src/operations/compress.rs`
   - Implement `deduplicate_streams(&mut lopdf::Document)` function.
   - Inspect `/ColorSpace` for `/DeviceCMYK` to prevent distortion.
2. **CLI & Parameters**:
   - Add `--dedup` flag / option to `paperpilot compress` subcommand.
3. **Unit Tests**:
   - `test_compress_deduplicate_identical_streams`: verifies 5 identical image streams on 5 pages collapse to 1 indirect object.
   - `test_compress_cmyk_stream_preservation`: verifies CMYK stream integrity.
