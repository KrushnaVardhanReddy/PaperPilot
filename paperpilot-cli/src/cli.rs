use clap::{Args, Parser, Subcommand};
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

    /// Output format for logs: text, json, compact, or pretty
    #[arg(global = true, long)]
    pub log_format: Option<String>,

    /// Log level verbosity: trace, debug, info, warn, error
    #[arg(global = true, long)]
    pub log_level: Option<String>,

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

    /// Force zero-copy memory-mapped (`memmap2`) streaming regardless of file size threshold
    #[arg(global = true, long)]
    pub mmap: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fill one AcroForm PDF per CSV/JSON record and write an execution receipt
    #[cfg(feature = "pdf")]
    #[command(name = "fill-batch", alias = "batch-fill", alias = "batch_fill", alias = "cli-fill-batch", alias = "cli_fill_batch", alias = "pdf-fill-batch")]
    FillBatch(FillBatchArgs),
    /// Launch the PaperPilot Graphical Desktop UI
    #[command(alias = "gui")]
    Ui,
    /// Run system diagnostics to check offline dependencies
    Doctor {
        #[arg(long)]
        json: bool,
    },
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
    #[command(name = "extract-pages", alias = "extract")]
    Extract {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "delete-pages", alias = "delete")]
    Delete {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "reorder-pages", alias = "reorder")]
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
        #[arg(long, default_value = "95")]
        sensitivity: u8,
        #[arg(long)]
        output: PathBuf,
    },

    // --- Group B: Document Operations ---
    #[command(flatten)]
    DocumentOps(DocumentOpsCommands),
    /// Inspect PDF Metadata & Page Count: Display PDF page count and version [Input: .pdf | Output: .json]
    #[command(name = "info", alias = "pdf-info", alias = "pdf_info")]
    Info {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Sign Document: Apply digital signature to document [Input: .pdf | Output: .pdf]
    #[command(alias = "sign", alias = "pdf-sign", alias = "pdf_sign")]
    Signature {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        cert: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "pdf-a", alias = "to-pdf-a", alias = "pdf-to-pdf-a", alias = "pdf_to_pdf_a")]
    PdfA {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Add Page Numbers: Add dynamic page numbering [Input: .pdf | Output: .pdf]
    #[command(name = "page-numbers", alias = "page_numbers", alias = "pdf-page-numbers", alias = "pdf_page_numbers")]
    PageNumbers {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "bottom-center")]
        position: String,
    },

    // --- Group C, D & E: Extraction, Search, Conversion & Forms ---
    #[command(flatten)]
    Extraction(ExtractionCommands),
    /// Images to PDF: Combine images into PDF [Input: images | Output: .pdf]
    ImagesToPdf {
        #[arg(long, required = true, num_args = 1..)]
        images: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(flatten)]
    ExtendedPdf(ExtendedPdfCommands),
    #[command(flatten)]
    AiMedia(AiMediaCommands),
    /// Declarative multi-step document workflow execution
    Run {
        /// TOML .ppflow or JSON recipe
        recipe: PathBuf,
        /// Local input PDF
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        /// With this template, --output is a directory; existing PDFs are preserved
        #[arg(long)]
        name_template: Option<String>,
    },
    /// List, inspect, or execute a document workflow
    #[command(name = "workflow", alias = "document-workflow", alias = "document_workflow", alias = "recipe-run", alias = "recipe_run")]
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

        /// Path to custom workflow recipe (.ppflow TOML or JSON)
        #[arg(long)]
        recipe: Option<PathBuf>,
        /// Output naming template; --output is then a directory
        #[arg(long)]
        name_template: Option<String>,
    },
    // --- Headless Server / Automation Gateway ---
    #[cfg(feature = "daemon")]
    Serve {
        #[arg(long, default_value = "7823")]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        bind: String,
        /// Maximum request body limit in MB (0 or "unlimited" to disable limit)
        #[arg(long = "max-body-mb", env = "PAPERPILOT_MAX_BODY_LIMIT_MB")]
        max_body_mb: Option<String>,
    },

    /// Ask in natural language: execute operations via natural language intent [e.g. "rotate 90 degrees" doc.pdf]
    #[command(name = "ask", alias = "cli-ask", alias = "cli_ask")]
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

    /// List all available tools, capabilities, and launch status
    ListTools,

    // --- Office Document Conversion ---
    #[cfg(feature = "office")]
    Office {
        #[command(subcommand)]
        command: crate::commands::office::OfficeCommands,
    },
}

