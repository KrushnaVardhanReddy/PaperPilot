# Spec 054 — Context-Aware Dynamic Tool Dispatcher & Feature Pack Filtering

## 1. Overview & Problem Statement
Currently, PaperPilot displays an exhaustive list of 44+ tools in a static directory structure. While comprehensive, this presents several usability challenges:
1. **Cognitive Overload**: A user loading a single `.xlsx` file must sift past 35+ PDF-specific operations (e.g. Rotate Pages, Flatten Forms, Extract Bookmarks).
2. **Precondition Friction**: Users can click tools that are inapplicable to their current selection (e.g., clicking "Merge" with only 1 document open, or "Split" with zero pages), resulting in runtime validation errors.
3. **Modular Feature Pack Disconnect**: When users toggle Feature Packs (e.g., Office Tools or OCR Models) in Settings, the UI does not dynamically explain missing dependencies or offer streamlined in-place enablement.

---

## 2. Technical Architecture & Design

### 2.1 Tool Context & Capability Metadata
In `apps/desktop/src/lib/services/toolRegistry.ts` (or `toolMetadata.ts`), enrich `ToolDefinition` with formal capability rules:

```typescript
export interface ToolConditionRule {
  /** Applicable file extensions ('pdf', 'xlsx', 'docx', 'png', etc. or '*' for all) */
  supportedExtensions: string[];
  /** Minimum number of documents required in workspace */
  minDocuments: number;
  /** Maximum number of documents allowed (null for unlimited) */
  maxDocuments?: number;
  /** Requires specific Feature Pack (e.g. 'core', 'office', 'ocr', 'dev') */
  requiredPack?: 'core' | 'office' | 'ocr' | 'dev';
  /** Requires page selection (e.g. rotate, delete, crop) */
  requiresSingleDocument?: boolean;
}
```

### 2.2 Intelligent Scoring & Recommendation Engine
Implement a reactive utility `getRankedTools(documents: File[], activePacks: string[]): { recommended: ToolDefinition[], available: ToolDefinition[], packLocked: ToolDefinition[] }`:

1. **Context Analysis**:
   - Inspect loaded file mime types and extensions:
     - **All `.xlsx` / `.xls`**: Rank `excel_to_json`, `excel_to_csv`, `modernize_legacy` at the top (Score: 100).
     - **Multiple `.pdf` files (>= 2)**: Rank `pdf_merge` and `pdf_compare` as Primary Recommendations.
     - **Single `.pdf` file**: Rank `pdf_compress`, `pdf_to_docx`, `pdf_ocr`, `pdf_split`, `pdf_rotate`.
     - **Empty Workspace**: Rank general creation tools (`img_to_pdf`, `json_to_pdf`, `md_to_pdf`) and prominent drop prompts.
2. **Feature Pack Awareness**:
   - If a tool requires `office` and the pack is disabled, place it in `packLocked`.
   - In the UI, instead of an unhelpful disabled state, render a smart 1-click prompt:
     `📦 Requires Office Pack (45 MB) • [Enable Pack]` which invokes `appState.toggleFeaturePack('office', true)`.

### 2.3 UI Presentation & Controls
- **Context Filter Chips**:
  - `[ ⚡ Recommended (4) ]` (Default when file loaded)
  - `[ All Tools (44) ]`
  - `[ PDF ]` `[ Office ]` `[ Conversions ]` `[ Security ]`
- **Smart Badge Indicators**:
  - `Recommended for your file` (Green subtle pill)
  - `Batch operation` (When multiple files are selected)
- **Zero Breakage Guarantee**:
  - Clicking "All Tools" always shows the full registry.
  - The right-side `.operations-panel` continues to support search query filtering and all Playwright test selectors.

---

## 3. Verification & Test Plan
1. **Unit Tests (`apps/desktop/src/lib/services/toolRegistry.test.ts`)**:
   - Verify `getRankedTools` returns `excel_to_json` as top recommendation when an Excel file is loaded.
   - Verify `pdf_merge` is prioritized when 2+ PDFs are present.
   - Verify pack-locked tools properly flag missing feature packs.
2. **Playwright E2E**:
   - Verify desktop real execution continues passing 12/12 tools.
