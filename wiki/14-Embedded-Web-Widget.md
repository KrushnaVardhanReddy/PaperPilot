# Embedded Web Widget

The PaperPilot Embedded Web Widget is a lightweight, drop-in UI that brings 100% client-side PDF processing to any website. Built with Svelte 5 and isolated in a Shadow DOM, it requires no server and guarantees user privacy.

## 60-Second Quickstart

To embed the widget on your website, add the following two lines of HTML anywhere you want the widget to appear:

```html
<!-- Paste these 2 lines anywhere on your site -->
<script src="https://cdn.paperpilot.app/v1/embed.js" async></script>
<div id="paperpilot-portal" data-theme="dark" data-tools="merge,compress" data-brand-color="#2563EB"></div>
```

## Configuration (`data-*` Attributes)

You can customize the widget natively through standard HTML data attributes:

| Attribute | Type | Default | Example | Description |
|---|---|---|---|---|
| `data-tools` | string | `"all"` | `"merge,compress"` | Comma-separated list of PDF tools to show. |
| `data-theme` | string | `"system"` | `"dark"` | Color scheme (`light`, `dark`, or `system`). |
| `data-brand-color` | string | `"#3B82F6"` | `"#ff00ff"` | Custom accent color for buttons and hover states. |
| `data-logo-url` | string | `undefined` | `"https://example.com/logo.png"` | Optional URL to display a logo above the dropzone. |
| `data-hide-badge` | boolean | `false` | `"true"` | Hides the 'Powered by PaperPilot' badge (Requires Pro). |

## JavaScript Programmatic API

For advanced users and SPA frameworks (React, Vue, Svelte), PaperPilot exposes a programmatic API via `window.PaperPilot`:

```typescript
// Define Mount Options
interface EmbedOptions {
  tools?: string | string[];
  theme?: 'light' | 'dark' | 'system';
  brandColor?: string;
  logoUrl?: string;
  hideBadge?: boolean;
}

// 1. Mount the widget
const instance = window.PaperPilot.mount('#my-container', {
  tools: ['merge', 'compress'],
  theme: 'light',
  brandColor: '#7C3AED',
});

// 2. Listen for completion events
window.PaperPilot.on('process-complete', ({ tool, outputSizeBytes }) => {
  console.log(`${tool} done — ${outputSizeBytes} bytes`);
});

// 3. Unmount the widget
window.PaperPilot.unmount('#my-container');
```

## Events Reference

The widget emits native `CustomEvent`s which bubble up the DOM. You can listen to them on `document`, `window`, or via `PaperPilot.on()`:

| Event Name | Detail Payload | Description |
|---|---|---|
| `paperpilot:ready` | `{}` | Fired when the widget has successfully mounted. |
| `paperpilot:process-start` | `{ tool: string }` | Fired when a file processing operation begins. |
| `paperpilot:process-complete` | `{ tool: string, outputSizeBytes: number }` | Fired upon successful processing and file generation. |
| `paperpilot:error` | `{ tool: string, message: string }` | Fired if a processing operation fails. |

## CMS Quick-Links

Integrating into a CMS? We provide plugins and native apps for major platforms:

- **WordPress:** Read the [WordPress Plugin Guide](#) for shortcodes and Gutenberg block usage.
- **WooCommerce:** Read the [WooCommerce Integration Guide](#) for automatic PDF watermarking at checkout.
- **Webflow:** Install our [Webflow App](#) to visually configure the embed.
- **Shopify:** Install [Secure PDF Delivery by PaperPilot](#) to protect digital downloads.
- **Squarespace & Wix:** The HTML quickstart snippet above works natively out of the box!
