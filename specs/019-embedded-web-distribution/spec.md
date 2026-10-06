# Spec 019: Embedded Web Distribution — Drop-In Widget & CMS Plugins

## Status: ACTIVE (Phase 4.9 — v1.0 Co-Launch Target)

## Release Scope: Simultaneous launch with Free Desktop & Cloudflare Edge (v1.0)
## Dependencies: `paperpilot-wasm` core engine (Phase 4.8 ✅, Phase 5.1.1 ✅, Phase 5.5.1 ✅ — 22 Pure-Rust WASM Tools Ready)

---

## 1. Context & Motivation

PaperPilot's WASM engine (`paperpilot-wasm`) already runs 15 PDF operations 100% inside a visitor's browser with zero server load. The next distribution leap is **making this engine embeddable on any website** — turning PaperPilot from a standalone app into infrastructure that every WordPress blog, Webflow agency site, or Shopify store can embed in under 60 seconds.

**The architectural moat that makes this uniquely possible:**
- Other PDF tools require PHP servers, Node backends, or paid cloud APIs — they cannot embed without a server.
- PaperPilot runs 100% client-side via WASM — the embed is simply a branded skin over work already done.
- The Cloudflare Edge microservice (`apps/edge/`, PR #140) handles the few operations (WooCommerce watermarking) that require a server-side trigger.

**Reference:** [`docs/EMBEDDED_WEB_DISTRIBUTION_STRATEGY.md`](../../docs/EMBEDDED_WEB_DISTRIBUTION_STRATEGY.md)

---

## 2. The Three Distribution Models

### Model 1: Universal Drop-In Embed (`embed.js`)
Works on **any** website: Webflow, Framer, Squarespace, Wix, raw HTML.

```html
<!-- Paste these 2 lines anywhere on your site -->
<script src="https://cdn.paperpilot.app/v1/embed.js" async></script>
<div
  id="paperpilot-portal"
  data-theme="dark"
  data-tools="merge,compress,watermark,split"
  data-brand-color="#2563EB"
  data-logo-url="https://clientfirm.com/logo.png">
</div>
```

### Model 2: WordPress & WooCommerce Plugin (`paperpilot-wp`)
- Covers 43% of the web (800M+ sites).
- Gutenberg Block + Shortcode: `[paperpilot_tools category="all"]`.
- WooCommerce: Auto-watermark & encrypt PDF purchases at checkout.

### Model 3: Shopify App ("Secure PDF Delivery")
- 4.5M+ online merchants.
- Auto-watermarks and encrypts digital PDF downloads with order metadata.
- Powered by Cloudflare Edge API (< 10ms, zero-disk).

---

## 3. Architecture

```
PaperPilot/
├── paperpilot-wasm/              # Core WASM engine (already exists)
├── apps/
│   ├── web/                      # Standalone web app (already exists)
│   ├── edge/                     # Cloudflare Workers microservice (already exists)
│   └── embed/                    # NEW: Embeddable widget (Phase 4.9)
│       ├── src/
│       │   ├── embed.ts          # Tiny loader script (< 5KB gzip)
│       │   ├── Widget.svelte     # Brandable Svelte 5 UI component
│       │   └── theme.ts          # CSS custom property token system
│       ├── tests/
│       │   └── e2e_embed.spec.ts # Playwright embed E2E suite
│       └── vite.config.ts
└── plugins/
    └── wordpress/                # NEW: PHP WordPress plugin (Phase 4.9.5+)
        └── paperpilot-wp/
            ├── paperpilot-wp.php
            ├── blocks/           # Gutenberg block
            └── includes/
                └── woocommerce.php
```

---

## 4. Task Breakdown

### 4.9.1 — `embed.js` Universal CDN Loader Script

**File:** `apps/embed/src/embed.ts` → compiled to `dist/embed.js`

**Responsibilities:**
1. Read configuration from the host `<div>` element's `data-*` attributes:
   - `data-tools` — comma-separated list (e.g. `"merge,compress"`)
   - `data-theme` — `"dark"` | `"light"` (default: `"dark"`)
   - `data-brand-color` — hex color (default: PaperPilot blue `#3B82F6`)
   - `data-logo-url` — optional custom logo image URL
2. Create a **Shadow DOM root** inside the target `<div>` to isolate styles from the host site.
3. Lazy-load `paperpilot_wasm_bg.wasm` from `https://cdn.paperpilot.app/v1/`.
4. Mount the Svelte 5 `Widget.svelte` component into the shadow root with configuration.

**Bundle target:** `< 5KB` gzip (loader only; WASM is fetched separately on demand).

---

### 4.9.2 — Brandable Svelte 5 Embed Widget (`apps/embed/`)

**File:** `apps/embed/src/Widget.svelte`

A configurable version of `apps/web/` adapted for embedding:
- Accepts `tools`, `theme`, and `brandColor` as props.
- Renders only the tool cards corresponding to the `data-tools` list.
- Shows the "⚡ Powered by PaperPilot" badge in the footer (removable in Pro tier via `data-hide-badge="true"`).
- Fully responsive at any container width (adapts from 300px mobile to 1200px desktop layouts).
- Reuses the same Web Worker bridge as `apps/web/` — no duplicated WASM logic.

---

### 4.9.3 — Shadow DOM Isolation & CSS Theme Tokens

The widget renders inside a **Shadow DOM root** (`element.attachShadow({ mode: 'open' })`).

**CSS Custom Properties (theme tokens):**
```css
:host {
  --pp-brand-color: #3B82F6;
  --pp-background: #0f0f13;
  --pp-surface: #1a1a2e;
  --pp-text: #e2e8f0;
  --pp-radius: 12px;
  --pp-font: 'Inter', sans-serif;
}
```

These tokens are injected dynamically from `data-brand-color` and `data-theme` by `embed.js` so the widget matches the host site's brand without CSS leakage.

---

### 4.9.4 — JavaScript Embed API (`window.PaperPilot`)

For advanced users who want programmatic control (React/Vue/Angular apps):

```js
// Mount
window.PaperPilot.mount('#my-container', {
  tools: ['merge', 'compress'],
  theme: 'light',
  brandColor: '#7C3AED',
});

// Listen for completion events
window.PaperPilot.on('complete', ({ tool, outputSizeBytes }) => {
  console.log(`${tool} done — ${outputSizeBytes} bytes`);
});

// Unmount
window.PaperPilot.unmount('#my-container');
```

---

### 4.9.5 — WordPress Plugin: Gutenberg Block

**Directory:** `plugins/wordpress/paperpilot-wp/`

**Plugin features:**
- **Gutenberg Block** (`PaperPilot PDF Portal`): Visual block editor with tool multi-select, theme picker, and brand color swatch in the block settings sidebar.
- **Shortcode**: `[paperpilot_tools tools="merge,compress" theme="dark"]`
- **No server required**: The block renders the `<div id="paperpilot-portal" ...>` HTML and enqueues `embed.js` from CDN.
- **Publication target**: `wordpress.org/plugins/paperpilot` (free organic discovery across 800M+ sites).

---

### 4.9.6 — WooCommerce Digital Downloads Integration

**Trigger:** WooCommerce `woocommerce_order_status_completed` PHP action hook.

**Flow:**
1. Order completes → WooCommerce fires the hook with `order_id`, `customer_email`, `product_id`.
2. Plugin fetches the attached PDF product file.
3. Calls `POST https://edge.paperpilot.app/api/v1/pdf/watermark` (Cloudflare Edge API) with:
   - `input_pdf`: PDF bytes.
   - `text`: `"Licensed to {email} — Order #{order_id}"`.
   - Optional: `POST .../pdf/encrypt` to password-protect with customer postal code.
4. Delivers the watermarked/encrypted PDF as the customer's secure download link.

**Privacy guarantee:** No PDF content is stored on PaperPilot servers. Cloudflare Edge processes purely in-memory and streams the result back.

---

### 4.9.7 — Webflow App & Framer Component

- **Webflow App**: Registered in Webflow Marketplace. Injects `embed.js` via a `<script>` tag and provides a no-code panel for tool selection, theming, and brand color.
- **Framer Component**: Published to Framer Community. Accepts tool selection and brand color as component props and renders the live embed widget on the Framer canvas.

---

### 4.9.8 — Shopify App ("Secure PDF Delivery")

**Shopify App Bridge** integration:
- Registers an `orders/fulfilled` webhook in the Shopify Admin.
- On fulfillment, calls the Cloudflare Edge API to watermark the PDF with `{customer_name} — Order #{order_number}`.
- Updates the digital download link with the processed file.
- Published to Shopify App Store as `"Secure PDF Delivery by PaperPilot"`.

---

### 4.9.9 — Embed E2E Playwright Test Suite

**File:** `apps/embed/tests/e2e_embed.spec.ts`

Test cases:
1. **Default mount** — inject `embed.js` into a bare HTML fixture page, assert Shadow DOM root is created.
2. **Tool filtering** — set `data-tools="merge"`, assert only the Merge card is rendered.
3. **Theme tokens** — set custom `data-brand-color`, assert `--pp-brand-color` CSS var is applied inside shadow root.
4. **Merge E2E** — drag two test PDFs into the embed widget, trigger merge, assert output download is a valid `%PDF-` binary.
5. **JS API** — call `window.PaperPilot.mount(...)`, assert successful mount, register `complete` event listener, verify it fires, call `.unmount()`.
6. **Badge visibility** — assert badge present by default; hidden when `data-hide-badge="true"`.

---

### 4.9.10 — Embed Analytics & Upgrade Funnel

**Privacy-first telemetry** (no file content, no PII):
```json
{
  "event": "tool_used",
  "tool": "merge",
  "file_size_bucket": "1MB-5MB",
  "success": true,
  "embed_origin_domain": "example.com"
}
```

Sent to a lightweight Cloudflare Worker analytics endpoint. Powers the conversion funnel from the "⚡ Powered by PaperPilot" badge click-through to `paperpilot.app` for Desktop or Cloud Pro subscription.

---

## 5. Monetization Tiers

| Tier | Offering | Price |
|---|---|---|
| **Community Embed** | All 15 tools, "⚡ Powered by PaperPilot" badge, CDN hosted | **$0/month** |
| **White-Label Pro** | Custom brand color, custom logo, badge removal, 1 site | **$19/site/month** |
| **Agency Unlimited** | All Pro features, unlimited sites | **$79/month** |
| **WooCommerce / Shopify** | Automated watermarking & encryption at checkout | **$15/month** |

---

## 6. File Ownership

| Path | Owner |
|---|---|
| `apps/embed/` | Phase 4.9.1 – 4.9.4 |
| `apps/embed/tests/` | Phase 4.9.9 |
| `plugins/wordpress/paperpilot-wp/` | Phase 4.9.5 – 4.9.6 |
| `reports/EMBED_WIDGET_REPORT.md` | Generated on completion |
| `wiki/13-Embedded-Web-Distribution.md` | Generated on completion |

> **DO NOT touch:** `paperpilot-wasm/`, `apps/web/`, `apps/desktop/`, `apps/edge/`, or any Rust crate. This phase is purely a distribution layer over the existing WASM engine.

---

## 7. Exit Criteria

- [ ] `embed.js` < 5KB gzip, served via Cloudflare CDN.
- [ ] Embed widget renders correctly on Webflow, WordPress, and raw HTML test fixtures.
- [ ] All 6 E2E Playwright embed tests pass (`apps/embed/tests/`).
- [ ] WordPress plugin installable from `.zip` and activates without PHP errors.
- [ ] WooCommerce watermarking webhook processes a test order end-to-end correctly.
- [ ] `reports/EMBED_WIDGET_REPORT.md` documents the full delivery.
- [ ] `wiki/13-Embedded-Web-Distribution.md` is published.
