# PaperPilot Pricing Strategy (v2 — Competitive Update)

**Target Market:** Individual developers, power users, SMBs, and Enterprise PDF teams.
**Overhead:** $0.00 compute cost on local processing (our core advantage over every cloud-based tool).
**Goal:** Beat Stirling PDF's usage-based model with a simpler "pay once, run forever locally" message, while building a sustainable B2B revenue stream from teams and enterprise.

> **Key Counter-Positioning vs. Stirling PDF:**
> Stirling charges $0.01 per automation run (their Processor product). PaperPilot runs **100% locally** — you never pay per operation. Every automation, every pipeline run, every batch job is **free on your machine**. For developers and power users running hundreds of automations a month, we are dramatically cheaper.

---

## Tier Overview

| Tier | Price | Best For |
|---|---|---|
| **Community** | **Free** | Everyone — individuals, students, open source |
| **Pro** | **$5/month** (or $48/yr — save 20%) | Power users, freelancers, consultants |
| **Teams** | **$12/user/month** (or $115/user/yr — save 20%) | Small teams, agencies, law firms, 2–50 seats |
| **Enterprise** | **$22/user/month** (or $210/user/yr) | 50+ seats, IT-managed, compliance-driven orgs |
| **Cloud (Usage-Based)** | **$0.01/run** (first 1000 runs free) | Serverless/CI users who can't run locally |

> Prices are positioned below Stirling's ecosystem cost (editor free + $0.01/run adds up fast for teams), below Adobe Acrobat ($20/user/month), and well below typical B2B PDF SaaS ($25–$50/user/month).

---

## Tier 1: Community (Free Forever)

* **Price:** $0 — always, unconditionally.
* **Target:** Individual users, students, open-source developers, self-hosters, and anyone evaluating PaperPilot.
* **Features:**
  - **All 50+ core PDF operations** (Merge, Split, Rotate, Compress, Redact, Encrypt, OCR, Forms, Annotations, Convert, etc.)
  - **Full PDF Viewer** with annotations and form filling
  - **Offline AI Mode** (rule-based NLP intent routing — zero API key, zero network)
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
  - **Centralized license portal** — admin manages seats, assigns roles
  - **Shared pipeline/recipe library** — team recipes stored in a private registry
  - **Team audit log** — searchable job history per user (what ran, when, result)
  - **Concurrent pipeline execution** across team members
  - **Priority support** + 4-hour response SLA
  - **SDK access** (Python, JavaScript) — early access to PaperPilot SDK libraries
* **Why $12:** The shared recipe library is our retention engine — once a firm standardizes their invoice, contract, or Bates stamping pipeline across staff, switching cost is high.

---

## Tier 4: Enterprise (Fleet Governance & Compliance)

* **Price:** $22/user/month (~$210/user/year billed annually)
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

## Tier 5: Cloud (Usage-Based) — Developer / CI Tier

* **Price:** **$0.01 per pipeline run** (first **1,000 runs free**, no credit card required)
* **Target:** Developers integrating PaperPilot into CI/CD pipelines, serverless functions, or automation scripts — who cannot or prefer not to run a local binary.
* **Features:**
  - Hosted PaperPilot MCP endpoint (Cloud MCP Gateway)
  - All 40+ MCP tools available via HTTP
  - Pay only for what you use; no monthly seat commitment
  - Scales to millions of runs with volume discounts (see below)
  - Usage dashboard with per-operation cost breakdown
* **Volume pricing:**
  | Monthly Runs | Price/Run |
  |---|---|
  | 0–1,000 | **Free** |
  | 1,001–10,000 | $0.01/run |
  | 10,001–100,000 | $0.007/run |
  | 100,001–1,000,000 | $0.005/run |
  | 1,000,000+ | Custom / negotiate |

* **Why this tier:** This **directly matches and undercuts Stirling**. Stirling gives 500 free runs, we give 1,000. Stirling charges $0.01/run with no published volume discount. We start discounting at 10k runs. For DevOps teams and API integrators, this is the clear winner on price and flexibility.

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

### Why This Works
> Free users become plugin developers. Plugins attract more Enterprise customers. Revenue growth increases the pool. A bigger pool attracts more developers. This is a self-reinforcing flywheel.

---

## 🚀 Key Messaging vs. Competitors

### vs. Stirling PDF
> *"Stirling charges $0.01 per automation run. PaperPilot runs locally — every automation is free, forever, no run counter, no billing surprise. For teams running hundreds of document workflows a month, that's the difference between a $50 bill and $0."*

### vs. Adobe Acrobat Pro
> *"Adobe charges $20/user/month and uploads your documents to Adobe's cloud. PaperPilot Enterprise is $22/user/year — 11x cheaper — and your documents never leave your network."*

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
