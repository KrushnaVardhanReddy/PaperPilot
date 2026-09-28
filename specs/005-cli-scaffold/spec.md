# Phase 1.5: CLI Scaffold and Subcommands

## Objective
Build the command-line interface (CLI) for `paperpilot-cli` (the `paperpilot` binary). The CLI must wrap all implemented PDF operations from `paperpilot-pdf` using the `clap` crate for clean argument parsing and subcommands.

## Architecture

We use `clap` with the `derive` feature to define nested subcommands.

### `paperpilot-cli/src/main.rs`
The main entry point. Sets up logging (e.g. `env_logger`) and parses the top-level `Cli` struct.

### `paperpilot-cli/src/cli.rs`
Defines the `clap` struct:
```rust
#[derive(Parser)]
#[command(name = "paperpilot", version, about = "A fast, privacy-first PDF manipulation tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
```

### `paperpilot-cli/src/commands/mod.rs`
Holds the implementations mapping each CLI subcommand to its `paperpilot_pdf::operations` equivalent. 

## Command Groupings

Since there are 29 commands, they are structured logically:

### Group A: Page Manipulation
- `paperpilot merge --input a.pdf b.pdf --output out.pdf`
- `paperpilot split --input in.pdf --pages 1-5,8-10 --output out_dir/`
- `paperpilot extract --input in.pdf --pages 1,3,5 --output out.pdf`
- `paperpilot delete --input in.pdf --pages 2,4 --output out.pdf`
- `paperpilot reorder --input in.pdf --order 3,2,1 --output out.pdf`
- `paperpilot rotate --input in.pdf --pages 1-5 --degrees 90 --output out.pdf`
- `paperpilot crop --input in.pdf --pages 1 --rect "0,0,100,100" --output out.pdf`
- `paperpilot burst --input in.pdf --output out_dir/`

### Group B: Document Operations
- `paperpilot compress --input in.pdf --quality high --output out.pdf`
- `paperpilot repair --input in.pdf --output out.pdf`
- `paperpilot linearize --input in.pdf --output out.pdf`
- `paperpilot encrypt --input in.pdf --user-password "pass" --output out.pdf`
- `paperpilot decrypt --input in.pdf --password "pass" --output out.pdf`
- `paperpilot watermark --input in.pdf --text "DRAFT" --output out.pdf`
- `paperpilot redact --input in.pdf --pages 1 --rect "10,10,50,50" --output out.pdf`
- `paperpilot metadata --input in.pdf --title "New Title" --output out.pdf`
- `paperpilot signature --input in.pdf --cert cert.pem --output out.pdf`
- `paperpilot flatten --input in.pdf --output out.pdf`
- `paperpilot pdf-a --input in.pdf --output out.pdf`
- `paperpilot header-footer --input in.pdf --text "Page {page}" --output out.pdf`
- `paperpilot bates --input in.pdf --prefix "EXH-" --start 1 --output out.pdf`

### Group C & D: Extraction, Search & Conversion
- `paperpilot extract-text --input in.pdf --output text.txt`
- `paperpilot extract-images --input in.pdf --output out_dir/`
- `paperpilot search --input in.pdf --query "Invoice"`
- `paperpilot render --input in.pdf --output out_dir/` (Stub)
- `paperpilot compare --input a.pdf --input b.pdf` (Stub)
- `paperpilot ocr --input in.pdf --output out.pdf` (Stub)
- `paperpilot bookmarks --input in.pdf` (Stub)
- `paperpilot images-to-pdf --images a.png b.png --output out.pdf`
