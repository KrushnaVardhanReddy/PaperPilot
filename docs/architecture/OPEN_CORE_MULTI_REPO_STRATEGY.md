# PaperPilot Architecture: Open-Core & Multi-Repository Strategy

> **Governance Principle:** Strict separation of Open-Source Community Core from Commercial Enterprise Features using Rust's first-class crate dependency ecosystem. Zero code duplication, zero intellectual property leakage, and seamless upstream updates.

---

## 1. Architectural Philosophy: The Open-Core Model

PaperPilot follows the proven open-core architectural pattern used by successful infrastructure companies (GitLab, Supabase, ClickHouse, and HashiCorp):

* **Repository 1 (`PaperPilot` — Public / FOSS):** The open-source engine containing the core document manipulation kernel, all 50+ pure-Rust tools, native CLI, local MCP agent server, and community desktop/mobile UI.
* **Repository 2 (`PaperPilot-Enterprise` — Private / Commercial):** A downstream commercial wrapper that imports the open-source crates as external dependencies and layers on enterprise governance, identity, compliance, and air-gapped licensing.

```
┌────────────────────────────────────────────────────────────────────────┐
│ REPO 1: `PaperPilot` (Open Source — Apache 2.0 / MIT)                 │
│ Public GitHub: github.com/PaperPilot/paperpilot                        │
│                                                                        │
│ • paperpilot-core     (Traits, DOM, Errors, DocAST, URL Ingestion)     │
│ • paperpilot-pdf      (All 50+ pure-Rust document operations)          │
│ • paperpilot-cli      (Community CLI executable)                       │
│ • paperpilot-mcp      (Standard local MCP agent server for Claude)     │
│ • paperpilot-wasm     (Client-side browser WASM engine)                │
│ • paperpilot-gateway  (Local REST API server on port 7823)             │
│ • apps/desktop        (Community local desktop studio)                 │
│ • tools/penta-scaffold(Pure-Rust tool scaffolding generator)           │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    │ Cargo Remote Git Dependency
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│ REPO 2: `PaperPilot-Enterprise` (Private / Commercial Proprietary)    │
│ Private GitHub: github.com/PaperPilot/paperpilot-enterprise            │
│                                                                        │
│ • Downstream Cargo Import:                                             │
│     paperpilot-core = { git = "https://github.com/...", branch = "main"}│
│     paperpilot-pdf  = { git = "https://github.com/...", branch = "main"}│
│                                                                        │
│ • Proprietary Enterprise Modules (NOT in Open Source):                 │
│     ├── enterprise-license  (Air-Gapped Ed25519 Cryptographic Validator│
│     ├── enterprise-auth     (SAML 2.0, OIDC, Okta, Azure AD, SCIM)     │
│     ├── enterprise-audit    (Immutable SIEM Audit Trails: Splunk/DataDog│
│     ├── enterprise-drm      (Argon2id Memory-Burn Anti-Brute-Force)    │
│     ├── enterprise-cloud    (Multi-Tenant Cloudflare Edge & K8s Mesh)  │
│     └── enterprise-portal   (Admin Seat Management & Billing Metering) │
│                                                                        │
│ • Emits Commercial Deliverables:                                       │
│     - Air-Gapped Enterprise RPM/DEB/MSI Desktop Installers             │
│     - Hardened Commercial Docker Container (`paperpilot-enterprise`)   │
│     - Cloudflare Edge Managed SaaS API                                 │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Why Zero Code Duplication is Guaranteed

In the enterprise repository, developers **never copy or reimplement any document operations**. 

The enterprise repo's `Cargo.toml` simply declares the open-source crates:

```toml
[dependencies]
# Upstream Open-Source Kernel
paperpilot-core = { git = "https://github.com/KrushnaVardhanReddy/PaperPilot.git", branch = "main" }
paperpilot-pdf  = { git = "https://github.com/KrushnaVardhanReddy/PaperPilot.git", branch = "main" }
paperpilot-wasm = { git = "https://github.com/KrushnaVardhanReddy/PaperPilot.git", branch = "main", optional = true }

