# PaperPilot Master Roadmap & Dependency Graph

> **Architecture Philosophy:** Pure-Rust engine, Penta-Interface parity (CLI, MCP, REST Gateway, WASM, Cloudflare Edge), and zero external C++/Python dependencies.

---

## 🎯 Dependency Architecture Flow

```
[Wave 1: Tool Scaffolding & Foundations]
  ├── 5.9.79: Pure-Rust Tool Scaffolding (`penta-scaffold`)
  └── 5.9.78: Universal Remote PDF URL Ingestion
           │
           ▼
[Wave 2: Critical Core Operations Sprint]
  ├── 5.9.61: Digital Signature Verification Engine (`pdf_validate_signatures`)
  ├── 5.9.62: Accessibility & PDF/UA Compliance (`pdf_validate_accessibility`)
  ├── 5.9.63: Semantic Legal Diffing (`pdf_compare_semantic`)
  ├── 5.9.66: Multi-Page Image Exporter (`pdf_to_images`)
  ├── 5.3.1:  Booklet Imposition (`pdf_booklet`)
  ├── 5.3.2:  Multi-Page N-Up Layout (`pdf_nup`)
  ├── 5.3.3:  Page Dimension Scaling (`pdf_scale_pages`)
  └── 5.3.8:  Overlay / Underlay Letterhead (`pdf_overlay_underlay`)
           │
           ▼
[Wave 3: Enterprise B2B Governance & Compliance]
  ├── 5.9.76: SIEM Compliance & Tamper-Evident Audit Logging
  ├── 5.9.77: Air-Gapped Ed25519 Cryptographic License Validator
  └── 6.1.1–6.1.6: Multi-Tenant RBAC, SSO (SAML/OIDC), and SCIM
           │
           ▼
[Wave 4: Cloudflare Edge & Remote Agent MCP]
  ├── 5.7.4:  Cloudflare Workers Edge Microservice
  ├── 5.5.3:  Remote MCP Server over SSE (`/mcp/sse`)
  └── 5.7.6:  Smart Hybrid Dispatch Router (Edge WASM ↔ K8s)
           │
           ▼
[Wave 5: Resilient Batch & Headless Workflows]
  ├── 5.9.64: Multi-Step Declarative Workflow Runner (`document_workflow`)
  ├── 5.9.65: Resilient Batch Directory Processing (`document_batch_process`)
  └── 5.5.1:  CLI Headless Pipeline Runner (`paperpilot pipeline run`)
           │
           ▼
[Wave 6: Adobe Acrobat Pro Inline Typography Editor]
  ├── 5.6.1:  Text Block & Paragraph Boundary Detection
  ├── 5.6.2:  In-Place Text Editing & Stream Rewriting
  └── 5.6.4:  Object-Level Image & Shape Transformation
           │
           ▼
[Wave 7: Universal Document Operating System (Phase 10)]
  ├── 10.1:   Native Pure-Rust DOCX/Office Read & Write Engine
  ├── 10.2:   Spreadsheet Processing & Formula Calculation
  └── 10.3:   Universal Format Converters (PDF ↔ DOCX ↔ XLSX ↔ PPTX ↔ Markdown)
```

---

## 📋 Task Matrix & Status Tracker

### 1. In-Flight Tasks (Active Jules Sessions)
| Task # | Feature Name | Dependency | Status / Jules Session |
|---|---|---|---|
| **5.9.58** | Real Table Spatial Extraction (Stage 2) | Spec 090 | ⏳ In-Flight (`17392106531061751840`) |
| **5.9.78** | Universal Remote PDF URL Ingestion | Spec 093 | ⏳ In-Flight (`6136209794630860269`) |
| **5.9.79** | Pure-Rust Tool Scaffolding Generator (`penta-scaffold`) | Spec 094 | ⏳ In-Flight (`8915713543148419332`) |

---

