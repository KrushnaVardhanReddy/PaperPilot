## Phase 6 — Enterprise

**Goal:** Organizations can deploy and govern PaperPilot at scale.

**Architecture & Repository Boundary:**
> ⚠️ **IMPORTANT ARCHITECTURAL MANDATE:** All Phase 6 enterprise governance tasks (SAML/SSO, SCIM, SIEM Audit, Air-Gapped Ed25519 Licensing, Argon2id DRM) are implemented in the private **`PaperPilot-Enterprise`** downstream repository.
> The enterprise repository imports `paperpilot-core` and `paperpilot-pdf` as upstream Cargo dependencies, ensuring **zero code duplication** and **zero intellectual property leakage**.
> See detailed architectural blueprint: [`docs/architecture/OPEN_CORE_MULTI_REPO_STRATEGY.md`](file:///home/krushna/Project/PaperPilot/docs/architecture/OPEN_CORE_MULTI_REPO_STRATEGY.md).

**Exit condition:** An IT admin can deploy PaperPilot Enterprise, set up SSO, configure AI policies, and review audit logs — without touching source code.

**[BLOCKED BY Phase 4]**
**[Can run in parallel with Phase 5]**

---

### 6.1 — Identity `[PARALLEL within group]`

| # | Task | Notes |
|---|---|---|
| 6.1.1 | Organization model | Multi-org data model |
| 6.1.2 | User and team management | CRUD for users, teams, roles |
| 6.1.3 | RBAC | Role definitions, permission checks |
| 6.1.4 | SSO — SAML 2.0 | |
| 6.1.5 | SSO — OIDC | |
| 6.1.6 | SCIM provisioning | Automated user lifecycle from IdP |

---

### 6.2 — Security and Policy `[PARALLEL with 6.1, BLOCKED BY 6.1.3]`

| # | Task | Notes |
|---|---|---|
| 6.2.1 | AI provider policy engine | Allow/block specific providers per org |
| 6.2.2 | Data processing policies | Local-only, region restrictions |
| 6.2.3 | Document retention policies | Auto-delete rules |
| 6.2.4 | Encryption configuration | At-rest and in-transit options |
| 6.2.5 | Central configuration management | Push config to all instances |

---

### 6.3 — Audit `[PARALLEL with 6.1 and 6.2]`

| # | Task | Notes |
|---|---|---|
| 6.3.1 | Audit event schema | Typed events: who, what, when, result |
| 6.3.2 | Audit log storage | Append-only, tamper-evident |
| 6.3.3 | Audit log UI | Filterable, exportable |
| 6.3.4 | Configurable verbosity | What gets logged vs. what doesn't |

---

### 6.4 — Enterprise Deployment `[PARALLEL with 6.1, 6.2, 6.3]`

| # | Task | Notes |
|---|---|---|
| 6.4.1 | Docker image ✅ | **Completed** — Multi-stage production `Dockerfile` bundling `paperpilot-gateway`, `paperpilot-cli`, `paperpilot-mcp`, and embedded `paperpilot-storage` on Debian Bookworm Slim with zero external compilation dependencies. |
| 6.4.2 | Docker Compose setup ✅ | **Completed** — Production `docker-compose.yml` with healthchecks (`:7823/health`), persistent SQLite/libSQL volume mounts (`/data`), and models persistence (`/models`) documented in `DOCKER.md`. |
| 6.4.3 | Kubernetes manifests | Helm chart or plain manifests |
| 6.4.4 | Air-gapped deployment guide | No external network dependencies |
| 6.4.5 | Upgrade / migration tooling | Safe schema migrations, zero-downtime updates |

---

### 6.5 — License & EULA Enforcement `[PARALLEL with 6.1]`

**Goal:** Prevent unauthorized commercial use by individuals without requiring server calls for the free tier.

| # | Task | Notes |
|---|---|---|
| 6.5.1 | EULA gate at first launch | Show Personal vs. Commercial choice on first run; log acceptance |
| 6.5.2 | License key validation | Lightweight offline-first key check (cryptographic signature, no phone-home required) |
| 6.5.3 | License key backend API | Simple serverless function (Cloudflare Worker) to issue and revoke keys |
| 6.5.4 | Enterprise feature gating | MSI/MDM, SSO, Audit Logs, Group Policy features locked behind valid commercial key |
| 6.5.5 | License dashboard | Admin UI to manage seats, assign/revoke users per organization |
| 6.5.6 | Grace period & offline mode | License valid offline for 30 days before requiring re-validation |
| 6.5.7 | License expiry notifications | Warn admins 30/14/7 days before renewal |
| 6.5.8 | E2E Enterprise integration tests | Automated deployment and SSO workflow testing (No Mocking) |

---

### 6.6 — Enterprise DRM, Anti-Brute-Force & Memory-Burn Encryption `[PARALLEL with 6.2]`

> High-security defense against unauthorized data exfiltration, GPU brute-forcing, and dictionary cracking. Prioritizes zero-server offline cryptographic memory-hardness.

| # | Task | Notes |
|---|---|---|
| 6.6.1 | Pure Offline Argon2id Memory-Burn (Primary Moat) | Configurable memory-hard key derivation (1–2 GB RAM + 2s compute per guess). Neutralizes offline GPU rack brute-forcing (Hashcat) with zero server dependencies or network connectivity |
| 6.6.2 | Exponential Backoff & Local Lockout Enforcement | Enforce progressive delay penalties on repeated incorrect decryption attempts across Desktop, CLI, and REST Gateway |
| 6.6.3 | Coercion / Duress Password & Honeypot Decoy | Secondary password triggers silent decoy mode: opens sanitized mock PDF while quietly wiping sensitive memory/cache |
| 6.6.4 | Optional Enterprise Remote Burn Switch (Key Escrow / KMS) | Optional cloud/VPC policy: Master key $K_{doc}$ stored in Vault/HSM; after $N$ failed verification attempts or revoke signal, purges key rendering copies unreadable |
| 6.6.5 | Exfiltration & Brute-Force Alerting Webhooks | Fire real-time SIEM alerts (PagerDuty, Slack, Syslog) when repeated decryption failures indicate brute-force reconnaissance |

---

### 6.7 — Enterprise Document Engine Parity (Apryse / PDFTron Gap Closure) `[POST-v1.0 ROADMAP]`

> Closes the capability gap with heavy enterprise document SDKs (Apryse / formerly PDFTron) while maintaining PaperPilot's lightweight pure-Rust speed and client-side privacy. (Reference: `docs/COMPETITIVE_GAP_ANALYSIS_APRYSE.md`)

| # | Task | Notes |
|---|---|---|
| 6.7.1 | Interactive PDF Form Engine (AcroForms & XFA) | Interactive HTML5/canvas input overlay over `hayro` mapped to `/AcroForm` and `/Annots` dictionaries in `lopdf`. Read, fill, validate, and serialize field values back to binary without third-party plugins. |
| 6.7.2 | Collaborative SVG Annotation & Markup Layer | Freehand ink drawing with stylus pressure sensitivity, comment threads, callouts, text highlights, and sticky notes. Export/import via Adobe-standard XFDF (XML) and permanent burn-in support. |
| 6.7.3 | Certified Cryptographic Signatures (PAdES / PKCS#7) | X.509 digital certificate signing using Rust cryptographic crates (`rsa`, `ed25519-dalek`, `x509-parser`). Adobe-compliant ByteRange cryptographic hashing, LTV (Long-Term Validation), and TSA timestamping. |
| 6.7.4 | Semantic Glyph & Vector Redaction Engine | Parse `/Contents` streams via `hayro_syntax` and `lopdf` to completely excise intersected glyphs, vector paths, and metadata under redaction bounding boxes (guaranteed zero text extraction leakage). |
| 6.7.5 | Client-Side MS Office Document Viewer (`.docx` / `.xlsx` to PDF) | In-browser pure-Rust OpenXML parser (`docx-rs`) or sandboxed micro-WASM converter enabling direct drag-and-drop conversion of Office files to PDF without Microsoft Office or server dependencies. |
| 6.7.6 | 2D CAD & BIM Blueprint Vector Viewer (`.dxf` / `.dwg`) | Pure-Rust CAD entity parser (`dxf` crate) converting blueprint layers and 2D vector primitives directly into `tiny-skia` paths for high-precision architectural viewing and measurement. |

---

### 6.8 — Enterprise Asynchronous Processing, Webhooks & Embedded Database Engine `[ENTERPRISE]`

> High-throughput enterprise automation, asynchronous execution, and persistent event dispatching for headless and integration workloads.

| # | Task | Notes |
|---|---|---|
| 6.8.1 | **Async Job Engine & HTTP Callbacks** | Per-request `callback_url` with HMAC-SHA256 signature verification for long-running batch operations (OCR, PDF/A conversion, high-compression) over REST Gateway to prevent Cloudflare/Nginx reverse proxy timeouts. |
| 6.8.2 | **Outbound Webhook Event Dispatcher** | Permanent organization-level webhook subscriptions (`diff.completed`, `ocr.finished`, `document.sanitized`, `signature.verified`) with retry queues, exponential backoff, and dead-letter routing to Slack, Teams, or SIEM endpoints. |
| 6.8.3 | **Primary Database Standard: Turso / libSQL Integration** `[Spec 103]` ✅ | **Completed Task 103A (PR #246)** — Standardized on **[Turso / libSQL](https://github.com/tursodatabase/turso)** (`libsql` crate) in dedicated `paperpilot-storage` crate:<br>• **Air-Gapped Local-First:** Runs as an embedded zero-setup local SQLite file in `get_app_data_root()` for Desktop & REST Gateway.<br>• **Enterprise Sync:** Supports 1-line optional cloud/edge replication to a remote Turso cluster for enterprise multi-node deployments.<br>• **Schema:** `jobs` table (async status, retries, output), `webhooks` table (endpoints, events, secrets), `document_chunks` table (vector embeddings), and `audit_logs` table (immutable compliance records). |
| 6.8.4 | **libSQL Vector Store & Semantic Search** `[Spec 103]` ✅ | **Completed Task 103B (PR #247)** — Pure-Rust cosine similarity vector search and document chunk storage (`store_chunks`, `search_chunks`, `delete_chunks_by_document`, `count_chunks_by_document`) directly on embedded `document_chunks` table, with in-memory libSQL integration tests (`wiki/103-Vector-Store-Engine.md`). |
| 6.8.4c | **Desktop Async Jobs Drawer & Vector Search Dock** `[Spec 103]` ✅ | **Completed Task 103C (PR #251)** — Tauri IPC commands for `StorageEngine` (`get_jobs_history`, `get_audit_events`, `search_document_vectors`, `store_document_chunks`), slide-out `<JobQueueDrawer />` for background task monitoring, active background badge counter in `<GlobalCommandBar />`, and semantic vector search toggle in `<WorkspaceSearchModal />`. |
| 6.8.5 | **Web WASM Portal Parity & Categorized Navigation** `[Spec 104]` ✅ | **Completed Task 104A (PR #249)** — Expanded Web Worker bridge (`apps/web/src/lib/wasm/`) to support all newly compiled `paperpilot-wasm` tools with categorized navigation (`pages`, `edit`, `optimize`, `security`, `convert`), search filtering, and interactive parameter controls (`OperationsView.svelte`). |
| 6.8.6 | **Embed Widget Tool Parity & `/embed-test` Sandbox Upgrade** `[Spec 105]` ✅ | **Completed Task 105A (PR #248)** — Expanded drop-in embed widget (`apps/embed`) with categorized embedding (`category` prop), Wave 2 tools (`pdf_nup`, `pdf_booklet`, `pdf_scale_pages`, `pdf_auto_rotate`, `pdf_stamp`, `pdf_overlay_underlay`, `pdf_validate_signatures`, `pdf_remove_signatures`), configuration controls, and updated `/embed-test.html` sandbox with 26/26 Playwright test suite passing. |
| 6.8.7 | **Embed Studio Experience & Portal Parity** `[Spec 106]` ✅ | **Completed Task 106A (PR #253)** — Upgraded `apps/embed` to deliver full macOS/Studio window parity (`● ● ●`, categorized functional tabs, search filtering, 2-column studio layout, active canvas, and parameter controls) with dual layout modes (`studio` vs `compact`), updated `/embed-test.html`, automated build synchronization, and 24/24 Playwright tests passing. |

---

