# Spec 105: Embed Widget Tool Parity & Categorized Embeds (`apps/embed` & `/embed-test`)

## 1. Background & Executive Summary
PaperPilot provides a drop-in web component / Shadow DOM widget (`apps/embed`) that developers can embed on any website via `<div data-paperpilot-widget ...></div><script src="/embed.js"></script>`.

Currently, the embed widget only exposes 8 basic PDF operations and is tested via `/embed-test` (`apps/web/public/embed-test.html`). It lacks the newly completed Wave 2 pure-Rust operations:
- **Imposition**: N-Up Grid (`pdf_nup`), Booklet Imposition (`pdf_booklet`), Dimensional Scaling (`pdf_scale_pages`), Auto Upright (`pdf_auto_rotate`).
- **Stationery & Stamping**: Image & Logo Stamp (`pdf_stamp`), Stationery Overlay/Underlay (`pdf_overlay_underlay`).
- **Security & Forensics**: Digital Signature Verification & Baseline Tamper Forensics (`pdf_validate_signatures`), Signature Stripping (`pdf_remove_signatures`).

**Spec 105** upgrades `apps/embed` to support:
1. **Categorized Widget Embedding**: Allow developers to embed specific tool suites via `data-category="pages|edit|security|optimize|convert"` or custom comma-separated tool lists (`data-tools="pdf_nup,pdf_booklet,pdf_scale_pages"`).
2. **Interactive Parameter Controls**: Provide inline controls inside the Shadow DOM for grid imposition, format scaling, image stamping, and signature verification.
3. **Comprehensive `/embed-test` Demonstrations**: Update the live sandbox with dedicated showcase cards for Imposition, Contract Forensics, and multi-step pipeline automation.

---

## 2. Widget Configuration & Attributes

| Attribute | Options | Description |
|---|---|---|
| `data-category` | `pages`, `edit`, `optimize`, `security`, `convert`, `all` | Automatically loads all tools belonging to that category |
| `data-tools` | e.g. `pdf_nup,pdf_booklet,pdf_scale_pages` | Comma-separated list of specific tools to activate |
| `data-pipeline` | JSON string of pipeline steps | Multi-step automated pipeline executed in memory |
| `data-theme` | `dark`, `light`, `auto` | Visual theme |
| `data-brand-color` | Hex string (e.g. `#3B82F6`) | Accent color for buttons, badges, and highlights |
| `data-hide-badge` | `true`, `false` | White-label mode (removes "Powered by PaperPilot" badge) |

---

## 3. Tool Expansion in `apps/embed/src/Widget.svelte`

Expand `allTools` to include:
```typescript
const allTools = [
  // Page Operations & Imposition
  { id: 'merge', name: 'Merge PDFs', category: 'pages', description: 'Combine multiple PDFs into one' },
  { id: 'split', name: 'Split PDF', category: 'pages', description: 'Split by page range' },
  { id: 'rotate', name: 'Rotate Pages', category: 'pages', description: 'Rotate pages clockwise' },
  { id: 'extract', name: 'Extract Pages', category: 'pages', description: 'Extract specific pages or range' },
  { id: 'delete', name: 'Delete Pages', category: 'pages', description: 'Remove specific pages or range' },
  { id: 'reorder', name: 'Reverse Order', category: 'pages', description: 'Reverse page sequence' },
  { id: 'pdf_nup', name: 'N-Up Imposition', category: 'pages', description: 'Combine multiple pages per sheet (2, 4, 9, 16)' },
  { id: 'pdf_booklet', name: 'Booklet Imposition', category: 'pages', description: 'Impose pages for fold-and-staple booklet printing' },
  { id: 'pdf_scale_pages', name: 'Scale Dimensions', category: 'pages', description: 'Resize pages to A4, Letter, Legal, or percentage' },
  { id: 'pdf_auto_rotate', name: 'Auto Upright', category: 'pages', description: 'Automatically upright orientation for landscape pages' },

  // Edit & Stationery
  { id: 'watermark', name: 'Watermark', category: 'edit', description: 'Add custom watermark stamp' },
  { id: 'pdf_stamp', name: 'Image Stamp', category: 'edit', description: 'Stamp coordinate-exact PNG/JPEG logos and badges' },
  { id: 'pdf_overlay_underlay', name: 'Stationery Overlay', category: 'edit', description: 'Apply letterheads and stationery background/foreground' },

  // Optimize
  { id: 'compress', name: 'Compress PDF', category: 'optimize', description: 'Compress and optimize PDF streams' },

  // Security & Forensics
  { id: 'pdf_validate_signatures', name: 'Validate Signatures', category: 'security', description: 'Verify digital signatures and baseline tamper detection' },
  { id: 'pdf_remove_signatures', name: 'Strip Signatures', category: 'security', description: 'Remove signature fields and cryptographic locks' }
];
```

---

## 4. Updates to `/embed-test` (`apps/web/public/embed-test.html`)

Add four distinct interactive embed showcases:
1. **Showcase 1: Imposition & Layout Suite**:
   ```html
   <div data-paperpilot-widget data-theme="dark" data-tools="pdf_nup,pdf_booklet,pdf_scale_pages,pdf_auto_rotate" data-brand-color="#6366F1"></div>
   ```
2. **Showcase 2: Digital Signature & Tamper Detection Forensics**:
   ```html
   <div data-paperpilot-widget data-theme="dark" data-tools="pdf_validate_signatures,pdf_remove_signatures" data-brand-color="#10B981"></div>
   ```
3. **Showcase 3: Classic Page Editing & Optimization**:
   ```html
   <div data-paperpilot-widget data-theme="dark" data-tools="merge,split,rotate,watermark,compress" data-brand-color="#3B82F6"></div>
   ```
4. **Showcase 4: Automated Multi-Step Pipeline**:
   ```html
   <div data-paperpilot-widget data-theme="dark" data-pipeline='[{"tool":"pdf_auto_rotate"},{"tool":"pdf_scale_pages","target":"a4"},{"tool":"pdf_nup","grid":2},{"tool":"compress"}]'></div>
   ```

---

## 5. Implementation Phases
- **Task 105A**: Widget Tool Parity, Category Filtering, and Parameter Wiring (`apps/embed`).
- **Task 105B**: `/embed-test.html` Showcase Upgrade and Sandbox Verification.
