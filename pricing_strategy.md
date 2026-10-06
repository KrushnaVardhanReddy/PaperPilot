# PaperPilot Pricing Strategy (v2 — Competitive Update)

**Target Market:** Individual developers, power users, SMBs, and Enterprise PDF teams.
**Overhead:** $0.00 compute cost on local processing (our core advantage over every cloud-based tool).
**Goal:** Beat Stirling PDF's usage-based model with a simpler "pay once, run forever locally" message, while building a sustainable B2B revenue stream from teams and enterprise.

> **Key Counter-Positioning vs. Stirling PDF:**
> Stirling charges $0.01 per automation run (their Processor product) — even on self-hosted setups. PaperPilot's **local processing is always free**: every automation, every pipeline run, every batch job on your machine costs $0, forever. Our Cloud API tier offers usage-based pricing for teams that need hosted infrastructure — but that's opt-in, not a tax on local workflows. For developers and power users running hundreds of automations a month on their own hardware, we are dramatically cheaper.

---

## Tier Overview

| Tier | Price | Best For |
|---|---|---|
| **Community** | **Free Forever** | Everyone — individuals, students, open-source developers |
| **Pro (Desktop)** | **$5/month** (or $48/yr) | Power users, freelancers, consultants (unlimited steps, watch folders, BYOK) |
| **Cloud Web Pro (SaaS)**| **$4/month** (or $38/yr) | Zero-install web browser users (iLovePDF alternative, ephemeral RAM compute) |
| **Teams** | **$12/user/month** (or $115/yr) | Small teams, agencies, law firms (2–50 seats, shared recipe library) |
| **Enterprise** | **$22/user/month** (or $211/yr) | 50+ seats, IT-managed, compliance-driven orgs (SAML, SCIM, Air-gap, audit) |
| **Cloud API Packages**| **$0 / $29 / $89 / mo** *(Waitlist / Phase 7)* | Developers, CI/CD, SaaS platforms (1k Free, 5k Dev, 20k Scale; sub-10ms pure-Rust edge) |

> Prices are positioned below Stirling's ecosystem cost (editor free + $0.01/run adds up fast for teams), below Adobe Acrobat ($20/user/month), below iLovePDF ($7/month), and well below typical B2B PDF SaaS ($25–$50/user/month).

---

## Tier 1: Community (Free Forever)

