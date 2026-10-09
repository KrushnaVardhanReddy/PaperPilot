# PaperPilot Development Law: The Penta-Interface Mandate & E2E Protocol

> **CRITICAL RULE**: Whenever a new tool or operation is introduced into PaperPilot, it **MUST NOT** be implemented as a single-interface or CLI-only feature. Every tool is a citizen of all **5 deployment surfaces**, accompanied by **Rust-Native E2E real semantic assertions** and **performance benchmarks**.

---

## 🏛️ The 5 Deployment Surfaces

Every operation implemented in `paperpilot-pdf` must expose an adapter across all 5 surfaces:

```
                          ┌──────────────────────────┐
                          │   paperpilot-core /      │
                          │   paperpilot-pdf         │
                          │   (Pure Rust Engine)     │
                          └─────────────┬────────────┘
                                        │
        ┌──────────────┬────────────────┼──────────────┬──────────────┐
        ▼              ▼                ▼              ▼              ▼
    1. 💻 CLI      2. 🤖 MCP       3. 🌐 REST      4. ⚡ WASM     5. ☁️ Edge
   (Terminal)     (Claude/Cursor)  (HTTP Gateway)  (In-Browser)   (Cloudflare)
```

### 1. 💻 CLI (`paperpilot-cli`)
- Add CLI subcommand in `paperpilot-cli/src/cli.rs`.
- Wire handler in `paperpilot-cli/src/commands/`.
- Support `--json`, standard input/output paths, and global flags.
- Command naming standard: `kebab-case` (`page-numbers`, `header-footer`, `extract-text`).

### 2. 🤖 MCP (`paperpilot-mcp` & Gateway `/mcp/messages`)
- Declare JSON Schema in `paperpilot-mcp/src/server.rs` (`list_tools`).
- Implement execution in `handle_tool_call` returning JSON-RPC 2.0 result object.
- Parameters must accept structured JSON arguments (`{"input": "...", "output": "..."}`).

### 3. 🌐 REST API (`paperpilot-gateway` :7823)
- Register generic tool route `POST /api/v1/pdf/tools/{tool_name}` (and bespoke endpoint if applicable).
- Accept JSON body with structured parameters.
- Provide Swagger / OpenAPI documentation tags.

### 4. ⚡ WASM (Browser In-Memory Engine: `paperpilot-wasm`)
- Expose `wasm-bindgen` in-memory entrypoint taking `Uint8Array` bytes.
- Return processed `Uint8Array` or structured JSON without filesystem dependencies.
- Update `apps/desktop/src/lib/wasm/` or `apps/web/src/lib/wasm/` worker bridges.

### 5. ☁️ Cloudflare Edge (`apps/edge`)
- Expose serverless handler in `apps/edge/src/index.ts` under `/api/v1/{tool_name}`.
- Zero-disk, stream-in / stream-out memory processing.
- Add Vitest spec in `apps/edge/tests/edge.spec.ts`.

---

## 🧪 Rust-Native E2E Testing Protocol (`tools/penta-interface-e2e`)

Whenever adding a tool, you **MUST** register test cases in `tools/penta-interface-e2e/src/cases/`:

### 4-Tier Matrix (20 Tests Minimum per Tool)
Every tool must define 4 complexity tiers across all 5 interfaces:
1. **Tier 1 (Simple)**: Minimal single-page valid input.
2. **Tier 2 (Medium)**: Multi-page structured input with non-default options.
3. **Tier 3 (Complex / Hard)**: Stress conditions (unicode, tables, overlapping geometry, multi-file).
4. **Negative Case**: Hostile / invalid input asserting structured error codes without process panic.

$$5\text{ Interfaces} \times 4\text{ Tiers} = 20\text{ Verified Test Assertions per Tool}$$

### Strict Testing Rules:
1. **Real Assertions Only**: Never stub, mock, or use `if !passed { passed = true }`.
2. **Physical Verification**: Assert real page counts, extracted text, metadata, or rotation angles using `PdfAssertions`.
3. **Report Generation**: Running `cargo run -p penta-interface-e2e` must regenerate the master scorecard in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT.md`.

---

## ⚡ Performance & Concurrency Benchmark Integration

Every new tool must be profiled in the high-throughput test runner:
1. Add tool to latency profiling list in `tools/penta-interface-e2e/src/cases/bench.rs`.
2. Run benchmark:
   ```bash
   make bench-penta-e2e
   # or
   cargo run -p penta-interface-e2e -- --group bench
   ```
3. Assert:
   - **Throughput**: Measures operations/sec.
   - **Latency SLA**: Records p50, p95, and p99 percentiles across CLI, MCP, and REST.
   - **Memory Leak Test**: Zero memory leaks under 1,000 continuous burst iterations (<35MB RSS).

---

## 📋 Checklist for New Tool PRs

When opening a PR adding a tool `pdf_<tool_name>`:
- [ ] Core operation implemented in `paperpilot-pdf/src/operations/<tool>.rs`
- [ ] CLI command added in `paperpilot-cli/src/cli.rs`
- [ ] MCP schema and execution added in `paperpilot-mcp/src/server.rs`
- [ ] Gateway endpoint exposed in `paperpilot-gateway` (`/api/v1/pdf/tools/<tool>`)
- [ ] WASM binding exported in `paperpilot-wasm`
- [ ] Edge worker route added in `apps/edge/src/index.ts`
- [ ] 20 E2E assertions (4 tiers × 5 interfaces) added to `tools/penta-interface-e2e/src/cases/`
- [ ] Edge cases & hostile inputs covered in `edge_cases.rs`
- [ ] Added to concurrency benchmark in `bench.rs`
- [ ] `make test-penta-e2e` passes 100%
- [ ] `make test-penta-edge-cases` passes 100%
