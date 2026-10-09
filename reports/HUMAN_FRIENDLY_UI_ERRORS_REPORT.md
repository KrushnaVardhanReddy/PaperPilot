# Human-Friendly UI Errors Report

## Overview
This report summarizes the implementation of the Human-Friendly UI Error Normalization and Action Hints feature (Spec 033). Raw technical error messages from the backend are now formatted into clean, user-friendly headlines with actionable resolution hints, while still allowing power users to inspect and copy raw technical logs via a "Details" toggle on toasts.

## Error Taxonomy and Mapping Table

| Category | Example Trigger Strings | Headline | Action Hint |
|----------|-------------------------|----------|-------------|
| Password / Encryption | `password`, `encrypted`, `decrypt` | Protected Document | Please provide the correct password to unlock and process this PDF. |
| Corrupted PDF | `xref`, `trailer`, `syntax`, `corrupt` | Unable to Read PDF | The document structure appears damaged. Try using the 'Repair' tool first. |
| Invalid Page Range | `out of bounds`, `page count`, `range` | Invalid Page Range | Please verify that the selected page numbers exist in this document. |
| Missing Input / Not Found | `missing`, `not found`, `enoent` | File Not Found | The specified file could not be found. Please re-select the document. |
| File In Use / Permissions | `permission denied`, `in use`, `busy` | File Is Busy or Inaccessible | Please close the document in other applications and try again. |
| Limit / Timeout | `too large`, `memory limit`, `timeout` | File Limit Reached | The document is too large or complex for this operation. |
| Empty Operation | `cannot merge 0 files`, `empty array` | No Documents Selected | Please add at least one document to proceed. |
| Fallback | `unknown`, `system fault` | [Operation] could not be completed | An unexpected issue occurred. Please check the document and retry. |

## Before/After Examples

**Example 1: Corrupted PDF**
* **Before:** `lopdf: cross-reference table missing offset 0x4f2`
* **After:**
  * Headline: `Unable to Read PDF`
  * Action Hint: `The document structure appears damaged. Try using the 'Repair' tool first.`
  * Details Toggle: Shows `lopdf: cross-reference table missing offset 0x4f2`

**Example 2: Password Protected**
* **Before:** `Document requires owner password`
* **After:**
  * Headline: `Protected Document`
  * Action Hint: `Please provide the correct password to unlock and process this PDF.`
  * Details Toggle: Shows `Document requires owner password`

**Example 3: Missing File**
* **Before:** `ENOENT: no such file or directory`
* **After:**
  * Headline: `File Not Found`
  * Action Hint: `The specified file could not be found. Please re-select the document.`
  * Details Toggle: Shows `ENOENT: no such file or directory`

## Test Coverage Output
```
 RUN  v5.0.2 /app/apps/desktop

 ✓ src/lib/components/layout/SettingsPanel.test.ts (2 tests) 280ms
 ✓ src/lib/components/__tests__/CancelButton.test.ts (4 tests) 305ms
 ✓ src/lib/components/layout/ApiDocsView.test.ts (6 tests) 948ms
 ✓ src/lib/components/__tests__/Toast.test.ts (3 tests) 274ms
 ✓ src/lib/components/__tests__/OperationsPanel.test.ts (2 tests) 326ms
 ✓ src/lib/components/__tests__/ProgressBar.test.ts (4 tests) 88ms
 ✓ src/lib/components/__tests__/DocumentList.test.ts (2 tests) 84ms
 ✓ src/lib/components/__tests__/PipelineRunner.test.ts (2 tests) 68ms
 ✓ src/lib/components/__tests__/PipelineCanvas.test.ts (2 tests) 84ms
 ✓ src/lib/utils/__tests__/errorFormatter.test.ts (9 tests) 9ms
 ✓ src/lib/components/__tests__/BottomNav.test.ts (1 test) 62ms

 Test Files  11 passed (11)
      Tests  37 passed (37)
   Start at  00:20:48
   Duration  9.46s
```
All desktop Vitest tests pass, successfully asserting UI and formatting utility behaviors.