impl Commands {
    /// Returns the canonical variant name used in receipts and JSON command output.
    pub fn operation_name(&self) -> String {
        let dbg = match self {
            Self::DocumentOps(sub) => format!("{sub:?}"),
            Self::Extraction(sub) => format!("{sub:?}"),
            Self::ExtendedPdf(sub) => format!("{sub:?}"),
            Self::AiMedia(sub) => format!("{sub:?}"),
            other => format!("{other:?}"),
        };
        dbg.split('(')
            .next()
            .unwrap_or("Unknown")
            .split_whitespace()
            .next()
            .unwrap_or("Unknown")
            .to_string()
    }
}

#[derive(Subcommand, Debug)]
pub enum DocumentOpsCommands {
    /// Compress PDF: Reduce document file size [Input: .pdf | Output: .pdf]
    Compress {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        quality: String,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "pdf-repair", alias = "repair")]
    Repair {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Fast Web View: Optimize PDF for fast web viewing [Input: .pdf | Output: .pdf]
    #[command(name = "pdf-linearize", alias = "linearize")]
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
        #[arg(long, visible_alias = "password", alias = "password")]
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
    #[command(name = "watermark", alias = "pdf-watermark", alias = "pdf_watermark")]
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
    #[command(name = "redact", alias = "pdf-redact", alias = "pdf_redact")]
    Redact(Box<RedactArgs>),
    /// Deep Security Sanitizer: Strip metadata, active scripts, embedded files, thumbnails [Input: .pdf | Output: .pdf]
    /// Scan document for PII and sensitive information [Input: .pdf | Output: JSON report]
    #[command(name = "pii-scan", alias = "document-pii-scan", alias = "document_pii_scan", alias = "pii_scan")]
    PiiScan(Box<PiiScanArgs>),
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
    /// Flatten Annotations: Bake annotations and forms into page [Input: .pdf | Output: .pdf]
    #[command(alias = "pdf-flatten", alias = "pdf_flatten")]
    Flatten {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Invert PDF colors or apply dark mode
    #[command(name = "pdf-invert-colors", alias = "invert-colors")]
    InvertColors {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        mode: Option<String>,
        #[arg(long)]
        pages: Option<String>,
    },
    /// Header & Footer: Add headers and footers to pages [Input: .pdf | Output: .pdf]
    #[command(name = "header-footer", alias = "header_footer", alias = "pdf-header-footer", alias = "pdf_header_footer")]
    HeaderFooter {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        text: String,
        #[arg(long, default_value = "header")]
        position: String,
        #[arg(long)]
        output: PathBuf,
    },
    /// Bates Numbering: Apply Bates stamps to pages [Input: .pdf | Output: .pdf]
    #[command(name = "bates", alias = "pdf-bates", alias = "pdf_bates")]
    Bates {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "CONF-")]
        prefix: String,
        #[arg(long, default_value_t = 1)]
        start: u32,
        #[arg(long, default_value = "bottom-right")]
        position: String,
        #[arg(long)]
        output: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum ExtractionCommands {
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
    #[command(name = "extract-images", alias = "pdf-extract-images", alias = "pdf_extract_images")]
    ExtractImages {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, alias = "output-dir")]
        output: PathBuf,
    },
    /// Search Text: Search for text across document [Input: .pdf | Output: .json]
    #[command(name = "search", alias = "pdf-search", alias = "pdf_search")]
    Search {
        #[arg(long)]
        input: Option<PathBuf>,
        #[arg(long)]
        query: String,
        #[arg(long)]
        dir: Option<PathBuf>,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Render Page: Render page to image format [Input: .pdf | Output: png/jpeg/webp images]
    #[command(name = "render", alias = "pdf-render", alias = "pdf_render")]
    Render {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "1")]
        page: u32,
        #[arg(long, default_value_t = 150.0)]
        dpi: f32,
        #[arg(long)]
        format: Option<String>,
        #[arg(long)]
        output: PathBuf,
    },
    /// Compare PDFs: Diff two documents and highlight changes [Input: 2 .pdfs | Output: .pdf / .json]
    Compare {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        input_b: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
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
    /// Add Annotations: Programmatically add annotations [Input: .pdf | Output: .pdf]
    #[command(name = "annotate", alias = "pdf-annotate", alias = "pdf_annotate")]
    Annotate {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "[]")]
        data: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "classify-type", alias = "classify", alias = "pdf-classify-type", alias = "pdf_classify_type")]
    Classify {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Validate PDF: Verify document structure and standards [Input: .pdf | Output: .txt/.json]
    #[command(alias = "pdf-validate", alias = "pdf_validate")]
    Validate {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Generate Hash: Calculate document checksums [Input: .pdf | Output: .txt]
    #[command(alias = "pdf-hash", alias = "pdf_hash")]
    Hash {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Verify {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        expected_hash: String,
    },
    #[command(alias = "pdf-form", alias = "pdf_form")]
    Form {
        #[command(subcommand)]
        command: FormCommands,
    },
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
}

#[derive(Subcommand, Debug)]
pub enum ExtendedPdfCommands {
    /// Validate Digital Signatures: Cryptographically verify digital signatures, certificate chains, and document integrity.
    #[command(name = "pdf-validate-signatures", alias = "validate-signatures", alias = "validate_signatures", alias = "pdf_validate_signatures")]
    PdfValidateSignatures {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        /// Optional path to the baseline reference PDF for pre-sign tamper detection
        #[arg(long)]
        baseline: Option<PathBuf>,
    },
    /// Booklet Imposition: Reorder and impose pages for 2-sided folding booklet printing.
    #[command(name = "pdf-booklet")]
    PdfBooklet {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// N-Up Page Imposition: Combine multiple pages per sheet in a 2, 4, 9, or 16-up grid layout.
    #[command(name = "pdf-nup", alias = "nup", alias = "pdf_nup")]
    PdfNup(Box<PdfNupArgs>),
    /// Scale Page Dimensions: Scale page dimensions between standard formats or percentages.
    #[command(name = "pdf-scale-pages", alias = "scale-pages", alias = "scale_pages", alias = "pdf_scale_pages")]
    PdfScalePages {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "A4")]
        target_size: Option<String>,
        #[arg(long, default_value = "1.0")]
        scale_factor: Option<f32>,
    },
    /// Remove Digital Signatures: Strip digital signature fields, cryptographic locks, and widget annotations.
    #[command(name = "pdf-remove-signatures", alias = "remove-signatures", alias = "remove_signatures", alias = "pdf_remove_signatures")]
    PdfRemoveSignatures {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Stamp Image & Logo: Place coordinate-exact images, logos, and badges onto PDF pages.
    #[command(name = "pdf-stamp", alias = "stamp", alias = "pdf_stamp")]
    PdfStamp(Box<PdfStampArgs>),
    /// Fill AcroForm Interactive Form: Populate text boxes, checkboxes and choices with JSON dataset.
    #[command(name = "pdf-fill-form", alias = "fill-form", alias = "pdf_fill_form")]
    PdfFillForm {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        data: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Add AcroForm Field: Append interactive widget annotation with coordinates and field name to target page.
    #[command(name = "pdf-create-form-field", alias = "create-form-field", alias = "pdf_create_form_field", alias = "add-field")]
    PdfCreateFormField(Box<PdfCreateFormFieldArgs>),
    /// Overlay & Underlay Stationary: Apply letterheads, watermarked stationary, and templates as background or foreground.
    #[command(name = "pdf-overlay-underlay", alias = "overlay-underlay", alias = "overlay_underlay", alias = "pdf_overlay_underlay")]
    PdfOverlayUnderlay(Box<PdfOverlayUnderlayArgs>),
    /// Auto Upright Orientation: Heuristically detect text baseline and rotation angle to automatically upright pages.
    #[command(name = "pdf-auto-rotate")]
    PdfAutoRotate {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Stitch Continuous Canvas: Combine multiple pages vertically or horizontally into a continuous single-page canvas.
    #[command(name = "pdf-stitch-canvas")]
    PdfStitchCanvas {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "vertical")]
        direction: String,
        #[arg(long, default_value_t = 0.0)]
        margin: f32,
        #[arg(long, default_value_t = 0.0)]
        spacing: f32,
    },
    /// PDF Portfolio Attachments: Inspect, extract, or embed file attachments into PDF portfolios.
    #[command(name = "pdf-attachments")]
    PdfAttachments {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        action: Option<String>,
        #[arg(long)]
        file_name: Option<String>,
        #[arg(long)]
        file: Option<PathBuf>,
    },
    /// Cryptographic Timestamp TSA: Apply RFC 3161 cryptographically verified TSA timestamp tokens.
    #[command(name = "pdf-timestamp", alias = "timestamp")]
    PdfTimestamp {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        server_url: Option<String>,
    },
    /// Modify PDF Permissions: Configure user and owner permissions for printing, copying, and modification.
    #[command(name = "pdf-change-permissions", alias = "change-permissions")]
    PdfChangePermissions {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        allow_print: Option<bool>,
        #[arg(long)]
        allow_copy: Option<bool>,
        #[arg(long)]
        allow_modify: Option<bool>,
    },
    /// Split Scanned Book Spine: Detect center binding gutter spine and split two-up page scans into single pages.
    #[command(name = "pdf-split-scanned-book")]
    PdfSplitScannedBook(Box<PdfSplitScannedBookArgs>),
    /// Comic Archive Conversion: Convert between PDF documents and comic book archive formats (CBZ/CBR).
    #[command(name = "pdf-comic-archive", alias = "comic-archive", alias = "pdf_comic_archive")]
    PdfComicArchive {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Split PDF by Outline: Split multi-chapter PDF documents based on their /Outlines bookmark hierarchy
    #[command(name = "pdf-split-by-outline")]
    PdfSplitByOutline {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Alternating PDF Merge: Interleave pages from two PDF documents.
    #[command(name = "pdf-merge-alternate")]
    PdfMergeAlternate(Box<PdfMergeAlternateArgs>),
    /// Multi-Sheet Poster Tile Slicer: Tile oversized PDF pages across a grid of standard printable sheets with overlap margins and cut guides
    #[command(name = "pdf-posterize")]
    PdfPosterize(Box<PdfPosterizeArgs>),
    /// Lossless Stream Deduplication: Lossless hash-based indirect object stream deduplication & CMYK-safe image compression
    #[command(name = "pdf-dedup-streams")]
    PdfDedupStreams {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// PDF to EPUB 3.0: Convert fixed-layout PDF documents into reflowable EPUB 3.0 eBooks
    #[command(name = "pdf-to-epub", alias = "to-epub", alias = "pdf_to_epub")]
    PdfToEpub {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "pdf-to-html")]
    PdfToHtml {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "pdf-to-markdown")]
    PdfToMarkdown {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "pdf-to-json")]
    PdfToJson {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    #[command(name = "json-to-pdf", alias = "json_to_pdf")]
    JsonToPdf {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum AiMediaCommands {
    /// Render Markdown source documents into publication-ready styled PDF files
    #[command(name = "md-to-pdf", alias = "md_to_pdf", alias = "convert-markdown", alias = "pdf-convert-markdown", alias = "pdf_convert_markdown")]
    MdToPdf(Box<StyledConversionArgs>),
    /// Convert HTML source code and web pages directly into styled PDF documents
    #[command(name = "html-to-pdf", alias = "html_to_pdf", alias = "convert-html", alias = "pdf-convert-html", alias = "pdf_convert_html")]
    HtmlToPdf(Box<StyledConversionArgs>),
    /// Convert spreadsheet (Excel / CSV) to formatted PDF document
    #[command(name = "convert-excel", alias = "convert_excel", alias = "pdf-convert-excel", alias = "pdf_convert_excel", alias = "excel-to-pdf", alias = "excel_to_pdf")]
    ConvertExcel(Box<StyledConversionArgs>),
    #[command(name = "rag-chunks", alias = "rag_chunks", alias = "pdf-rag-chunks", alias = "pdf_rag_chunks")]
    RagChunks {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Intelligent structure-aware semantic chunking with dynamic overlap for AI agents [Input: .pdf | Output: .json]
    #[command(name = "pdf-chunk", alias = "pdf_chunk", alias = "chunk")]
    PdfChunk(Box<PdfChunkArgs>),
    #[command(name = "workspace-search", alias = "workspace_search")]
    WorkspaceSearch(Box<WorkspaceSearchArgs>),
    #[command(name = "semantic-search", alias = "semantic_search")]
    SemanticSearch(Box<SemanticSearchArgs>),
    /// Extract Keywords & Patterns: Scan document for custom keywords and regex patterns with match contexts [Input: .pdf | Output: .json]
    #[command(name = "pdf-extract-patterns", alias = "extract-patterns", alias = "extract_patterns", alias = "pdf_extract_patterns")]
    PdfExtractPatterns(Box<PdfExtractPatternsArgs>),
    /// Convert PDF to Animated GIF: Convert multi-page PDF documents into animated looping GIF previews
    #[command(name = "pdf-to-gif")]
    PdfToGif(Box<PdfToGifArgs>),
    /// Generate Showcase GIF: Generate animated showcase demo GIF comparing before and after transformations
    #[command(name = "showcase-gif")]
    ShowcaseGif(Box<ShowcaseGifArgs>),
    /// Combine Images into Animated GIF: Stitch a sequence of images into a single animated looping GIF
    #[command(name = "images-to-gif")]
    ImagesToGif(Box<ImagesToGifArgs>),
    /// Optimize Image for Vision LLMs: Downscale and optimize images for LLM vision token economy and clarity
    #[command(name = "img-optimize-for-llm")]
    ImgOptimizeForLlm {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, help = "Max width or height in pixels")]
        max_dim: Option<u32>,
        #[arg(long, help = "Quality (1-100)")]
        quality: Option<u8>,
    },
    /// Deskew Document Image: Detect text angle and straighten tilted scanned document images
    #[command(name = "img-deskew")]
    ImgDeskew {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        angle: Option<f32>,
    },
    /// Convert Image Format: Convert images between PNG, JPEG, WebP, TIFF, and AVIF formats
    #[command(name = "img-convert")]
    ImgConvert {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, help = "Target format: png, jpeg, webp, tiff, avif")]
        format: Option<String>,
        #[arg(long, help = "Compression quality (1-100)")]
        quality: Option<u8>,
    },
    /// Auto-Crop Image to Content: Detect document boundaries and auto-crop background margins from scanned images
    #[command(name = "img-crop-to-content")]
    ImgCropToContent {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, help = "Background difference threshold (0.0-1.0)")]
        threshold: Option<f32>,
        #[arg(long, help = "Padding margin in pixels")]
        padding: Option<u32>,
    },
    /// Redact Sensitive Image Regions: Apply permanent, irreversible pixel-level redaction over sensitive image regions
    #[command(name = "img-redact")]
    ImgRedact {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, help = "Redaction rectangle x,y,w,h (can be specified multiple times)")]
        rect: Option<Vec<String>>,
        #[arg(long, help = "Fill color: hex (#000000) or name (black, white, red)")]
        color: Option<String>,
    },
}

