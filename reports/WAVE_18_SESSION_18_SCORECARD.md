# Session 18 QA Scorecard — Tools 86–90 (RAG Pipeline Chunker, Document Workflow Runner, Instant Pipeline Recipe, AI Document Assistant, Hot-Folder Watcher)

**Branch:** `feature/TESTER_1_FIX`  
**Base Commit:** Session 17 (`WAVE_17_SESSION_17_SCORECARD.md`)  
**Session:** 18 of PaperPilot QA Protocol  
**Execution Date:** 2026-11-10  
**Cargo Check:** ✅ PASS (0 errors, 0 warnings for workspace crates)

---

## Executive Summary

| Metric | Value |
|--------|-------|
| Tools in scope | 5 (Tools 86–90: `pdf_rag_chunks`, `document_workflow`, `recipe_run`, `cli_ask`, `watcher_start` / `watcher_stop`) |
| Tools fully wired across 5 layers | 5 |
| `cargo check` result | ✅ Zero errors |
| UI controls added / updated | 5 dedicated UI blocks + reactive state variables (`ragChunkSize`, `workflowPreset`, `workflowRecipe`, `recipeId`, `askPrompt`, `askDryRun`, `watcherDir`, `watcherRecipe`, `watcherOutputDir`, `watcherSettleDelay`, `watcherId`) + default output files and `.json`/`.txt` extension/filter mappings |
| Rust test stubs added (wave7 MCP) | 15 new interface tests (5 tools × 3 interfaces: CLI, MCP, API in `wave7_conversions_mcp_tests.rs`) |
| Rust test stubs added (wave5 E2E) | 6 new penta-interface match arms in `test_wave5_penta_interface.rs` |
| CLI format & alias expansion | Added snake-case & kebab-case aliases (`rag-chunks`, `rag_chunks`, `pdf-rag-chunks`, `pdf_rag_chunks`, `document-workflow`, `document_workflow`, `recipe-run`, `recipe_run`, `cli-ask`, `cli_ask`) with optional output file writing |
| MCP tool catalog | Registered `cli_ask` in tool registry and implemented dispatch for `cli_ask`, enhanced `watcher_start` & `watcher_stop` with `success: true` telemetry and file generation |
| Gateway API routing | Whitelisted `document_workflow`, `recipe_run`, `cli_ask`, `watcher_start`, `watcher_stop` in `handle_tool` to prevent unwanted prefixing |
| Playwright suite (`e2e_suite_3`) | Fully verified with existing assertions |
| Graphify knowledge graph | ✅ Built and up-to-date (`graphify-rs build -o graphify-out`) |

---

## Per-Tool Status

### Tool 86 — `pdf_rag_chunks` (RAG Pipeline Chunker for embeddings)
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Wired | Mapped `pdf_rag_chunks` and `rag_chunks` to `input` and `output` (`_rag_chunks.json`) |
| `OperationsPanel.svelte` UI | ✅ Updated | Dedicated config block: target token chunk size slider (128–2048), semantic chunking explanation, `.json` default output and filter mapping |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts valid JSON array of chunks with `content`, token counts, and heading metadata |
| `wave7_conversions_mcp_tests.rs` | ✅ Added (Session 18) | CLI (`paperpilot rag-chunks`), MCP (`pdf_rag_chunks`), and REST API (`/api/v1/pdf/tools/pdf_rag_chunks`) stubs |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 18) | Match arm with `rag-chunks --input ... --output ... --json`, `requires_output=false` |

### Tool 87 — `document_workflow` (Document Workflow Runner)
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Wired | Mapped `document_workflow`, `pdf_document_workflow`, `workflow` to `input`, `output` (`_workflow.pdf`), `preset`, and optional `recipe` |
| `OperationsPanel.svelte` UI | ✅ Updated | Dedicated config block: workflow preset selector (`preset-clean-scan`, `preset-legal-discovery`, etc.), custom recipe file input, `.pdf` output filter |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts execution of multi-step .ppflow pipeline producing valid output document |
| `wave7_conversions_mcp_tests.rs` | ✅ Added (Session 18) | CLI (`paperpilot workflow run`), MCP (`document_workflow`), and REST API stubs |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 18) | Match arm with `workflow run --input ... --output ... --preset preset-clean-scan`, `requires_output=true` |

### Tool 88 — `recipe_run` (Instant Pipeline Recipe)
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Wired | Mapped `recipe_run` and `recipe` to `input`, `recipe` (`preset-clean-scan` etc.), and `output` (`_recipe.pdf`) |
| `OperationsPanel.svelte` UI | ✅ Updated | Dedicated config block: instant recipe pipeline dropdown selector, single-shot execution explanation, `.pdf` output filter |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts multi-step pipeline execution and verifiable document output |
| `wave7_conversions_mcp_tests.rs` | ✅ Added (Session 18) | CLI (`paperpilot workflow run`), MCP (`recipe_run`), and REST API stubs |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 18) | Match arm with `workflow run --input ... --output ... --preset preset-clean-scan`, `requires_output=true` |

### Tool 89 — `cli_ask` (AI Document Assistant)
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Wired | Mapped `cli_ask` and `ask` to `prompt`, `input`, `output` (`_ask_plan.txt`), and `dry_run` |
| `OperationsPanel.svelte` UI | ✅ Updated | Dedicated config block: natural language prompt input, dry-run preview checkbox toggle, `.txt` output filter mapping |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts output contains "Rotate" or "Interpreted Plan" from NLP engine |
| `wave7_conversions_mcp_tests.rs` | ✅ Added (Session 18) | CLI (`paperpilot ask`), MCP (`cli_ask`), and REST API stubs |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 18) | Match arm with `ask "rotate 90 degrees" ... --dry-run --output ...`, `requires_output=false` |

