# PaperPilot Pricing Strategy

**Target Market:** B2B (Business-to-Business) PDF Utility Market
**Overhead:** $0.00 compute cost (local-first rendering)
**Goal:** Undercut Adobe Acrobat Pro ($240/yr) while building a sustainable revenue model and a self-reinforcing contributor flywheel.

---

## Tier Overview

| Tier | Price | Best For |
|---|---|---|
| Personal | **Free** | Everyone |
| Pro | **$7/month** (or $69/yr billed annually) | Power users, freelancers |
| Teams | **$15/user/month** (or $144/user/yr billed annually) | SMBs, 2–50 seats |
| Enterprise | **$25/user/month** (or $240/user/yr billed annually) | 50+ seats, IT-managed deployments |

---

## Tier 1: Personal (Free Forever)
* **Price:** $0
* **Target:** Individual users, students, open-source community.
* **Features:**
  - All core PDF manipulation features (Merge, Split, Rotate, Text Extraction, Encryption)
  - Offline AI Mode (Rule-based NLP intent routing, no API key required)
  - MCP Server (local, single-user)
  - CLI access
  - Community support only
* **Why:** Drives massive user acquisition, word-of-mouth marketing, and acts as a trojan horse into corporate environments. The free tier is how PaperPilot spreads.

---

## Tier 2: Pro
* **Price:** $7/month (~$69/year billed annually)
* **Target:** Freelancers, power users, consultants, solo developers.
* **Features (everything in Personal, plus):**
  - Visual Pipeline Builder (Phase 3.4.10)
  - Pro AI Mode (Full LLM multi-step reasoning, Bring Your Own Key)
  - LLM-Ready Export (PDF → RAG-optimized Markdown + JSON sidecar)
  - Watch Folder daemon mode
  - Webhooks & HMAC signing
  - 1 year of update entitlement + priority email support
* **Positioning:** $7/month is an impulse purchase. It's less than a Netflix subscription and has a clear, compelling feature hook (the Visual Pipeline Builder and AI mode).

---

## Tier 3: Teams
* **Price:** $15/user/month (or $144/user/year billed annually)
* **Target:** Agencies, small law firms, accounting teams, startups — 2 to 50 seats.
* **Features (everything in Pro, plus):**
  - Centralized license portal (admin manages seats)
  - Shared recipe/pipeline library (team recipes stored in a private registry)
  - Audit log (searchable job history per user)
  - Priority support + 4-hour response SLA
* **Why $15:** Teams share the tool actively, so per-seat value is higher. Still 6x cheaper than Adobe per year. The shared recipe library is the key hook — once a team builds workflows, they don't leave.

---

## Tier 4: Enterprise
* **Price:** $25/user/month (or $240/user/year billed annually)
* **Minimum:** 50 seats
* **Target:** Hospitals, law firms, financial institutions, government contractors, any org with an IT policy.
* **Features (everything in Teams, plus):**
  - MSI / MDM silent installers (Intune, Jamf, SCCM)
  - Group Policy lockdown (disable external network calls, enforce encryption)
  - SSO / SAML integration (Okta, Azure AD, Google Workspace)
  - Legal indemnification clause
  - DocuSign / E-Signature integrations
  - Dedicated Account Manager
  - 24-hour SLA + phone support option
  - On-premise deployment option (airgapped networks)
* **Why $25:** This is still **less than Adobe Acrobat** per user per year at scale, but with *zero cloud upload risk* — a massive differentiator for healthcare (HIPAA), legal (client privilege), and finance (SEC/FINRA) use cases. IT admins aren't buying PDF editing; they're buying compliance, control, and peace of mind.

---

## 💡 Plugin Ecosystem Revenue Sharing Model

**Philosophy:** While the core PaperPilot engine and Enterprise B2B features are built and maintained by the founding team, we want to incentivize a thriving ecosystem of community-built extensions.

### The Pool
- **10% of all paid subscription revenue** is reserved as the Plugin Developer Revenue Pool, distributed quarterly.
- This is fully transparent: pool size and top-earning plugins are published publicly on the website every quarter.

### How It Works (Value Allocation)
Instead of a flat scoring system, revenue is dynamically routed based on **plugin usage by paying customers**:

1. **Plugin Analytics:** When a Pro, Teams, or Enterprise user installs and executes a community plugin (e.g., "HIPAA Redactor" or "Invoice Normalizer"), that usage is anonymously tallied.
2. **Distribution:** The 10% quarterly pool is divided proportionally among plugin authors based on the aggregate usage of their plugins by paid tiers.
3. **Quality Control:** To qualify for the revenue pool, plugins must be open-source, undergo a security review, and be officially published to the PaperPilot Plugin Registry.

### Payout Process
1. Plugin Author registers their GitHub handle and payment info (Stripe / Wise).
2. At end of each quarter, the algorithm calculates usage metrics and splits the pool.
3. Payouts are sent automatically.
4. Contributors must submit a tax form (W-9 / W-8BEN equivalent) before first payout.

### Why This Works
> Free users become plugin developers. Plugin developers build niche tools that attract more Enterprise customers.
> Revenue growth increases the pool. A bigger pool attracts more developers.
> This is a self-reinforcing flywheel that allows the core team to focus exclusively on B2B infrastructure.

---

## 🚀 Growth Feature Roadmap

These features transform PaperPilot from a standalone tool into a platform:

### 1. Plugin / Extension System
Community-built `PdfOperation` plugins. Legal firms can publish a "HIPAA Redactor" plugin. Accounting firms can publish an "Invoice Normalizer" plugin. Plugin authors earn from the contributor pool.

### 2. Workflow / Recipe Builder (GUI) — Phase 3.4.10
Visual drag-and-drop pipeline builder: `Validate → Redact → Bates Number → Sign → Webhook`. Recipes can be saved, shared, and imported. Enterprise customers get private recipe libraries.

### 3. Watch Folder Mode (CLI Daemon)
`paperpilot watch ./incoming --recipe compress --output ./done --webhook $SLACK_WEBHOOK`
Automatically processes every new file dropped in a watched directory. Set-and-forget for IT departments.

### 4. SDK Libraries (Python, JavaScript, Go)
First-class SDKs that wrap the CLI/MCP interface. Makes PaperPilot a standard dependency in company automation scripts. Dramatically expands the developer audience.

### 5. Zapier / Make.com / n8n Integration Nodes
One-click PaperPilot nodes in no-code automation platforms. Unlocks a completely non-technical user segment — office managers and legal assistants building workflows without code.

### 6. Document Intelligence Dashboard
Analytics panel showing what operations run most often, error rates, average processing time, and a searchable job history. Enterprise admins love this for compliance reporting.
