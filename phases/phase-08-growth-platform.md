## Phase 8 — Growth Platform

**Goal:** Transform PaperPilot from a standalone tool into a community-driven ecosystem with a plugin registry, workflow builder, watch folder daemon, and SDK libraries.

**[BLOCKED BY Phase 3 — needs working desktop UI]**
**[Can run in parallel with Phase 6 and Phase 7]**

---

### 8.1 — Plugin System `[PARALLEL within group]`

| # | Task | Notes |
|---|---|---|
| 8.1.1 | Define `PdfPlugin` trait | Plugins are just `PdfOperation` implementors with metadata |
| 8.1.2 | Plugin discovery & loading | Load `.so`/`.dylib`/`.dll` plugins at runtime from `~/.paperpilot/plugins/` |
| 8.1.3 | Plugin registry website | Searchable public registry of community plugins |
| 8.1.4 | Plugin packaging CLI | `paperpilot plugin pack` and `paperpilot plugin publish` commands |
| 8.1.5 | Plugin sandboxing | Restrict filesystem and network access per plugin policy |

---

### 8.2 — Pipeline Orchestrator `[BLOCKED BY Phase 3, Phase 8.3]`

**Goal:** Let users define multi-step PDF processing pipelines (recipes) that run automatically, triggered by file system events, webhooks, or schedules. This is the feature that replaces expensive Enterprise Content Management (ECM) platforms.

---

#### 🏗️ Pipeline Format Decision (2026-10-01) — `.ppflow` TOML-first

> **Reference:** Stirling PDF pipeline docs were reviewed at https://docs.stirlingpdf.com/Configuration/Automation/Pipeline/

**Decision: Primary format is `.ppflow` (TOML). Secondary format is `.ppflow.json` (JSON) for machine exchange.**

**Why NOT pure JSON like Stirling:**
- Stirling's JSON pipeline format is a serialized batch of HTTP calls — operations are literal REST endpoint paths (`/api/v1/misc/compress-pdf`). Tightly coupled to their Java/Spring web server.
- JSON has no comment support — you can't annotate why a stage exists, reference a ticket number, or leave a warning.
- Stirling has **silent failure modes** — invalid ops return `200 OK` with an empty body; errors disappear into logs.
- Stirling has **no real branching** — filters silently drop files; there's no IF/ELSE. No retry. No per-stage error handling.
- Stirling has **two incompatible JSON formats** (UI format vs folder-scan format) — causes user confusion.

**Why TOML for authoring:**
| | Stirling (JSON) | PaperPilot (TOML) |
|---|---|---|
| Comments | ❌ | ✅ `# Process invoices nightly` |
| Multi-line strings | ❌ | ✅ Triple-quote `"""` blocks |
| Readability | Verbose | Clean key=value sections |
| Git diffs | Noisy | One value per line |
| Env var injection | ❌ | ✅ `"${env:MY_SECRET}"` |
| Machine generation | Easy | Easy |

**Why JSON as the export/API format:**
- REST API request body, MCP tool args, CLI `--json` flag all emit/accept JSON.
- Desktop "Export Recipe" button produces `.ppflow.json` (importable by other tools).
- CLI and Desktop accept **both formats transparently** at runtime.

**Automation Interface Strategy: CLI vs. API vs. MCP (Both CLI and API are supported):**
| Interface | Target Audience | Tier / Deployment | Primary Use Cases |
|---|---|---|---|
| **CLI (`paperpilot run ...`)** | Developers, Sysadmins, DevOps, Power Desktop Users | **Free / Community** (Local binary) | Shell scripts, cron jobs, local folder watchers, zero-network CI/CD pipelines, offline workstation automation |
| **REST API (`POST /pipelines/...`)** | Enterprise Teams, SaaS Integrations, Cloud Microservices | **Pro / Enterprise** (Headless Docker/Server) | Low-code orchestrators (n8n, Zapier, Make), batch cloud S3/GCS processing, webhook callbacks, multi-tenant scheduling |
| **MCP Tool (`pdf_run_pipeline`)** | AI Agents & AI Engineers | **Pro / Enterprise** (Desktop & Headless Server) | Autonomous LLM agents (Claude Desktop, Cursor, LangChain) executing entire multi-step recipes in a single atomic invocation |

