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
        #[arg(long, required = true, num_args = 1..)]
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
        pages: Option<String>,
        #[arg(long, alias = "angle")]
        degrees: i32,
        #[arg(long)]
        output: PathBuf,
    },
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
    Burst {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, aliases = &["output-dir"])]
        output: PathBuf,
    },
    #[command(name = "remove-blank")]
    RemoveBlank {
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
        #[arg(long, default_value = "1")]
        page: u32,
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
        #[arg(long, required = true, num_args = 1..)]
        images: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },

    // --- Validate ---
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
        #[arg(long)]
        css: Option<PathBuf>,
        #[arg(long)]
        css_preset: Option<String>,
        #[arg(long)]
        inline_css: Option<String>,
    },
    // --- Headless Server / Automation Gateway ---
    Serve {
        #[arg(long, default_value = "7823")]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        bind: String,
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
    fn test_merge_multiple_inputs() {
        let args = vec!["paperpilot", "merge", "--input", "a.pdf", "b.pdf", "--output", "out.pdf"];
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
    fn test_rotate_angle_alias() {
        let args = vec!["paperpilot", "rotate", "--input", "in.pdf", "--angle", "90", "--output", "out.pdf"];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Rotate { input, pages, degrees, output } => {
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
        let args = vec!["paperpilot", "burst", "--input", "in.pdf", "--output-dir", "out_dir"];
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
            "paperpilot", "crop", "--input", "in.pdf", "--x", "10", "--y", "20",
            "--width", "30", "--height", "40", "--output", "out.pdf",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Crop { input, pages, rect, x, y, width, height, output } => {
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
        let args = vec!["paperpilot", "remove-blank", "--input", "in.pdf", "--output", "out.pdf"];
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
            "paperpilot", "page-numbers", "--input", "in.pdf",
            "--output", "out.pdf", "--position", "top-left",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::PageNumbers { input, output, position } => {
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
            "paperpilot", "images-to-pdf", "--images", "1.png", "2.png", "--output", "out.pdf",
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
}
