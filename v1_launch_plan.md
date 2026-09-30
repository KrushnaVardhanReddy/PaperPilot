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
| 4.0.5 | Layer 2 — ONNX intent classifier (MobileBERT) | ✅ Completed |
| 4.0.6 | Entity extractor (regex + NER) | ✅ Completed |
| 4.0.7 | Ambiguity resolver | ✅ Completed |
| 4.0.8 | Offline NLP → `OperationPlan` output | ✅ Completed |
| 4.0.9 | Offline NLP unit tests | ✅ Completed |
| 4.0.10 | Offline NLP E2E tests (MCP tool dispatch) | ✅ Completed |

### Phase 4.E2E — System Validation Testing
| Task | What | Status |
|---|---|---|
| E.1 | CLI End-to-End Testing & Validation Report | ✅ Completed |
| E.2 | MCP End-to-End Testing & Validation Report | ✅ Completed |
| E.3 | Svelte UI End-to-End Testing & Validation Report | ✅ Completed |
| E.4 | Overall System Performance & Latency Benchmarking | ✅ Completed |

### Phase 3.1.7 — iOS Target
| Task | What | Status |
|---|---|---|
| 3.1.7 | iOS target (Requires Mac with Xcode) | Pending |

---

### Phase 4.F — PDF Viewer, Annotations & Native Menu *(Pre-Release Blocker)*

> All tasks here are required before release. Without a viewer and annotations, PaperPilot is not a complete PDF product.

#### 4.F.1 — Embedded PDF Viewer
| Task | What | Status |
|---|---|---|
| F.1.1 | Integrate `pdfjs-dist` into Svelte app | 🚧 In Progress |
| F.1.2 | `PdfViewer.svelte` component (canvas render, scroll, zoom) | 🚧 In Progress |
| F.1.3 | Page thumbnail strip for navigation | Pending |
| F.1.4 | Document info panel | Pending |
| F.1.5 | Viewer ↔ DropZone integration | Pending |

#### 4.F.2 — Annotations
| Task | What | Status |
|---|---|---|
| F.2.1 | Highlight tool (multi-colour) | Pending |
| F.2.2 | Underline & Strikethrough | Pending |
| F.2.3 | Sticky note / Comment | Pending |
| F.2.4 | Free-draw (pen) tool | Pending |
| F.2.5 | Annotation panel (sidebar) | Pending |
| F.2.6 | Save annotations to PDF | Pending |
| F.2.7 | Annotation unit tests | Pending |

#### 4.F.3 — Form Filling
| Task | What | Status |
|---|---|---|
| F.3.1 | Detect AcroForm fields via pdfjs | Pending |
| F.3.2 | Render editable overlays on fields | Pending |
| F.3.3 | Save filled form to PDF | Pending |
| F.3.4 | Form fill E2E test | Pending |

#### 4.F.4 — Native OS Menu Bar
| Task | What | Status |
|---|---|---|
| F.4.1 | Tauri `Menu` setup (File, Edit, View, Window, Help) | 🚧 In Progress |
| F.4.2–F.4.7 | All menus + keyboard shortcuts wired | 🚧 In Progress |

---

### Phase 4.E2E Round 2 — Full System Re-Validation (Post Phase 4.F)
| Task | What | Status |
|---|---|---|
| R2.E1 | CLI E2E Re-Validation (with new annotation/form commands) | Pending |
| R2.E2 | MCP E2E Re-Validation (with `pdf_annotate`, `pdf_form_fill`) | Pending |
| R2.E3 | UI Playwright Re-Validation (viewer + annotations + menu) | Pending |
| R2.E4 | System Performance Re-Benchmark (includes viewer render latency) | Pending |

## 📅 Timeline
- **Week 1:** Implement the Free Tier Offline NLP classification engine (Tasks 4.0.1 - 4.0.10).
- **Week 2:** Implement AI PDF Creation tools and integrate them into the Desktop UI (Tasks 4.6.1 - 4.6.6).
- **Week 3:** Finalize Offline OCR and product deployment builds (Phase 5 & 6).
