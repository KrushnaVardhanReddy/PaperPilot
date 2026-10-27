# Spec 129: Automated Security Regression & Fuzzing Harness for Malformed PDFs

## 1. Overview & Problem Statement
PDF parsers and document processors are prime targets for adversarial payloads: circular xref tables, infinite `/Parent` tree recursion, decompression bombs, corrupted byte streams, and giant declared `/Length` attributes. 

As part of PaperPilot's zero-cloud, enterprise-ready security foundation, the engine must offer a strict **Zero-Panic, Zero-Hang Guarantee**. No input—no matter how damaged, crafted, or adversarial—may cause an `unwrap()` panic, stack overflow, or memory exhaustion. All errors must be gracefully returned as `Err(PdfError::ParseError)` or `Err(PdfError::InvalidInput)`.

---

## 2. Architecture & Attack Vectors

```
                                  Adversarial / Malformed Payloads
                                                 │
            ┌────────────────────────────────────┼────────────────────────────────────┐
            ▼                                    ▼                                    ▼
    [Corrupt Headers]                  [Circular Page Trees]                 [Stream Bomb / Length Mismatch]
  Truncated bytes, EOF               `/Parent` pointing to self,          `/Length 1073741824` with 5 bytes,
  missing, junk offsets               infinite recursive loops             corrupt FlateDecode streams
            │                                    │                                    │
            └────────────────────────────────────┼────────────────────────────────────┘
                                                 ▼
                                   `LopdfDocument::from_bytes`
                                 & Core PDF Transformation Engine
                                                 │
                                                 ▼
                          Assert: NO Panics, NO Infinite Loops, NO OOM
                                Returns clean `Result::Err`
```

### Attack Vectors Covered:
1. **Truncated / Garbage Headers**:
   - Zero-length byte buffers (`b""`).
   - Incomplete headers (`b"%PDF"`).
   - Missing `startxref` or `%%EOF` offsets.
   - High-byte ASCII random binary noise.
2. **Circular Reference Loops**:
   - `/Pages` objects referencing themselves as `/Parent` or `/Kids`.
   - Circular indirect object references (`1 0 R -> 2 0 R -> 1 0 R`).
3. **Decompression & Stream Length Bombs**:
   - Streams with declared `/Length` exceeding real buffer size by orders of magnitude (e.g. declaring 1 GB but providing 5 bytes).
   - Incomplete or malformed `FlateDecode` streams.
4. **Deeply Nested Structures**:
   - Deeply nested dictionaries/arrays (100+ levels) to test against stack overflow.

---

## 3. Requirements & Objectives
1. **Panic-Free Operation Execution**:
   - Feed each malformed payload to multiple core operations (`ExtractTextOperation`, `SplitOperation`, `PageCountOperation`, `DeletePagesOperation`, `PosterizeOperation`).
   - Verify that operations either succeed gracefully (if the parser can recover safely) or return an explicit `Err` without panicking.
2. **Deterministic & Fast**:
   - Synthetic payloads must be generated dynamically in memory via byte constants (`&[u8]`).
   - The entire test suite must complete in `< 1.0 second` so it can run as a regular pre-commit and CI gate.
3. **CI Integration**:
   - Register the test suite under `paperpilot-pdf/tests/malformed_regression.rs`.
   - Verified via `cargo test -p paperpilot-pdf --test malformed_regression -j 2`.
