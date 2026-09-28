# PaperPilot

## Open-Source AI Document Automation Platform

### Tagline

> **Open-source document infrastructure for humans and AI agents.**

Alternative:

> **AI decides what to do. Rust decides how to do it.**

---

# 1. Vision

Build **PaperPilot**, an open-source, local-first document automation platform.

The initial focus is PDF manipulation, but the architecture should allow the platform to expand into broader document automation.

Users should be able to interact with documents through:

* Desktop GUI
* Natural language
* CLI
* MCP
* APIs
* Automation workflows

Example:

> "Merge these three PDFs, remove blank pages, OCR the scanned pages, compress the result, and save it as final.pdf."

The AI interprets the request.

The Rust engine executes it.

The user remains in control.

---

# 2. Product Philosophy

PaperPilot is NOT intended to be another online PDF website.

It should be:

> **An open-source document automation engine designed for both humans and AI agents.**

Core principle:

> **AI decides what to do. Rust decides how to do it.**

The AI should never directly manipulate document bytes.

Instead:

```text
Natural language
       │
       ▼
      AI
       │
       ▼
Structured operation plan
       │
       ▼
Rust validation
       │
       ▼
PaperPilot engine
       │
       ▼
Document output
```

---

# 3. Open-Source Strategy

The core PaperPilot product should be genuinely open source.

The community edition should not be artificially crippled.

Open-source functionality should include:

* Rust document engine
* PDF operations
* CLI
* MCP server
* Svelte/Tauri desktop application
* Local processing
* Local AI support
* Basic OCR
* Document extraction
* Automation capabilities
* Tool schemas
* Plugin architecture

Users should be able to download, build, self-host, modify, and redistribute the open-source project according to the chosen license.

---

# 4. Technology Stack

## Core Engine

**Rust** — primary implementation language.

Goals:

* Native performance
* Memory safety
* Cross-platform support
* Small deployment footprint
* Minimal runtime dependencies
* No Python runtime

## Frontend

* Svelte 5
* TypeScript
* Vite

## Desktop Shell

* Tauri 2

## AI Integration

* OpenAI-compatible REST API (provider-agnostic)
* llamafile for local model bundling
* Any local LLM server (Ollama, LM Studio, etc.)

## MCP

* `rmcp` — Rust MCP SDK

## Build Tooling

* Cargo (Rust)
* pnpm (frontend)

---

# 5. Desktop Application

Do NOT use Electron.

The Svelte frontend should remain relatively thin.

Document manipulation belongs in Rust.

Architecture:

```text
Svelte UI
    │
    ▼
Tauri 2
    │
    ▼
Rust Core
    │
    ├── PDF Engine
    ├── OCR
    ├── Document Extraction
    ├── AI Orchestration
    └── Job System
```

---

# 6. PDF Engine

The PDF engine is the most critical technical decision in Phase 1.

Evaluate these Rust/native options carefully before committing:

| Library | Type | Strengths | Weaknesses |
|---|---|---|---|
| `lopdf` | Pure Rust | Lightweight, read/write | Limited rendering, no font subsetting |
| `pdfium-render` | Binds Google Pdfium | Full rendering, text extraction | Large binary (~20MB), C++ dependency |
| `printpdf` | Pure Rust | Good for PDF creation | Write-only, no reading/editing |
| `qpdf` (via FFI) | Binds qpdf C++ | Battle-tested, full ops | FFI overhead, C++ build chain |

**Recommended approach:** Use `pdfium-render` for rendering and text extraction, `lopdf` for structural operations (merge, split, page manipulation). Abstract both behind PaperPilot traits so the underlying library can be swapped.

Avoid tightly coupling the entire project to one PDF library.

---

# 7. MCP

Use the Rust MCP SDK:

`rmcp`

PaperPilot should expose document operations as strongly typed MCP tools.

Example tools:

```text
pdf_merge
pdf_split
pdf_extract_pages
pdf_delete_pages
pdf_reorder_pages
pdf_rotate
pdf_compress
pdf_ocr
pdf_extract_text
pdf_search
```

External AI agents can then use PaperPilot as their document execution layer.

---

# 8. AI Layer

AI is optional.

PaperPilot should support:

* Local llamafile
* Local LLM servers (Ollama, LM Studio, etc.)
* OpenAI-compatible APIs
* Other LLM providers

The PDF engine must work without an LLM.

The AI provider must be replaceable.

No proprietary AI service should be required to use PaperPilot.

---

# 9. Natural Language Interface

Users should be able to write:

> Merge these PDFs.

> Remove pages 3 and 7.

> Extract pages 10 through 20.

> Compress this PDF.

> Rotate pages 4 through 8 clockwise.