*Architecture Note:* CLI, REST API, Desktop UI, and MCP all act as thin presentation layers dispatching to the same unified `paperpilot-core` pipeline runner and `PdfOperation` trait implementations.

**Trigger support matrix (vs Stirling):**
| Trigger | Stirling | PaperPilot |
|---|---|---|
| Manual (UI) | ✅ | ✅ Desktop GUI canvas |
| CLI file arg | Via REST only | ✅ `paperpilot run invoice.ppflow input.pdf` |
| Watch folder | ✅ | ✅ `type = "watch_folder"` |
| Cron schedule | ❌ | ✅ `type = "schedule", cron = "0 2 * * 1"` |
| Webhook | ❌ | ✅ `type = "webhook"` (Phase 8.2b) |
| MCP agent call | ❌ | ✅ `pdf_run_pipeline` MCP tool (Phase 5.5.4) |

**Reference `.ppflow` schema example:**
```toml
# invoice-processing.ppflow
name        = "Invoice Processing"
description = "Monthly AR invoice batch: OCR → watermark → encrypt → archive"
version     = "1"

[trigger]
type        = "watch_folder"
path        = "/mnt/invoices/incoming"
extensions  = ["pdf"]

[output]
dir         = "/mnt/invoices/processed/{date}"
filename    = "{stem}-processed-{date}"
on_conflict = "rename"   # "overwrite" | "rename" | "skip"

[[stage]]
id          = "ocr"
op          = "ocr"
[stage.params]
languages   = ["eng"]
skip_text   = true

[[stage]]
id          = "filter_large"
op          = "filter_page_count"
condition   = { comparator = "Greater", value = 3 }
on_no_match = "archive_original"   # "skip" | "archive_original" | "error"

[[stage]]
id          = "watermark"
op          = "watermark"
depends_on  = ["ocr"]            # DAG dependency — not just linear serial
[stage.params]
text        = "PROCESSED {date}"
opacity     = 0.4

[[stage]]
id          = "compress"
op          = "compress"
retry       = 2                   # per-stage retry on failure
[stage.params]
level       = "medium"

[[stage]]
id          = "encrypt"
op          = "encrypt"
[stage.params]
password    = "${env:INVOICE_PDF_PASSWORD}"  # env var injection — no hardcoded secrets
permissions = { print = true, edit = false }
```

**Key advantages over Stirling's JSON:**
1. `depends_on` — proper DAG dependency model; stages can depend on specific prior stages (not just serial)
2. `on_no_match` — explicit, named behavior when filters don't match (not silent empty response)
3. `retry = N` — per-stage retry count
4. `${env:VAR}` — environment variable injection for secrets
5. `[trigger]` section — first-class typed trigger config (folder, schedule, webhook, MCP)
6. `[output]` section — named output routing and file naming with template variables
7. Comments — every stage can explain *why* it exists, not just *what* it does

---

#### 8.2a — Pipeline Engine (Core)

