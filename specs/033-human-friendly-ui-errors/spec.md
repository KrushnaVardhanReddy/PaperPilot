# Spec 033: Human-Friendly UI Error Normalization & Action Hints

## 1. Overview & Problem Statement
Currently, when a tool execution fails in the Svelte UI (`apps/desktop`), raw backend error strings are displayed directly in user toasts and job status panels (e.g., `Failed to rotate page: lopdf: cross-reference table missing offset 0x4f2`, `Error (exit code Some(2))`, `Syntax error in trailer dict`).

These technical error strings:
1. Confuse and intimidate non-technical users.
2. Provide no constructive action or resolution hint (e.g., "Enter the password" or "Try PDF Repair").
3. Leak internal engine implementation details.

## 2. Goals
- Create a centralized error formatter in `apps/desktop/src/lib/utils/errorFormatter.ts`.
- Map technical errors to clear, friendly user headlines and actionable resolution hints.
- Enhance the Toast system (`ToastState` and `ToastContainer.svelte`) to optionally support detail lines and action hints while retaining backwards compatibility.
- Retain the raw technical error under an expandable/toggleable or tooltip element so power users can still copy technical logs.
- Update `OperationsPanel.svelte`, `+page.svelte`, and page operation handlers to route all errors through `formatUserFriendlyError`.
- Ensure 100% pass across all unit and component tests (`npm test` in `apps/desktop`).

## 3. Architecture & Error Mapping Taxonomy

### 3.1 Taxonomy Rules
| Category | Keywords / Triggers | User Headline | Action Hint |
|---|---|---|---|
| **Password / Security** | `password`, `encrypted`, `decrypt`, `unauthorized`, `owner password` | **Protected Document** | "Please provide the correct password to unlock and process this PDF." |
| **Corrupted PDF** | `xref`, `trailer`, `syntax`, `corrupt`, `damaged`, `invalid stream`, `unexpected token` | **Unable to Read PDF** | "The document structure appears damaged. Try using the 'Repair' tool first." |
| **Invalid Page Range** | `out of bounds`, `page count`, `range`, `empty range`, `cannot produce 0-page` | **Invalid Page Range** | "Please verify that the selected page numbers exist in this document." |
| **Missing Input** | `missing`, `not found`, `no such file`, `enoent` | **File Not Found** | "The specified file could not be found. Please re-select the document." |
| **File Locked / Permissions** | `permission denied`, `in use`, `busy`, `locked`, `access is denied` | **File Is Busy or Inaccessible** | "Please close the document in other applications and try again." |
| **Resource / Limits** | `too large`, `memory limit`, `timeout`, `size exceeded` | **File Limit Reached** | "The document is too large or complex for this operation." |
| **Empty Operation** | `cannot merge 0 files`, `empty array`, `no inputs` | **No Documents Selected** | "Please add at least one document to proceed." |
| **Generic Fallback** | *(Any unrecognized error)* | **{Operation Name} Could Not Be Completed** | "An unexpected issue occurred. Please check the document and retry." |

### 3.2 Formatter Signature
```typescript
export interface FormattedError {
  headline: string;
  actionHint: string;
  technicalDetails?: string;
}

export function formatUserFriendlyError(rawError: unknown, operationTitle?: string): FormattedError;
```

### 3.3 Enhanced Toast Model
In `apps/desktop/src/lib/state/toast.svelte.ts`:
```typescript
export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  actionHint?: string;
  technicalDetails?: string;
  duration?: number;
}
```
Update `toastState.error(headline: string, actionHint?: string, technicalDetails?: string, duration?: number)`:
- If called with a single string, behaves identically to legacy callers.
- If called with `actionHint` / `technicalDetails`, stores structured metadata.

### 3.4 UI Component Enhancements
In `apps/desktop/src/lib/components/layout/ToastContainer.svelte`:
- Display `toast.message` as bold / headline.
- If `toast.actionHint` is present, display it underneath in slightly smaller muted text.
- If `toast.technicalDetails` is present, show a small "Details" toggle button that expands the raw technical string with a "Copy" button.

## 4. Implementation Steps
1. Create `apps/desktop/src/lib/utils/errorFormatter.ts` with comprehensive keyword matching and test cases in `apps/desktop/src/lib/utils/__tests__/errorFormatter.test.ts`.
2. Update `apps/desktop/src/lib/state/toast.svelte.ts` to support `actionHint` and `technicalDetails`.
3. Update `apps/desktop/src/lib/components/layout/ToastContainer.svelte` to render headline, action hint, and expandable technical details.
4. Update `apps/desktop/src/lib/components/layout/OperationsPanel.svelte` catch blocks to call `formatUserFriendlyError(error, activeTool.title)`.
5. Update `apps/desktop/src/routes/+page.svelte` page rotation, deletion, duplication, and extraction handlers to use `formatUserFriendlyError`.
6. Run `npm test` in `apps/desktop` to verify all existing and new tests pass.

## 5. Verification
- `apps/desktop/src/lib/utils/__tests__/errorFormatter.test.ts` passes with 100% coverage on all keyword categories.
- `apps/desktop/src/lib/components/__tests__/Toast.test.ts` updated and passing with action hints and technical details.
- `npm test` runs green across all test files.
