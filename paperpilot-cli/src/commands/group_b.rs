#![cfg(feature = "pdf")]
use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;

pub fn handle_compress(
    input: &std::path::Path,
    quality: &str,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op =
        paperpilot_pdf::operations::compress::CompressOperation::new(Some(quality.to_string()));
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_repair(input: &std::path::Path, output: &std::path::Path) -> OperationResult<()> {
    paperpilot_pdf::operations::repair::RepairOperation::repair_file(input, output)
}

pub fn handle_linearize(input: &std::path::Path, output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::linearize::LinearizeOperation::new();
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_encrypt(
    input: &std::path::Path,
    user_password: &Option<String>,
    owner_password: &Option<String>,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::encrypt::EncryptOperation {
        user_password: user_password.clone(),
        owner_password: owner_password.clone(),
    };
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_decrypt(
    input: &std::path::Path,
    password: &Option<String>,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::decrypt::DecryptOperation::new(password.clone());
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_watermark(
    input: &std::path::Path,
    text: &str,
    angle: f32,
    opacity: f32,
    color: Option<String>,
    font_size: Option<f32>,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let mut op = paperpilot_pdf::operations::watermark::WatermarkOperation::new(text.to_string());
    op.angle = angle;
    op.opacity = opacity;
    op.color = color;
    op.font_size = font_size;
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_redact(
    input: &std::path::Path,
    pages: &str,
    rect: &str,
    data: Option<&str>,
    query: Option<&str>,
    regex: Option<&str>,
    overlay_text: Option<&str>,
    occurrence: Option<&str>,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;

    let target_pages: Vec<u32> = pages
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let op = if let Some(d) = data {
        let json_content = if let Ok(content) = std::fs::read_to_string(d) {
            content
        } else {
            d.to_string()
        };

        if let Ok(rules) = serde_json::from_str::<Vec<paperpilot_pdf::operations::redact::RedactRule>>(&json_content) {
            paperpilot_pdf::operations::redact::RedactOperation::from_rules(rules)
        } else if let Ok(wrapper) = serde_json::from_str::<serde_json::Value>(&json_content) {
            if let Some(rules_val) = wrapper.get("rules") {
                let rules: Vec<paperpilot_pdf::operations::redact::RedactRule> =
                    serde_json::from_value(rules_val.clone()).unwrap_or_default();
                paperpilot_pdf::operations::redact::RedactOperation::from_rules(rules)
            } else {
                paperpilot_pdf::operations::redact::RedactOperation::from_rules(vec![])
            }
        } else {
            paperpilot_pdf::operations::redact::RedactOperation::from_rules(vec![])
        }
    } else if query.is_some() || regex.is_some() {
        let rule = paperpilot_pdf::operations::redact::RedactRule {
            search: query.map(|s| s.to_string()),
            regex: regex.map(|s| s.to_string()),
            case_sensitive: false,
            pages: if target_pages.is_empty() {
                None
            } else {
                Some(target_pages)
            },
            rect: None,
            page: None,
            occurrence: occurrence.unwrap_or("all").to_string(),
            overlay_text: overlay_text.map(|s| s.to_string()),
        };
        paperpilot_pdf::operations::redact::RedactOperation::from_rules(vec![rule])
    } else {
        let parts: Vec<&str> = rect.split(',').collect();
        let x = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let width = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let height = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0);

        let page_number = target_pages.first().copied().unwrap_or(1);

        let mut redact_op = paperpilot_pdf::operations::redact::RedactOperation::new(
            page_number,
            paperpilot_pdf::operations::redact::BoundingBox {
                x,
                y,
                width,
                height,
            },
        );
        if let Some(ot) = overlay_text {
            redact_op = redact_op.with_overlay_text(ot);
        }
        redact_op
    };

    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_metadata(
    input: &std::path::Path,
    title: &Option<String>,
    author: &Option<String>,
    subject: &Option<String>,
    keywords: &Option<String>,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let mut op = paperpilot_pdf::operations::metadata::MetadataOperation::new();
    op.title = title.clone();
    op.author = author.clone();
    op.subject = subject.clone();
    op.keywords = keywords.clone();
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_signature(
    input: &std::path::Path,
    _cert: &std::path::Path,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::signature::SignatureOperation::new();
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_flatten(input: &std::path::Path, output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::flatten::FlattenOperation::new();
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_pdf_a(input: &std::path::Path, output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::pdf_a::PdfAConversionOperation::new();
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_header_footer(
    input: &std::path::Path,
    text: &str,
    position: &str,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::header_footer::HeaderFooterOperation::new(
        text.to_string(),
        position.to_string(),
    );
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_bates(
    input: &std::path::Path,
    prefix: &str,
    start: u32,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::bates::BatesNumberingOperation::new(
        start,
        prefix.to_string(),
        6,
    );
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_page_numbers(
    input: &std::path::Path,
    output: &std::path::Path,
    position: &str,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::page_numbers::PageNumbersOperation::new(
        position.to_string(),
        "Page {n} of {total}".to_string(),
    );
    op.execute(&mut doc)?;
    doc.save(output)
}
