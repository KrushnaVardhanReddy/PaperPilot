# Spec 134: CLI Natural Language Interface (`paperpilot ask`)

## Metadata
- **Spec Number**: 134
- **Phase**: Phase 4.5 / Task 4.5.1
- **Status**: 📋 In Development
- **Target Branch**: `feat/cli-ask-command-task-4-5`
- **Impacted Area**: `paperpilot-cli`, `paperpilot-nlp`

---

## 1. Overview & Objective

PaperPilot already possesses a sophisticated natural language resolution engine (`paperpilot-nlp::resolver::OfflineNlpResolver`) and a dynamic supervisor (`paperpilot-ai`), but natural language interaction is currently locked exclusively within the Tauri Desktop UI (`PdfChatPanel.svelte`).

The goal of this spec is to bring conversational, natural language PDF manipulation directly to the command line via:
```bash
paperpilot ask "<natural language instruction>" [INPUT_PDF] [OPTIONS]
```

This allows developers, sysadmins, and terminal users to perform operations like:
```bash
paperpilot ask "rotate all pages 90 degrees clockwise and save to rotated.pdf" report.pdf
paperpilot ask "compress this document aggressively" contract.pdf
paperpilot ask "merge invoice1.pdf and invoice2.pdf into combined.pdf"
```

---

## 2. CLI Command Specification

### Syntax
```bash
paperpilot ask [OPTIONS] <PROMPT> [INPUT]
```

### Arguments
- `<PROMPT>` (required): The natural language instruction describing what the user wants to do with the document(s).
- `[INPUT]` (optional): Positional path to an input PDF document. If provided, it populates `ResolverContext.active_document` so prompts like `"compress this"` or `"rotate 90 degrees"` know which file to target.

### Flags & Options
- `--yes`, `-y`: Non-interactive execution. Skips interactive confirmation prompt (`[Y/n]`) and executes the resolved operation immediately. Essential for bash scripts and automated CI pipelines.
- `--offline`: Explicitly force offline rule-based NLP resolution (bypassing any network or remote LLMs).
- `--output`, `-o <PATH>`: Explicit output file override. If omitted, uses the resolved plan's output path or defaults to `<input_stem>_processed.pdf`.
- `--dry-run`: Parse and resolve the natural language prompt into an `OperationPlan`, print the JSON / plan summary, and exit with code `0` without running any PDF manipulation.

---

## 3. Architecture & Data Flow

```
User CLI Invocation
  │
  ▼
`paperpilot ask "<prompt>" [input]`
  │
  ▼
Build `ResolverContext`
  ├── active_document = input (if specified)
  └── open_documents = [input]
  │
  ▼
`OfflineNlpResolver::resolve_with_context(query, &context)`
  │
  ▼
Matches Intent & Extracts Entities:
  ├── Intent::Rotate ──> angle: 90, pages: "all"
  ├── Intent::Compress ──> level: "high"
  ├── Intent::Merge ──> files: [file1, file2], output: combined.pdf
  └── Intent::Watermark ──> text: "CONFIDENTIAL"
  │
  ▼
Plan Display & Confirmation (unless --yes or --dry-run)
  ┌──────────────────────────────────────────────┐
  │ 📋 Plan Generated:                           │
  │ • Operation: Rotate                          │
  │ • Input: report.pdf                          │
  │ • Parameters: angle=90, pages=all            │
  │ • Output: report_processed.pdf               │
  │                                              │
  │ Execute this operation? [Y/n]                │
  └──────────────────────────────────────────────┘
  │
  ▼
Execution Dispatch (reusing existing CLI commands in `paperpilot-cli::commands`)
  │
  ▼
Success / Error Output + Exit Code (0 for success, non-zero for failure)
```

---

## 4. Supported Intents
The CLI natural language interface supports all 40+ canonical intents defined in `paperpilot_nlp::intent::Intent`:
- **Page Manipulation**: `Merge`, `Split`, `Rotate`, `Crop`, `Delete`, `Reorder`, `Burst`, `Extract`
- **Document Optimization & Security**: `Compress`, `Watermark`, `Encrypt`, `Decrypt`, `Linearize`, `Repair`, `Redact`, `Flatten`, `PdfA`, `Sanitize`, `Sign`, `Validate`, `HeaderFooter`, `Bates`, `Hash`
- **Extraction & Analysis**: `ExtractText`, `ExtractImages`, `Ocr`, `Search`, `Compare`, `Metadata`, `Classify`, `Render`, `ImagesToPdf`
- **Format Conversions & Forms**: `ToMarkdown`, `ToJson`, `ToDocx`, `ToPptx`, `ToHtml`, `FormFill`, `FormRead`, `FormCreate`, `Bookmarks`

The dispatcher in `ask.rs` matches against `plan.intent` and routes to the matching handlers in `paperpilot-cli::commands::*` (or returns `PdfError::UnsupportedOperation` for any unhandled variants).

---

## 5. Acceptance Criteria
1. `paperpilot ask "rotate 90 degrees" sample.pdf --dry-run` successfully parses the intent and prints the plan without modifying disk.
2. `paperpilot ask "compress this" sample.pdf --yes -o out.pdf` executes `PdfCompressOperation` non-interactively and generates `out.pdf`.
3. If intent cannot be resolved, exits with informative error code and message explaining the ambiguity.
4. Comprehensive integration test in `paperpilot-cli/tests/test_ask.rs`.
