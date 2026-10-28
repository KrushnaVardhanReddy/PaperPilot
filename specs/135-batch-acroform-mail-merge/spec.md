# Spec 135 — Batch AcroForm Mail Merge (`paperpilot fill-batch`)

**Phase**: 4 — AI Layer (Community)  
**Task**: 4.7.4  
**Branch**: `feat/cli-fill-batch-task-4-7-4`  
**Status**: 🟡 Ready for Implementation  
**Priority**: P2 — High

---

## 1. Overview

`paperpilot fill-batch` is a CSV/JSON-driven mail-merge engine for AcroForm PDFs.  
Given a template PDF with interactive form fields and a tabular data source (CSV or JSON array), it:

1. Iterates every row/record in the data file.
2. Substitutes each AcroForm field with the matching column value.
3. Optionally flattens the filled PDF (makes fields non-editable).
4. Saves each output file under a user-specified naming token pattern (e.g. `{name}_{date}.pdf`).

This builds **directly on top** of the existing single-fill infrastructure:
- `paperpilot_pdf::operations::form::FillFormOperation`
- `paperpilot_pdf::operations::flatten::FlattenOperation`
- `paperpilot_cli::commands::group_e::handle_form_fill`

No new PDF engine code is required — only a new CLI subcommand + batch loop.

---

## 2. User-Facing Interface

### CLI Signature

```
paperpilot fill-batch \
  --template   <form.pdf>           # AcroForm template PDF (required)
  --data       <records.csv|.json>  # CSV or JSON array of field values (required)
  --output-dir <./out/>             # Destination directory for output files (required)
  --name-pattern <PATTERN>          # Token pattern for output filenames (optional, default: {_index}.pdf)
  --flatten                         # Flatten form fields after filling (optional flag)
  --fail-fast                       # Abort entire batch on first error (default: skip & warn)
  --dry-run                         # Print resolved filenames without writing
```

### Name Pattern Tokens

| Token | Resolves to |
|---|---|
| `{_index}` | 0-based row index (zero-padded to 4 digits) |
| `{_row}` | 1-based row number |
| `{_date}` | Current date `YYYYMMDD` |
| `{_time}` | Current time `HHMMSS` |
| `{<column>}` | Value of any column in the current row |

**Examples**
```
--name-pattern "{name}_{_date}.pdf"
--name-pattern "cert_{_row:04}_{city}.pdf"
--name-pattern "{employee_id}_offer.pdf"
```

### Data File Formats

**CSV** (header row = AcroForm field names):
```csv
name,email,amount
Alice,alice@example.com,5000
Bob,bob@example.com,7500
```

**JSON array** (keys = AcroForm field names):
```json
[
  {"name": "Alice", "email": "alice@example.com", "amount": "5000"},
  {"name": "Bob",   "email": "bob@example.com",   "amount": "7500"}
]
```

---

## 3. Implementation Scope

### 3.1 New CLI Subcommand — `paperpilot-cli/src/cli.rs`

Add `FillBatch` variant to the top-level `Commands` enum:

```rust
FillBatch {
    /// AcroForm template PDF
    #[arg(long)]
    template: PathBuf,
    /// CSV or JSON data file
    #[arg(long)]
    data: PathBuf,
    /// Output directory
    #[arg(long)]
    output_dir: PathBuf,
    /// Filename token pattern, e.g. "{name}_{_date}.pdf"
    #[arg(long, default_value = "{_index}.pdf")]
    name_pattern: String,
    /// Flatten form fields after filling
    #[arg(long)]
    flatten: bool,
    /// Abort on first error (default: skip row, print warning)
    #[arg(long)]
    fail_fast: bool,
    /// Dry-run: print output paths without writing files
    #[arg(long)]
    dry_run: bool,
},
```

### 3.2 New Handler — `paperpilot-cli/src/commands/group_e.rs`

Add `handle_form_fill_batch(...)` function:

```rust
pub fn handle_form_fill_batch(
    template: &Path,
    data: &Path,
    output_dir: &Path,
    name_pattern: &str,
    flatten: bool,
    fail_fast: bool,
    dry_run: bool,
) -> OperationResult<()>
```