| # | Task | Notes |
|---|---|---|
| 8.2.1 | `.ppflow` pipeline schema (runtime subset implemented) | Version-1 name/id, stages, params, conditions, single dependencies and no-match behavior are implemented under Spec 136. Trigger/output/retry/secret fields remain planned and are rejected. **Full planned schema: TOML primary, JSON secondary** — see design decision above. Fields: `name`, `version`, `[trigger]`, `[output]`, `[[stage]]` with `op`, `depends_on`, `condition`, `on_no_match`, `retry`, `[stage.params]`. `${env:VAR}` injection for secrets. |
| 8.2.2 | CLI pipeline execution (implemented, pending review) | `paperpilot run invoice.ppflow input.pdf --output out.pdf` — accepts TOML or JSON and preserves existing `workflow run` syntax; `--json` includes per-stage outcomes. |
| 8.2.3 | Conditional branching engine `[Spec 136]` | Implemented, pending review: per-stage scan/page-count predicates, complementary tests against a shared dependency, explicit `skip` / `archive_original` / `error`, and detailed outcomes in the shared runner. Desktop paired IF/ELSE nodes remain Task 3.4.11; multi-artifact joins remain separate. |
| 8.2.4 | Stage retry & error handling | Per-stage `retry = N`, quarantine folder on final failure, optional Slack/webhook failure alert |
| 8.2.5 | Pipeline sharing & community library | Export/import `.ppflow` files; public community registry of pipeline templates |
| 8.2.6 | Enterprise private pipelines | Org-scoped pipeline libraries, access controlled via RBAC |
| 8.2.7 | Pipeline audit trail | Every execution logged: which file, which stage, result, duration, who triggered it |
| 8.2.8 | Per-stage integrity hashing | SHA-256 the file before and after every pipeline stage; mismatch = automatic abort + alert |
| 8.2.9 | E2E Orchestrator tests | Run real pipelines against test PDF corpus end-to-end (No Mocking) |

---

#### 8.2b — Orchestrator API `[BLOCKED BY 8.2a, ENTERPRISE FEATURE]`

**Goal:** Enterprises can define, trigger, and monitor pipelines programmatically via a REST/WebSocket API — no desktop app required. This is how large-scale automated document processing works (e.g. processing 50,000 PDFs a night from S3).

| # | Task | Notes |
|---|---|---|
| 8.2b.1 | REST API for pipeline management | CRUD endpoints: create, update, delete, list pipelines |
| 8.2b.2 | Pipeline trigger API | `POST /pipelines/{id}/run` with file payload or S3/GCS URL |
| 8.2b.3 | Real-time execution status (WebSocket) | Stream stage-by-stage progress to caller in real-time |
| 8.2b.4 | Pipeline execution history API | Query past runs, filter by status/date/file |
| 8.2b.5 | Webhook callbacks | Fire a webhook at each stage completion or final success/failure |
| 8.2b.6 | S3 / GCS / Azure Blob input sources | Pull input files directly from cloud storage buckets |
| 8.2b.7 | Output routing | Route processed files back to S3/GCS/local/SFTP |
| 8.2b.8 | API key management | Issue, rotate, and revoke API keys per org/team |
| 8.2b.9 | Rate limiting & quotas | Per-org throttling to prevent abuse on cloud tier |
| 8.2b.10 | Load testing & concurrency profiling | Benchmark orchestrator with 10,000+ concurrent pipeline runs to guarantee zero deadlocks and predictable memory usage |

---

### 8.3 — Watch Folder Daemon `[BLOCKED BY 1.5 CLI]`

