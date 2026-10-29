# Task 134A: Implement `paperpilot ask` CLI Subcommand

## Metadata
- **Spec**: [Spec 134: CLI Natural Language Interface (`paperpilot ask`)](file:///home/krushna/Project/PaperPilot/specs/134-cli-natural-language-interface/spec.md)
- **Phase**: Phase 4.5 / Task 4.5.1
- **Status**: 📋 In Progress
- **Target Branch**: `feat/cli-ask-command-task-4-5`
- **Primary Language**: Rust (2024 edition)
- **Impacted Crates**: `paperpilot-cli`, `paperpilot-nlp`

---

## Task Checklist & Step-by-Step Instructions

### Step 1: Add Dependency to `paperpilot-cli/Cargo.toml`
Ensure `paperpilot-nlp` is added as a dependency in `paperpilot-cli/Cargo.toml`:
```toml
paperpilot-nlp = { path = "../paperpilot-nlp" }
```

### Step 2: Add `Ask` Subcommand to `paperpilot-cli/src/cli.rs`
In `Commands` enum:
```rust
/// Ask in natural language: execute operations via natural language intent [e.g. "rotate 90 degrees" doc.pdf]
Ask {
    /// Natural language instruction
    prompt: String,

    /// Optional target input PDF file
    #[arg(value_name = "INPUT")]
    input: Option<PathBuf>,

    /// Skip confirmation prompt and execute immediately
    #[arg(short = 'y', long)]
    yes: bool,

    /// Force offline NLP resolution only
    #[arg(long)]
    offline: bool,

    /// Output path override
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Parse and output plan without executing
    #[arg(long)]
    dry_run: bool,
},
```

### Step 3: Implement Handler in `paperpilot-cli/src/commands/ask.rs`
Create `paperpilot-cli/src/commands/ask.rs`:
1. Initialize `ResolverContext`:
   - If `input` is provided, set `context.active_document = Some(input.to_string_lossy().to_string())`.
2. Instantiate `OfflineNlpResolver::new()`.
3. Call `resolver.resolve_with_context(&prompt, &context)`.
4. If resolution fails:
   - Output error explaining why (e.g. ambiguous intent or missing parameters).
   - Return `Err(PdfError::InvalidInput(...))`.
5. If `--dry-run`:
   - Pretty-print the resolved `OperationPlan` (or JSON if `--json` is active).
   - Return `Ok(())`.
6. If not `--yes` and stdin is a TTY:
   - Print human-readable plan:
     ```
     📋 Interpreted Plan:
       • Intent: {plan.intent:?}
       • Inputs: {plan.input_files}
       • Output: {target_output}
     Execute plan? [Y/n]:
     ```
   - Read line from `std::io::stdin()`. If user answers 'n' or 'N', abort with message.
7. Map `plan.intent` to existing CLI operation execution:
   - Handle all intents supported by `paperpilot_nlp::intent::Intent` (40+ intents including `Merge`, `Split`, `Compress`, `Rotate`, `Crop`, `Watermark`, `Encrypt`, `Decrypt`, `ExtractText`, `ExtractImages`, `Ocr`, `Bates`, `Flatten`, `Linearize`, `Repair`, `Redact`, `PdfA`, `HeaderFooter`, `Metadata`, etc.).
   - Delegate each intent to the matching existing command handlers in `paperpilot_cli::commands::*` (e.g. `group_a`, `group_b`, `group_c`, etc.) so that the entire 40+ operation catalog can be triggered via natural language.
   - For any operations not yet wired to a headless CLI handler, return a clean `PdfError::UnsupportedOperation` error rather than a silent failure.

### Step 4: Wire Subcommand in `paperpilot-cli/src/commands/mod.rs`
- Add `pub mod ask;`
- In `execute_command`:
  ```rust
  Commands::Ask { prompt, input, yes, offline, output, dry_run } => {
      ask::handle_ask(prompt, input.as_deref(), *yes, *offline, output.as_deref(), *dry_run, matches_json)
  }
  ```

### Step 5: Integration Tests in `paperpilot-cli/tests/test_ask.rs`
Write automated tests using `assert_cmd::Command`:
1. `test_ask_dry_run_rotate`: Runs `paperpilot ask "rotate 90 degrees" tests/fixtures/sample.pdf --dry-run` and asserts exit 0 and stdout contains `Rotate`.
2. `test_ask_non_interactive_execution`: Runs with `--yes -o /tmp/out.pdf`, verifies output file is created and valid PDF.
3. `test_ask_unknown_query`: Verifies friendly error message on gibberish inputs.
