# Spec 176: In-App GitHub Feedback & Issue Reporting System

## Metadata
- **Spec Number**: 176
- **Title**: In-App GitHub Feedback & Issue Reporting System
- **Phase**: Document Workflows, Developer Experience & Community Transparency
- **Status**: ✅ Completed
- **Impacted Systems**:
  - `apps/desktop`: Svelte 5 FeedbackModal, titlebar feedback entry, sidebar trigger, error toast action handler
  - `apps/web`: Web studio header feedback trigger, footer feedback action, Svelte 5 FeedbackModal
  - `.github/ISSUE_TEMPLATE`: `bug_report.yml`, `feature_request.yml`, `performance_issue.yml`
  - Documentation: `wiki/Feedback-And-Issue-Reporting.md`

---

## 1. Executive Summary & Problem Statement

Users of both the PaperPilot Desktop application (`apps/desktop`) and the zero-install Web Portal (`apps/web` / `usepaperpilot.com`) need a fast, non-intrusive, and transparent way to report functional bugs, performance slowdowns, UI glitches, and feature enhancements.

Previously, users encountering failures (such as parsing encrypted PDFs, memory spikes on large scanned documents, or missing niche tools like Bates numbering) had to manually navigate to GitHub, locate the repository, and figure out the issue templates without environment context.

**Spec 176** implements a comprehensive, direct-to-GitHub feedback and issue reporting architecture:
1. **Zero-Popup / Passive UX**: No intrusive popups, banners, or interruptions during user workflows.
2. **Clear Categorization**:
   - 🐞 **Functional Bug / Crash**: Operation failed, corrupted output, unhandled exceptions.
   - ⚡ **Performance / Slowdown / Memory**: Long processing times, RAM spikes, UI freezes.
   - 🖥️ **UI / Layout Glitch**: Theme mismatches, misaligned buttons, responsive issues.
   - 💡 **New Tool Request**: Proposing tools not yet in the 101 suite.
   - 🚀 **Enhance Existing Tool**: Adding options or flags to existing tools.
   - 💬 **General Feedback / UX Improvement**: Workflow ideas and feedback.
   - ⭐ **Praise & Testimonials**: Opt-in passive support directing to GitHub Discussions (Show & Tell).
3. **Structured GitHub Pre-fill**: Encodes title, template, affected tool, reproduction steps, sanitized error logs, and environment metadata (client type, OS, and version `0.8.2`) directly into GitHub Issue URLs.
4. **Seamless Launching**: Uses `@tauri-apps/plugin-opener` in desktop environments and `window.open` in browsers.

---

## 2. Architecture & Data Flow

```
[ User Interaction ]
       │
       ├──► 1. Trigger Entry Point:
       │      • Desktop Titlebar: "💬 Feedback"
       │      • Desktop Sidebar Footer: "💬" icon
       │      • Error Toast: "🐞 Report Issue" (Auto-injects error log)
       │      • Web Portal Header: "💬 Feedback & Bugs"
       │      • Web Portal Footer: "💬 Feedback & Bugs"
       │
       ├──► 2. FeedbackModal.svelte
       │      • Category Tabs (Bug / Performance / Feature / Praise)
       │      • Context Auto-Population (Active Tool, Sanitized Error Trace)
       │      • Privacy-Preserving Environment Discovery (OS, Client Type, v0.8.2)
       │
       └──► 3. URL Synthesis (`buildGitHubIssueUrl`)
              │
              ├──► Bug / Performance / Feature ──► https://github.com/KrushnaVardhanReddy/PaperPilot/issues/new?...
              └──► Praise / Testimonials        ──► https://github.com/KrushnaVardhanReddy/PaperPilot/discussions/new?...
```

---

## 3. GitHub Issue Templates Configuration

Three GitHub Issue Templates are defined in `.github/ISSUE_TEMPLATE/`:
- `bug_report.yml`: Includes dropdown for subcategories (Functional Bug / Crash, Performance / Slowdown / Memory, UI / Layout Glitch), surface selection, affected tool, reproduction steps, expected behavior, logs/stack trace, version, and environment.
- `performance_issue.yml`: Tailored for high-throughput and heavy PDF processing bottlenecks with dedicated fields for file size, page count, observed processing duration, RAM/memory spikes, tool, logs, and hardware specs.
- `feature_request.yml`: Categorized by request type (New Tool Request, Enhance Existing Tool, General Feedback), targeted surfaces, problem statement, proposed solution, and considered alternatives.

---

## 4. Verification & Testing

1. **Unit Tests**:
   - `apps/desktop/src/lib/utils/feedback.test.ts`: Verifies URL serialization for bugs, performance metrics, feature requests, and community discussions.
2. **Build Integrity**:
   - `pnpm --filter web build` passes cleanly.
   - `pnpm --filter desktop build` passes cleanly.
   - `cargo check --workspace -j 4` validates zero workspace regressions.
