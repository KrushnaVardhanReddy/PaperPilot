# Task 135A — Implement `paperpilot fill-batch` (Batch AcroForm Mail Merge)

**Spec**: [Spec 135](./spec.md)  
**Branch**: `feat/cli-fill-batch-task-4-7-4`  
**Phase**: 4.7.4  
**Assignee**: Jules (AI)  
**Status**: 🟡 Not Started

---

## Pre-flight Checklist

- [ ] Branch `feat/cli-fill-batch-task-4-7-4` is checked out from `feature/dev`.
- [ ] `cargo build -p paperpilot-cli` is green on the base branch.
- [ ] `paperpilot-cli/tests/fixtures/` contains an AcroForm PDF fixture (reuse from `test_form_fill`).

---

## Implementation Checklist

### A. CLI Definition (`paperpilot-cli/src/cli.rs`)

- [ ] Add `FillBatch { template, data, output_dir, name_pattern, flatten, fail_fast, dry_run }` variant to `enum Commands`.
- [ ] All fields annotated with correct `#[arg(long)]` / default values as specified.

### B. Handler (`paperpilot-cli/src/commands/group_e.rs`)

- [ ] Add private `fn resolve_pattern(pattern: &str, row: &HashMap<String, String>, index: usize) -> String`.
  - Handles `{_index}`, `{_row}`, `{_date}`, `{_time}`, `{<column>}` tokens.
- [ ] Add `pub fn handle_form_fill_batch(template, data, output_dir, name_pattern, flatten, fail_fast, dry_run) -> OperationResult<()>`.
  - Detect data format by extension (`.csv` → CSV parser, `.json` → serde_json).
  - Parse CSV using `csv::Reader`.
  - Parse JSON using `serde_json::from_str::<Vec<HashMap<String, String>>>`.
  - Create output directory with `fs::create_dir_all`.
  - For each row: resolve pattern → load template fresh → FillFormOperation → (optional) FlattenOperation → save.
  - `--dry-run`: print resolved paths, skip file writes.
  - `--fail-fast`: propagate first error; without it, warn and continue.
  - Print final summary line.

### C. Dependency Check (`paperpilot-cli/Cargo.toml`)

- [ ] Verify `csv = "1"` is present; add if missing.
- [ ] Verify `chrono` is present (for date/time tokens); add if missing.

### D. Dispatch (`paperpilot-cli/src/commands/mod.rs`)

- [ ] Add match arm for `Commands::FillBatch { ... }` calling `group_e::handle_form_fill_batch(...)`.

### E. Integration Tests (`paperpilot-cli/tests/test_fill_batch.rs`)

- [ ] T1: CSV 3-row batch, default name pattern — verify 3 files exist.
- [ ] T2: JSON 2-row batch — verify 2 files exist.
- [ ] T3: `--flatten` — verify output has no interactive AcroForm fields.
- [ ] T4: `--dry-run` — verify zero files written; stdout contains expected paths.
- [ ] T5: Token with missing column — verify no panic, token left literal.
- [ ] T6: `--fail-fast` + bad template path — verify `Err` returned, zero output files.
- [ ] T7: `--name-pattern "{name}_{_date}.pdf"` — verify filename format.
- [ ] T8: Non-existent output dir — verify auto-created and files written.

---

## Verification

```bash
# Build
cargo build -p paperpilot-cli

# Run all batch fill tests
cargo test -p paperpilot-cli --test test_fill_batch -- --nocapture

# Lint
cargo clippy -p paperpilot-cli -- -D warnings
cargo fmt -p paperpilot-cli -- --check

# Quick smoke test
paperpilot fill-batch \
  --template paperpilot-cli/tests/fixtures/acroform.pdf \
  --data /tmp/test_records.csv \
  --output-dir /tmp/batch_out/ \
  --name-pattern "{name}_{_date}.pdf" \
  --flatten
```

---

## Definition of Done

- All 8 integration tests pass.
- `cargo clippy` reports zero warnings.
- `cargo fmt` reports no formatting changes.
- PR targets `feat/cli-fill-batch-task-4-7-4` → `feature/dev`.
- PR description links this task and Spec 135.
