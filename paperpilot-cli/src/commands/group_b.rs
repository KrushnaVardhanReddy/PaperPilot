use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::{
    bates::BatesNumberingOperation, compress::CompressOperation, decrypt::DecryptOperation,
    encrypt::EncryptOperation, flatten::FlattenOperation, header_footer::HeaderFooterOperation,
    linearize::LinearizeOperation, metadata::MetadataOperation, pdf_a::PdfAConversionOperation,
    redact::RedactOperation, repair::RepairOperation, signature::SignatureOperation,
    watermark::WatermarkOperation,
};
use std::path::PathBuf;

pub fn handle_compress(input: PathBuf, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = CompressOperation;
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_repair(input: PathBuf, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = RepairOperation;
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_linearize(input: PathBuf, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = LinearizeOperation;
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_encrypt(
    input: PathBuf,
    user_password: Option<String>,
    owner_password: Option<String>,
    output: PathBuf,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = EncryptOperation {
        user_password,
        owner_password,
    };
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_decrypt(
    input: PathBuf,
    _password: Option<String>,
    output: PathBuf,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = DecryptOperation {};
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_watermark(input: PathBuf, text: String, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = WatermarkOperation::new(text);
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_header_footer(input: PathBuf, text: String, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = HeaderFooterOperation::new(text, "header".to_string());
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_bates(
    input: PathBuf,
    prefix: String,
    start: u32,
    output: PathBuf,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = BatesNumberingOperation::new(start, prefix, 4);
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_metadata(
    input: PathBuf,
    title: Option<String>,
    author: Option<String>,
    subject: Option<String>,
    keywords: Option<String>,
    output: PathBuf,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = MetadataOperation {
        title,
        author,
        subject,
        keywords,
    };
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_signature(input: PathBuf, _cert: PathBuf, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = SignatureOperation {};
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_redact(
    input: PathBuf,
    pages: Vec<u32>,
    rect: String,
    output: PathBuf,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let rect_parts: Vec<&str> = rect.split(',').collect();
    let bounding_box = paperpilot_pdf::operations::redact::BoundingBox {
        x: rect_parts
            .first()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0),
        y: rect_parts
            .get(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0),
        width: rect_parts
            .get(2)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0),
        height: rect_parts
            .get(3)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0),
    };
    let page = if !pages.is_empty() { pages[0] } else { 1 };
    let op = RedactOperation::new(page, bounding_box);
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_flatten(input: PathBuf, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = FlattenOperation;
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_pdf_a(input: PathBuf, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = PdfAConversionOperation::new();
    op.execute(&mut doc)?;
    doc.save(&output)
}