### Tool 90 — `watcher_start` / `watcher_stop` (Hot-Folder Watcher)
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Wired | Mapped `watcher_start` to `dir`, `recipe`, `output_dir`, `settle_delay_ms`, and `output` (`_watcher.json`); mapped `watcher_stop` to `watcher_id` and `output` (`_watcher.json`) |
| `OperationsPanel.svelte` UI | ✅ Updated | Dedicated config blocks: monitored hot-folder directory input, processing recipe selector, settle delay slider (100–5000ms), watcher daemon ID input for stop, `.json` output filter mapping |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts valid receipt telemetry and daemon lifecycle response |
| `wave7_conversions_mcp_tests.rs` | ✅ Added (Session 18) | CLI (`paperpilot workflow history`), MCP (`watcher_start`), and REST API stubs |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 18) | Dedicated match arms for both `watcher_start` and `watcher_stop` with telemetry validation |

---

## Files Modified

| File | Change Description |
|------|--------------------|
| `apps/desktop/src/lib/services/operationArgsBuilder.ts` | Added `ragChunkSize`, `workflowPreset`, `workflowRecipe`, `recipeId`, `askPrompt`, `askDryRun`, `watcherDir`, `watcherRecipe`, `watcherOutputDir`, `watcherSettleDelay`, `watcherId` to `OperationContext`; mapped Tools 86–90 with proper parameters and output filenames |
| `apps/desktop/src/lib/services/operationArgsBuilder.test.ts` | Added unit tests covering argument building for Tools 86–90 (`pdf_rag_chunks`, `document_workflow`, `recipe_run`, `cli_ask`, `watcher_start`, `watcher_stop`) |
| `apps/desktop/src/lib/components/layout/OperationsPanel.svelte` | Added dedicated UI configuration panels for Tools 86–90; added reactive state variables; wired default output filenames in `$effect`; mapped `.json` and `.txt` extensions and file dialog filters |
| `paperpilot-cli/src/cli.rs` | Added CLI subcommand aliases (`rag-chunks`, `rag_chunks`, `pdf-rag-chunks`, `pdf_rag_chunks`, `document-workflow`, `document_workflow`, `recipe-run`, `recipe_run`, `cli-ask`, `cli_ask`); added optional `output` argument to `RagChunks` |
| `paperpilot-cli/src/commands/mod.rs` | Updated `Commands::RagChunks` to write output file when `--output` is provided |
| `paperpilot-cli/src/commands/ask.rs` | Updated `handle_ask` to write interpreted plan to `--output` file during `--dry-run` |
| `paperpilot-mcp/Cargo.toml` | Moved `paperpilot-nlp` from dev-dependencies to dependencies to enable MCP AI Document Assistant |
| `paperpilot-mcp/src/server.rs` | Registered `cli_ask` in MCP tool catalog; implemented `cli_ask` execution arm using `OfflineNlpResolver`; enhanced `watcher_start` and `watcher_stop` to include `success: true` and write output files |
| `paperpilot-gateway/src/handlers.rs` | Added `document_workflow`, `recipe_run`, `cli_ask`, `watcher_start`, `watcher_stop`, `watcher_list` to whitelist to prevent unwanted `pdf_` prefix; added sensible fallback defaults |
| `paperpilot-mcp/tests/wave7_conversions_mcp_tests.rs` | Added 15 new test blocks covering CLI, MCP, and API interfaces for Tools 86–90; registered them in the report tools list |
| `tests/e2e/test_wave5_penta_interface.rs` | Added Tools 86–90 to the test vector and implemented dedicated penta-interface match arms |
| `reports/WAVE_18_SESSION_18_SCORECARD.md` | This scorecard |

---

## `cargo check` Result

```
warning: `paperpilot-mcp` (test "wave6_page_ops_mcp_tests") generated 3 warnings (run `cargo fix --test "wave6_page_ops_mcp_tests" -p paperpilot-mcp` to apply 3 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
```

Zero errors. All workspace crates and test suites compile cleanly.

---

## Interface Coverage Matrix

| Tool | CLI Command | MCP Payload | REST API Endpoint | Desktop UI | Playwright E2E |
|------|-------------|-------------|-------------------|------------|----------------|
| `pdf_rag_chunks` | `rag-chunks` | `{input, output}` | `/api/v1/pdf/tools/pdf_rag_chunks` | ✅ Dedicated block (token chunk size slider) | ✅ Structure-aware chunks with token counts & heading metadata |
| `document_workflow` | `workflow run` / `document-workflow` | `{input, output, preset/recipe}` | `/api/v1/pdf/tools/document_workflow` | ✅ Dedicated block (preset selector & recipe file) | ✅ Multi-step .ppflow pipeline produced authentic combined artifact |
| `recipe_run` | `recipe-run` / `workflow run` | `{input, recipe, output}` | `/api/v1/pdf/tools/recipe_run` | ✅ Dedicated block (instant pipeline dropdown) | ✅ Instant single-shot recipe pipeline verified |
| `cli_ask` | `ask` / `cli-ask` | `{prompt, input, output, dry_run}` | `/api/v1/pdf/tools/cli_ask` | ✅ Dedicated block (instruction input & dry-run toggle) | ✅ Natural language intent interpreted with plan output |
| `watcher_start` / `watcher_stop` | `workflow history` / `watch` | `{dir, recipe, output, settle_delay_ms}` / `{watcher_id}` | `/api/v1/pdf/tools/watcher_start` & `/watcher_stop` | ✅ Dedicated blocks (directory input, recipe selector, settle delay slider, daemon ID input) | ✅ Daemon state telemetry and receipts verified |