> Extract all images from this document.

> Make this scanned PDF searchable.

Complex workflows:

> Merge all the invoices, remove blank pages, OCR the scanned pages, compress the result, and save it as invoices-2026.pdf.

The AI converts the request into a structured operation plan.

Rust validates and executes it.

---

# 10. Initial PDF Operations

Implement:

* Merge
* Split
* Extract pages
* Delete pages
* Reorder pages
* Rotate pages
* Compress
* Repair
* Metadata read/write
* Extract text
* Extract images
* Render pages (to PNG/JPEG)
* Watermark
* Encrypt
* Decrypt
* Search
* Compare
* Images → PDF

OCR is a planned capability but is **post-MVP**. See Section 28 and Phase 5.

---

# 11. CLI

The CLI binary should use lowercase: `paperpilot`.

Everything important should be available through the CLI.

Examples:

```bash
paperpilot merge a.pdf b.pdf -o merged.pdf
```

```bash
paperpilot split document.pdf --pages 1-5 -o output.pdf
```

```bash
paperpilot remove document.pdf --pages 2,7 -o cleaned.pdf
```

```bash
paperpilot compress document.pdf -o compressed.pdf
```

```bash
paperpilot rotate document.pdf --pages 3,4 --degrees 90 -o rotated.pdf
```

```bash
paperpilot extract-text document.pdf -o text.txt
```

Potentially:

```bash
paperpilot ask "merge all PDFs in this folder and compress them"
```

---

# 12. Desktop UX

The application should provide:

* Drag-and-drop PDFs
* File browser
* PDF preview
* Page thumbnails
* Page selection
* Page reordering
* Page rotation
* Page deletion
* Natural-language command box
* Operation preview
* Progress
* Output management
* Job history

Example:

```text
┌──────────────────────────────────────────────────────┐
│ PaperPilot                                      ⚙      │
├──────────────────────────────────────────────────────┤
│                                                      │
│              Drop documents here                    │
│                                                      │
│                 + Add Documents                     │
│                                                      │
│  Files                                               │
│  ──────────────────────────────────────────────────  │
│  📄 report-01.pdf                         2.4 MB     │
│  📄 report-02.pdf                         8.1 MB     │
│  📄 appendix.pdf                          1.2 MB     │
│                                                      │
│  ┌────────────────────────────────────────────────┐  │
│  │ What would you like to do?                    │  │
│  │                                               │  │
│  │ "Merge these, remove blank pages and          │  │
│  │  compress the result..."                      │  │
│  │                                         ➜     │  │
│  └────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

---

# 13. AI Operation Confirmation

For complex or destructive operations, show the interpreted plan before execution.

Example:

```text
I understood:

1. Merge report-01.pdf
2. Merge report-02.pdf
3. Merge appendix.pdf
4. Remove blank pages
5. OCR scanned pages
6. Compress
7. Save as final.pdf

              [ Cancel ]    [ Run ]
```

The user can modify the plan before execution.

---

# 14. Job System

Long-running operations should execute as asynchronous jobs.

Example:

```text
OCR document

████████████████░░░░ 82%

Page 82 / 100

[ Cancel ]
```

Rust emits progress events.

Svelte displays progress.

The UI must remain responsive.

---

# 15. Privacy

PaperPilot should be local-first.

Default:

```text
Document
   │
   ▼
Local PaperPilot
   │
   ▼
Local output
```

Documents should not be uploaded anywhere by default.

No account should be required for the open-source desktop application.

No cloud service should be required.

AI can run locally using llamafile or another local model.

---

# 16. MCP as an AI Infrastructure Layer

PaperPilot should not only be a desktop PDF application.

It should also become a document capability for AI agents.

Example:

```text
AI Agent
   │
   ▼
PaperPilot MCP
   │
   ├── Merge
   ├── Split
   ├── OCR
   ├── Extract
   ├── Compress
   ├── Search
   └── Convert
```

External agents could perform document workflows without implementing their own PDF engine.

This is a major part of PaperPilot's long-term positioning.

---

# 17. Enterprise Edition

The open-source core remains useful by itself.

The commercial enterprise layer provides organizational controls.

Enterprise features may include:

## Identity

* SSO
* SAML
* OIDC
* SCIM

## Organization

* Organizations
* Teams
* Users
* Roles
* RBAC
* Central configuration

## Security

* Security policies
* AI provider restrictions
* Data processing policies
* Local-only processing policies
* Document retention policies
* Encryption configuration

Example:

```text
Enterprise AI Policy

Allowed:
✓ Local models
✓ Company-hosted models

