# Architecture & Strategy: `usepaperpilot.com` Official Web Portal

> **Purpose:** Blueprint for the official marketing, dogfooding playground, and developer distribution portal for PaperPilot (`usepaperpilot.com`).

---

## 1. Core Philosophy: "The Product IS the Landing Page"

Rather than a static marketing brochure, `usepaperpilot.com` serves as the ultimate live demonstration of PaperPilot's pure-Rust client-side WASM engine. Visitors immediately interact with the engine in under 1 second without signup, login, or uploading files.

---

## 2. Page Section Architecture

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
│ 3. THE TRI-SURFACE SHOWCASE                                                 │
│    ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐     │
│    │   1. Desktop     │    │   2. Drop-In     │    │  3. Cloud API &  │     │
│    │   Application    │    │   Web Embed      │    │     Swagger      │     │
│    │  Offline GUI &   │    │  2 lines of HTML │    │  REST + MCP for  │     │
│    │  Local AI Chat   │    │  for any website │    │  AI agents       │     │
│    └──────────────────┘    └──────────────────┘    └──────────────────┘     │
├─────────────────────────────────────────────────────────────────────────────┤
│ 4. INTERACTIVE EMBED GENERATOR (For Webmasters & Agencies)                  │
│    Interactive controls: select tools, theme, brand color, logo.            │
│    Instant output: Copy-pasteable 2-line HTML snippet with live preview.    │
├─────────────────────────────────────────────────────────────────────────────┤
│ 5. INTERACTIVE API & SWAGGER EXPLORER (For Backend Developers)              │
│    Interactive cURL / TypeScript / Python snippets linking to `/swagger-ui`  │
│    and OpenAPI 3.1 JSON specification.                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│ 6. THE SPEED & PRIVACY BENCHMARK MATRIX                                     │
│    Side-by-side comparison vs Adobe Acrobat, iLovePDF, and Stirling PDF.     │
│    Highlights: Pure-Rust (<10ms), zero server storage, Apache 2.0 license.   │
├─────────────────────────────────────────────────────────────────────────────┤
│ 7. FOOTER                                                                   │
│    Docs · GitHub · Discord · Privacy Policy · Apache 2.0 License            │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Technical Implementation & Hosting

* **Framework:** Svelte 5 + Vite + Vanilla CSS (located in `apps/web/`).
* **Hosting:** Cloudflare Pages (Direct Git deployment).
* **Cost:** **$0.00 / month** (Unlimited global bandwidth, 300+ edge locations).
* **Domain DNS:** Point `usepaperpilot.com` and `www.usepaperpilot.com` to Cloudflare Pages.
* **CDN Subdomain:** `cdn.usepaperpilot.com` serves static `embed.js` and `paperpilot_wasm_bg.wasm`.

---

## 4. Key Funnels & Conversions

1. **Consumer & Casual Users:** Solves immediate PDF problem in the Hero playground → prompt to download Desktop for 44 offline tools and AI assistant.
2. **Webmasters & Agencies:** Use the Embed Code Generator → paste into WordPress/Webflow in 60s → viral "⚡ Powered by PaperPilot" badge spreads brand.
3. **Enterprise & Developers:** Explore Swagger/API explorer → Join Developer API Waitlist / explore self-hosted Apache 2.0 gateway.
