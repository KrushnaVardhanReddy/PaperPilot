use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "paperpilot",
    version,
    about = "A fast, privacy-first PDF manipulation tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output result as JSON
    #[arg(global = true, long)]
    pub json: bool,

    /// Post result as JSON to webhook URL
    #[arg(global = true, long)]
    pub webhook: Option<String>,

    /// Secret key to compute HMAC-SHA256 signature of webhook payload
    #[arg(global = true, long)]
    pub webhook_secret: Option<String>,

    /// Fire webhook only on operation success
    #[arg(global = true, long)]
    pub webhook_on_success: bool,

    /// Fire webhook only on operation failure
    #[arg(global = true, long)]
    pub webhook_on_failure: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // --- Group A: Page Manipulation ---
    Merge {
        #[arg(long, required = true)]
        input: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },
    Split {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        output: PathBuf,
    },
    Extract {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        output: PathBuf,
    },
    Delete {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        output: PathBuf,
    },
    Reorder {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        order: String,
        #[arg(long)]
        output: PathBuf,
    },
    Rotate {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        degrees: i32,
        #[arg(long)]
        output: PathBuf,
    },
    Crop {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        rect: String,
        #[arg(long)]
        output: PathBuf,
    },
    Burst {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },

    // --- Group B: Document Operations ---
    Compress {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        quality: String,
        #[arg(long)]
        output: PathBuf,
    },
    Repair {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Linearize {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Encrypt {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        user_password: Option<String>,
        #[arg(long)]
        owner_password: Option<String>,
        #[arg(long)]
        output: PathBuf,
    },
    Decrypt {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        output: PathBuf,
    },
    Watermark {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        text: String,
        #[arg(long)]
        output: PathBuf,
    },
    Redact {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        rect: String,
        #[arg(long)]
        output: PathBuf,
    },
    Metadata {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        author: Option<String>,
        #[arg(long)]
        subject: Option<String>,
        #[arg(long)]
        keywords: Option<String>,
        #[arg(long)]
        output: PathBuf,
    },
    Signature {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        cert: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Flatten {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    PdfA {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    HeaderFooter {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        text: String,
        #[arg(long)]
        output: PathBuf,
    },
    Bates {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        prefix: String,
        #[arg(long)]
        start: u32,
        #[arg(long)]
        output: PathBuf,
    },

    // --- Group C & D: Extraction, Search & Conversion ---
    ExtractText {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "text")]
        format: String,
    },
    ExtractImages {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Search {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        query: String,
    },
    Render {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Compare {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        input_b: PathBuf,
    },
    Ocr {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Bookmarks {
        #[arg(long)]
        input: PathBuf,
    },
    ImagesToPdf {
        #[arg(long, required = true)]
        images: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },

    // --- Validate ---
    Classify {
        #[arg(long)]
        input: PathBuf,
    },
    Validate {
        #[arg(long)]
        input: PathBuf,
    },
    Hash {
        #[arg(long)]
        input: PathBuf,
    },
    Verify {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        expected_hash: String,
    },

    // --- Group E: Forms ---
    Form {
        #[command(subcommand)]
        command: FormCommands,
    },
    // --- Group D: Conversion ---
    Convert {
        #[arg(long, required = true)]
        format: String,
        #[arg(long, required = true)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}


#[derive(Subcommand, Debug)]
pub enum FormCommands {
    Read {
        input: PathBuf,
    },
    Fill {
        input: PathBuf,
        #[arg(long)]
        data: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    AddField {
        input: PathBuf,
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "text")]
        r#type: String,
        #[arg(long, default_value = "1")]
        page: i32,
        #[arg(long)]
        rect: String,
        #[arg(short, long)]
        output: PathBuf,
    },
}
