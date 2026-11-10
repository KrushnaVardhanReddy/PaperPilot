# PaperPilot — Development Phases & Executive Dashboard

> **Single Source of Truth for Architecture Milestones & Development Progress**  
> For detailed tasks, specs, and session handoffs, navigate to each modular phase below.  
> Each phase has a clear exit condition — don't move to the next phase until it's met.

---

## 🏛️ Licensing Model (LOCKED)

**Decision: Open Core** — like GitLab, HashiCorp Vault, and Outline.

| Tier | License | What's included |
|---|---|---|
| **Community (Free)** | Apache 2.0 | Core engine, CLI, MCP server, Desktop app, all PDF operations |
| **Enterprise (Paid)** | Commercial | SSO (SAML/OIDC), RBAC, Audit logs, Multi-org, License key management, Group Policy |
| **Cloud (Paid)** | Commercial | Managed hosting, Cloud MCP gateway, Billing, SLA monitoring |

**Rules:**
- Everything in Phases 1–5 is **Apache 2.0** — always free, always open.
- Everything in Phase 6 (Enterprise) and Phase 7 (Cloud) is **Commercial** — gated behind a license key.
- The boundary must never move. Enterprise features must never sneak into the Apache-licensed core.

---

## 🧭 Modular Phases Navigation & Progress Dashboard

| Phase | Scope & Title | Status | Exit Condition / Summary | Detailed Spec File |
|:---:|---|:---:|---|:---:|
| **01** | **Rust Core Engine** | ✅ **100% Done** | 90 pure-Rust operations, zero-panic parser, verified CLI | [phase-01-rust-core.md](file:///home/krushna/Project/PaperPilot/phases/phase-01-rust-core.md) |
| **02** | **MCP Server** | ✅ **100% Done** | 90 MCP tools registered, parameter-aligned, and validated across Waves 1–21 | [phase-02-mcp-server.md](file:///home/krushna/Project/PaperPilot/phases/phase-02-mcp-server.md) |
| **03** | **Desktop + Mobile App** | ✅ **100% Done** | All 90 tools wired with dedicated Svelte 5 UI parameter panels and validated via zero-mock Playwright E2E suites | [phase-03-desktop-mobile-app.md](phases/phase-03-desktop-mobile-app.md) |
| **04** | **AI Layer & Workflows** | ✅ **100% Done** | Watched folders, declarative `.ppflow` workflows, instant recipes, and `cli_ask` natural language assistant | [phase-04-ai-layer.md](phases/phase-04-ai-layer.md) |
| **05** | **Advanced Intelligence** | ✅ **100% Done** | Neural OCR, on-device SLM, RAG chunking, pattern extraction, and vector semantic search | [phase-05-advanced-intelligence.md](file:///home/krushna/Project/PaperPilot/phases/phase-05-advanced-intelligence.md) |
| **06** | **Enterprise Security** | 📋 **Planned (Tier 4)** | SSO (SAML/OIDC), RBAC, Ed25519 offline license validation, SIEM logs, Spec 150 Hexagonal DatabasePort | [phase-06-enterprise.md](file:///home/krushna/Project/PaperPilot/phases/phase-06-enterprise.md) |
| **07** | **Managed Cloud Gateway** | 📋 **Planned (Tier 4)** | Multi-tenant REST gateway, Cloudflare Edge workers, usage metering | [phase-07-managed-cloud.md](file:///home/krushna/Project/PaperPilot/phases/phase-07-managed-cloud.md) |
| **08** | **Growth & Security** | ✅ **100% Done** | Multi-scanner audit clean (Semgrep SAST 0 findings, Cargo Audit 0 criticals, Trivy FS clean, dependencies pruned) | [phase-08-growth-platform.md](file:///home/krushna/Project/PaperPilot/phases/phase-08-growth-platform.md) |
| **09** | **Advanced Editing** | ✅ **100% Done** | Form filling, digital signatures, flattening, visual annotations, bates numbering, watermarks | [phase-09-advanced-editing.md](file:///home/krushna/Project/PaperPilot/phases/phase-09-advanced-editing.md) |
| **10** | **WebAssembly (WASM)** | ✅ **100% Done** | Zero-server WASM sandbox, pure-Rust bundle compiled cleanly without `mio`/network bloat | [phase-10-webassembly-wasm.md](file:///home/krushna/Project/PaperPilot/phases/phase-10-webassembly-wasm.md) |
| **11** | **Browser Extensions** | 🚀 **Next Up (Tier 3)** | Manifest V3 companion extension (Chrome/Edge/Firefox) with in-tab PDF actions & clean-article to PDF/A exporter | [ROADMAP.md](ROADMAP.md) |
| **12** | **Office Ecosystem** | ✅ **100% Done** | Complete `.docx`, `.xlsx`, `.pptx`, `.md`, `.csv` bidirectional conversions and modernization | [phase-12-future-roadmap-office.md](file:///home/krushna/Project/PaperPilot/phases/phase-12-future-roadmap-office.md) |
| **13** | **Universal Document OS** | 🔮 **Future Vision** | Cross-format conversion matrix and air-gapped document intelligence | [phase-13-universal-document-os.md](file:///home/krushna/Project/PaperPilot/phases/phase-13-universal-document-os.md) |

---

## 🎯 Active Development Focus
- **Hexagonal DatabasePort Architecture (Spec 150)**: Decoupled database domain models into `paperpilot-core::storage` and introduced `pub trait DatabasePort: Send + Sync` (PR #324). Implemented on `paperpilot-storage` with embedded libSQL, enabling `PaperPilot-Enterprise` to implement PostgreSQL/Supabase without upstream code changes.
- **Production Desktop & Release Packaging Hardening**: Fixed production WebKitGTK PDF blank canvas failure by passing raw `Uint8Array` buffers and updating Tauri CSP. Resolved desktop window/dock icon configuration (`#[cfg(all(not(target_os = "android"), not(target_os = "ios")))]`) and added `PaperPilot.desktop` launcher to release archives.
- **WASM & CI Runner Stability**: Gated native dependencies for zero-network WASM compilation, modernized GitHub Actions workflows to `@v4`/`@v2`, and resolved macOS runner clang target incompatibility.
- **Multi-Agent Delegation Framework**: Enhanced `tools/jules-submit` and `scripts/gemini_pair.sh` to support both Cloud Jules API and local Gemini CLI pair programming (`agy`). Developers can run prompts autonomously (`make gemini-run FILE=...`) or interactively (`make gemini-pair FILE=...`) with zero cloud latency or session timeout risks.

For session handoff, checklist, and developer guides, see [`HANDOFF.md`](file:///home/krushna/Project/PaperPilot/HANDOFF.md).

