# Spec 179: `paperpilot-py` Phase 2 — In-Memory Bytes API, Extended Core Tools, & PyPI CI/CD

## Overview
Phase 2 expands `paperpilot-py` from its Phase 1 MVP (`specs/178-paperpilot-py-mvp`) into a production-ready Python library for RAG, FastAPI, and document-automation pipelines:
1. **Explicit `*_bytes` In-Memory API**: Provide `compress_bytes`, `to_markdown_bytes`, `extract_text_bytes`, `chunk_rag_bytes`, `merge_bytes`, and `info_bytes` (with `py.allow_threads` GIL release) for zero-disk-I/O pipelines.
2. **Additional High-Value Core Tools**: Expose `rotate` (and `rotate_bytes`) and `watermark` (and `watermark_bytes`) from `paperpilot-pdf` / `paperpilot-core`.
3. **Type Stubs & Tests**: Update `paperpilot-py/paperpilot.pyi` and `paperpilot-py/tests/test_paperpilot.py` to cover all new functions.
4. **CI/CD Wheel Publishing**: Add `.github/workflows/pypi.yml` using `PyO3/maturin-action` to build and publish wheels for Linux, macOS (`x86_64` + `arm64`), and Windows on `v*.*.*` tag pushes.

## Task Breakdown
- [ ] **Task 1**: Create `specs/179-paperpilot-py-phase2/spec.md` with requirements and task breakdown.
- [ ] **Task 2**: Implement `*_bytes` variants (`compress_bytes`, `to_markdown_bytes`, `extract_text_bytes`, `chunk_rag_bytes`, `merge_bytes`, `info_bytes`) in `paperpilot-py/src/lib.rs` with `py.allow_threads`.
- [ ] **Task 3**: Implement `rotate` / `rotate_bytes` and `watermark` / `watermark_bytes` in `paperpilot-py/src/lib.rs` using `paperpilot-pdf` operations.
- [ ] **Task 4**: Update `paperpilot-py/paperpilot.pyi` type stubs and `paperpilot-py/tests/test_paperpilot.py` unit tests.
- [ ] **Task 5**: Add `.github/workflows/pypi.yml` using `PyO3/maturin-action` for Linux, macOS (`x86_64` + `arm64`), and Windows on `v*.*.*` tags.
- [ ] **Task 6**: Verify with `cargo check --offline -p paperpilot-py -j 4` (0 errors, 0 warnings), update `ROADMAP.md`, run `graphify-rs build .`, and commit/push to `feature/roadmap-review`.
