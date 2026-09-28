use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "paperpilot",
    version,
    about = "A fast, privacy-first PDF manipulation tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Group B
    Compress {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Repair {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Linearize {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Encrypt {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        user_password: Option<String>,
        #[arg(short, long)]
        owner_password: Option<String>,
        #[arg(short, long)]
        output: PathBuf,
    },
    Decrypt {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        password: Option<String>,
        #[arg(short, long)]
        output: PathBuf,
    },
    Watermark {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        text: String,
        #[arg(short, long)]
        output: PathBuf,
    },
    HeaderFooter {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        text: String,
        #[arg(short, long)]
        output: PathBuf,
    },
    Bates {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        prefix: String,
        #[arg(short, long)]
        start: u32,
        #[arg(short, long)]
        output: PathBuf,
    },
    Metadata {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        author: Option<String>,
        #[arg(short, long)]
        subject: Option<String>,
        #[arg(short, long)]
        keywords: Option<String>,
        #[arg(short, long)]
        output: PathBuf,
    },
    Signature {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        cert: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Redact {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long, value_delimiter = ',')]
        pages: Vec<u32>,
        #[arg(short, long)]
        rect: String,
        #[arg(short, long)]
        output: PathBuf,
    },
    Flatten {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    PdfA {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },

    // Group C
    ExtractText {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    ExtractImages {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf, // output dir
    },
    ImagesToPdf {
        #[arg(short, long, num_args = 1..)]
        images: Vec<PathBuf>,
        #[arg(short, long)]
        output: PathBuf,
    },
    Search {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        query: String,
    },
    Compare {
        #[arg(short, long)]
        input1: PathBuf,
        #[arg(short, long)]
        input2: PathBuf,
    },
    Render {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf, // output dir
    },
    Ocr {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Bookmarks {
        #[arg(short, long)]
        input: PathBuf,
    },
}
