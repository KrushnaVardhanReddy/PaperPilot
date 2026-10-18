# Spec 067: Production-Grade PDF Compression Optimization (Image Resampling & Stream Flate Compaction)

## Status: APPROVED / READY FOR IMPLEMENTATION

## 1. Problem Statement & Motivation
Currently in PaperPilot, the `pdf_compress` operation (available across CLI, MCP, REST Gateway, Desktop UI, and WASM) delegates down to `CompressOperation` in `paperpilot-pdf/src/operations/compress.rs`.

In real-world testing (e.g. `reports/TOP_12_CORE_TOOLS_AUDIT_REPORT.md` and live CLI audits):
1. On minimal or text-only documents (such as `tests/e2e_fixtures/real/large_doc.pdf`), the output file size (`2582` bytes) actually increases relative to the input (`2577` bytes).
2. For image-bearing PDFs, `CompressOperation` attempts to re-encode streams as JPEG, but:
   - When images are already compressed or formatted with non-standard filters, the re-encoded length might be slightly larger, causing the tool to skip re-encoding.
   - Text streams, uncompressed font streams, and page `/Contents` streams are not systematically checked or compressed with Flate.
   - There is no size guard: if the processed document ends up larger than or equal to the original document, it outputs the larger document rather than preserving the more compact original.

---

## 2. Technical Architecture & Implementation Details

### A. Core Engine (`paperpilot-pdf/src/operations/compress.rs`)
1. **Compress Quality Presets**:
   - `low` / `screen`: Max dimension 1024px, JPEG quality 40, Flate compression level 9. Target size reduction: 50%–80%.
   - `medium` / `ebook` (Default): Max dimension 1600px, JPEG quality 60, Flate compression level 6. Target size reduction: 30%–50%.
   - `high` / `prepress`: Max dimension 2400px, JPEG quality 80, Flate compression level 6. Target size reduction: 15%–30%.
   - Numeric quality (1–100 integer string) mapped to JPEG quality.

2. **Stream Optimization & Content Flate Encoding**:
   - Traverse all streams in `lopdf_doc.inner.objects`:
     - If `/Subtype /Image`:
       - Decompress content.
       - Downsample resolution if width or height exceeds `max_dim`.
       - Re-encode with `image::codecs::jpeg::JpegEncoder::new_with_quality`.
       - Replace stream content only if the new JPEG is strictly smaller than the existing stream.
     - For non-image streams (page `/Contents`, font streams, metadata):
       - If uncompressed (no `/Filter` or Filter is not `/FlateDecode`), compress with `flate2::write::ZlibEncoder`.
       - Set `/Filter` to `Object::Name(b"FlateDecode".to_vec())`.
       - Update `/Length`.

3. **Orphan & Unused Object Pruning**:
   - Call `lopdf_doc.inner.prune_objects()` to sweep unreferenced objects, orphan streams, or dangling annotations prior to serialization.
   - Run `lopdf_doc.inner.compress()`.

4. **Size Non-Expansion Guard**:
   - If the processed document after serialization is larger than or equal to the original document, ensure the tool does not bloat minimal PDFs.

---

## 3. Sample PDF Generation & Test Fixtures
Create a dedicated reproducible test fixture `tests/e2e_fixtures/real/scanned_photo_doc.pdf` containing:
- At least 1 high-resolution uncompressed RGB image (e.g. 1600x1200).
- An uncompressed text content stream.
- A verifiable compression test in `paperpilot-pdf/src/operations/compress.rs` demonstrating:
  - `CompressOperation::new(Some("low".to_string()))` reduces file size by at least **30%**.
  - Text-only documents do not suffer uncontrolled size inflation.

---

## 4. Acceptance Criteria
1. `cargo test -p paperpilot-pdf compress -j 4` passes cleanly.
2. `paperpilot compress --input tests/e2e_fixtures/real/scanned_photo_doc.pdf --quality low --output /tmp/compressed.pdf` demonstrates genuine file size reduction.
3. No breaking changes to existing CLI, MCP, or Gateway schemas.
4. All existing tests in `paperpilot-pdf` continue to pass.
