# Desktop automation and condition compatibility — 2026-10-29

Implemented on `feat/desktop-watched-folders-and-smart-filters-task-4-7-8` for review into `feature/dev`. The base is `origin/fix/current-ui-polish` at `dda6861`, including its contrast fixes, Specs 141/142, and PR #305. The refreshed integration baseline remains `0069d48`. This is development work, not a new published release.

## Result

Settings → **Watched Folders** now starts actual filesystem workers. Desktop, embedded REST and MCP share one local manager; the CLI uses the same implementation in its own process. Registration and recipe validation precede start success. Stop joins the worker, remove stops before deletion, and application exit cancels active workers. Progress comes from actual PDF processing rather than placeholder registry records.

The manager sweeps existing PDFs and processes arriving nonrecursive filesystem events after settling. Outputs/receipts are published without replacing existing files, unchanged events are deduplicated during a run, and output folders cannot feed active watcher inputs. Processing/write failures are counted, reported and receipted while later valid files can succeed. Sources remain in place by default. Explicit legacy archival options remain available, and CLI completion webhooks preserve their existing fields with added artifact paths and bounded delivery time.

Desktop users can select presets, saved flows or local recipes; start/stop/remove; inspect progress/errors; open the produced PDF; and view the actual JSON receipt. Reopening the panel recovers the last output/receipt from native runtime snapshots. Preferences retain configuration only, never active status, counters or serialized recipes. Cross-interface removal is reconciled by a local status refresh without resurrecting stale events.

Smart filters use real PDF page, image/text, AcroForm and filled-signature facts from the shared engine. Metadata requests are bounded and keyed by `File` identity. Counts update after addition/removal; filtering and reordering preserve the file selected/opened/removed. Unknown and failed inspections remain in All documents.

The recursive condition editor now retains false values, zero legacy thresholds, nested `ALL`/`ANY`/`NOT`, all byte-size comparisons/ranges, encryption predicates and no-match behavior. Opening a rule never rewrites it. Changes require validated **Apply conditions**; unknown saved fields remain preserved until explicitly replaced. Editing is disabled during execution. Step-card summaries use the same condition model. Browser-only operations no longer return fabricated success when native IPC is missing.

## Verification

All Cargo commands use `-j 2`; real PDF operations are never replaced with mock bytes or successful operation responses. The scoped suites pass 52 Rust tests and 147 desktop tests.

| Check | Result |
|---|---|
| Native desktop command compilation (`cargo check -p desktop -j 2 --offline`) | Passed on Windows GNU |
| Core desktop automation / metadata (`desktop_automation`) | 8 passed |
| Shared workflow schema / graph validation (`workflow::`) | 6 passed |
| Conditional workflow regressions: core / MCP / REST | 14 + 2 + 2 passed |
| CLI conditional recipe compatibility (`test_conditional_workflow`) | 7 passed |
| WASM workflow wrapper on the host without native OCR | 2 passed |
| CLI real watcher / one-shot / malformed-input handling and opt-in webhook (`test_watch`) | 4 passed |
| MCP real worker lifecycle / recipe / invalid requests | 4 passed, including final parameter validation |
| REST start/list/delete / invalid config / binary multipart recipe | 3 passed |
| Desktop Vitest (`node node_modules/vitest/vitest.mjs run --maxWorkers=2` in `apps/desktop`) | 147 passed across 24 files |
| `pnpm --dir apps/desktop check` | 0 errors; 22 existing warnings |
| `pnpm --dir apps/desktop build` | Passed |
| Handbook regeneration and `npm --prefix apps/docs run build` | Passed on final documentation; 80 tool reference pages, 93 built pages |
| Browser interaction/layout QA | Passed, no page errors; 900 px viewport condition panel fits its container (422 px width and scroll width) |
| `git diff --check` | Passed |
| Whole-workspace `cargo fmt --all -- --check` | Fails in 9 untouched files, including `ai_commands.rs`, core workflow/recipe and gateway SSE files; changed Rust files are formatted; no full-CI green claim |

The Rust regression command covered:

```text
cargo test -p paperpilot-pdf -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway --test desktop_automation --test conditional_workflow --test test_watch -j 2 --offline -- --test-threads=2
cargo test -p paperpilot-cli --test test_watch -p paperpilot-mcp --test watcher_lifecycle --test test_automation_tools -p paperpilot-gateway --test test_automation -j 2 --offline -- --test-threads=2
cargo test -p paperpilot-core --lib workflow:: -j 2 --offline
cargo test -p paperpilot-cli --test test_conditional_workflow -j 2 --offline -- --test-threads=2
cargo test -p paperpilot-wasm --test conditional_workflow -j 2 --offline -- --test-threads=2
```

Complete synthetic PDFs and the native engine's recorded facts/events are committed in [`apps/desktop/tests/fixtures/automation`](../apps/desktop/tests/fixtures/automation/README.md). They include drawable scan imagery, extractable text, a two-page document, an actual AcroForm, unsigned signature flags and a detached CMS-signed PDF generated with an ephemeral QA key. Native tests also encrypt genuine PDFs, truncate real inputs, force real filesystem write failures, preserve pre-existing output bytes, exercise concurrent stops/restarts and confirm that post-stop arrivals are not processed.

Browser QA uses actual UI interactions and seeded configuration/condition inputs. It verifies truthful unavailable/disabled filesystem controls in a browser, restored stopped preferences, nested conditions, explicit Apply and narrow-layout geometry. It does not simulate a native processing response. Evidence: [recorded checks](desktop-automation/browser-qa.json), [watched-folders preview](desktop-automation/watched-folders-browser.png), [condition editor](desktop-automation/condition-editor.png).

## Boundaries and environment

- Watchers are native, nonrecursive and process-local. Every start processes existing PDFs again. Generic schedules, automatic restarts, durable job history, retry policies and the reverted Spec 140 V2 workspace remain separate work. `.error` contains failure receipts; failed originals are not silently relocated.
- Metadata inspection is native desktop functionality with a 128 MiB limit and two concurrent requests. Scan detection is the existing image/text heuristic; form detection covers AcroForms. Locked encrypted PDFs expose encryption/size while other facts remain unknown. Existing workflows still require decrypted input.
- Signed means a populated signature with nonempty byte ranges and contents. It does not establish cryptographic validity, certificate trust, expiry or revocation status.
- This validation covers the changed runtime paths, native compilation and browser UI preview. It does not claim a fresh whole-workspace test/security audit, packaged desktop interaction or cross-platform release packaging.
- On this Windows GNU environment, executable linking used the installed MinGW pthread library through a local linker helper; a local compiler mutex reduced peak memory after dependency compilation hit memory limits. These helpers are outside the repository and do not alter product build configuration. The genuine opt-in webhook test could not receive a local callback inside the sandbox and passed in the approved host environment. Vitest used a workspace temporary directory; Astro build telemetry was disabled.
- `graphify-rs build .` was attempted, but the executable is unavailable, so the optional local code graph could not be refreshed.

ROADMAP, PHASES, phase tasks, Specs 141/142 and HANDOFF now distinguish implementation awaiting review from merged/released work. Phase 3/4 are no longer labeled 100% complete, and the V2 workspace remains reverted.