Blocked:
✗ Public AI APIs
```

---

# 18. Enterprise Audit

Provide organization-level audit events.

Example:

```text
2026-09-28 10:31
Jane Smith

Operation:
Merged 4 PDFs

AI:
Local model

Result:
contract-final.pdf
```

Audit logging should be configurable.

Document contents should not automatically be stored in audit logs.

---

# 19. Enterprise Deployment

Support corporate deployment options:

* Docker
* Kubernetes
* Linux servers
* Windows
* Private infrastructure
* On-premises
* Air-gapped environments

The goal is:

> **Companies can run PaperPilot entirely inside their own infrastructure.**

This is especially important for organizations with strict document privacy requirements.

---

# 20. Managed PaperPilot

Offer an optional hosted version for organizations that do not want to manage infrastructure.

Architecture:

```text
Company
   │
   ▼
PaperPilot Cloud
   │
   ├── MCP
   ├── AI
   ├── Document processing
   ├── Admin
   ├── Audit
   └── Policy engine
```

The cloud version should not be required for the open-source product.

Organizations can choose:

```text
Self-host
     OR
PaperPilot Cloud
```

---

# 21. Business Model

The project should follow an open-core / managed-service model.

## Community

Free and open source.

Includes:

* Desktop
* CLI
* MCP
* PDF operations
* Local AI
* OCR (post-MVP)
* Document extraction
* Local processing

## Team

Potential paid tier.

Could include:

* Team management
* Shared configuration
* Centralized deployment
* Team workflows
* Usage management

## Enterprise

Custom pricing.

Potential features:

* SSO
* SAML
* OIDC
* SCIM
* RBAC
* Audit logs
* Security policies
* Central administration
* Enterprise MCP gateway
* On-prem deployment
* Air-gapped deployment
* Priority support
* SLA
* Security documentation
* Custom integrations

## Managed Cloud

Optional hosted PaperPilot.

Organizations pay for:

* Hosting
* Infrastructure
* Updates
* Monitoring
* Scaling
* Enterprise controls
* Support

---

# 22. Do Not Artificially Limit the Open-Source Product

Do NOT use artificial limitations such as:

```text
Free:
Merge 3 PDFs

Paid:
Merge 4 PDFs
```

or:

```text
Free:
100 pages

Paid:
101 pages
```

The open-source project should be genuinely useful.

The commercial value should come from:

* Organization management
* Security
* Governance
* Deployment
* Support
* Scale
* Managed infrastructure
* Enterprise integrations

This encourages adoption instead of discouraging it.

---

# 23. Long-Term Document Platform

PDF is the initial entry point.

The architecture should eventually support:

```text
PDF
DOCX
XLSX
PPTX
Images
Scanned documents
Invoices
Contracts
Forms
```

Long-term capabilities:

* Document conversion
* OCR
* Extraction
* Classification
* Search
* Semantic search
* Document comparison
* Summarization
* Structured extraction
* Table extraction
* Document Q&A
* Workflow automation

The long-term product becomes:

> **Document infrastructure for humans and AI agents.**

---

# 24. Architecture

```text
                         PaperPilot
                            │
            ┌───────────────┼────────────────┐
            │               │                │
            ▼               ▼                ▼
        Desktop            CLI              MCP
       Svelte/Tauri                          │
            │                                │
            └───────────────┬────────────────┘
                            ▼
                       Rust Core
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
          ▼                 ▼                 ▼
     PDF Engine           OCR          Document Engine
          │                 │                 │
          └─────────────────┼─────────────────┘
                            ▼
                       AI Layer
                            │
             ┌──────────────┼──────────────┐
             ▼              ▼              ▼
          llamafile      Local LLM      Remote API
