# Tasks: Glama MCP TDQS Grade A+ Enhancement

- [x] 1. Schema Hardening & Property Coverage <!-- id: 1 -->
  - [x] Audit all tool schemas in `paperpilot-mcp/src/server.rs` for `"additionalProperties": false`.
  - [x] Verify that every property in every tool's JSON schema defines an explicit `"description"`.
  - [x] Add explicit `"enum"` constraints and defaults for constrained parameters (`format`, `position`, `direction`, `orientation`, etc.).

- [x] 2. TDQS 6-Dimension Description Audit <!-- id: 2 -->
  - [x] Verify 100% compliance across all tools with the 4 mandatory sections:
    - `When to use:`
    - `Difference from related tools:` / `unlike:` with explicit "do NOT use this for..." negative constraints
    - `Key parameters:`
    - `[Input: ... | Output: ...]`
  - [x] Disambiguate newly added tools.

- [x] 3. Alias Suppression Verification <!-- id: 3 -->
  - [x] Ensure `execute_list_tools()` returns only canonical tools (no duplicate alias pairs).
  - [x] Ensure `execute_call_tool()` safely resolves both canonical and legacy aliases.

- [x] 4. Test Suite & Verification <!-- id: 4 -->
  - [x] Run `cargo check -p paperpilot-mcp -j 4`.
  - [x] Run `cargo test -p paperpilot-mcp -j 4`.
  - [x] Verify schema formatting and tool count sanity.

- [x] 5. Version Release & Commit <!-- id: 5 -->
  - [x] Commit all changes cleanly.
  - [x] Ready `v0.8.0` for Glama re-index and scoring sweep.
