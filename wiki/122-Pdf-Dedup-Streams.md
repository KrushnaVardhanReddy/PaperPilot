# Lossless Stream Deduplication (pdf_dedup_streams)

PDF documents frequently contain duplicate binary assets, such as repeated logos, headers/footers, embedded font subset programs, and identical ICC profiles attached to various objects. The PaperPilot `pdf_dedup_streams` tool utilizes techniques from high-performance parsers to safely and efficiently merge these redundant objects into a single canonical source of truth.

## Technical Approach

### 1. Cryptographic Hashing
Deduplication is driven by computing a SHA-256 hash of the decompressed stream contents alongside critical metadata keys (e.g., `Width`, `Height`, `ColorSpace`). The hash acts as an initial filter for detecting potential collisions.

### 2. Byte-for-Byte Verification
Whenever a hash collision is encountered, the engine performs a rigorous byte-by-byte equality check. This mechanism completely eliminates the possibility of false positives. If the streams strictly match, one is designated as canonical.

### 3. Cross-Document Reference Rewriting
After the canonical maps are built, the tool traverses the entire PDF Document Object Model (DOM)—including dictionaries, arrays, catalogs, and pages—to rewrite references pointing to the duplicate instances so that they point exclusively to the single canonical instance.

### 4. Pruning Orphans
Following reference rewriting, any indirect objects that become fully unreferenced (the leftover duplicates) are securely pruned from the document layout.

## CMYK-Safe Recompression Guard

In addition to deduplication, the underlying PDF compression workflows feature an integrated CMYK Color Space Guard (`paperpilot-pdf/src/operations/compress.rs`).

Many naive compression tools mistakenly map professional `/DeviceCMYK` or 4-channel `/ICCBased` CMYK streams to RGB JPEG images, resulting in fatal color shifting, visual artifacts, and complete loss of prepress calibration. PaperPilot actively inspects stream dictionaries for CMYK footprints and safely bypasses the standard RGB downsampling operations, opting only for safe Flate compression if required.

This strict separation maintains optimal visual integrity for prepress and commercial print.
