# Spec 042 — Agentic Code Quality & Dead-Scaffolding Pruning via `fossil-mcp`

## 1. Overview
With dozens of autonomous AI-driven development sessions (Jules PRs, Antigravity orchestrations) spanning 8 Rust workspace crates, 3 frontend apps (Svelte 5 desktop, web portal, embed widget), and Cloudflare Edge microservices, codebase entropy accumulates:
- Unused helper functions and dead types across crate boundaries.
- Duplicate utility clones across crates.
- Leftover mock scaffolding from completed verification sessions.

`fossil-mcp` is an agentic code quality toolkit built for AI workflows that identifies dead code, clones, and unused scaffolding across 15 programming languages.

**Objective:**
Integrate `fossil-mcp` into PaperPilot's verification and CI pipeline to **automatically detect and prune dead code and unused scaffolding**, enforcing compact binary size and $<35\,\text{MB}$ memory footprints.

---

## 2. Technical Architecture & Verification Flow

1. **Static Analysis & Clone Detection**:
   - Run `fossil-mcp` scan across:
     - `paperpilot-core`, `paperpilot-pdf`, `paperpilot-mcp`, `paperpilot-gateway`, `paperpilot-wasm`, `paperpilot-cli`, `paperpilot-nlp`, `paperpilot-ai`.
     - `apps/desktop`, `apps/web`, `apps/embed`, `apps/edge`.
2. **Pruning Targets**:
   - Dead Rust functions, traits, and unused struct fields.
   - Unused Svelte components and duplicated TypeScript helper functions.
3. **Automated Makefile Integration**:
   - `make audit-code-quality`: Executes `fossil-mcp` scan and reports dead-code score.
4. **CI Integration**:
   - Integrated into `.github/workflows/rust.yml` to block PRs introducing orphan scaffolding.

---

## 3. Acceptance Criteria
1. Scan runs across all 8 crates and Svelte apps.
2. Zero regression: all 880+ E2E tests and clippy checks pass.
3. Documented in `reports/FOSSIL_CODE_QUALITY_AUDIT_REPORT.md`.