### 2. High-Priority Tool Sprint (Wave 2)
*Blocked only by `5.9.79` (`penta-scaffold` makes generating each tool a 1-command operation).*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **5.9.61** | `pdf_validate_signatures` | PKCS#7 / CMS cryptographic signature & cert chain verification | Legal, Banking, Government |
| **5.9.62** | `pdf_validate_accessibility` | PDF/UA, Section 508, `/Lang`, and `/Alt` tags validation | Higher Ed & Enterprise Procurement |
| **5.9.63** | `pdf_compare_semantic` | Legal clause & sentence meaning diffing | Contract Review & M&A Due Diligence |
| **5.9.66** | `pdf_to_images` | Batch page rendering to PNG/JPEG/WebP directories | Archival & Web Previews |
| **5.3.1** | `pdf_booklet` | 2-sided booklet signature imposition | Commercial & Office Printing |
| **5.3.2** | `pdf_nup` | Multi-page layout (2, 4, 9, 16 pages per sheet) | Paper Reduction & Handouts |
| **5.3.3** | `pdf_scale_pages` | Dimensional scaling (A4 $\leftrightarrow$ Letter) with aspect preservation | Multi-region Document Normalization |
| **5.3.8** | `pdf_overlay_underlay` | Digital letterhead & background stamping | Corporate Invoicing & Branding |
| **5.3.10** | `pdf_auto_rotate` | Heuristic font baseline + OCR automatic page uprighting | Scanned Document Intake |

---

### 3. Enterprise B2B Governance & Compliance (Wave 3)
*Required for enterprise licensing, defense contracts, and commercial SaaS.*

| Task # | Feature Name | Dependency | Core Functionality |
|---|---|---|---|
| **5.9.76** | SIEM Compliance & Audit Logging | Core Engine | Structured immutable JSON audit events (Splunk, Datadog) |
| **5.9.77** | Air-Gapped Ed25519 License Validator | Core Engine | Offline signed `license.key` validation (SCIF / DoD ready) |
| **6.1.1** | Organization & Multi-Tenant Model | Gateway | Multi-tenant schema and data boundaries |
| **6.1.3** | Enterprise RBAC | 6.1.1 | Role definitions and endpoint permission guards |
| **6.1.4** | SSO — SAML 2.0 | 6.1.1 | Okta, Azure AD, PingIdentity SAML authentication |
| **6.1.5** | SSO — OIDC | 6.1.1 | Google Workspace, Microsoft Entra OIDC login |
| **6.1.6** | SCIM Provisioning | 6.1.2 | Automated employee provisioning/deprovisioning from IdP |

---

### 4. Cloudflare Edge & Remote Agent MCP (Wave 4)
*Unlocks instant cloud API execution and universal AI Agent tool consumption.*

| Task # | Feature Name | Dependency | Core Functionality |
|---|---|---|---|
| **5.7.4** | Cloudflare Workers Edge Microservice | `paperpilot-wasm` | Sub-10ms serverless execution across 300+ global PoPs |
| **5.5.3** | Remote Streamable MCP Server over SSE | 5.7.4 | Cloud-hosted MCP endpoint (`/mcp/sse`) for Claude & Cursor |
| **5.7.6** | Smart Hybrid Dispatch Router | 5.7.4 | Auto-routes light tasks to Edge WASM and heavy OCR to K8s Pods |

---

### 5. Resilient Headless Automation (Wave 5)
*Enables enterprise batch processing and visual-to-code pipelines.*

| Task # | Feature Name | Dependency | Core Functionality |
|---|---|---|---|
| **5.9.64** | Multi-Step Declarative Workflow Runner | Wave 2 Tools | Intermediate in-memory piping without disk roundtrips |
| **5.9.65** | Resilient Batch Directory Processor | 5.9.64 | Multi-threaded directory processing with retries and reports |
| **5.5.1** | CLI Headless Pipeline Runner | 5.9.64 | `paperpilot pipeline run recipe.json` |

---

### 6. In-Place Inline Typography Editor (Wave 6)
*Direct Adobe Acrobat Pro killer feature.*

| Task # | Feature Name | Dependency | Core Functionality |
|---|---|---|---|
| **5.6.1** | Text Block & Paragraph Boundary Detection | `pdf_oxide` | Segment raw `BT...ET` streams into paragraphs |
| **5.6.2** | In-Place Text Editing & Stream Rewriting | 5.6.1 | Direct typography and typo edits without re-exporting |
| **5.6.4** | Object-Level Image & Shape Transformation | 5.6.1 | Move, resize, replace images and shapes on canvas |

---

### 7. Universal Document Operating System (Phase 10)
*Expands PaperPilot beyond PDF to universal document manipulation.*

| Task # | Feature Name | Dependency | Core Functionality |
|---|---|---|---|
| **10.1** | Pure-Rust DOCX/Office Read & Write | `office_oxide` | 100% native Word document manipulation |
| **10.2** | Pure-Rust XLSX Spreadsheet Engine | `office_oxide` | Formula calculation, cell styling, sheet extraction |
| **10.3** | Universal Multi-Format Converters | 10.1, 10.2 | Zero-dependency PDF $\leftrightarrow$ DOCX $\leftrightarrow$ XLSX $\leftrightarrow$ Markdown |