# Proprietary Enterprise Crates (Internal Workspace)
enterprise-license = { path = "crates/enterprise-license" }
enterprise-auth    = { path = "crates/enterprise-auth" }
enterprise-audit   = { path = "crates/enterprise-audit" }
```

### The Upstream/Downstream Flow:
1. When a new PDF/Office operation is added to the open-source repo (e.g. `pdf_booklet`, `pdf_scale_pages`), it is tested and merged into `PaperPilot/main`.
2. The enterprise repository runs `cargo update -p paperpilot-pdf`.
3. The enterprise distribution **instantly inherits all new tools and performance optimizations** with zero manual code porting.

---

## 3. Strict Boundary Matrix: What Belongs Where

| Capability / Module | Public Open-Source Repo (`PaperPilot`) | Private Enterprise Repo (`PaperPilot-Enterprise`) |
|---|:---:|:---:|
| **All 50+ Document Operations** (Merge, Split, Compress, Rotate, Redact, Sign, Tables, etc.) | ✅ Open Source | 🔄 Inherited via Cargo |
| **Local Pure-Rust Engine** (`lopdf`, `pdf_oxide`, `tiny-skia`) | ✅ Open Source | 🔄 Inherited via Cargo |
| **Standard Native CLI** (`paperpilot <cmd>`) | ✅ Open Source | 🔄 Inherited via Cargo |
| **Local MCP Server** (Claude Desktop, Cursor, OpenCode) | ✅ Open Source | 🔄 Inherited via Cargo |
| **Desktop Svelte 5 Studio** (Local Single-User Experience) | ✅ Open Source | 🔄 Branded / Gated wrapper |
| **Universal Remote URL Ingestion** (`url_fetch`) | ✅ Open Source | 🔄 Inherited via Cargo |
| **Air-Gapped Ed25519 License Key Validator** (Spec 092) | ❌ No | ✅ Proprietary |
| **Enterprise SIEM Audit Logging** (Splunk, Datadog - Spec 091) | ❌ No | ✅ Proprietary |
| **SSO: SAML 2.0 & OIDC** (Okta, Azure AD, Entra - Phase 6.1) | ❌ No | ✅ Proprietary |
| **SCIM Automated User Lifecycle Provisioning** (Phase 6.1.6) | ❌ No | ✅ Proprietary |
| **Enterprise DRM / Argon2id Memory-Burn Encryption** (Phase 6.6) | ❌ No | ✅ Proprietary |
| **Multi-Tenant Cloudflare SaaS API & Stripe Metering** (Phase 7) | ❌ No | ✅ Proprietary |
| **Admin Seat Management & License Server Dashboard** (Phase 6.5) | ❌ No | ✅ Proprietary |

---

## 4. How the Enterprise Layer Wraps the Open-Source Engine

In the enterprise repository, the commercial wrapper acts as an **intercepting decorator**:

```rust
// In paperpilot-enterprise/src/main.rs:

pub async fn execute_enterprise_operation(
    req: EnterpriseRequest,
) -> Result<EnterpriseResponse, EnterpriseError> {
    // 1. Check Air-Gapped Cryptographic License
    enterprise_license::verify_seat_and_expiration()?;

    // 2. Validate SSO / RBAC Permissions
    enterprise_auth::check_permission(&req.user, &req.operation)?;

    // 3. Record SIEM Audit Event (Who, What, Hash, Timestamp)
    let audit_event = enterprise_audit::start_audit_trail(&req);

    // 4. INVOKE UPSTREAM OPEN-SOURCE ENGINE (Zero Duplication!)
    let result = paperpilot_pdf::operations::run_operation_bytes(
        &req.input_bytes,
        &req.operation_args,
    )?;

    // 5. Emit SIEM Success Telemetry
    enterprise_audit::record_success(audit_event, &result);

    Ok(result)
}
```

---

## 5. Summary of Commercial Benefits

1. **Clean Legal Clearances:** The open-source repo has a permissive license (MIT/Apache 2.0), encouraging global developer adoption, GitHub stars, and community bug reports without legal friction.
2. **Enterprise Monetization Moat:** Defense contractors, banks, and healthcare institutions cannot use the open-source version for enterprise compliance because it lacks SAML/SSO, audit trails, and multi-user seat licensing. They **must purchase the Enterprise edition**.
3. **Infinite Maintainability:** 1 human engineer + AI subagents can maintain both products simultaneously because all core document logic is written **only once**.
