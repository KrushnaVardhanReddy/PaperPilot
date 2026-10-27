# Production & Live-Product Rules (STRICT)

PaperPilot is a live, publicly trending open-source product with active users downloading binaries and running the web sandbox.
Every AI agent and contributor working on this workspace MUST adhere strictly to the following rules:

## 1. Zero-Mock Testing Policy
- **Never mock PDF bytes or responses in tests.**
- All Rust tests (`paperpilot-pdf`, `paperpilot-mcp`, `paperpilot-gateway`) and UI tests (`apps/desktop`) must execute against real PDF disk fixtures (`fixtures/`, `tests/e2e_fixtures/real/`) or dynamically synthesized in-memory PDF byte buffers.
- If a tool fails on real-world malformed PDFs, fix the parser or handle the error gracefully; never paper over it with mocks.

## 2. Zero-Cloud / Absolute Privacy Guarantee
- PaperPilot's primary value proposition is **100% Zero-Cloud / 0 Bytes Uploaded**.
- **No unexpected telemetry or network requests**: Never introduce telemetry, analytics beacons, external cloud API pings, or phone-home mechanisms into the Core engine, CLI, MCP server, WASM package, or Desktop app.
- On the web portal (`apps/web`), the only permitted outbound network call is fetching public GitHub release counts for the download badge (strictly cached for 15 minutes in `sessionStorage`).

## 3. Strict Verification Before Any Commit to `main`
Before committing and pushing any feature or fix to `main`, the following verification matrix MUST pass:
1. `cargo check --workspace -j 2`
2. `cargo test -p paperpilot-pdf -j 2`
3. `cargo test -p paperpilot-mcp -j 2`
4. `pnpm --dir apps/desktop test` (all 20 test files, 105+ unit tests)

## 4. Single Core Runner + 5 Wrappers Architecture Pattern
- Every PDF operation must have exactly **ONE** core implementation in `paperpilot-pdf`:
  ```rust
  pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
  ```
- Then wrap cleanly across all 5 surfaces without duplicating core business logic:
  1. CLI (`paperpilot-cli`)
  2. MCP Server (`paperpilot-mcp`)
  3. REST Gateway (`paperpilot-gateway`)
  4. WASM Engine (`paperpilot-wasm`)
  5. Desktop & Embed UI (`apps/desktop`, `apps/embed`)

## 5. Branching Strategy (No Direct Pushes to `main`)
- Active development must take place on `feature/dev` or dedicated feature branches (e.g. `feat/...`).
- **Never push unverified experimental commits directly to `main`**.
- Code merges into `main` only after complete local validation across all test suites.

## 6. Host Safety & Concurrency Constraint
- **ALWAYS** pass `-j 2` for all local `cargo test`, `cargo build`, or `cargo clippy` commands. Unbounded compilation (`cargo test` without `-j 2`) will exhaust RAM and freeze the development environment.
- Never execute destructive git commands: `git reset --hard`, `git clean -fd`, or `git restore .`.

## 7. Mandatory Tool Documentation & Handbook Synchronization
- Whenever a tool is added, renamed, or modified:
  1. Add/update its dedicated documentation page at `apps/docs/src/content/docs/reference/tools/<tool_name>.md`.
  2. Regenerate the master handbook to keep https://docs.usepaperpilot.com/reference/50-tools-handbook/ in sync:
     ```bash
     python3 apps/docs/src/content/docs/reference/generate_handbook.py
     ```
  3. Ensure the documentation site builds cleanly without broken links:
     ```bash
     npm --prefix apps/docs run build
     ```

