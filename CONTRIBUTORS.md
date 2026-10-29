# Contributors & Community Acknowledgements

We are deeply grateful to our open-source contributors and the community members who have guided PaperPilot's design, reported security vulnerabilities, identified UI pain points, and submitted code.

---

### Code Contributors
- **Qiaochu Hu ([@hobostay](https://github.com/hobostay))**:
  - Implemented the in-memory Open-WebUI & Apache Tika PDF ingestion adapter ([PR #273](https://github.com/KrushnaVardhanReddy/PaperPilot/pull/273) / Task 126A).
  - Authored the Open-WebUI integration guide and Python folder bridge tool.
  - Implemented the unified Conditional Pipeline Execution Engine across CLI, Desktop, WASM, MCP, and Gateway (`[Spec 136]` / Task 4.7.6 / 8.2.3), adding runtime document inspections (`if_scanned`, page count boundary comparators), `.ppflow` TOML/JSON parsing, atomic CLI publication, and shared engine architecture.
  - Implemented the desktop Watched Folders daemon and reactive PDF Smart Filters ([PR #306](https://github.com/KrushnaVardhanReddy/PaperPilot/pull/306) / Spec 142 / Task 4.7.8), adding pure-Rust Tauri IPC bindings, bounded concurrency, filesystem event listening, and fact-based document filtering (`isScanned`, `isSigned`, `hasFormFields`, `pageCount > 1`).

---

### Security & Vulnerability Disclosures
- **Hacker News Community Security Reviewers**:
  - **Client-Side Stored XSS**: Identified unescaped search snippet rendering in `WorkspaceSearchModal.svelte` where malicious document titles/content could execute script payloads. Remediated via strict HTML entity escaping and Tauri Content Security Policy (`[Spec 123]` / Task 123A).
  - **Feature Pack Path Traversal**: Disclosed potential directory traversal via crafted `pack_id` in desktop pack commands. Remediated via alphanumeric regex validation (`[Spec 123]` / Task 123A).
  - **SSRF & DNS Rebinding**: Reported risks of automatic HTTP redirect following in `url_fetch.rs`. Remediated by disabling automatic redirects and blocking private/loopback/link-local/CGNAT IP addresses (`[Spec 123]` / Task 123A).
  - **Gateway CORS & Body Limits**: Flagged unrestricted wildcard CORS on `/api/v1/pdf/*` and unbounded request body sizes. Remediated via trusted localhost origins and 100 MiB `DefaultBodyLimit` (`[Spec 123]` / Task 123A).

---

### Community Feature Prompters & UX Feedback
- **[@setop](https://github.com/setop) (GitHub Issue [#266](https://github.com/KrushnaVardhanReddy/PaperPilot/issues/266))**: Reported the distracting webapp motion sickness issue caused by dynamic main pane movement on the web playground, helping us stabilize the hero layout and eliminate jarring viewport displacement.
- **`mkl` (Hacker News)**: Reported mobile UI layout bug on Firefox for Android where tool parameter upload boxes and dialogs were cramped and required awkward vertical/horizontal scrolling. Prompted **Task 124A / Spec 124** (Overhauled `HeroPlayground.svelte` and `OperationsView.svelte` with fluid drawer layouts and >=44px touch targets).
- **`spwa4` (Hacker News)**: Prompted **Task 125A / Spec 125** — Requesting a 100% self-contained, air-gapped `paperpilot-portable.html` distribution with inline base64 WASM for defense and regulated legal environments without local web servers.
- **`Tepix` (Hacker News)**: Prompted **Task 126A / Spec 126** — Requesting an in-memory sub-15ms Apache Tika drop-in replacement for Open-WebUI without JVM or Python runtimes.
- **`Shorel` (Hacker News)**: Prompted **Task 127A / Spec 127** — Requesting a semantic PDF-to-EPUB 3.0 intermediate AST engine with reconstructed headings and reflowable text.
- **`SimonBure` ([SimonBure/presse](https://github.com/SimonBure/presse))**: Inspired **Task 122A / Spec 122** — Architectural inspiration for hash-based indirect stream deduplication and CMYK safety guards in PDF compression.