#[derive(Args, Debug)]
pub struct RedactArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long, default_value = "1")]
    pub pages: String,
    #[arg(long, default_value = "0,0,100,100")]
    pub rect: String,
    #[arg(long)]
    pub data: Option<String>,
    #[arg(long)]
    pub query: Option<String>,
    #[arg(long)]
    pub regex: Option<String>,
    #[arg(long)]
    pub overlay_text: Option<String>,
    #[arg(long, default_value = "all")]
    pub occurrence: String,
    #[arg(long)]
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct PiiScanArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: Option<PathBuf>,
    #[arg(long, default_value_t = true)]
    pub scan_ssn: bool,
    #[arg(long, default_value_t = true)]
    pub scan_credit_cards: bool,
    #[arg(long, default_value_t = true)]
    pub scan_emails: bool,
    #[arg(long, default_value_t = true)]
    pub scan_phones: bool,
    #[arg(long, default_value_t = true)]
    pub scan_iban: bool,
}

#[derive(Args, Debug)]
pub struct PdfNupArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long, default_value_t = 2)]
    pub nup: u32,
    #[arg(long, default_value = "horizontal")]
    pub ordering: String,
    #[arg(long, default_value_t = 18.0)]
    pub margin: f32,
    #[arg(long, default_value_t = 10.0)]
    pub spacing: f32,
    #[arg(long, default_value_t = false)]
    pub draw_borders: bool,
}

