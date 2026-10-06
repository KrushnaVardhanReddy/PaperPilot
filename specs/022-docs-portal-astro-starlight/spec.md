# Spec 022: `docs.usepaperpilot.com` Official Documentation Hub (Astro Starlight)

## Status: ACTIVE (Task 4.9.12 — v1.0 Co-Launch Target)

## 1. Context & Motivation
PaperPilot is launching with 3 complementary surfaces:
1. **Free Desktop App** (Tauri + Svelte 5, 44 operations, local AI assistant, zero cloud).
2. **Drop-In Embed Widget** (`embed.js` + `apps/embed/`, Shadow DOM isolated, pure WASM in client browser).
3. **Developer API & MCP Server** (Headless CLI, REST Gateway with Swagger UI, Model Context Protocol for Claude / Cursor).

To give developers, agencies, and users a world-class documentation hub at **`docs.usepaperpilot.com`** without adding maintenance overhead or server costs, we use **Astro Starlight** inside `apps/docs/`. It renders markdown documentation statically, provides instant client-side search via Pagefind, supports built-in code highlighting and copy buttons, and deploys directly to Cloudflare Pages for free ($0/mo).

---

## 2. Architecture & File Structure (`apps/docs/`)

```
apps/docs/
├── astro.config.mjs          # Starlight config, title, sidebar taxonomy, social links
├── package.json              # astro, @astrojs/starlight, pagefind
├── tsconfig.json             # Astro TypeScript configuration
├── public/
│   └── favicon.svg           # PaperPilot paper plane logo
└── src/
    ├── assets/               # Brand assets & diagrams
    └── content/
        └── docs/
            ├── index.mdx                       # Docs Hub Homepage & quick cards
            ├── getting-started/
            │   ├── quickstart.md               # 2-minute overview across CLI, Desktop, Web
            │   ├── desktop-installation.md     # Win (.msi), Mac (.dmg), Linux (.deb/.AppImage)
            │   ├── zero-cloud-privacy.md       # Privacy guarantee, memory isolation, HIPAA/GDPR
            │   └── cli-guide.md                # CLI install, options, piping stdout/stderr
            ├── embed/
            │   ├── quickstart.md               # 2-line HTML drop-in snippet
            │   ├── configuration.md            # data-* attributes reference & branding
            │   ├── programmatic-api.md         # PaperPilot.mount(), event listeners
            │   └── cms-integration.md          # WordPress, Webflow, Shopify, Framer
            ├── reference/
            │   ├── 44-tools-handbook.md        # Master 44-tool index with CLI, REST & MCP
            │   ├── page-operations.md          # Merge, split, rotate, delete, extract, reorder
            │   ├── conversion-render.md        # OCR (ocrs), Render (hayro), Images to PDF
            │   └── security-metadata.md        # Password encrypt/decrypt, watermark, hash
            └── developer/
                ├── swagger-and-openapi.md      # Swagger UI (/swagger-ui) & OpenAPI 3.1 JSON
                └── mcp-agent-integration.md    # Claude Desktop & Cursor MCP config
```

---

## 3. Sidebar Taxonomy (4 Pillars)

The Starlight sidebar is organized into 4 logical pillars:

1. **Getting Started & Core Guides**:
   - Quickstart
   - Desktop App Installation
   - Zero-Cloud Privacy Guarantee
   - Headless CLI Guide
2. **Embed.js Drop-In Widget**:
   - 60-Second Quickstart
   - Configuration Attributes (`data-theme`, `data-tools`, `data-brand-color`, `data-hide-badge`)
   - JavaScript API & Event Listeners
   - CMS Integration (WordPress, Webflow, Shopify)
3. **Master 44-Tool Reference**:
   - Comprehensive reference with CLI flags, cURL payload, and MCP JSON-RPC format for all 44 tools (sourced from `reports/TRI_INTERFACE_E2E_100_VERIFIED.md`).
4. **Developer Gateway & AI Agents**:
   - REST API & Interactive Swagger UI
   - OpenAPI 3.1 Spec export
   - Model Context Protocol (MCP) Setup for Claude Desktop and Cursor

---

## 4. Cloudflare Pages Deployment
- Framework preset: `Astro`
- Build command: `npm --prefix apps/docs run build`
- Output directory: `apps/docs/dist`
- Custom Domain: `docs.usepaperpilot.com`
- Cost: $0/month (Cloudflare Pages unlimited bandwidth).

---

## 5. Verification & Acceptance Criteria
1. `npm --prefix apps/docs install && npm --prefix apps/docs run build` builds cleanly without warnings or broken links.
2. Search indexing (Pagefind) generates valid search bundles in `apps/docs/dist/pagefind/`.
3. High-contrast dark/light theme toggle, mobile drawer navigation, and copy-to-clipboard code blocks work out of the box.
4. All 44 operations documented with accurate parameter tables and snippets.
