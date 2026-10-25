# Spec 106: Embed Studio Experience & Portal Parity (`apps/embed` & `/embed-test`)

## 1. Background & Executive Summary
In Spec 104, PaperPilot's web portal (`apps/web`) introduced a sleek, categorized macOS/Studio window frame with:
- Top macOS window dots (`● ● ●`) and title bar
- Categorized functional filter tabs (`⭐ All Tools`, `📑 Pages & Imposition`, `🎨 Edit & Stationery`, `🗜️ Optimize`, `🔒 Security`, `🔁 Convert`)
- Search filtering and live tool count badges
- Structured configuration panels for all 30+ WASM operations (N-Up grid selection, scale format dropdowns, orientation toggles, image stamping, digital signature audit reports).

However, the standalone drop-in embed widget (`apps/embed`) and its `/embed-test` sandbox still render a legacy flat layout: a bare dotted upload box with plain text buttons underneath, missing the entire studio structure, categories, and parameter controls.

**Spec 106** aligns `apps/embed` with the **PaperPilot WASM Engine** studio design language while supporting dual embedding modes:
1. **Full Portal / Category Studio Mode** (`data-layout="studio"` or default when multiple tools/categories are embedded):
   - Renders the macOS window frame (`● ● ●`), title bar, categorized tool tabs, search bar, and active tool header.
   - Embeds seamlessly into client websites, developer documentation, and enterprise portals.
2. **Compact Single-Tool Card Mode** (`data-layout="compact"`):
   - Streamlined single-action card for dedicated drop-in tasks (e.g. just a fast PDF Compressor or Contract Signature Validator).

---

## 2. Technical Architecture & Component Redesign

In `apps/embed/src/`:

### 2.1 Studio Layout in `Widget.svelte`
Upgrade `Widget.svelte` template to adopt the studio structure:

```svelte
<div class="pp-widget-container" class:pp-studio={layout === 'studio'} data-theme={theme}>
  {#if layout === 'studio'}
    <!-- Studio Header Bar -->
    <div class="pp-studio-header">
      <div class="pp-window-dots">
        <span class="dot close"></span>
        <span class="dot minimize"></span>
        <span class="dot maximize"></span>
      </div>
      <div class="pp-studio-title">
        {customTitle || 'PaperPilot Embed Studio — 100% Client-Side WASM'}
      </div>
      {#if !hideBadge}
        <a href="https://usepaperpilot.com" target="_blank" class="pp-studio-badge">⚡ Powered by PaperPilot</a>
      {/if}
    </div>

    <!-- Category Tabs Filter -->
    <div class="pp-category-tabs">
      {#each activeCategories as cat}
        <button
          class="pp-cat-tab"
          class:active={selectedCategory === cat.id}
          onclick={() => selectCategory(cat.id)}
        >
          <span class="cat-icon">{cat.icon}</span>
          <span class="cat-label">{cat.label}</span>
          <span class="cat-count">{getToolCountForCategory(cat.id)}</span>
        </button>
      {/each}
    </div>

    <!-- 2-Column Studio Body (Sidebar + Main Action Canvas) -->
    <div class="pp-studio-body">
      <!-- Tool Selector Sidebar -->
      <div class="pp-studio-sidebar">
        <div class="pp-search-box">
          <input type="text" bind:value={searchQuery} placeholder="Search tools..." />
        </div>
        <div class="pp-tools-scroll">
          {#each filteredTools as tool}
            <button
              class="pp-studio-tool-btn"
              class:active={activeToolId === tool.id}
              onclick={() => selectTool(tool.id)}
            >
              <span class="tool-icon">{tool.icon}</span>
              <div class="tool-meta">
                <span class="tool-name">{tool.name}</span>
                <span class="tool-desc-short">{tool.shortDescription}</span>
              </div>
            </button>
          {/each}
        </div>
      </div>

      <!-- Active Tool Canvas & Configuration -->
      <div class="pp-studio-canvas">
        <div class="pp-canvas-header">
          <h2>{currentTool.name}</h2>
          <p>{currentTool.description}</p>
        </div>

        <!-- Dropzone -->
        <DropZone ... />

        <!-- Rich Tool Config Inputs (N-Up grid, Scale format, Stamp image, etc.) -->
        <div class="pp-tool-controls">
          ...
        </div>

        <!-- Action / Process Button -->
        <button class="pp-run-btn" ...>Execute {currentTool.name}</button>
      </div>
    </div>
  {:else}
    <!-- Compact Single-Tool Card Layout -->
    ...
  {/if}
</div>
```

---

## 3. Tool Parity & Categorization Taxonomy
Ensure all 30+ in-browser WASM tools have icons, categories, and descriptions:
- **`pages` (📑 Pages & Imposition)**: `merge`, `split`, `rotate`, `delete`, `extract`, `reorder`, `crop`, `pdf_nup`, `pdf_booklet`, `pdf_scale_pages`, `pdf_auto_rotate`
- **`edit` (🎨 Edit & Stationery)**: `watermark`, `header_footer`, `page_numbers`, `pdf_stamp`, `pdf_overlay_underlay`, `flatten`, `metadata`
- **`optimize` (🗜️ Optimize)**: `compress`, `pdf_hash`, `pdf_info`
- **`security` (🔒 Security & Forensics)**: `pdf_validate_signatures`, `pdf_remove_signatures`, `encrypt`, `decrypt`, `sanitize`, `pii_scan`
- **`convert` (🔁 Convert)**: `images_to_pdf`, `extract_images`, `extract_text`, `render_page`, `json_to_pdf`, `pdf_to_docx`, `ocr`

---

## 4. Updates to `/embed-test` (`apps/web/public/embed-test.html`) & Build Pipeline
1. Update `/embed-test.html` so Demo 1 features the full **Studio Layout** (`data-layout="studio"`).
2. Demo 2 features the **Category Studio** for Contract Forensics (`data-category="security"`).
3. Demo 3 features the **Category Studio** for Imposition & Layout (`data-category="pages"`).
4. Demo 4 features the **Automated Pipeline**.
5. Automated Build Synchronization: Configure `apps/embed` build script to copy `dist/embed.js` automatically into `apps/web/public/embed.js` so `https://usepaperpilot.com/embed.js` is always 100% in sync with the source.
