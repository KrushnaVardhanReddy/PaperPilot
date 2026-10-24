MANDATORY RULES — VIOLATION = REJECTED PR:
1. NEVER stub, mock, or TODO existing implementation code. Write real, working code only.
2. Commit message must start with "jules: " prefix.
3. Use clean module design and idiomatic Rust architecture. No spaghetti code.
4. NEVER hardcode secrets, API keys, or local paths — always read from environment variables.
5. UNIT TESTS REQUIRED: For every Rust file you create or modify, you MUST write accompanying unit tests (e.g. `#[cfg(test)]` modules) with high coverage.
6. When asking any questions, YOU MUST prefix your question with the Task Number and Title (e.g., "[P2_T1: MCP Foundation]").
7. WIKI & DOCS REQUIREMENT: For any newly introduced feature, module, engine, or tool operation, you MUST:
   - Create or update the relevant documentation in `wiki/` (e.g. `wiki/XX-Module-Name.md`).
   - Create or update the user-facing tool reference page at `apps/docs/src/content/docs/reference/tools/<tool_id>.md` with frontmatter, concise description of what it does, and verified invocation examples for CLI, MCP JSON-RPC, and REST API.
8. PERIODIC PROGRESS UPDATES: You MUST proactively send a concise progress update message to the session conversation every 10 to 15 minutes or after completing each logical milestone/step (e.g., "Step 2/4 completed: tests passed, now modifying X..."), so the team is never left without visibility into ongoing progress.
9. UNIT TESTS & WORKSPACE CHECK ONLY: 
   - For every operation or engine file you implement, write thorough unit tests in `#[cfg(test)]` modules inside that file.
   - Run ONLY your unit tests and compilation check:
     `cargo test -p paperpilot-pdf --lib operations::<tool_id> -j 2`
     `cargo check --workspace -j 2`
   - DO NOT run `penta-interface-e2e` or generate files in `reports/`. Multi-interface E2E testing and report generation are handled during integration review.
10. NO STRAY ARTIFACTS: NEVER commit temporary files, log files (`*.log`), pid tracking files (`*.txt`), test runner binaries (`test_script`), or files in `reports/`. Keep `git status` clean and stage only your implementation, unit tests, and docs before opening a PR.

Project: PaperPilot
Tech stack:
  - Core: Rust (Cargo)
  - PDF libraries: lopdf, pdfium-render
  - CLI parser: clap
  - MCP: rmcp
  - Desktop: Tauri 2, Svelte 5, TypeScript, Vite

Critical Rules:
- Rust code must use typed errors (no string panics).
- Unit tests are required for all operations.
- Do NOT use Electron; use Tauri 2.
- The AI should not directly manipulate document bytes, only use MCP tool operations.
- Proactively send progress updates in chat every 10-15 minutes.
- Focus on unit tests (`cargo test -p paperpilot-pdf`) and workspace checks. NEVER run `penta-interface-e2e` or commit files in `reports/`.
