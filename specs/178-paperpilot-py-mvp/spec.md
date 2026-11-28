# Spec 178: `paperpilot-py` Phase 1 MVP (PyO3 + Maturin Python Bindings)

## Metadata
- **Spec Number**: 178
- **Title**: `paperpilot-py` Phase 1 MVP — Native Python Wheel via PyO3 & Maturin
- **Phase**: Phase 1 MVP / Ecosystem & Distribution
- **Status**: ✅ Completed
- **Impacted Systems**:
  - `paperpilot-py/` (new crate: `Cargo.toml`, `pyproject.toml`, `src/lib.rs`, `paperpilot.pyi`, `tests/test_paperpilot.py`)
  - `Cargo.toml` (workspace member registration)

---

## 1. Executive Summary & Goals

Python is the primary language for AI/ML, RAG pipelines, document ingestion, and enterprise automation. Currently, Python developers must shell out to `paperpilot` CLI or call the REST Gateway. `paperpilot-py` provides **zero-copy, in-process native Python bindings** to `paperpilot-pdf` and `paperpilot-core` using **PyO3** and **maturin**.

### Key Objectives (Phase 1 MVP):
1. **Native Python Wheel**: Package `paperpilot-py` as an installable Python module (`paperpilot`) built with `maturin` and `pyo3` (`abi3-py39` compatible).
2. **GIL-Released Execution**: All CPU-bound PDF/RAG operations release the Python GIL via `py.allow_threads(...)` so Python multi-threading and `asyncio.to_thread` scale across cores.
3. **8–10 High-Value Core & RAG Tools**:
   - `merge(pdfs: list[bytes]) -> bytes`
   - `split(pdf: bytes, ranges: str) -> list[bytes]`
   - `compress(pdf: bytes, level: str = "medium") -> bytes`
   - `to_markdown(pdf: bytes) -> str`
   - `chunk_rag(pdf: bytes, max_tokens: int = 512, overlap: int = 50) -> list[dict]`
   - `extract_text(pdf: bytes, pages: str | None = None) -> str`
   - `redact(pdf: bytes, terms: list[str]) -> bytes`
   - `encrypt(pdf: bytes, user_password: str, owner_password: str | None = None) -> bytes`
   - `info(pdf: bytes) -> dict` (bonus inspection helper)
   - `repair(pdf: bytes) -> bytes` (bonus self-healing helper)
4. **Type Stubs (`paperpilot.pyi`)**: Full PEP 484 type annotations for IDE autocomplete and `mypy`/`pyright` support.
5. **Smoke Tests**: `pytest` suite verifying all exposed functions end-to-end on real generated PDF bytes.

---

## 2. Task Breakdown

- [ ] **Task 1**: Create `paperpilot-py/Cargo.toml` and `paperpilot-py/pyproject.toml` with `pyo3` + `maturin` configuration, and register `"paperpilot-py"` in root `Cargo.toml`.
- [ ] **Task 2**: Inspect `paperpilot-pdf` operation signatures (`merge`, `split`, `compress`, `conversion::to_markdown`, `pdf_chunk`, `extract_text`, `redact`, `encrypt`, `info`, `repair`) and implement `paperpilot-py/src/lib.rs` with `py.allow_threads` GIL release and clean `PyValueError` / `PyRuntimeError` error mapping.
- [ ] **Task 3**: Create `paperpilot-py/paperpilot.pyi` type stubs matching the Rust `#[pyfunction]` signatures.
- [ ] **Task 4**: Add `paperpilot-py/tests/test_paperpilot.py` smoke tests and verify with `cargo check -p paperpilot-py -j 4` (and `maturin` / `pytest` if available).
- [ ] **Task 5**: Update `graphify-rs build .`, commit, and push to `origin feature/roadmap-review`.
