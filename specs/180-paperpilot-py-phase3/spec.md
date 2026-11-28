# Spec 180: `paperpilot-py` Phase 3 — Penta-Scaffold Integration & Generic Tool Dispatch

## Status
- **Status:** In Progress
- **Branch:** `feature/roadmap-review`
- **Crate:** `paperpilot-py`, `tools/penta-scaffold`

## Goals
1. **Penta-Scaffold Python Binding & Stub Hooks:**
   - Wire `paperpilot-py` into `tools/penta-scaffold/src/generator.rs` so that scaffolding a new tool automatically reports/wires Python binding and `.pyi` stub hooks (e.g., generating or updating `paperpilot-py/paperpilot.pyi` and `paperpilot-py/src/lib.rs` or providing a clean scaffolded binding/stub hook).
2. **Generic Tool Discovery & Execution in `paperpilot-py`:**
   - Expose `list_tools() -> list[dict[str, str]]` (or `ToolInfo` TypedDict) returning all registered `paperpilot-core` tools (`id`, `name`, `description`, `category`).
   - Expose `run_tool(tool_id: str, params_json: str = "{}") -> str` (or `dict`) so Python users can invoke any `paperpilot-core` tool cleanly via `paperpilot_core::registry` / `paperpilot_core::dispatch` without waiting for dedicated per-tool PyO3 wrappers.
3. **Verification:**
   - Rust unit tests in `paperpilot-py/src/lib.rs` for `list_tools()` and `run_tool()`.
   - Real non-mocked Python integration tests in `paperpilot-py/tests/test_paperpilot.py`.
   - `cargo test --offline -p paperpilot-py -j 4` and `cargo check --offline -p penta-scaffold -j 4` pass with 0 errors.
   - `ROADMAP.md` updated and `graphify-rs build .` run.
