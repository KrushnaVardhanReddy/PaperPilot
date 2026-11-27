# Engine Architecture: Zero-Copy Memory Mapping & Hexagonal Storage

## 1. Overview
PaperPilot is designed to operate under strict resource constraints across heterogeneous deployment models: from 512MB RAM cloud edge workers and local CLI processes to enterprise document processing nodes.

To handle multi-gigabyte document archives without risking out-of-memory (OOM) termination and to enable clean pluggability of storage backends, PaperPilot implements:
1. **Zero-Copy Memory-Mapped Streaming (`memmap2`)**
2. **Hexagonal Storage Port Architecture (`DatabasePort`)**
3. **In-Memory Parser Self-Healing**

---

## 2. Zero-Copy Memory-Mapped Document Handling
- **Spec Reference**: [Spec 163: Zero-Copy Memory-Mapped Large Document Handling](file:///home/krushna/Project/PaperPilot/specs/163-zero-copy-mmap-large-doc-handling/spec.md)
- **Key Implementation**: `paperpilot-core/src/io/document_source.rs` and `paperpilot-pdf/src/document.rs`

### The Problem
Previously, operations read entire PDF files into heap memory (`std::fs::read`). Processing concurrent 300MB–1.5GB legal or medical document archives resulted in 2–4 GB heap spikes, causing memory exhaustion.

### The Solution: `DocumentSource`
```rust
pub enum DocumentSource {
    Bytes(Vec<u8>),
    #[cfg(not(target_arch = "wasm32"))]
    Mmap(memmap2::Mmap),
}
```
- **Threshold**: Documents $\ge$ 10MB are mapped directly from disk into virtual memory via the OS page cache. Smaller documents remain in standard heap buffers.
- **Constant Memory Footprint**: Read-only operations (`pdf_search`, `pdf_extract_patterns`, `document_pii_scan`, `pdf_chunk`, `pdf_extract_tables`) achieve sub-100MB resident set size (RSS) regardless of document file size.
- **WASM Neutrality**: Strictly gated with `#[cfg(not(target_arch = "wasm32"))]` to allow clean compilation in web browsers and edge workers without syscall errors.

---

## 3. Hexagonal Storage Port Architecture
- **Spec Reference**: [Spec 150: Hexagonal Storage Port Architecture](file:///home/krushna/Project/PaperPilot/specs/150-hexagonal-storage-port-architecture/spec.md)
- **Crates**: `paperpilot-core` (Port definition) and `paperpilot-storage` (Default adapter)

```
                    ┌────────────────────────────────────────┐
                    │            paperpilot-core             │
                    │ ────────────────────────────────────── │
                    │      trait DatabasePort (Async)        │
                    │  JobRecord, AuditEvent, VectorChunk    │
                    └───────────────────▲────────────────────┘
                                        │
                 ┌──────────────────────┴──────────────────────┐
                 │                                             │
    ┌────────────┴─────────────┐                 ┌─────────────┴────────────┐
    │    paperpilot-storage    │                 │   PaperPilot-Enterprise  │
    │ ──────────────────────── │                 │ ──────────────────────── │
    │  impl DatabasePort for   │                 │  impl DatabasePort for   │
    │      StorageEngine       │                 │  PostgresStorageEngine   │
    │   (Embedded libSQL)      │                 │  (Postgres / Supabase)   │
    └──────────────────────────┘                 └──────────────────────────┘
```

### The `DatabasePort` Contract
Decouples domain workflows (background daemons, asynchronous batch jobs, audit logging, and vector chunk persistence) from specific storage engines:
- **Default Community Engine**: Embedded Turso / libSQL (`StorageEngine`), supporting offline-first desktop apps and local CLI usage with zero external database dependencies.
- **Enterprise Adapter**: Allows drop-in replacement with PostgreSQL, Supabase, or distributed SQL without modifying upstream business logic.

---

## 4. In-Memory Parser Self-Healing
- **Spec Reference**: [Spec 175: In-Memory Parser Self-Healing for Corrupted and Truncated PDFs](file:///home/krushna/Project/PaperPilot/specs/175-in-memory-parser-self-healing/spec.md)
- **Key Implementation**: `paperpilot-pdf/src/operations/repair.rs` and `paperpilot-pdf/tests/self_healing_test.rs`

Corrupted files (truncated EOF markers, invalid xref tables, bad header offsets) automatically trigger a transparent recovery sequence:
1. Strip leading junk bytes before `%PDF-`.
2. Scan backwards to locate intact objects and synthesize valid trailers.
3. Reconstruct cross-reference tables in memory.
4. Transparently resume operation without disk side-effects.
