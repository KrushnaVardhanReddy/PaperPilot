# PaperPilot v1.0 Launch Plan

The goal is to complete all of these base version tasks by tomorrow, followed by a one-week deep testing and bug-fixing period.

## 🔲 Pending Base Version Tasks

### Phase 1 (Core + CLI) — 4 tasks left
| Task | What | Status |
|---|---|---|
| 1.1.4 | Collect 20-30 real-world test fixture PDFs (scanned, malformed, encrypted) | Pending |
| 1.4.4 | Property-based tests with `proptest` for page range invariants | 🚧 In Progress |
| 1.4.5 | Error handling tests (malformed PDFs, wrong passwords, empty inputs) | 🚧 In Progress |
| 1.4.6 | PDF Validation layer (pre-flight checks before every operation) | 🚧 In Progress |

> *Note: CLI security features (1.5.18–1.5.24) are currently in progress with Jules.*

### Phase 2 (MCP Server) — 1 tool left
| Task | What | Status |
|---|---|---|
| 2.2.13 | `pdf_sign` tool (DocuSign / local signatures) | Pending |

> *Note: MCP tool wiring for Groups B & C (2.4.1–2.4.13) is currently in progress with Jules.*

### Phase 3 (Desktop UI) — 6 tasks left
| Task | What | Status |
|---|---|---|
| 3.1.7 | iOS target (needs Mac with Xcode) | Pending |
| 3.5.3 | Progress bar component (job name, %, current page) | 🚧 In Progress |
| 3.5.4 | Job cancel support (cancel button → Rust cancellation) | 🚧 In Progress |
| 3.5.6 | Responsive mobile layout | Pending |
| 3.4.10 | **Visual Pipeline Builder** (the big feature) | Pending |
| 3.6.1–3.6.4 | Portable builds (`.exe`, `AppImage`, `.dmg`, MSI) | 🚧 In Progress (3.6.1-3.6.3) |

## 📅 Timeline
- **Today/Tomorrow:** Complete all tasks above.
- **Next 7 Days:** In-depth manual and automated testing, edge-case validation, and bug fixing.
