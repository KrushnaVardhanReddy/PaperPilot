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
| **01** | **Rust Core Engine** | ✅ **100% Done** | 45+ pure-Rust operations, zero-panic parser, verified CLI | [phase-01-rust-core.md](file:///home/krushna/Project/PaperPilot/phases/phase-01-rust-core.md) |
| **02** | **MCP Server** | ✅ **100% Done** | 78+ MCP tools registered for Claude Desktop, Cursor, OpenCode | [phase-02-mcp-server.md](file:///home/krushna/Project/PaperPilot/phases/phase-02-mcp-server.md) |
| **03** | **Desktop + Mobile App** | ✅ **100% Done** | Tauri 2.0 desktop + responsive touch mobile layout | [phase-03-desktop-mobile-app.md](file:///home/krushna/Project/PaperPilot/phases/phase-03-desktop-mobile-app.md) |
| **04** | **AI Layer & Workflows** | ✅ **100% Done** | Natural language routing, action cards, declarative pipelines | [phase-04-ai-layer.md](file:///home/krushna/Project/PaperPilot/phases/phase-04-ai-layer.md) |
| **05** | **Advanced Intelligence** | 🚀 **In Progress (92%)** | Pure-Rust OCR (`ocrs`), on-device Qwen-0.5B SLM, visual pipeline generator | [phase-05-advanced-intelligence.md](file:///home/krushna/Project/PaperPilot/phases/phase-05-advanced-intelligence.md) |
| **06** | **Enterprise Security** | 📋 **Planned (P1 B2B)** | SSO (SAML/OIDC), RBAC, Ed25519 offline license validation, SIEM logs | [phase-06-enterprise.md](file:///home/krushna/Project/PaperPilot/phases/phase-06-enterprise.md) |
| **07** | **Managed Cloud Gateway** | 📋 **Planned (P2)** | Multi-tenant REST gateway, Cloudflare Edge workers, usage metering | [phase-07-managed-cloud.md](file:///home/krushna/Project/PaperPilot/phases/phase-07-managed-cloud.md) |
| **08** | **Growth & Security** | 🚀 **In Progress (88%)** | Community contributions, HackerNews feedback, Semgrep & Supply Chain audit | [phase-08-growth-platform.md](file:///home/krushna/Project/PaperPilot/phases/phase-08-growth-platform.md) |
| **09** | **Advanced Editing** | ✅ **100% Done** | Form filling, digital signatures, flattening, visual annotations | [phase-09-advanced-editing.md](file:///home/krushna/Project/PaperPilot/phases/phase-09-advanced-editing.md) |
| **10** | **WebAssembly (WASM)** | ✅ **100% Done** | In-browser zero-server WASM sandbox + single-file `paperpilot-portable.html` | [phase-10-webassembly-wasm.md](file:///home/krushna/Project/PaperPilot/phases/phase-10-webassembly-wasm.md) |
| **11** | **`.ppdoc` Native Format** | 📋 **Backlog** | Fast-saving non-destructive container format for project edits | [phase-11-ppdoc-native-format.md](file:///home/krushna/Project/PaperPilot/phases/phase-11-ppdoc-native-format.md) |
| **12** | **Office Ecosystem** | 🚀 **In Progress (85%)** | Wave 4 `.docx`, `.xlsx`, `.pptx` conversion and modernization | [phase-12-future-roadmap-office.md](file:///home/krushna/Project/PaperPilot/phases/phase-12-future-roadmap-office.md) |
| **13** | **Universal Document OS** | 🔮 **Future Vision** | Cross-format conversion matrix and air-gapped document intelligence | [phase-13-universal-document-os.md](file:///home/krushna/Project/PaperPilot/phases/phase-13-universal-document-os.md) |

---

## 🎯 Active Development Focus
- **Task 84D (Phase 5.9.73 / 5.9.99)**: Compound workflow dataset generation & visual pipeline editor for on-device Qwen-0.5B SLM.
- **Task 122A (Phase 5.9.91)**: Lossless indirect stream deduplication & CMYK safety (`SimonBure/presse` port in Jules).
- **Task 129A (Phase 5.9.98)**: Malformed PDF adversarial regression test suite in Jules (`malformed_regression.rs`).

For session handoff and active Jules dispatch status, see [`HANDOFF.md`](file:///home/krushna/Project/PaperPilot/HANDOFF.md).
