# PaperPilot Pricing & Business Strategy (Two-Tier Model)

**Core Model:** Open-Core with **Two Tiers Only**:
1. **Community Edition (100% Free Forever, Apache 2.0)**: Individual developers, power users, solo consultants, researchers, and open-source contributors.
2. **Enterprise Edition (Commercial License)**: Organizations requiring fleet governance, security compliance, air-gapped deployments, and audit trails.

**The Golden Philosophy:**
- **Zero local tax**: No paywalls on local processing. 0 bytes uploaded to external servers, 0 limits on tool execution, 0 user caps for self-hosted instances.
- **We monetize fleet governance, NOT individual tools**: Everything that runs on a single user's workstation or personal homelab is completely free. We charge organizations for central IT administration, compliance audits, SCIM, and signed BAA agreements.

---

## The Two Tiers

| Dimension | **Community Edition** | **Enterprise Edition** |
|---|---|---|
| **Price** | **$0 / Free Forever (Apache 2.0)** | **$22/user/month** or **$15k–$35k/year Flat VPC** |
| **Target** | Individuals, developers, researchers, small teams | Mid-market, healthcare (HIPAA), legal firms, defense, banking |
| **All 75+ Core Tools** | ✅ Full access (Merge, Split, OCR, Redact, Sign, Permissions, Imposition, etc.) | ✅ Full access |
| **CLI & Local REST API** | ✅ Included (`localhost:7823`) | ✅ Included |
| **Model Context Protocol (MCP)** | ✅ Native MCP server for Claude Desktop, Cursor, and OpenCode | ✅ Native MCP server |
| **Desktop Application** | ✅ Full Tauri 2 + Svelte 5 GUI (macOS, Windows, Linux) | ✅ Pre-packaged silent installers (MSI, Jamf, SCCM) |
| **Client-Side WASM Web Engine** | ✅ 100% in-browser privacy (usepaperpilot.com) | ✅ Optional Managed White-Label Edge Portal |
| **Offline Vector Search & RAG** | ✅ Embedded `libsql_vector` + citation canvas | ✅ Embedded `libsql_vector` + citation canvas |
| **Pipeline Builder** | ✅ Unlimited steps & local execution | ✅ Unlimited steps & shared team recipe library |
| **Single Sign-On (SSO)** | ✅ Standard OIDC / OAuth (Google, GitHub, Keycloak, Authentik) | ✅ Enterprise SAML 2.0 with custom claim mapping |
| **Lifecycle Provisioning** | ❌ Manual | ✅ Automated **SCIM 2.0** (Okta, Azure AD, OneLogin) |
| **Air-Gap Hardening** | Manual configuration | ✅ Enforced GPO/MDM policies (hard-blocks all network egress) |
| **SIEM Compliance Auditing** | Local SQLite logs | ✅ Real-time audit log streaming to Splunk, Datadog, CloudWatch |
| **Compliance & Legal** | Apache 2.0 license | ✅ Signed **BAA (HIPAA)**, SOC2 report, commercial indemnification |
| **Support** | Community (GitHub Discussions, Discord) | Dedicated account manager, 24/7 phone SLA, custom builds |

---

## 1. Community Edition ($0 — 100% Free Forever)

* **Who it's for:** Every developer, student, freelancer, privacy advocate, and self-hoster.
* **Why it's completely free:**
  - Traditional competitors (Adobe, Smallpdf) pay millions in cloud server and GPU costs, so they are forced to charge $10–$25/month.
  - Because PaperPilot runs **100% locally on the user's CPU/RAM via Rust and WASM**, our marginal server cost per user is **$0.00**.
  - We do not gatekeep local pipelines, watch folders, or CLI commands.
* **Included out of the box:**
  - Complete 75+ operation catalog with no artificial limits.
  - Full desktop app, terminal CLI, and in-browser WASM sandbox.
  - Model Context Protocol (MCP) server for local AI agents.
  - Zero telemetry, zero cloud tracking, zero file retention.

---

## 2. Enterprise Edition (Fleet Governance & Compliance)

* **Who it's for:** Hospitals, law firms, financial institutions, and government agencies.
* **Why they pay:** Enterprises do not pay for basic PDF merging; they pay for **risk mitigation, compliance, and IT fleet control**:
  1. **Automated Provisioning (SCIM 2.0)**: Automatically assign and revoke seats when employees join or leave via Okta / Azure Active Directory.
  2. **Air-Gap SCIF Deployment**: Validated Ed25519 offline license files with zero phone-home calls for classified government networks.
  3. **Immutable Audit Trails**: Streams every document hash, redaction event, and signature to enterprise SIEMs (Splunk, Datadog) for regulatory compliance.
  4. **Signed HIPAA BAA & Legal Indemnification**: Required by corporate legal teams before software can touch protected health information (PHI).

---

## Strategic Advantage vs. Competitors

- **vs. Adobe Acrobat ($240/user/yr)**: PaperPilot is 100% private, never sends client documents to external clouds, boots in <100ms, and integrates natively with AI tools via MCP.
- **vs. Stirling-PDF (Open-Core with usage taxes)**: Stirling charges $0.01 per automation run on their Processor and gates team features. PaperPilot's local automation is **$0 forever**.
- **vs. BentoPDF (Browser only)**: BentoPDF has no CLI, no REST API, no desktop app, and no MCP server for AI agents. PaperPilot gives users the complete penta-interface.