#[derive(Args, Debug)]
pub struct PdfStampArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long)]
    pub image: Option<PathBuf>,
    #[arg(long)]
    pub x: Option<f32>,
    #[arg(long)]
    pub y: Option<f32>,
    #[arg(long)]
    pub width: Option<f32>,
    #[arg(long)]
    pub height: Option<f32>,
    #[arg(long)]
    pub opacity: Option<f32>,
    #[arg(long)]
    pub pages: Option<String>,
}

#[derive(Args, Debug)]
pub struct PdfCreateFormFieldArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub label: Option<String>,
    #[arg(long, default_value = "text")]
    pub r#type: String,
    #[arg(long, default_value = "1")]
    pub page: i32,
    #[arg(long)]
    pub rect: String,
    #[arg(short, long)]
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct PdfOverlayUnderlayArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long)]
    pub stationery: Option<PathBuf>,
    #[arg(long)]
    pub template: Option<PathBuf>,
    #[arg(long)]
    pub mode: Option<String>,
    #[arg(long)]
    pub pages: Option<String>,
    #[arg(long)]
    pub opacity: Option<f32>,
}

#[derive(Args, Debug)]
pub struct PdfSplitScannedBookArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    /// Reading direction (ltr or rtl)
    #[arg(long, default_value = "ltr")]
    pub reading_direction: String,
    /// Gutter overlap in points
    #[arg(long, default_value = "0.0")]
    pub gutter_overlap: f32,
    /// Split horizontally (top/bottom)
    #[arg(long)]
    pub split_horizontal: bool,
}

