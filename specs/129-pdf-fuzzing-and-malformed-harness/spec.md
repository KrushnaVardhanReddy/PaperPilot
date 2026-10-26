# Spec 129: Automated Security Regression & Fuzzing Harness for Malformed PDFs

## 1. Overview & Problem Statement
PDF parsers are prime targets for malicious payloads, circular xref loops, deeply nested object trees, and decompression bombs. We need an automated regression test suite and fuzzing corpus to ensure PaperPilot rejects malformed inputs without panics, memory leaks, or crashes.

## 2. Requirements & Objectives
1. **Malformed PDF Test Corpus**:
   - Create `paperpilot-pdf/tests/malformed/` containing synthetic malformed PDFs:
     - Truncated header/trailer
     - Circular `/Parent` page tree references
     - Malformed cross-reference table offsets
     - Huge stream length attributes vs actual bytes
     - Deeply nested array objects
2. **Panic-Free Guarantee**:
   - Assert that every operation (`pdf_merge`, `pdf_split`, `pdf_compress`, `pdf_extract_text`, `pdf_posterize`) returns `Err(PdfError::ParseError)` or `Err(PdfError::InvalidInput)` rather than panicking with `unwrap()` or overflowing memory.
3. **CI Regression Gate**:
   - Add a test target `cargo test -p paperpilot-pdf --test malformed_regression` running in CI.
