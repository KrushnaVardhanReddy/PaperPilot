# Feedback & Issue Reporting System

PaperPilot features a comprehensive, direct-to-GitHub feedback and issue reporting system integrated into both the native **Desktop Application** (`apps/desktop`) and the **Web Portal** (`apps/web` / `usepaperpilot.com`).

---

## 1. Design Philosophy: Zero-Interrupt & Privacy Preserving

1. **Passive & Opt-In**: PaperPilot strictly prohibits intrusive popups, ratings nag screens, or unexpected interruptions. The modal only opens when explicitly requested by the user.
2. **Local Privacy**: No telemetry, analytics, or background reporting beacons are transmitted to remote servers. All reports are formatted into standard GitHub Issue URLs pre-filled in your browser, allowing full user review before submission.
3. **Praise & Testimonials**: Positive feedback and shoutouts are hosted transparently in **GitHub Discussions (Show & Tell)** rather than walled-garden app store reviews.

---

## 2. Issue Categorization

### 1. Problems / Failures
- 🐞 **Functional Bug / Crash**: Tool execution failed, corrupted output stream, or unhandled exception.
- ⚡ **Performance / Slowdown / Memory**: Excessive processing duration on large documents, memory spikes, or interface freezes.
- 🖥️ **UI / Layout Glitch**: Button misalignment, theme rendering glitches, or canvas resizing artifacts.

### 2. Feature & Enhancement Requests
- 💡 **New Tool Request**: Suggestions for capabilities not yet present in the 101 core suite (e.g. Bates numbering, CAD DWG to PDF).
- 🚀 **Enhance Existing Tool**: Added parameter flags, threshold adjustments, or quality presets.
- 💬 **General Feedback / UX Improvement**: General developer experience or pipeline ideas.

### 3. Community Showcase & Support
- ⭐ **Praise & Testimonial**: Share user stories and support the project on GitHub Discussions.

---

## 3. UI Entry Points

| Surface | Entry Point | Trigger Action |
| :--- | :--- | :--- |
| **Desktop App** | Window Titlebar | "💬 Feedback" button |
| **Desktop App** | Sidebar Footer | "💬" icon |
| **Desktop App** | Error Toasts | "🐞 Report Issue" button (automatically captures error message & stack trace) |
| **Web Portal** | Header Navigation | "💬 Feedback & Bugs" button |
| **Web Portal** | Footer Links | "💬 Feedback & Bugs" link |

---

## 4. URL Pre-fill Schema

Reports automatically populate GitHub issue parameters:

- `template`: Specifies `bug_report.yml`, `performance_issue.yml`, or `feature_request.yml`.
- `title`: Formatted with prefixes (`[BUG]: ...`, `[PERF]: ...`, `[FEATURE]: ...`).
- `subcategory`: Selected classification.
- `surface`: Client platform (`Desktop App (Tauri)` vs `Browser WASM (usepaperpilot.com)`).
- `tool`: Targeted operation identifier (e.g. `pdf_split`, `pdf_edit_text`).
- `logs`: Sanitized error stack traces and terminal output.
- `file_size`, `processing_time`, `memory_usage`: Benchmark metrics for performance bottlenecks.
- `version`: Engine version (`0.8.2`).
- `environment`: Operating system platform and architecture.