#[derive(Args, Debug)]
pub struct PdfMergeAlternateArgs {
    #[arg(long)]
    pub input_a: PathBuf,
    #[arg(long)]
    pub input_b: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long, default_value = "false")]
    pub reverse_a: bool,
    #[arg(long, default_value = "false")]
    pub reverse_b: bool,
    #[arg(long, default_value = "1")]
    pub step: usize,
    #[arg(long, default_value = "1")]
    pub step_b: usize,
}

#[derive(Args, Debug)]
pub struct PdfPosterizeArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long, default_value = "a4")]
    pub target_sheet: String,
    #[arg(long)]
    pub grid: Option<String>,
    #[arg(long, default_value = "14.17")]
    pub overlap: f32,
    #[arg(long, default_value = "1.0")]
    pub scale: f32,
    #[arg(long, default_value = "true")]
    pub guides: bool,
}

#[derive(Args, Debug)]
pub struct StyledConversionArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long)]
    pub preset: Option<String>,
    #[arg(long)]
    pub css: Option<PathBuf>,
    #[arg(long)]
    pub custom_css: Option<String>,
    #[arg(long)]
    pub orientation: Option<String>,
}

#[derive(Args, Debug)]
pub struct PdfChunkArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long, default_value_t = 500)]
    pub target_tokens: usize,
    #[arg(long, alias = "overlap-tokens", default_value_t = 50)]
    pub overlap: usize,
    #[arg(long, default_value_t = true, num_args = 0..=1, default_missing_value = "true", action = clap::ArgAction::Set)]
    pub strip_boilerplate: bool,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_strip_boilerplate: bool,
    #[arg(long, default_value_t = true, num_args = 0..=1, default_missing_value = "true", action = clap::ArgAction::Set)]
    pub break_on_headings: bool,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_break_on_headings: bool,
    #[arg(long, alias = "json-out")]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct WorkspaceSearchArgs {
    #[arg(long)]
    pub input: Option<PathBuf>,
    #[arg(long)]
    pub dir: Option<PathBuf>,
    #[arg(long)]
    pub query: Option<String>,
    #[arg(long)]
    pub formats: Option<String>,
    #[arg(long)]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct SemanticSearchArgs {
    #[arg(long)]
    pub input: Option<PathBuf>,
    #[arg(long)]
    pub dir: Option<PathBuf>,
    #[arg(long)]
    pub query: Option<String>,
    #[arg(long)]
    pub limit: Option<usize>,
    #[arg(long)]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct PdfExtractPatternsArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: Option<PathBuf>,
    /// Keywords to match
    #[arg(long, value_delimiter = ',')]
    pub keywords: Option<Vec<String>>,
    /// Regex patterns formatted as label=regex (e.g. ssn=\d{3}-\d{2}-\d{4})
    #[arg(long = "pattern", value_name = "LABEL=REGEX")]
    pub patterns: Option<Vec<String>>,
    /// Perform case-sensitive matching (default: false)
    #[arg(long)]
    pub case_sensitive: bool,
    /// Context length in characters before and after match (default: 30)
    #[arg(long)]
    pub context_len: Option<usize>,
}

