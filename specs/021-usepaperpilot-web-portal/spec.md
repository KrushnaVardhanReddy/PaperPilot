# Spec 021: `usepaperpilot.com` Official Portal & Tri-Surface Landing Page

## Status: ACTIVE (Task 4.9.11 — v1.0 Co-Launch Target)

## 1. Context & Motivation
PaperPilot requires an official marketing and conversion portal for `usepaperpilot.com`. Rather than a static brochure, the landing page must follow the principle: **"The Product IS the Landing Page"**.

The portal must:
1. Provide an immediate **Live Playground** in the Hero section where visitors drop PDFs and experience the pure-Rust 22-tool WASM engine in <10ms with 0 bytes sent to the cloud.
2. Feature an **Interactive Embed Code Generator** enabling webmasters, agencies, and Shopify/WordPress merchants to customize tool selections, themes, and brand colors and copy 2 lines of HTML.
3. Feature a **Tri-Surface Showcase & Download Center** directing visitors to:
   - **Desktop App** (Windows `.msi`, macOS `.dmg`, Linux `.deb`/`.AppImage` with 44 tools & local AI chat).
   - **Drop-In Embed Widget** (`embed.js`).
   - **Developer API & MCP Protocol** (Swagger UI `/swagger-ui` & OpenAPI 3.1 JSON).

---

## 2. Page Architecture (`apps/web/src/`)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. NAVBAR                                                                   │
│    [Logo] PaperPilot     Tools ▾    Docs / API    Pricing    GitHub ★    [Download App]
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. HERO: "THE DOGFOODING PLAYGROUND"                                         │
│    Headline: "The Pure-Rust, Zero-Cloud PDF Powerhouse."                     │
│    Subheadline: "Merge, compress, and edit PDFs 100% locally in your browser.│
│                 No files uploaded. Ever."                                   │
│                                                                             │
│    ┌───────────────────────────────────────────────────────────────────┐    │
│    │  [ LIVE EMBED WIDGET PLAYGROUND ]                                 │    │
│    │  (Drop PDF here to test: Merge, Compress, OCR, Watermark...)       │    │
│    │  ⚡ 100% Client-Side WASM · 0 Bytes Sent to Cloud                 │    │
│    └───────────────────────────────────────────────────────────────────┘    │
│    CTAs: [ Download Desktop (Linux/Mac/Win) ]    [ Get Embed Code < / > ]   │
├─────────────────────────────────────────────────────────────────────────────┤
│ 3. THE TRI-SURFACE SHOWCASE (Desktop / Embed / Cloud API)                   │
├─────────────────────────────────────────────────────────────────────────────┤
│ 4. INTERACTIVE EMBED BUILDER & CODE GENERATOR                               │
│    Left: Controls (Theme, Brand Color, Tool Selection Checkboxes)           │
│    Right: Live Code snippet (<script src=".../embed.js">) + Copy Button     │
├─────────────────────────────────────────────────────────────────────────────┤
│ 5. INTERACTIVE API & SWAGGER EXPLORER                                       │
│    Tabbed cURL, TypeScript, and Python snippets linking to `/swagger-ui`     │
├─────────────────────────────────────────────────────────────────────────────┤
│ 6. SPEED & PRIVACY BENCHMARK MATRIX (vs Adobe Acrobat, iLovePDF, Stirling)  │
├─────────────────────────────────────────────────────────────────────────────┤
│ 7. FOOTER                                                                   │
│    Downloads · Documentation · GitHub · Privacy Policy · Apache 2.0         │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Component Hierarchy

* `apps/web/src/App.svelte` — Main shell with sticky top navbar, smooth scrolling sections, and footer.
* `apps/web/src/components/HeroPlayground.svelte` — Hero headline with integrated drag-and-drop 22-tool WASM engine.
* `apps/web/src/components/TriSurfaceShowcase.svelte` — Cards showcasing Desktop, Drop-In Embed, and Headless API.
* `apps/web/src/components/EmbedGenerator.svelte` — Reactive embed code configurator with live 2-line snippet generation.
* `apps/web/src/components/ApiExplorer.svelte` — Interactive code switcher (cURL / TS / Python / MCP JSON) with Swagger link.
* `apps/web/src/components/BenchmarkMatrix.svelte` — Competitive speed and zero-disk privacy comparison table.

---

## 4. Verification & Quality Gates
1. **Type Checking**: `npm --prefix apps/web run check` passes with 0 errors and 0 warnings.
2. **Production Build**: `npm --prefix apps/web run build` compiles clean Vite bundle.
3. **E2E Playwright**: Playwright test asserting navigation, Hero tool execution, and Embed snippet copy functionality.