* **Price:** $0 — always, unconditionally.
* **Target:** Individual users, students, open-source developers, self-hosters, and anyone evaluating PaperPilot.
* **Features:**
  - **All 50+ core PDF operations** (Merge, Split, Rotate, Compress, Redact, Encrypt, OCR, Forms, Annotations, Convert, etc.)
  - **Full PDF Viewer** with annotations and form filling
  - **Offline AI Mode** (100% local, zero API key, zero network — powered by embedded NeuML's `bert-hash-nano-embeddings` <1.1MB ONNX model & Aho-Corasick rule engine for instant command resolution)
  - **Basic Pipeline Builder** — up to 3 steps, manual trigger only
  - **MCP Server** (local, single-user, all 40+ tools)
  - **Full CLI** (all commands, JSON output, webhooks)
  - **Unlimited Self-Hosted Users** (Zero user caps — no 5-user paywall)
  - **Standard OIDC / OAuth SSO** (Google, GitHub, Authentik, Keycloak) — free for internal self-hosters with zero "SSO Tax"
  - **Community support** (GitHub Discussions, Discord)
* **Why this is aggressive:** We give away significantly more than Stirling's free tier. Their self-hosted version imposes a strict 5-user cap, paywalls SSO, and meters automation. PaperPilot provides **unlimited self-hosted users, free standard OIDC/OAuth, and 100% free local processing forever**.

---

## Tier 2: Pro

* **Price:** $5/month (~$48/year billed annually)
* **Target:** Freelancers, power users, consultants, solo developers who want advanced automation and AI.
* **Features (everything in Community, plus):**
  - **Unlimited pipeline steps** (Community is capped at 3)
  - **Advanced Pipeline triggers:** Watch Folder daemon mode, Webhook trigger, Cron scheduling
  - **Pro AI Mode:** Full LLM multi-step reasoning — bring your own key (OpenAI, Ollama, Gemini, local llamafile)
  - **LLM-Ready Export:** PDF → RAG-optimized Markdown + structured JSON sidecar
  - **Voice input** for NLP command box
  - **Plugin Registry access:** Install and use community plugins
  - **Webhooks & HMAC signing** for pipeline outputs
  - **1 year of update entitlement** + priority email support (48h response SLA)
* **Why $5:** We charge a small, impulse price ($5/mo — less than a coffee per week) for unlimited background daemons, watch folders, and advanced multi-step LLM reasoning.

---

## Tier 3: Teams

* **Price:** $12/user/month (~$115/user/year billed annually)
* **Minimum:** 2 seats
* **Target:** Agencies, small law firms, accounting teams, startups — 2 to 50 seats.
* **Features (everything in Pro, plus):**
  - **Centralized license portal** ✅ — admin manages seats, assigns roles, revokes access from one dashboard
  - **Concurrent pipeline execution** ✅ — team members run pipelines simultaneously without blocking each other
  - **Priority support** ✅ — 4-hour response SLA (vs. 48h for Pro)
  - **Shared pipeline/recipe library** 🚧 *(Phase 3.4.11 — Q1 2025)* — team recipes stored in a private registry; once a firm standardizes their invoice, contract, or Bates stamping pipeline across staff, switching cost is high
  - **Team audit log** 🚧 *(Phase 6.3)* — searchable job history per user (what ran, when, result)
  - **SDK access** 🚧 *(Phase 4 early access)* — Python and JavaScript SDK libraries for embedding PaperPilot into internal tooling

> **Why Teams over two individual Pro licenses?** At 2 seats you pay $24/month vs $10/month for two Pro licenses — the immediate concrete value is centralized seat management and concurrent pipeline execution. The shared recipe library (shipping Phase 3.4.11) is what makes Teams the obvious permanent home for any firm once it's live: a standardized pipeline your whole team runs is worth far more than each person maintaining their own copy.

* **Why $12:** Centralized administration and concurrent execution are day-one differentiators. The recipe library becomes the retention engine the moment it ships — once a team standardizes their workflows, switching cost is high.

---

## Tier 4: Enterprise (Fleet Governance & Compliance)

* **Price:** $22/user/month (~$211/user/year billed annually)
* **Minimum:** 50 seats; annual contract required.
* **Target:** Hospitals, law firms, financial institutions, defense contractors — organizations governed by strict IT, security, and audit standards.
* **Why Enterprise Pays (We monetize Fleet Governance, NOT the Login Button):**
  - **SCIM 2.0 Automated Lifecycle Provisioning** — Instant Okta / Azure AD automated user on/off-boarding
  - **Enterprise SAML 2.0 Federation** with custom attribute mapping and conditional access policies
  - **MSI / MDM Silent Enterprise Deployments** — Pre-packaged Intune, Jamf, and SCCM deployment manifests
  - **Air-Gap Group Policy Lockdown (GPO / Plist)** — IT-enforced hard policies disabling all external network calls, blocking cloud LLM keys, and locking execution to 100% on-premise
  - **SIEM / Compliance Audit Log Streaming** — Real-time event streaming to Splunk, Datadog, or AWS CloudWatch (tracking every document redact, export, and signature)
  - **Commercial Indemnification & BAA (Business Associate Agreement)** for HIPAA compliance
  - **On-premise offline license validation** (works in air-gapped SCIFs and defense environments)
  - **Dedicated Account Manager & 24/7 SLA with phone support**
* **Why $22:** Large enterprises don't pay for login convenience; they pay for **legal compliance, risk mitigation, and zero data-leakage guarantees**. At $22/user/month, PaperPilot is cheaper than Adobe Acrobat Enterprise while providing 100% on-premise data residency.

---

## 🏢 Enterprise Delivery Models: Managed White-Label Edge vs. Self-Hosted On-Prem

Enterprise customers fall into two distinct buyer categories with completely different infrastructure preferences:

| Attribute | **Managed White-Label Edge (SaaS)** | **Self-Hosted Enterprise License (On-Prem)** |
|---|---|---|
| **Target Buyer** | Mid-market businesses, law firms, CPAs, clinics, agencies wanting their own branded portal without IT ops. | Banks, defense, healthcare giants, strict air-gapped orgs with dedicated SecOps/DevOps teams. |
| **Delivery Model** | **Cloudflare Workers (WASM Isolates)** with SSL for SaaS. | **Docker Container / Standalone Binary** (`paperpilot-gateway` / desktop). |
| **Domain Setup** | Custom enterprise CNAME (e.g. `pdf.firmname.com`). | Internal intranet / VPC URL (e.g. `pdf.internal.bank.com`). |
| **Maintenance** | **Zero Ops**: Cloudflare handles SSL, global CDN, and automated WASM upgrades. Zero disk storage. | **Client Managed**: Installed on customer's AWS/Azure VPC or bare-metal servers. |
| **Compliance** | HIPAA/SOC2 compliance through physical zero-disk V8 Isolates and in-memory execution. | 100% air-gapped network isolation. Zero external network egress. |
| **Pricing** | **$149 – $399 / month** (or $1,500 – $3,900 / year). | **$2,500 – $10,000 / year** (flat per-server or per-seat license). |

### 1. Managed White-Label Edge ($149 – $399/mo)
* **The Value Proposition:** A 30-person law or accounting firm cannot afford a full-time DevOps engineer to babysit a Linux/Docker server, update SSL certificates, and debug crashes. Paying $199/month for a fully managed Cloudflare custom domain (`pdf.smithlegal.com`) is **dramatically cheaper than hiring an IT consultant ($150/hr)**.
* **White-Label Features:**
  - Dynamic CNAME routing (`pdf.customer.com` -> PaperPilot Edge Worker).
  - Client logo, custom favicon, primary brand colors, and custom email notifications.
  - Automated SSL provisioning via Cloudflare Custom Hostnames (SSL for SaaS).
  - Multi-tenant isolated Cloudflare KV configuration for access controls and API key management.
  - Physical zero-disk guarantee: all documents execute in V8 RAM buffers and are immediately wiped.
* **Unit Economics & Margin:**
  - Cloudflare Workers cost: $5/mo base + $2/mo per custom hostname.
  - Cost to serve: ~$7.00/mo per tenant.
  - Price charged: $149 – $399/mo.
  - **Gross Margin: >95%**.

### 2. Self-Hosted Enterprise License ($2,500 – $10,000/yr)
* **The Value Proposition:** Banks, government agencies, and defense contractors whose compliance rules strictly forbid sending document bytes outside their firewall. They pay an annual software license to run our single-binary Rust server or Docker image entirely inside their own AWS/Azure/On-Prem VPC.
* **Deliverables:**
  - Single standalone binary (or lightweight Docker image <25MB).
  - Offline license file activation (zero phone-home requirement).
  - SAML 2.0 / SCIM 2.0 enterprise identity integration.
  - Annual upgrade entitlement, security patches, and 24/7 priority SLA.

---

## Tier 5: PaperPilot Cloud (Web SaaS & Developer API)
 
PaperPilot Cloud unlocks the power of our Rust engine directly in the browser and via cloud endpoints for users who cannot or prefer not to install local binaries.

### 5A. Consumer Web SaaS ("The Modern iLovePDF Alternative")
* **Target:** Chromebooks, iPad/tablet users, students, accountants, and corporate employees whose locked-down machines forbid installing desktop binaries (`.deb`/`.dmg`/`.exe`).
* **Web Free Tier:**
  - Free web processing up to **3 tasks per day**.
  - All 44 core tools accessible via web browser.
* **Cloud Web Pro:** **$4/month** (or **$38/year** billed annually)
  - **Unlimited tasks and batch runs** in any modern web browser.
  - Zero file size restrictions (up to 200MB/file).
  - High-speed cloud OCR (Tesseract / multi-lingual engine).
  - Hybrid Perk: Desktop Pro ($5/mo) subscribers automatically receive Cloud Web Pro for free!
* **The "Zero-Knowledge" Ephemeral Privacy Guarantee:**
  - Files are processed **strictly in-memory (RAM / `/dev/shm`)** on secure edge workers.
  - Documents **never touch persistent hard drives**.
  - Immediate cryptographic wipe: files are purged immediately upon download completion (or strictly 10 minutes post-upload).
  - No logs, no telemetry, no document indexing.

---

### 5B. Cloud API & MCP Gateway ("Stripe / Cloudinary for PDFs")
* **Target:** B2B SaaS platforms, FinTech apps, ERP platforms, and AI Agent builders (Claude, OpenAI Custom GPTs, LangChain) that need reliable programmatic PDF generation.
* **v1.0 Launch Strategy (Safest Rollout):** 
  - For the **v1.0 launch**, the Cloudflare Edge engine (`apps/edge/`) is deployed strictly for **Internal Web Demo support, WooCommerce watermarking webhooks, and public demonstration**.
  - No public unauthenticated write endpoints are exposed without rate limiting.
  - Public commercial usage-based API key management, domain whitelisting, and Stripe metered billing are scheduled for **Phase 7 (Managed Cloud)**. A "Join Developer API Waitlist" CTA captures inbound demand during v1.0.
* **Planned Commercial Pricing (Phase 7 — Strategy 2 Developer Packages):**
  - **Hobby / Free**: **$0/month** (1,000 runs/mo included, hard stop — perfect for evaluation & hackathons).
  - **Developer / Startup**: **$29/month** (5,000 runs/mo included, +$0.008/run overage).
  - **Growth / Scale**: **$89/month** (20,000 runs/mo included, +$0.005/run overage).
  - **Enterprise**: **Custom quote** (100k+ runs, dedicated SLA, custom data residency).
* **Unit Economics & Margin Assurance (Zero Harm Guarantee):**
  - Developer Tier ($29/mo for 5,000 runs): Cloudflare compute cost is **~$0.0025** (a quarter of a cent). Net profit after Stripe fees ($1.14) is **$27.83 (96.0% profit margin)**.
  - Growth Tier ($89/mo for 20,000 runs): Cloudflare compute cost is **~$0.010** (one cent). Net profit after Stripe fees ($2.88) is **$86.11 (96.8% profit margin)**.
  - The business cannot be harmed by high volume because Cloudflare's marginal CPU cost is fractions of a millicent.
* **Dual-Tier Hybrid Cloud Execution Model:**
  - **Tier A (Fast Edge): Cloudflare Workers (Rust Wasm)**:
    - Used for 90%+ of standard operations (merge, split, rotate, compress, stamp, encrypt, metadata).
    - **Physical Zero-Disk Guarantee**: Cloudflare V8 Isolates do not have hard drive access; files are processed in-memory and immediately destroyed.
    - Sub-10ms latency worldwide across 300+ edge locations with 0ms cold starts.
  - **Tier B (Heavy Compute): Ephemeral Kubernetes Sandbox**:
    - Used for resource-heavy workloads (scanned document OCR, multi-hundred-megabyte files, and chained batch pipelines).
    - Spawns isolated Pods on-demand (`emptyDir: { medium: "Memory" }` RAM mount) that self-destruct upon completion.
* **Smart Hybrid Pricing**:
  - Edge Operations (Tier A): Billed at low-cost tier (**$0.005–$0.01/run**).
  - Heavy OCR & Bulk Compute (Tier B): Billed transparently with compute add-on (**$0.02–$0.03/run**).
* **Features:**
  - **Hosted Cloud MCP Server (SSE & HTTP Gateway)**: Plug directly into AI agents to perform real PDF transforms with natural language tools.
  - **REST API Endpoints**: Full parity with `paperpilot-gateway` (`POST /api/v1/pdf/merge`, `POST /api/v1/pdf/compress`, etc.).
  - **Blazing Speed & Low Infrastructure Cost**: Because our engine is pure Rust (sub-10ms execution, <50MB RAM), our server costs are ~50x lower than Java-based Stirling PDF or Python wrappers, allowing high-margin, aggressive pricing.
  - **Developer Dashboard**: Live API key management, rate limits, latency telemetry, and usage metering.

---

## 💡 Plugin Ecosystem Revenue Sharing Model

**Philosophy:** The core PaperPilot engine is built and maintained by the core team, but a thriving ecosystem of community-built plugins is what makes PaperPilot a platform, not just a tool.

### The Pool
- **10% of all paid subscription revenue** is reserved as the Plugin Developer Revenue Pool, distributed quarterly.
- Fully transparent: pool size and top-earning plugins are published publicly on the website every quarter.

### How It Works
Revenue is dynamically routed based on **plugin usage by paying customers**:
1. When a Pro, Teams, or Enterprise user runs a community plugin (e.g., "HIPAA Redactor" or "Invoice Normalizer"), that usage is anonymously tallied.
2. The 10% quarterly pool is divided proportionally among plugin authors by paid-tier usage share.
3. To qualify: plugins must be open-source, undergo a security review, and be published to the PaperPilot Plugin Registry.

### Payout Process
1. Plugin author registers GitHub handle + payment info (Stripe / Wise).
2. End of each quarter: algorithm calculates usage metrics and splits the pool automatically.
3. Contributors submit tax form (W-9 / W-8BEN) before first payout.
4. **Minimum pool threshold:** Distributions only begin when the **quarterly pool exceeds $500**. Below that threshold, amounts roll over to the next quarter and accumulate. This prevents a disappointing early rollout (e.g. a $30 pool split 10 ways) that would kill developer enthusiasm before the ecosystem gains traction. The threshold and current pool balance are published transparently on the website alongside the quarterly report.

### Why This Works
> Free users become plugin developers. Plugins attract more Enterprise customers. Revenue growth increases the pool. A bigger pool attracts more developers. This is a self-reinforcing flywheel.

---

## 🚀 Key Messaging vs. Competitors

### vs. Stirling PDF
> *"Stirling charges $0.01 per automation run on local self-hosted workflows. PaperPilot's local processing is free, forever — no run counter, no billing surprise. For teams running hundreds of document workflows a month on their own machine, that's the difference between a $50 bill and $0. Our Cloud API tier also offers usage-based pricing for teams that need hosted infrastructure, but local processing will never cost you a cent."*

### vs. Adobe Acrobat Pro
> *"Adobe charges $20/user/month and uploads your documents to Adobe's cloud. PaperPilot Enterprise is $22/user/month — and your documents never leave your network. At scale, the on-premise data residency alone eliminates compliance risk that money can't fully offset."*

### vs. iLovePDF / Smallpdf
> *"Those tools require uploading your sensitive documents to a third-party server. PaperPilot processes everything on your device. Your legal briefs, medical records, and financial statements stay yours."*

### vs. PDFgear
> *"PDFgear is free and closed-source — you're trusting their privacy policy. PaperPilot is free and open-source — you can read the code yourself and verify exactly what happens with your documents."*

---

## 🚀 Growth Feature Roadmap

These features transform PaperPilot from a tool into a platform:

### 1. Plugin / Extension System
Community-built `PdfOperation` plugins. Legal firms publish a "HIPAA Redactor". Accounting firms publish an "Invoice Normalizer". Plugin authors earn from the contributor pool. This is the network effect moat.

### 2. Workflow / Pipeline Builder (GUI) — Phase 3.4.10+
Visual drag-and-drop pipeline: `Validate → Redact → Bates Number → Sign → Webhook`. Basic pipelines are **free in Community**. Advanced triggers (scheduling, webhooks, watch folders) require Pro.

### 3. Watch Folder Mode (CLI Daemon)
```
paperpilot watch ./incoming --recipe compress --output ./done --webhook $SLACK_WEBHOOK
```
Auto-processes every new file dropped in a watched folder. Pro tier feature. The "set-and-forget" for IT departments.

### 4. SDK Libraries (Python, JavaScript, Go)
First-class SDKs wrapping the CLI/MCP interface. PaperPilot becomes a standard dependency in company automation scripts. Teams tier gets early access.

### 5. Zapier / Make.com / n8n Integration Nodes
One-click PaperPilot nodes in no-code automation platforms. Unlocks non-technical users — office managers and legal assistants building workflows without code.

### 6. Document Intelligence Dashboard
Analytics panel: what operations run most often, error rates, average processing time, searchable job history. Enterprise admins love this for compliance reporting.

### 7. Cloud MCP Gateway (Phase 7)
Hosted MCP endpoint for teams that can't run a local binary. Billed on usage-based Cloud tier. Enables PaperPilot as a drop-in PDF backend for any AI agent via the internet.

### 8. Website Builder Plugins & Drop-In Embed Widgets (`embed.js`)
- **Universal Embed**: 1-line `<script>` tag allowing any Webflow, Framer, Squarespace, or custom site to embed a client-side, zero-server-load PDF portal.
- **WordPress & WooCommerce Plugin**: Gutenberg block + automatic WooCommerce digital file watermarking & dynamic password encryption.
- **Shopify App**: Sub-10ms invoice generation & digital asset delivery at checkout via Cloudflare Edge.
- **Viral Growth Engine**: Free embeds carry a tasteful "⚡ Powered by PaperPilot" badge driving organic adoption to Desktop Pro.
*(See detailed architecture in [`docs/EMBEDDED_WEB_DISTRIBUTION_STRATEGY.md`](docs/EMBEDDED_WEB_DISTRIBUTION_STRATEGY.md))*