#[derive(Args, Debug)]
pub struct PdfToGifArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long, help = "Frames per second (e.g. 1, 2)")]
    pub fps: Option<u8>,
    #[arg(long, help = "Delay between frames in centiseconds (1/100s, e.g. 100 for 1s)")]
    pub delay: Option<u16>,
    #[arg(long, help = "Maximum pages to render into the animation")]
    pub max_pages: Option<usize>,
    #[arg(long, help = "Target width of GIF frames in pixels (default: 800)")]
    pub width: Option<u32>,
}

#[derive(Args, Debug)]
pub struct ShowcaseGifArgs {
    #[arg(long)]
    pub input: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long, help = "Frames per second (e.g. 4, 10)")]
    pub fps: Option<u8>,
    #[arg(long, help = "Delay between frames in centiseconds (1/100s, e.g. 25 for 0.25s)")]
    pub delay: Option<u16>,
    #[arg(long, help = "Target width of GIF frames in pixels (default: 1200)")]
    pub width: Option<u32>,
    #[arg(long, help = "Number of animation frames to render (default: 10)")]
    pub transition_frames: Option<u8>,
}

#[derive(Args, Debug)]
pub struct ImagesToGifArgs {
    #[arg(
        long,
        value_delimiter = ',',
        num_args = 1..,
        help = "Input image file paths (supports multiple values or comma-separated)"
    )]
    pub input: Vec<PathBuf>,
    #[arg(
        long = "inputs",
        alias = "images",
        value_delimiter = ',',
        num_args = 1..,
        help = "Alternative alias for input images"
    )]
    pub inputs: Option<Vec<PathBuf>>,
    #[arg(long, help = "Output GIF file path")]
    pub output: PathBuf,
    #[arg(long, help = "Frames per second")]
    pub fps: Option<u8>,
    #[arg(long, help = "Delay between frames in centiseconds (1/100s)")]
    pub delay: Option<u16>,
    #[arg(long, help = "Target frame width (pixels)")]
    pub width: Option<u32>,
}

