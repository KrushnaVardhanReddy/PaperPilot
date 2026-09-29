# PaperPilot Post-Launch Plan: AI & Production (Phase 4+)

With the v1.0 Base Version (Phases 1-3) successfully completed, we are now shifting focus to the advanced features. This includes the Free Tier Offline NLP, full AI generation capabilities, and product/production deployment tasks.

## 🔲 Pending Post-Launch Tasks

### Phase 4.0 — Offline NLP Mode (Free Tier)
*A fast, embedded intent classifier that runs 100% locally with zero API key. Handles single-intent commands (~80% of real user needs). Ships by default.*

| Task | What | Status |
|---|---|---|
| 4.0.1 | Create `paperpilot-nlp` crate | ✅ Completed |
| 4.0.2 | Define `NlpResolver` trait | ✅ Completed |
| 4.0.3 | Intent vocabulary definition (~45 operations) | ✅ Completed |
| 4.0.4 | Layer 1 — Keyword & regex rule engine | ✅ Completed |
| 4.0.5 | Layer 2 — ONNX intent classifier (MobileBERT) | 🚧 In Progress (Session 12133795038104614276) |
| 4.0.6 | Entity extractor (regex + NER) | 🚧 In Progress (Session 4945848464737601940) |
| 4.0.7 | Ambiguity resolver | Pending |
| 4.0.8 | Offline NLP → `OperationPlan` output | Pending |
| 4.0.9 | Offline NLP unit tests | Pending |
| 4.0.10 | Offline NLP E2E tests (MCP tool dispatch) | Pending |

### Phase 4.E2E — System Validation Testing
| Task | What | Status |
|---|---|---|
| E.1 | CLI End-to-End Testing & Validation Report | Pending |
| E.2 | MCP End-to-End Testing & Validation Report | Pending |
| E.3 | Svelte UI End-to-End Testing & Validation Report | Pending |

### Phase 3.1.7 — iOS Target
| Task | What | Status |
|---|---|---|
| 3.1.7 | iOS target (Requires Mac with Xcode) | Pending |

## 📅 Timeline
- **Week 1:** Implement the Free Tier Offline NLP classification engine (Tasks 4.0.1 - 4.0.10).
- **Week 2:** Implement AI PDF Creation tools and integrate them into the Desktop UI (Tasks 4.6.1 - 4.6.6).
- **Week 3:** Finalize Offline OCR and product deployment builds (Phase 5 & 6).
