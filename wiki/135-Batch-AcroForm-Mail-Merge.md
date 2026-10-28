# 135 Batch AcroForm Mail Merge

The `fill-batch` tool enables CSV/JSON-driven mail merging on AcroForm PDF templates.

## Features
- Reads multiple rows of data from `.csv` or `.json` files.
- Replaces tokens in `name_pattern` to generate custom filenames. Supported tokens include:
    - `{_index}` (zero-padded 4 digit index)
    - `{_row}` (1-based row number)
    - `{_date}` (YYYYMMDD)
    - `{_time}` (HHMMSS)
    - `{<column>}` (value from the current row's column, e.g., `{Name}`)
- Outputs a single PDF for each row.
- Optional `--flatten` mode to permanently burn form fields into the PDF.
- Optional `--dry-run` flag to preview generated output file names.
- Optional `--fail-fast` mode to halt processing immediately upon the first error (default behavior skips failed rows).