#[derive(Args, Debug)]
pub struct FillBatchArgs {
    /// Local AcroForm PDF template (text fields and checkboxes)
    #[arg(long)]
    pub template: PathBuf,
    /// UTF-8 CSV with headers or a JSON array of record objects
    #[arg(long)]
    pub data: PathBuf,
    #[arg(long)]
    pub output_dir: PathBuf,
    /// Output filename tokens: {row.column}, {date}, {index}, {index:04}
    #[arg(long, default_value = "{index:04}.pdf")]
    pub name_template: String,
    /// Optional JSON object mapping PDF field names to input column names
    #[arg(long)]
    pub field_map: Option<PathBuf>,
    /// Retain interactive fields instead of flattening their appearances
    #[arg(long)]
    pub keep_editable: bool,
    /// Override the UTC batch date (YYYY-MM-DD), useful for repeatable runs
    #[arg(long)]
    pub date: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum WorkflowSubcommand {
    /// Query durable workflow, watcher or batch receipts from a local directory
    History {
        #[arg(long)]
        directory: PathBuf,
        #[arg(long)]
        id: Option<String>,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// List all available workflow presets
    List,
    /// Inspect stages and conditions of a preset or TOML/JSON recipe
    Inspect {
        #[arg(long)]
        preset: Option<String>,
        #[arg(long)]
        recipe: Option<PathBuf>,
    },
    /// Execute workflow preset or TOML/JSON recipe against input PDF
    Run {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        preset: Option<String>,
        #[arg(long)]
        recipe: Option<PathBuf>,
        /// Output naming template; --output is then a directory
        #[arg(long)]
        name_template: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum FormCommands {
    #[command(name = "read-form", alias = "read", alias = "pdf_read_form", alias = "pdf-read-form")]
    Read {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    #[command(name = "fill-form", alias = "fill", alias = "pdf_fill_form", alias = "pdf-fill-form")]
    Fill {
        input: PathBuf,
        #[arg(long)]
        data: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    #[command(name = "create-form-field", alias = "add-field", alias = "pdf_create_form_field", alias = "pdf-create-form-field")]
    AddField {
        input: PathBuf,
        #[arg(long)]
        name: String,
        #[arg(long)]
        label: Option<String>,
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
            Commands::RemoveBlank { input, sensitivity, output } => {
                assert_eq!(input, PathBuf::from("in.pdf"));
                assert_eq!(sensitivity, 95);
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
                        ..
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

    #[test]
    fn test_global_mmap_and_info_parsing() {
        let args = vec![
            "paperpilot",
            "--mmap",
            "info",
            "--input",
            "large.pdf",
            "--output",
            "info.json",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.mmap);
        assert_eq!(cli.command.operation_name(), "Info");
        match cli.command {
            Commands::Info { input, output } => {
                assert_eq!(input, PathBuf::from("large.pdf"));
                assert_eq!(output, Some(PathBuf::from("info.json")));
            }
            _ => panic!("Expected Info command"),
        }

        let sub_args = vec![
            "paperpilot",
            "extract-text",
            "--input",
            "doc.pdf",
            "--output",
            "out.txt",
            "--mmap",
        ];
        let cli2 = Cli::try_parse_from(sub_args).unwrap();
        assert!(cli2.mmap);
        assert_eq!(cli2.command.operation_name(), "ExtractText");
    }
}
