# Spec 163: Zero-Copy Memory-Mapped Large Document Handling (`paperpilot-core`)

## 1. Executive Summary & Problem Statement
When running PaperPilot as a continuous CLI, local REST Gateway (`localhost:7823`), or Desktop background process, users frequently process massive PDFs:
- Multi-thousand page legal discoveries and scanned court archives (300 MB - 2 GB).
- High-resolution architectural drawings and technical blueprints.
- Large quarterly financial disclosures with dense tables.

Under the current architecture, operations load the entire file into a heap-allocated `Vec<u8>` via `std::fs::read` before parsing. This causes sudden RAM spikes (1-3 GB under concurrent REST gateway requests) and risks Out-Of-Memory (OOM) termination on resource-constrained agent environments or laptops.

Spec 163 introduces **Zero-Copy Memory-Mapped Document Handling (`memmap2`)** across all read-heavy analytical operations (`pdf_search`, `pdf_extract_patterns`, `document_pii_scan`, `pdf_chunk`, and `pdf_to_markdown`). It incorporates safe fallbacks and strict feature gating to preserve 100% compatibility with WASM and browser targets.

---

## 2. Technical Architecture & Memory Abstraction

### 2.1 The `DocumentSource` Abstraction
Location: `crates/paperpilot-core/src/io/document_source.rs`

Instead of accepting raw `Vec<u8>` or bare file paths, operations accept a zero-copy `DocumentSource`:

```rust
use std::path::Path;
use std::ops::Deref;

pub enum DocumentSource {
    /// In-memory byte buffer (used for WASM, small files, or stream inputs)
    Bytes(Vec<u8>),
    /// Memory-mapped file slice (used on native OS for files > threshold)
    #[cfg(not(target_arch = "wasm32"))]
    Mmap(memmap2::Mmap),
}

impl Deref for DocumentSource {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            DocumentSource::Bytes(b) => b.as_slice(),
            #[cfg(not(target_arch = "wasm32"))]
            DocumentSource::Mmap(m) => m.deref(),
        }
    }
}

impl DocumentSource {
    /// Opens a document using memory-mapping if above threshold (e.g. 10 MB) on native platforms.
    /// Falls back to standard heap read for small files or in WASM.
    pub fn open<P: AsRef<Path>>(path: P, mmap_threshold_bytes: u64) -> Result<Self, std::io::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let file = std::fs::File::open(&path)?;
            let meta = file.metadata()?;
            if meta.len() >= mmap_threshold_bytes {
                let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
                return Ok(DocumentSource::Mmap(mmap));
            }
        }
        let bytes = std::fs::read(path)?;
        Ok(DocumentSource::Bytes(bytes))
    }
}
```

### 2.2 Safety & Platform Guardrails
1. **WASM Neutrality**: `memmap2` is gated with `#[cfg(not(target_arch = "wasm32"))]`. `paperpilot-wasm` unconditionally uses `DocumentSource::Bytes`, compiling cleanly without syscall errors.
2. **Read-Only Targeting**: `DocumentSource` is strictly leveraged for **non-mutating** operations:
   - `pdf_search` / `pdf_extract_patterns`
   - `document_pii_scan`
   - `pdf_chunk`
   - `pdf_extract_tables`
   - `pdf_validate`
3. **Document Mutation Exemption**: Tools modifying document structures (`pdf_rotate`, `pdf_delete_pages`, `pdf_merge`, `pdf_sign`) continue to load mutable AST graphs (`lopdf::Document`) to allow structural re-indexing and trailer rebuilding.
4. **Short-Lived File Locks**: In the REST Gateway and Desktop runner, memory maps are scoped to the individual request/job thread and immediately dropped upon task completion, preventing file lock collisions on Windows.

---

## 3. Thresholds & Performance Targets
- **Threshold**: Files $\ge$ **10 MB** automatically leverage `mmap`. Files $< 10$ MB use standard buffer reads to avoid OS page mapping overhead.
- **Memory Ceiling**: Scanning a 1.2 GB PDF for PII or keywords must retain a resident memory footprint below **100 MB** (vs. ~1.8 GB without mmap).
- **Zero-Trace Parity**: Memory-mapped pages reside in kernel page cache and are discarded on unmap, leaving zero scratch traces on disk.