```

---

# 25. Repository Structure

```text
paperpilot/
│
├── apps/
│   └── desktop/
│       ├── src/
│       │   ├── components/
│       │   ├── routes/
│       │   ├── stores/
│       │   └── lib/
│       └── package.json
│
├── crates/
│   ├── paperpilot-core/
│   ├── paperpilot-pdf/
│   ├── paperpilot-compress/
│   ├── paperpilot-ocr/
│   ├── paperpilot-extract/
│   ├── paperpilot-ai/
│   ├── paperpilot-mcp/
│   ├── paperpilot-cli/
│   └── paperpilot-app/
│
├── tests/
│   └── fixtures/          ← PDF test corpus for engine validation
├── examples/
├── docs/
│
├── Cargo.toml
├── LICENSE
└── README.md
```

Crate names use lowercase kebab-case following Rust/Cargo conventions.

---

# 26. Testing Strategy

The Rust core handles binary document formats.
Correctness is non-negotiable.

Testing should include:

* Unit tests per crate
* Integration tests using a PDF fixture corpus (varied sizes, formats, edge cases)
* Round-trip tests for split/merge operations (output re-merged should match input structure)
* Property-based tests using `proptest` or `quickcheck` for operation invariants
* CLI integration tests
* MCP schema validation tests

All operations that modify documents should be tested against known-good output files.

---

# 27. Development Phases

## Phase 1: Rust Core

Build:

* Merge
* Split
* Extract
* Delete
* Reorder
* Rotate
* Compress
* CLI

No AI.

No GUI.

Goal:

A reliable, well-tested document engine.

---

## Phase 2: MCP

Add:

* rmcp
* Typed tool schemas
* MCP server
* Validation
* Error handling

Goal:

Allow AI agents to use PaperPilot.

---

## Phase 3: Desktop

Build:

* Tauri 2
* Svelte 5
* TypeScript
* File management
* Drag and drop
* PDF preview
* Page thumbnails
* Basic operations
* Job system
* Progress

Goal:

A polished standalone PDF application.

---

## Phase 4: AI

Add:

* Natural-language interface
* Local LLM
* llamafile
* MCP-based execution
* Operation planning
* Confirmation UI

Goal:

Users can talk to their documents.

---

## Phase 5: Advanced Intelligence

Add:

* OCR (this is where OCR enters the product)
* Document understanding
* Semantic search
* Table extraction
* Document comparison
* PDF Q&A
* Batch workflows

---

## Phase 6: Enterprise

Add:

* Organizations
* Teams
* RBAC
* SSO
* SAML
* OIDC
* SCIM
* Audit logs
* Security policies
* Central administration
* Enterprise MCP
* On-prem deployment

---

## Phase 7: Managed Cloud

Add:

* Hosted PaperPilot
* Organization management
* Cloud MCP
* Monitoring
* Scaling
* Billing
* Enterprise support

---

# 28. Licensing

The license choice has strategic implications and should be decided before the first public release.

| License | Summary | Tradeoffs |
|---|---|---|
| MIT | Permissive | Simple, widely understood. No patent protection. Cloud providers can use freely without contributing back. |
| Apache-2.0 | Permissive + patent grant | Better for enterprise adoption. Includes explicit patent termination clause. Still allows free cloud use. |
| AGPL-3.0 | Copyleft (network use) | Forces cloud providers to open-source their modifications. Strong community protection. May deter some enterprise users. |

**Recommendation:** Apache-2.0 for the open-source core. It provides patent protection that matters to enterprise customers, while remaining familiar and permissive.

Enterprise/managed components may use a separate commercial license (e.g., BSL, proprietary) if appropriate. The boundary between open-source and commercial code should be defined clearly and documented before launch.

Evaluate all third-party Rust crate licenses before making the final decision. Pay particular attention to any crates with GPL-family licenses.

---

# 29. MVP

Do not build everything initially.

The first MVP should contain:

### Rust

* Merge
* Split
* Extract pages
* Delete pages
* Reorder
* Rotate
* Compress
* CLI

OCR is **not** part of the MVP. It is a Phase 5 capability.

### MCP

* MCP server
* Core PDF tools
* Typed schemas

### Desktop

* Svelte 5
* Tauri 2
* File picker
* Drag and drop
* PDF list
* Basic preview
* Natural-language command box
* Operation plan confirmation
* Progress
* Output management

### AI

* Optional local LLM
* llamafile support
* Natural-language → tool calls

This is enough to demonstrate the complete product concept.

---

# 30. Final Product Positioning

PaperPilot should eventually be presented as:

> **PaperPilot is an open-source, local-first document automation platform for humans and AI agents.**

For individuals:

> Manipulate PDFs using a beautiful desktop application or natural language.

For developers:

> Use the CLI and Rust libraries.

For AI developers:

> Give your agents document capabilities through MCP.

For enterprises:

> Deploy document automation inside your own infrastructure with security, governance, identity, and audit controls.

For organizations that do not want to operate infrastructure:

> Use managed PaperPilot Cloud.

---

# Core Principle

```text
                HUMAN
                  │
                  ▼
            Svelte Desktop
                  │
                  ▼
              Rust Core
                  │
        ┌─────────┴─────────┐
        ▼                   ▼
    PDF Engine             MCP
        │                   │
        ▼                   ▼
  Document Operations    AI Agents
        │                   │
        └─────────┬─────────┘
                  ▼
              AI Layer
                  │
       ┌──────────┼──────────┐
       ▼          ▼          ▼
   llamafile   Local LLM   Cloud API
```

**PaperPilot is open at the core, local by default, AI-native, MCP-compatible, and commercially extensible through enterprise governance and managed infrastructure.**
