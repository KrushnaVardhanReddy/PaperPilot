# Spec 142: Desktop Real Watched Folders & Document Smart Filters

## Metadata

- **Phase**: Task 4.7.8
- **Status**: Implemented on feature branch; awaiting integration review
- **Branch**: `feat/desktop-watched-folders-and-smart-filters-task-4-7-8`
- **Date**: 2026-10-29
- **Parents**: [Spec 139](../139-hot-folder-watcher-daemon/spec.md), [Spec 136](../136-conditional-workflow-engine/spec.md)

## Objective

Replace discontinued placeholder watcher/category UI with real native folder automation and reactive filters over locally inspected PDFs. Keep the existing desktop workspace layout; Spec 140's V2 redesign remains reverted.

## Shared native runtime

`paperpilot-pdf/src/operations/watch/manager.rs` owns filesystem registration, file settling, workflow execution, output publication, counters, receipts, and cancellation. CLI, desktop, MCP, and REST call this same implementation. Desktop and its embedded endpoints share a process-wide `Arc<WatcherManager>`; independent processes do not share state.

- Validate recipes, directories, settle delays and conflicting watcher paths before returning success.
- Sweep existing PDFs on every start; then process nonrecursive create/modify events once the size is stable. Ignore unchanged duplicate events during a run.
- Default outputs to `INPUT/processed` and failure receipts to `INPUT/.error`. Never overwrite source/output/receipt files. Failures retain the source and are counted separately from successes.
- Success receipts include input/output SHA-256 and the workflow report. Failure receipts include the actual error and input hash. Error directories contain receipts; PDFs are not silently moved there.
- Stop waits for current processing and joins the worker; remove stops before deleting the entry. Restart creates a new `run_id`. Main-window/app exit cancels and joins active workers.
- Webhooks are explicit CLI opt-ins. No telemetry, cloud processing, or automatic workflow uploads are added.

## Tauri commands and events

Implemented in `apps/desktop/src-tauri/src/watch_commands.rs`, registered in `lib.rs`:

| Command | Contract |
|---|---|
| `start_watch_folder(id, config)` | Returns a real `WatchSnapshot` after OS registration; config contains directory, recipe, optional output and settle delay. |
| `stop_watch_folder(folder_id)` | Returns the joined worker's final snapshot. |
| `list_watched_folders()` | Returns actual runtime entries, including stopped/error entries until removed. |
| `remove_watch_folder(folder_id)` | Stops/joins and removes the runtime entry. |
| `list_watch_recipes()` | Lists only presets accepted by shared workflow preflight. |
| `inspect_document(path, bytes)` | Accepts exactly one local path or byte buffer, up to 128 MiB. Runs inspection off the UI thread. |

Events `watch:file_processed`, `watch:error`, and `watch:status` carry a snapshot and optional source/output/receipt/error. The frontend merges events by run and progress and refreshes the local runtime list every 1.5 seconds to reflect changes through embedded MCP/REST. It rejects stale responses that would erase newer user actions.

## Desktop behavior

Settings → **Watched Folders** provides input/output directory pickers, validated presets, saved workflows, and local TOML/JSON `.ppflow` files. Users explicitly add, start, stop, and remove watchers. The panel shows real status, success/failure totals, last file/time/error, output, and receipt. Users can open the output as a document and inspect the JSON receipt. Existing pipeline controls provide manual workflow execution against loaded documents.

`watchers.svelte.ts` persists versioned folder preferences only: ID, path, recipe reference, optional output directory. Running flags, counters and serialized recipe content are not persisted. Restoring preferences never auto-starts a daemon. Missing workflows, corrupt storage, absent native IPC and OS errors are visible failures. Browser/mobile filesystem controls are disabled.

## Smart filters

`document_inspection.rs` supplies the same scan/form/signature facts used by workflow conditions. Metadata is keyed by the actual `File`, inspected with at most two concurrent requests, and discarded on removal. Viewing, filtering and reordering retain original file identity and selection.

| Filter | Actual rule |
|---|---|
| All documents | All loaded files, including non-PDFs and unavailable inspection results |
| Scanned | Image content drawn on a page with no extractable text, using the existing shared heuristic |
| Signed | A populated signature with nonempty byte ranges and contents; flags/empty signature widgets do not qualify |
| Form fields | Nonempty actual AcroForm field tree |
| Multi-page | Actual page count greater than one |

Signature presence is not a certificate-trust/cryptographic-validity verdict. Encrypted PDFs without a password expose encryption/size while other facts remain unknown; malformed PDFs report an error. Unknown facts never enter a specialized category. Metadata is native desktop functionality; browser documents remain in All with a clear unavailable notice.

## Acceptance and evidence

- [x] Real registered start/stop/list/remove and application-exit cleanup
- [x] Persistent configuration with explicit starts and truthful lifecycle status
- [x] Folder/workflow/output controls, progress, errors, output and receipt viewing
- [x] Real PDF metadata, reactive counts, filtered open/remove and reorder behavior
- [x] Synthetic scan, text, form, multi-page, unsigned and genuinely CMS-signed PDF fixtures
- [x] Native filesystem/metadata tests and UI condition/filter/lifecycle tests

Exact commands, results and environment limits are recorded in [the validation report](../../reports/DESKTOP_AUTOMATION_REPORT.md). This spec does not implement generic scheduling, webhook triggers, retry policies or the reverted V2 workspace.
