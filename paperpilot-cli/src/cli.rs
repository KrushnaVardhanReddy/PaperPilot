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
    /// Generate shell completions
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    // --- Group A: Page Manipulation ---
    /// Merge PDFs: Combine multiple PDF files [Input: 2+ .pdfs | Output: .pdf]
    Merge {
        #[arg(long, required = true, num_args = 1..)]
        input: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },
    /// Split PDF: Divide document into separate pages [Input: .pdf | Output: multiple .pdfs]
    Split {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "")]
        pages: String,
        #[arg(long)]
        every: Option<u32>,
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
    /// Rotate Pages: Rotate pages by 90, 180, or 270 degrees [Input: .pdf | Output: .pdf]
    Rotate {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: Option<String>,
        #[arg(long, alias = "angle")]
        degrees: i32,
        #[arg(long)]
        output: PathBuf,
    },
    /// Crop Pages: Crop pages to rectangular area [Input: .pdf | Output: .pdf]
    Crop {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: Option<String>,
        #[arg(long)]
        rect: Option<String>,
        #[arg(long)]
        x: Option<f32>,
        #[arg(long)]
        y: Option<f32>,
        #[arg(long)]
        width: Option<f32>,
        #[arg(long)]
        height: Option<f32>,
        #[arg(long)]
        output: PathBuf,
    },
    /// Burst PDF: Split document into single pages [Input: .pdf | Output: multiple .pdfs]
    Burst {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, aliases = &["output-dir"])]
        output: PathBuf,
    },
    /// Remove Blank Pages: Remove empty or nearly empty pages [Input: .pdf | Output: .pdf]
    #[command(name = "remove-blank")]
    RemoveBlank {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },

    // --- Group B: Document Operations ---
    /// Compress PDF: Reduce document file size [Input: .pdf | Output: .pdf]
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
    /// Fast Web View: Optimize PDF for fast web viewing [Input: .pdf | Output: .pdf]
    Linearize {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Encrypt with Password: Protect document [Input: .pdf | Output: .pdf]
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
    /// Remove Password: Remove password restrictions [Input: .pdf | Output: .pdf]
    Decrypt {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        output: PathBuf,
    },
    /// Add Watermark: Stamp text or branding [Input: .pdf | Output: .pdf]
    Watermark {
        #[arg(long)]
        input: std::path::PathBuf,
        #[arg(long)]
        text: String,
        #[arg(long, default_value_t = 45.0)]
        angle: f32,
        #[arg(long, default_value_t = 0.2)]
        opacity: f32,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        font_size: Option<f32>,
        #[arg(long)]
        output: std::path::PathBuf,
    },
    /// Redact Text: Permanently remove sensitive information [Input: .pdf | Output: .pdf]
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
    /// Deep Security Sanitizer: Strip metadata, active scripts, embedded files, thumbnails [Input: .pdf | Output: .pdf]
    /// Scan document for PII and sensitive information [Input: .pdf | Output: JSON report]
    #[command(name = "pii-scan")]
    PiiScan {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = true)]
        scan_ssn: bool,
        #[arg(long, default_value_t = true)]
        scan_credit_cards: bool,
        #[arg(long, default_value_t = true)]
        scan_emails: bool,
        #[arg(long, default_value_t = true)]
        scan_phones: bool,
        #[arg(long, default_value_t = true)]
        scan_iban: bool,
    },
    Sanitize {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = true)]
        remove_metadata: bool,
        #[arg(long, default_value_t = true)]
        remove_attachments: bool,
        #[arg(long, default_value_t = true)]
        remove_scripts: bool,
    },
    /// Edit Metadata: Edit document metadata [Input: .pdf | Output: .pdf]
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
    /// Sign Document: Apply digital signature to document [Input: .pdf | Output: .pdf]
    #[command(alias = "sign")]
    Signature {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        cert: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Flatten Annotations: Bake annotations and forms into page [Input: .pdf | Output: .pdf]
    Flatten {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Convert to PDF/A: Archive-ready PDF format conversion [Input: .pdf | Output: .pdf]
    #[command(alias = "to-pdf-a")]
    PdfA {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Header & Footer: Add headers and footers to pages [Input: .pdf | Output: .pdf]
    HeaderFooter {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        text: String,
        #[arg(long)]
        output: PathBuf,
    },
    /// Bates Numbering: Apply Bates stamps to pages [Input: .pdf | Output: .pdf]
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
    /// Add Page Numbers: Add dynamic page numbering [Input: .pdf | Output: .pdf]
    #[command(name = "page-numbers")]
    PageNumbers {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "bottom-center")]
        position: String,
    },

    // --- Group C & D: Extraction, Search & Conversion ---
    /// PDF to Plain Text: Extract all raw textual content from a PDF [Input: .pdf | Output: .txt]
    ExtractText {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Extract Tabular Data: Extract tables to JSON or CSV [Input: .pdf | Output: .json / .csv]
    ExtractTables {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long, default_value = "all")]
        pages: String,
    },
    ExtractImages {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Search Text: Search for text across document [Input: .pdf | Output: .json]
    Search {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        query: String,
    },
    /// Render Page: Render page to image format [Input: .pdf | Output: png images]
    Render {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "1")]
        page: u32,
        #[arg(long)]
        output: PathBuf,
    },
    /// Compare PDFs: Diff two documents and highlight changes [Input: 2 .pdfs | Output: .pdf]
    Compare {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        input_b: PathBuf,
    },
    /// OCR Text Recognition: Run OCR on scanned pages [Input: .pdf | Output: .txt / .pdf]
    Ocr {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "pdf")]
        format: String,
        #[arg(long)]
        clipboard: bool,
        #[arg(long)]
        tier: Option<String>,
    },
    /// Extract Bookmarks: Read document outline and bookmarks [Input: .pdf | Output: .json]
    Bookmarks {
        #[arg(long)]
        input: PathBuf,
    },
    /// Images to PDF: Combine images into PDF [Input: images | Output: .pdf]
    ImagesToPdf {
        #[arg(long, required = true, num_args = 1..)]
        images: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },

    // --- Validate ---
    /// Add Annotations: Programmatically add annotations [Input: .pdf | Output: .pdf]
    Annotate {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        data: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Classify {
        #[arg(long)]
        input: PathBuf,
    },
    /// Validate PDF: Verify document structure and standards [Input: .pdf | Output: .txt]
    Validate {
        #[arg(long)]
        input: PathBuf,
    },
    /// Generate Hash: Calculate document checksums [Input: .pdf | Output: .txt]
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
        #[arg(long)]
        css: Option<PathBuf>,
        #[arg(long)]
        css_preset: Option<String>,
        #[arg(long)]
        inline_css: Option<String>,
    },
    /// Declarative multi-step document workflow execution
    Workflow {
        #[command(subcommand)]
        subcommand: Option<WorkflowSubcommand>,

        /// Input PDF file (when running workflow directly)
        #[arg(long)]
        input: Option<PathBuf>,

        /// Output PDF file (when running workflow directly)
        #[arg(long)]
        output: Option<PathBuf>,

        /// Preset workflow name (e.g. preset-legal-discovery, preset-clean-scan)
        #[arg(long)]
        preset: Option<String>,

        /// Path to custom workflow recipe JSON file
        #[arg(long)]
        recipe: Option<PathBuf>,
    },
    // --- Headless Server / Automation Gateway ---
    #[cfg(feature = "daemon")]
    Serve {
        #[arg(long, default_value = "7823")]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        bind: String,
    },

    /// List all available tools, capabilities, and launch status
    ListTools,

    // --- Office Document Conversion ---
    #[cfg(feature = "office")]
    Office {
        #[command(subcommand)]
        command: crate::commands::office::OfficeCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum WorkflowSubcommand {
    /// List all available workflow presets
    List,
    /// Inspect step sequence of a preset or recipe JSON file
    Inspect {
        #[arg(long)]
        preset: Option<String>,
        #[arg(long)]
        recipe: Option<PathBuf>,
    },
    /// Execute workflow preset or recipe JSON against input PDF
    Run {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        preset: Option<String>,
        #[arg(long)]
        recipe: Option<PathBuf>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_rotate_angle_alias() {
        let args = vec![
            "paperpilot",
            "rotate",
            "--input",
            "in.pdf",
            "--angle",
            "90",
            "--output",
            "out.pdf",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Rotate {
                input,
                pages,
                degrees,
                output,
            } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(pages, None);
                assert_eq!(degrees, 90);
                assert_eq!(output, PathBuf::from("out.pdf"));
            }
            _ => panic!("Expected Rotate command"),
        }
    }

    #[test]
    fn test_burst_output_dir_alias() {
        let args = vec![
            "paperpilot",
            "burst",
            "--input",
            "in.pdf",
            "--output-dir",
            "out_dir",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Burst { input, output } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(output, PathBuf::from("out_dir"));
            }
            _ => panic!("Expected Burst command"),
        }
    }

    #[test]
    fn test_crop_individual_flags() {
        let args = vec![
            "paperpilot",
            "crop",
            "--input",
            "in.pdf",
            "--x",
            "10",
            "--y",
            "20",
            "--width",
            "30",
            "--height",
            "40",
            "--output",
            "out.pdf",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Crop {
                input,
                pages,
                rect,
                x,
                y,
                width,
                height,
                output,
            } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(pages, None);
                assert_eq!(rect, None);
                assert_eq!(x, Some(10.0));
                assert_eq!(y, Some(20.0));
                assert_eq!(width, Some(30.0));
                assert_eq!(height, Some(40.0));
                assert_eq!(output, PathBuf::from("out.pdf"));
            }
            _ => panic!("Expected Crop command"),
        }
    }

    #[test]
    fn test_remove_blank() {
        let args = vec![
            "paperpilot",
            "remove-blank",
            "--input",
            "in.pdf",
            "--output",
            "out.pdf",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::RemoveBlank { input, output } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(output, PathBuf::from("out.pdf"));
            }
            _ => panic!("Expected RemoveBlank command"),
        }
    }

    #[test]
    fn test_page_numbers() {
        let args = vec![
            "paperpilot",
            "page-numbers",
            "--input",
            "in.pdf",
            "--output",
            "out.pdf",
            "--position",
            "top-left",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::PageNumbers {
                input,
                output,
                position,
            } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(output, PathBuf::from("out.pdf"));
                assert_eq!(position, "top-left");
            }
            _ => panic!("Expected PageNumbers command"),
        }
    }

    #[test]
    fn test_images_to_pdf_multiple_inputs() {
        let args = vec![
            "paperpilot",
            "images-to-pdf",
            "--images",
            "1.png",
            "2.png",
            "--output",
            "out.pdf",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::ImagesToPdf { images, output } => {
                assert_eq!(images.len(), 2);
                assert_eq!(images[0], PathBuf::from("1.png"));
                assert_eq!(images[1], PathBuf::from("2.png"));
                assert_eq!(output, PathBuf::from("out.pdf"));
            }
            _ => panic!("Expected ImagesToPdf command"),
        }
    }

    #[test]
    fn test_signature_alias() {
        let args = vec![
            "paperpilot",
            "sign",
            "--input",
            "in.pdf",
            "--cert",
            "cert.pem",
            "--output",
            "out.pdf",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Signature {
                input,
                cert,
                output,
            } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(cert, PathBuf::from("cert.pem"));
                assert_eq!(output, PathBuf::from("out.pdf"));
            }
            _ => panic!("Expected Signature command"),
        }
    }

    #[test]
    fn test_pdfa_alias() {
        let args = vec![
            "paperpilot",
            "to-pdf-a",
            "--input",
            "in.pdf",
            "--output",
            "out.pdf",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::PdfA { input, output } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(output, PathBuf::from("out.pdf"));
            }
            _ => panic!("Expected PdfA command"),
        }
    }

    #[test]
    fn test_merge_multiple_inputs() {
        let args = vec![
            "paperpilot",
            "merge",
            "--input",
            "a.pdf",
            "b.pdf",
            "--output",
            "out.pdf",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Merge { input, output } => {
                assert_eq!(input.len(), 2);
                assert_eq!(input[0], PathBuf::from("a.pdf"));
                assert_eq!(input[1], PathBuf::from("b.pdf"));
                assert_eq!(output, PathBuf::from("out.pdf"));
            }
            _ => panic!("Expected Merge command"),
        }
    }

    #[test]
    fn test_workflow_run_args() {
        let args = vec![
            "paperpilot",
            "workflow",
            "run",
            "--input",
            "in.pdf",
            "--output",
            "out.pdf",
            "--preset",
            "preset-clean-scan",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Workflow {
                subcommand:
                    Some(WorkflowSubcommand::Run {
                        input,
                        output,
                        preset,
                        recipe,
                    }),
                ..
            } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(output, PathBuf::from("out.pdf"));
                assert_eq!(preset, Some("preset-clean-scan".to_string()));
                assert_eq!(recipe, None);
            }
            _ => panic!("Expected Workflow Run command"),
        }
    }
}
