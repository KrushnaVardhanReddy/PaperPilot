# Documentation Architecture & Strategy: `docs.usepaperpilot.com`

> **Document Purpose:** Blueprint for PaperPilot's official documentation hub, developer integration guides, and 44-tool API/MCP reference.

---

## 1. Executive Overview

PaperPilot's documentation serves three distinct audiences:
1. **End Users & Power Users:** Desktop app installation, offline AI chat, batch watch folders.
2. **Webmasters, Agencies & No-Code Designers:** 60-second drop-in `embed.js` widget, WordPress Gutenberg block, Shopify apps.
3. **Software Engineers & AI Developers:** Headless CLI, interactive Swagger UI, OpenAPI 3.1 specification, and Model Context Protocol (MCP) tool integration with Claude Desktop / Cursor.

---

## 2. The 4-Pillar Documentation Structure

```
                               ┌─────────────────────────────┐
                               │     PaperPilot Docs Hub     │
                               └──────────────┬──────────────┘
                                              │
         ┌───────────────────┬────────────────┴───────────────────┬───────────────────┐
         ▼                   ▼                                    ▼                   ▼
     Pillar 1:           Pillar 2:                            Pillar 3:           Pillar 4:
  Getting Started      Master 44-Tool                       Embed.js & CMS       API, Swagger &
  & Core Guides          Reference                           Integration          MCP Protocol
  ───────────────     ─────────────────                    ─────────────────    ─────────────────
  • Quickstart        • Page Operations                    • 2-line Drop-in     • Swagger UI
  • Desktop Install   • Security & Encryption              • Shadow DOM & CSS   • OpenAPI 3.1 Spec
  • CLI Scaffolding   • Pure-Rust OCR & Render             • WordPress & CMS    • Claude/Cursor
  • Privacy Guarantee • Forms & Compression                • Event Listeners      MCP Integration
```

---

## 3. Detailed Pillar Breakdown

### Pillar 1: Getting Started & Core Guides
* **The Zero-Cloud Privacy Guarantee:** Explaining why memory-only processing in Rust and client-side WASM completely eliminates data leak liability (GDPR, HIPAA, SOC2).
* **Desktop App Installation:** Guides and download links for Windows (`.msi`), macOS (`.dmg`), and Linux (`.deb` / `.AppImage`).
* **CLI Quickstart:** Installing `paperpilot` via cargo / binary, piping stdout, and JSON output formatting.

### Pillar 2: The Master 44-Tool Reference Handbook
*(Derived from `reports/TRI_INTERFACE_E2E_100_VERIFIED.md` and `reports/TRI_INTERFACE_E2E_AND_DOCS.md`)*
Every single tool features 3 instant copy-pasteable snippets:
1. **CLI Command:** `paperpilot compress --input doc.pdf --level high`
2. **cURL REST API:** `curl -X POST http://localhost:7823/api/v1/pdf/compress -H "Content-Type: application/json" -d '{"input": "doc.pdf"}'`
3. **MCP Tool Call JSON:** `{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "pdf_compress", "arguments": {"input": "doc.pdf"}}}`

### Pillar 3: Embedded Web Widget (`embed.js`)
* **60-Second Quickstart:** The 2 lines of HTML for Webflow, Framer, Squarespace, Wix, and plain HTML.
* **Attributes Reference:** Complete table of configuration options (`data-tools`, `data-theme`, `data-brand-color`, `data-logo-url`, `data-hide-badge`).
* **JavaScript Programmatic API:** `PaperPilot.mount()`, `PaperPilot.on('process-complete', callback)`, `PaperPilot.unmount()`.
* **CMS Plugins:** WordPress Gutenberg block, WooCommerce checkout auto-watermarking, and Shopify app integration.

### Pillar 4: Developer API, Swagger & MCP Protocol
* **Interactive Swagger UI:** Embedded live console for exploring all 14+ REST endpoints.
* **OpenAPI 3.1 Specification:** Downloadable `/api-docs/openapi.json` for generating SDKs in Python, Node.js, Go, or Java.
* **AI Agent Model Context Protocol (MCP):** Configuration snippets for connecting PaperPilot to **Claude Desktop (`claude_desktop_config.json`)**, **Cursor IDE**, and **Goose**.

---

## 4. Hosting, Tooling & Deployment

* **Framework:** Integrated `/docs` route inside `apps/web/` (or dedicated Astro Starlight in `apps/docs/`).
* **Hosting:** Cloudflare Pages (Free tier, unlimited bandwidth, global edge caching).
* **URL:** `docs.usepaperpilot.com` (or `usepaperpilot.com/docs`).
* **Search:** Local client-side instant full-text search powered by Pagefind or embedded SQLite-vec RAG.