**Algorithm:**
1. Parse `data` → `Vec<HashMap<String, String>>` (detect CSV vs JSON by `.csv`/`.json` extension).
2. `fs::create_dir_all(output_dir)` unless `--dry-run`.
3. For each `(index, row)`:
   a. Resolve `name_pattern` tokens → `filename`.
   b. Construct `output_path = output_dir / filename`.
   c. If `--dry-run`: `println!("[dry-run] {}", output_path.display())` → continue.
   d. `LopdfDocument::load(template)` (fresh load per row — template is read-only).
   e. `FillFormOperation::new(row.clone()).execute(&mut doc)`.
   f. If `--flatten`: `FlattenOperation::new().execute(&mut doc)`.
   g. `doc.save(&output_path)`.
   h. On error: if `--fail-fast` → return `Err`; else `eprintln!("[warn] row {}: {}", index, e)`.
4. Print summary: `"Wrote N files to <output_dir>"`.

### 3.3 Wire in `commands/mod.rs`

Add dispatch arm for `Commands::FillBatch { ... }`:

```rust
Commands::FillBatch {
    template, data, output_dir, name_pattern, flatten, fail_fast, dry_run
} => group_e::handle_form_fill_batch(
    &template, &data, &output_dir, &name_pattern,
    flatten, fail_fast, dry_run,
),
```

### 3.4 CSV Parsing Dependency

Use `csv` crate. Add to `paperpilot-cli/Cargo.toml` if not already present:

```toml
[dependencies]
csv = "1"
```

### 3.5 Token Resolver (inline helper in `group_e.rs`)

```rust
fn resolve_pattern(pattern: &str, row: &HashMap<String, String>, index: usize) -> String {
    let now = chrono::Local::now();
    let date = now.format("%Y%m%d").to_string();
    let time = now.format("%H%M%S").to_string();
    let mut result = pattern.to_string();
    result = result.replace("{_index}", &format!("{:04}", index));
    result = result.replace("{_row}",   &(index + 1).to_string());
    result = result.replace("{_date}",  &date);
    result = result.replace("{_time}",  &time);
    for (k, v) in row {
        result = result.replace(&format!("{{{}}}", k), v);
    }
    result
}
```

> `chrono` is already in the workspace.

---

## 4. Integration Tests — `paperpilot-cli/tests/test_fill_batch.rs`

| # | Test | Assertion |
|---|---|---|
| T1 | CSV batch, 3 rows, no flatten | 3 output files exist with correct names |
| T2 | JSON batch, 2 rows | 2 output files exist |
| T3 | `--flatten` flag | Output fields are read-only (no interactive AcroForm dict present) |
| T4 | `--dry-run` | No files written; stdout lists resolved paths |
| T5 | Missing column in pattern | Unresolved token left as-is, no panic |
| T6 | `--fail-fast` with unreadable template | Returns `Err`, zero output files |
| T7 | `--name-pattern "{name}_{_date}.pdf"` | Filename matches `<name>_YYYYMMDD.pdf` |
| T8 | Output dir auto-created | Works even when output dir does not pre-exist |

Use the AcroForm fixture in `paperpilot-cli/tests/fixtures/` (same as `test_form_fill`).

---

## 5. Acceptance Criteria

- [ ] `paperpilot fill-batch --template form.pdf --data records.csv --output-dir ./out/` produces one filled PDF per CSV row.
- [ ] `--name-pattern "{name}_{_date}.pdf"` resolves correctly from row columns.
- [ ] `--flatten` produces non-interactive (flattened) PDFs.
- [ ] `--dry-run` prints paths without writing any files.
- [ ] `--fail-fast` aborts on first error; without it, bad rows are skipped with a warning.
- [ ] JSON input (`records.json`) works identically to CSV.
- [ ] All 8 integration tests pass: `cargo test -p paperpilot-cli --test test_fill_batch`.
- [ ] `cargo clippy` and `cargo fmt` pass with no warnings.

---

## 6. Out of Scope

- XFA form support (AcroForm only).
- MCP tool / Desktop UI exposure (future tasks).
- Parallel row processing (future optimization).
- Cloud / remote data sources (enterprise downstream).

---

## 7. Dependencies & Risk

| Item | Status |
|---|---|
| `FillFormOperation` | ✅ Exists — `paperpilot-pdf/src/operations/form.rs` |
| `FlattenOperation` | ✅ Exists — `paperpilot-pdf/src/operations/flatten.rs` |
| `csv` crate | ⚠️ Verify in `paperpilot-cli/Cargo.toml`; add `csv = "1"` if absent |
| `chrono` crate | ✅ Already in workspace |
| AcroForm test fixture | ✅ Available in `paperpilot-cli/tests/fixtures/` |
