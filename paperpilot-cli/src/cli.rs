use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// PaperPilot CLI - Advanced PDF manipulation tool
#[derive(Parser, Debug)]
#[command(name = "paperpilot", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Merge multiple PDFs into one
    Merge {
        /// Input PDF files to merge
        #[arg(short, long, required = true, num_args = 1..)]
        input: Vec<PathBuf>,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Split a PDF into individual pages
    Split {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for split pages
        #[arg(short, long)]
        output_dir: PathBuf,
    },
    /// Extract specific pages from a PDF
    Extract {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Pages to extract (1-based indices)
        #[arg(short, long, required = true, num_args = 1..)]
        pages: Vec<u32>,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Delete specific pages from a PDF
    Delete {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Pages to delete (1-based indices)
        #[arg(short, long, required = true, num_args = 1..)]
        pages: Vec<u32>,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Reorder pages in a PDF
    Reorder {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// New 1-based page order (e.g., 3 1 2)
        #[arg(long, required = true, num_args = 1..)]
        order: Vec<u32>,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Rotate specific pages in a PDF
    Rotate {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Pages to rotate (1-based indices)
        #[arg(short, long, required = true, num_args = 1..)]
        pages: Vec<u32>,

        /// Degrees to rotate (must be a multiple of 90)
        #[arg(short, long)]
        degrees: u16,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Crop specific pages in a PDF
    Crop {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Pages to crop (1-based indices)
        #[arg(short, long, required = true, num_args = 1..)]
        pages: Vec<u32>,

        /// Crop rectangle as string "x,y,width,height"
        #[arg(short, long)]
        rect: String,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Burst a PDF into multiple single-page documents
    Burst {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for split pages
        #[arg(short, long)]
        output_dir: PathBuf,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
