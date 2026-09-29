# PaperPilot Post-Launch Plan: AI & Production (Phase 4+)

With the v1.0 Base Version (Phases 1-3) successfully completed, we are now shifting focus to the advanced features. This includes the Free Tier Offline NLP, full AI generation capabilities, and product/production deployment tasks.

## 🔲 Pending Post-Launch Tasks

### Phase 4.0 — Offline NLP Mode (Free Tier)
*A fast, embedded intent classifier that runs 100% locally with zero API key. Handles single-intent commands (~80% of real user needs). Ships by default.*

| Task | What | Status |
|---|---|---|
| 4.0.1 | Create `paperpilot-nlp` crate | Pending |
| 4.0.2 | Define `NlpResolver` trait | Pending |
| 4.0.3 | Intent vocabulary definition (~45 operations) | Pending |
| 4.0.4 | Layer 1 — Keyword & regex rule engine | Pending |
| 4.0.5 | Layer 2 — ONNX intent classifier (MobileBERT) | Pending |
| 4.0.6 | Entity extractor (regex + NER) | Pending |
| 4.0.7 | Ambiguity resolver | Pending |
| 4.0.8 | Offline NLP → `OperationPlan` output | Pending |
| 4.0.9 | Offline NLP unit tests | Pending |
| 4.0.10 | Offline NLP E2E tests (MCP tool dispatch) | Pending |

### Phase 4.6 — AI PDF Creation (Free Tier & Pro)
*Generate professional PDFs from natural language prompts.*

| Task | What | Status |
|---|---|---|
| 4.6.1 | Natural language → PDF content (Markdown structure) | Pending |
| 4.6.2 | AI template selection | Pending |
| 4.6.3 | AI fills template variables | Pending |
| 4.6.4 | Review & edit before rendering | Pending |
| 4.6.5 | `pdf_create` MCP tool | Pending |
| 4.6.6 | `paperpilot create` CLI | Pending |

### Phase 5 & 6 — Advanced Intelligence & Enterprise (Product Build)
*The final offline and production tasks needed for the commercial product build.*

| Task | What | Status |
|---|---|---|
| 5.1 | Offline OCR (`leptess` / `ocrs`) | Pending |
| 5.2 | Semantic Search & Document Understanding (Local embeddings) | Pending |
| 6.1 | Identity & SSO (SAML 2.0, OIDC) | Pending |
| 6.2 | Security & Policy Configs | Pending |
| 6.3 | Audit Logging | Pending |
| 3.1.7 | iOS target (Requires Mac with Xcode) | Pending |

## 📅 Timeline
- **Week 1:** Implement the Free Tier Offline NLP classification engine (Tasks 4.0.1 - 4.0.10).
- **Week 2:** Implement AI PDF Creation tools and integrate them into the Desktop UI (Tasks 4.6.1 - 4.6.6).
- **Week 3:** Finalize Offline OCR and product deployment builds (Phase 5 & 6).