| # | Task | Notes |
|---|---|---|
| 8.3.1 | `paperpilot watch` command ✅ | Merged (PR #173, Session 10451471310659356299) — Monitor directory for new files using pure-Rust `notify` crate, with settling delay, recipe execution, and webhook notifications (`reports/WATCH_FOLDER_DAEMON_REPORT.md`, `wiki/27-Watch-Folder-Daemon.md`) |
| 8.3.2 | Auto-apply recipe on new files | Trigger a recipe automatically on each new file |
| 8.3.3 | Daemon mode (`--daemon`) | Run as background service / system daemon |
| 8.3.4 | systemd / launchd / Windows Service support | Native OS service integration |
| 8.3.5 | Watch folder + webhook | Fire webhook on each processed file |

---

### 8.4 — SDK Libraries `[BLOCKED BY Phase 2 MCP]`

#### 8.4a — Core SDKs

| # | Task | Notes |
|---|---|---|
| 8.4.1 | Python SDK (`paperpilot-py`) ✅ | Merged (PR #175, Session 11696627093046789817) — PyPI package in `packages/paperpilot-py`: pythonic wrappers, async support (`asyncio`), type hints, Pydantic models (14/14 tests pass) (`reports/SDK_LIBRARIES_REPORT.md`, `wiki/28-Universal-SDKs.md`) |
| 8.4.2 | JavaScript / Node SDK (`paperpilot-js`) ✅ | Merged (PR #175, Session 11696627093046789817) — npm package in `packages/paperpilot-js`: TypeScript SDK with local CLI and remote REST execution (8/8 Vitest pass) |
| 8.4.3 | Go SDK (`paperpilot-go`) | Go module, includes `VerifyWebhookSignature()` crypto utility |
| 8.4.4 | SDK documentation site | Auto-generated API docs for all SDKs |

---

#### 8.4b — AI Framework Integrations `[BLOCKED BY 8.4a]`

**Goal:** AI engineers should be able to drop PaperPilot into their existing agent pipelines in under 5 minutes. Target the most popular Python AI frameworks used in production today.

| # | Task | Notes |
|---|---|---|
| 8.4b.1 | LangChain Tools integration | Ship `paperpilot-langchain` — a set of `BaseTool` subclasses (e.g. `MergePdfTool`, `ExtractTextTool`) that plug directly into any LangChain agent or chain |
| 8.4b.2 | LlamaIndex Reader + Tool integration | Ship `paperpilot-llamaindex` — `PaperPilotReader` (load/parse PDFs as LlamaIndex Documents) and `PaperPilotToolSpec` (expose operations as LlamaIndex tools) |
| 8.4b.3 | CrewAI Tool integration | Ship `paperpilot-crewai` — PaperPilot operations as CrewAI `BaseTool` instances, ready for multi-agent task delegation |
| 8.4b.4 | LangGraph node integration | Ship `paperpilot-langgraph` — PaperPilot operations as stateful LangGraph nodes with typed `State` schemas |
| 8.4b.5 | AutoGen integration | Tool wrappers for Microsoft AutoGen multi-agent framework |
| 8.4b.6 | Haystack integration | `PaperPilotConverter` component for the Haystack document AI pipeline |
| 8.4b.7 | AI framework integration tests | End-to-end tests: spawn a real LangChain agent, have it call PaperPilot tools on real PDFs, verify outputs (No Mocking) |

---

### 8.5 — No-Code Integrations `[BLOCKED BY 8.4]`

| # | Task | Notes |
|---|---|---|
| 8.5.1 | Zapier integration | Official PaperPilot app in Zapier marketplace |
| 8.5.2 | Make.com (Integromat) integration | Official module |
| 8.5.3 | n8n community node | Self-hostable automation integration |
| 8.5.4 | Enterprise Workload Automation | Official guides & plugins for Control-M, JAMS, and AutoSys (using REST + Webhook callbacks) |

---

### 8.6 — Contributor Revenue Sharing System `[BLOCKED BY Phase 7]`

| # | Task | Notes |
|---|---|---|
| 8.6.1 | Contribution scoring algorithm | Weight PRs, reviews, docs, plugin publishes |
| 8.6.2 | Contributor registration portal | GitHub OAuth + payment info (Stripe / Wise) + tax form |
| 8.6.3 | Quarterly automated payouts | Calculate scores, split pool, pay out via Stripe Connect |
| 8.6.4 | Public transparency dashboard | Show pool size, scores, and payout amounts publicly |
| 8.6.5 | Plugin usage attribution | Track plugin downloads by Enterprise customers → plugin author earns |

---

### 8.7 — Document Intelligence Dashboard `[BLOCKED BY Phase 6]`

**Goal:** Give Enterprise admins a full-picture view of how PaperPilot is being used across their org.

| # | Task | Notes |
|---|---|---|
| 8.7.1 | Job history log | Searchable, filterable table of all operations (who, what, when, result) |
| 8.7.2 | Operation analytics | Bar/line charts: most-used operations, busiest hours, file sizes |
| 8.7.3 | Error rate tracking | Failure heatmaps, top error types, most problematic file types |
| 8.7.4 | User activity reports | Per-user operation volume (for compliance audits) |
| 8.7.5 | Dashboard export | Export reports to PDF or CSV on a schedule |

---

### 8.8 — Browser Extension `[BLOCKED BY Phase 2 MCP]`

**Goal:** Capture PDFs directly from the web browser and send them to PaperPilot for processing.

| # | Task | Notes |
|---|---|---|
| 8.8.1 | Chrome / Edge extension | Right-click any PDF link → "Open in PaperPilot" |
| 8.8.2 | Firefox extension | Same functionality |
| 8.8.3 | Deep link to desktop app | Extension calls `paperpilot://open?url=...` to launch the desktop app |
| 8.8.4 | "Save & Process" flow | Download PDF from web → apply recipe → save locally |

---

### 8.9 — Mobile App `[BLOCKED BY Phase 3 iOS/Android targets]`

**Goal:** Full-featured iOS and Android PaperPilot app built from the same Tauri 2.0 + Svelte codebase as the desktop app — not a separate project. All PDF processing runs locally on-device via the same `paperpilot-core` Rust library.

| # | Task | Notes |
|---|---|---|
| 8.9.1 | Ship iOS app to App Store | Archive + submit via Xcode with `cargo tauri ios build` |
| 8.9.2 | Ship Android app to Play Store | Sign + submit APK/AAB via `cargo tauri android build` |
| 8.9.3 | Push notifications | Get notified when a long job completes or fails |
| 8.9.4 | Document approval flow | Review and approve or reject a signed PDF from mobile |
| 8.9.5 | Quick scan → PDF | Capture a physical document with camera and send to PaperPilot for OCR |
| 8.9.6 | Mobile-specific gestures | Pinch-to-zoom on preview, swipe to delete pages |
| 8.9.7 | Share sheet integration | iOS Share Sheet / Android Intent support to open PDFs directly from other apps |
| 8.9.8 | E2E Mobile tests | iOS Simulator + Android Emulator automated tests via Appium or Detox (No Mocking) |

---

### 8.10 — Audio & Multimedia Generation `[BLOCKED BY Phase 5]`

**Goal:** Break PDFs out of their static format into dynamic multimedia — following the emerging trend of AI-generated audio summaries and presentation generation from documents.

| # | Task | Notes |
|---|---|---|
| 8.10.1 | PDF → Audio summary | Generate an AI-voiced conversational audio summary (MP3/WAV) from a PDF using a pluggable TTS engine (ElevenLabs / Kokoro / local TTS) |
| 8.10.2 | PDF → Podcast format | Two-voice AI conversational podcast generated from a PDF document (NotebookLM-style) |
| 8.10.3 | PDF → Presentation (AI-structured) | AI maps the structural outline of a PDF and generates a professionally structured PPTX slide deck — not a raw conversion, but an AI-authored presentation |
| 8.10.4 | Audio summary MCP tool | `pdf_audio_summary` MCP tool so AI agents can generate audio from any PDF |
| 8.10.5 | Audio summary CLI command | `paperpilot audio-summary <file> --voice kokoro --output summary.mp3` |
| 8.10.6 | E2E Multimedia tests | Generate real audio and presentation outputs from test PDFs and verify output quality (No Mocking) |

---

### 8.11 — Physical Document Ingestion `[BLOCKED BY Phase 5.1 OCR, PARALLEL with 8.9]`

**Goal:** Make PaperPilot the automatic destination for every scan that leaves a physical printer or phone camera — without requiring the user to change their existing scan workflow. Every scan that touches PaperPilot immediately becomes a first-class `.ppdoc` document: deskewed, compressed (DjVu 3-layer, Phase 1.3.50–1.3.56), OCR'd, and searchable.

**Exit condition:** A user can walk to any network printer, select "Scan to Computer → PaperPilot" on the printer's touchscreen, and the scanned document arrives as a fully processed `.ppdoc` on their desktop with OCR and AI embeddings — without installing any printer driver.

---

| # | Task | Notes |
|---|---|---|
| 8.11.1 | Hot folder scan-ingest recipe | Extend Phase 4.7.1 watcher to detect `.pdf`/`.jpg`/`.png` drops and auto-run `scan-ingest` recipe: deskew → smart compress (1.3.55) → OCR (Phase 5.1) → save `.ppdoc`. No user action needed after initial setup. |
| 8.11.2 | eSCL/AirScan local server | Implement an eSCL (Mopria/AirScan) HTTP server that advertises itself on mDNS/Bonjour as `PaperPilot on <hostname>`. Any modern network printer's "Scan to Computer" menu discovers and shows it as a destination. Scanner streams raw TIFF/JPEG directly to PaperPilot over HTTP — no driver install, zero user config beyond first enable. Works with every eSCL-compliant printer (HP, Epson, Canon, Brother, Ricoh all support eSCL). |
| 8.11.3 | Virtual CUPS printer (Linux/macOS) | Register PaperPilot as a CUPS print queue with a custom PPD. When a scanner's "Print to PDF" or "Scan to Application" sends a PostScript/PWG Raster job to this queue, PaperPilot intercepts the stream and converts to `.ppdoc`. Replaces the role of PDF24 / PDFCreator but adds OCR and full PaperPilot processing. |
| 8.11.4 | Windows WIA virtual scanner | On Windows, register a WIA (Windows Image Acquisition) minidriver so PaperPilot appears in "Windows Fax and Scan" and any WIA-aware scanner application as a scan destination. The WIA driver returns processed `.ppdoc` data rather than raw TIFF. |
| 8.11.5 | Mobile camera scan (native) | In the PaperPilot mobile app (8.9), expose a native document camera scanner with: real-time perspective correction guide overlay, auto-capture on document detection, multi-page session (keep scanning until user taps "Done"), and immediate DjVu-style compression + OCR pipeline on-device. |
| 8.11.6 | Mobile Share Extension (any scanner app) | Register a system Share Extension on iOS and an Intent filter on Android. User scans in **any** third-party scanner app (Apple Continuity Camera, Microsoft Lens, Adobe Scan) and taps Share → PaperPilot. The JPEG/PDF is ingested, processed, and saved as `.ppdoc`. No PaperPilot scanner UI required. |
| 8.11.7 | Scan-to-email receiver | Run a local SMTP listener (bound to `localhost:2525`). User configures their printer's "Scan to Email" to send to `scan@paperpilot.local` (DNS resolved via mDNS). PaperPilot receives the email, extracts the PDF/JPEG attachment, and runs the scan-ingest pipeline. Works with every enterprise MFP that supports scan-to-email. |
| 8.11.8 | Print-with-return-address (steganographic fingerprint) | When PaperPilot **prints** a `.ppdoc` document, embed two invisible markers in the printed output: (a) a 2×2 mm corner micro-QR code encoding `{doc_id, version_hash, ppdoc_server}` and (b) a DCT-domain steganographic watermark that survives the print→scan round-trip. When the physical document is later scanned back into PaperPilot, it detects the markers and: links the scan to the original `.ppdoc` source; overlays hand-written annotations as a new annotation layer on the original editable document; shows a diff "Original vs. annotated physical copy." Enables paper-based annotation workflows to flow back into the digital document. |
| 8.11.9 | Round-trip annotation merge UI | Desktop UI for reviewing a scanned-back document: side-by-side view of original `.ppdoc` vs. inbound scan; detected handwritten annotations shown as a new `annotation` layer; one-click "Accept all" merges annotations into the canonical document. |
| 8.11.10 | E2E Physical Ingestion tests | Automated tests: spin up a mock eSCL HTTP client, POST a test TIFF scan, verify PaperPilot produces a valid `.ppdoc` with OCR text and DjVu compression applied. Print-with-return-address: embed watermark, simulate scan (Gaussian blur + noise filter to simulate print/scan degradation), verify fingerprint is recovered and doc_id decoded correctly. (No Mocking of the compression or OCR pipeline.) |

