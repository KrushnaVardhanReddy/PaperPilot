# Task 163A: Implement Zero-Copy `DocumentSource` Memory-Mapping in Core

## Checklist
- [ ] Add `memmap2 = { version = "0.9", optional = true }` to `crates/paperpilot-core/Cargo.toml` with default features enabled for non-WASM targets
- [ ] Implement `DocumentSource` in `crates/paperpilot-core/src/io/document_source.rs`
- [ ] Implement `Deref<Target = [u8]>` and `AsRef<[u8]>` for `DocumentSource`
- [ ] Add auto-threshold switching (default 10 MB threshold)
- [ ] Integrate `DocumentSource` into read-only operations:
  - [ ] `pdf_extract_patterns`
  - [ ] `document_pii_scan`
  - [ ] `pdf_chunk`
  - [ ] `pdf_search`
- [ ] Add benchmark / endurance test `tests/test_mmap_large_document.rs` verifying flat memory usage on 200MB+ mock fixture
- [ ] Ensure WASM target compiles without errors: `cargo check --target wasm32-unknown-unknown -p paperpilot-wasm -j 4`
- [ ] Run workspace compilation and test suite with `cargo check --workspace -j 4`
