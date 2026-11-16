use super::apply_op;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::workflow::WorkflowStep;
use serde_json::Value;

fn invalid() -> PdfError {
    PdfError::InvalidInput(
        "Invalid workflow operation parameters (see document_workflow reference)".into(),
    )
}

fn string<'a>(s: &'a WorkflowStep, key: &str, default: &'a str) -> OperationResult<&'a str> {
    match s.params.get(key) {
        None => Ok(default),
        Some(v) => v.as_str().ok_or_else(invalid),
    }
}

fn integer(s: &WorkflowStep, key: &str, default: u64, max: u64) -> OperationResult<u64> {
    let value = match s.params.get(key) {
        None => default,
        Some(v) => v.as_u64().ok_or_else(invalid)?,
    };
    if value > max {
        return Err(invalid());
    }
    Ok(value)
}

fn quality(s: &WorkflowStep) -> OperationResult<Option<String>> {
    let Some(value) = s.params.get("quality").or_else(|| s.params.get("level")) else {
        return Ok(None);
    };
    let value = if let Some(text) = value.as_str() {
        text.to_string()
    } else if let Some(n) = value.as_u64() {
        n.to_string()
    } else {
        return Err(invalid());
    };
    let value = if value == "aggressive" {
        "low".to_string()
    } else {
        value
    };
    if !matches!(
        value.as_str(),
        "low" | "screen" | "medium" | "ebook" | "high" | "prepress"
    ) && !value.parse::<u8>().is_ok_and(|v| (1..=100).contains(&v))
    {
        return Err(invalid());
    }
    Ok(Some(value))
}

pub(super) fn validate_step(s: &WorkflowStep) -> OperationResult<()> {
    let keys: &[&str] = match s.operation_id.as_str() {
        "pdf_watermark" | "watermark" => &["text", "opacity", "angle"],
        "pdf_compress" | "compress" => &["quality", "level"],
        "pdf_rotate" | "rotate" => &["angle"],
        "pdf_bates" | "bates" => &["prefix", "start", "padding"],
        "pdf_page_numbers" | "page_numbers" => &["position", "format"],
        "pdf_remove_blank" | "remove_blank" => &["sensitivity"],
        "pdf_encrypt" | "encrypt" => &["password", "user_password", "owner_password"],
        "pdf_ocr" | "ocr" => &[],
        "pdf_sanitize" | "sanitize" => &[
            "remove_metadata",
            "remove_attachments",
            "remove_scripts",
            "remove_thumbnails",
            "remove_private_data",
        ],
        "pdf_flatten" | "flatten" | "pdf_linearize" | "linearize" => &[],
        "filter_page_count" if s.condition.is_some() => &[],
        _ => {
            return Err(PdfError::UnsupportedOperation(format!(
                "Workflow operation '{}' is not supported; no output was published",
                s.operation_id
            )));
        }
    };
    if s.params.keys().any(|key| !keys.contains(&key.as_str())) {
        return Err(invalid());
    }
    // Validate parameters without loading documents or initializing OCR models.
    match s.operation_id.as_str() {
        "pdf_watermark" | "watermark" => {
            string(s, "text", "CONFIDENTIAL")?;
            for key in ["opacity", "angle"] {
                if let Some(v) = s.params.get(key) {
                    let n = v.as_f64().ok_or_else(invalid)?;
                    if !n.is_finite()
                        || (key == "opacity" && !(0.0..=1.0).contains(&n))
                        || n.abs() > 360.0
                    {
                        return Err(invalid());
                    }
                }
            }
        }
        "pdf_compress" | "compress" => {
            if s.params.contains_key("quality") && s.params.contains_key("level") {
                return Err(invalid());
            }
            quality(s)?;
        }
        "pdf_rotate" | "rotate" => {
            if integer(s, "angle", 90, 270)? % 90 != 0 {
                return Err(invalid());
            }
        }
        "pdf_bates" | "bates" => {
            string(s, "prefix", "CONFIDENTIAL-")?;
            integer(s, "start", 1, u32::MAX as u64)?;
            integer(s, "padding", 6, 32)?;
        }
        "pdf_page_numbers" | "page_numbers" => {
            let p = string(s, "position", "bottom-right")?;
            if ![
                "top-left",
                "top-center",
                "top-right",
                "bottom-left",
                "bottom-center",
                "bottom-right",
            ]
            .contains(&p)
            {
                return Err(invalid());
            }
            string(s, "format", "Page {n} of {total}")?;
        }
        "pdf_remove_blank" | "remove_blank" => {
            integer(s, "sensitivity", 90, 100)?;
        }
        "pdf_encrypt" | "encrypt" => {
            if s.params.contains_key("password") && s.params.contains_key("user_password") {
                return Err(invalid());
            }
            let password = string(s, "user_password", string(s, "password", "")?)?;
            let owner = string(s, "owner_password", password)?;
            if password.is_empty() || [password, owner].iter().any(|p| p.contains("${")) {
                return Err(invalid());
            }
        }
        "pdf_sanitize" | "sanitize" => {
            if s.params.values().any(|v| !v.is_boolean()) {
                return Err(invalid());
            }
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn execute_step(bytes: &[u8], s: &WorkflowStep) -> OperationResult<Vec<u8>> {
    use crate::operations::*;
    match s.operation_id.as_str() {
        "pdf_watermark" | "watermark" => {
            let mut op =
                watermark::WatermarkOperation::new(string(s, "text", "CONFIDENTIAL")?.into());
            op.opacity = s
                .params
                .get("opacity")
                .and_then(Value::as_f64)
                .unwrap_or(0.2) as f32;
            op.angle = s
                .params
                .get("angle")
                .and_then(Value::as_f64)
                .unwrap_or(45.0) as f32;
            apply_op(bytes, op)
        }
        "pdf_compress" | "compress" => {
            apply_op(bytes, compress::CompressOperation::new(quality(s)?))
        }
        "pdf_flatten" | "flatten" => apply_op(bytes, flatten::FlattenOperation::new()),
        "pdf_linearize" | "linearize" => apply_op(bytes, linearize::LinearizeOperation::new()),
        "pdf_rotate" | "rotate" => apply_op(
            bytes,
            rotate::RotatePagesOperation::new(integer(s, "angle", 90, 270)? as u16, None),
        ),
        "pdf_page_numbers" | "page_numbers" => apply_op(
            bytes,
            page_numbers::PageNumbersOperation::new(
                string(s, "position", "bottom-right")?.into(),
                string(s, "format", "Page {n} of {total}")?.into(),
            ),
        ),
        "pdf_bates" | "bates" => apply_op(
            bytes,
            bates::BatesNumberingOperation::new(
                integer(s, "start", 1, u32::MAX as u64)? as u32,
                string(s, "prefix", "CONFIDENTIAL-")?.into(),
                integer(s, "padding", 6, 32)? as usize,
            ),
        ),
        "pdf_remove_blank" | "remove_blank" => apply_op(
            bytes,
            cleanup::RemoveBlankPagesOperation::new(integer(s, "sensitivity", 90, 100)? as u8),
        ),
        "pdf_encrypt" | "encrypt" => apply_op(
            bytes,
            encrypt::EncryptOperation {
                user_password: Some(string(s, "user_password", string(s, "password", "")?)?.into()),
                owner_password: s
                    .params
                    .get("owner_password")
                    .and_then(Value::as_str)
                    .map(String::from),
            },
        ),
        "pdf_sanitize" | "sanitize" => {
            let options =
                serde_json::from_value(serde_json::to_value(&s.params).map_err(|_| invalid())?)
                    .map_err(|_| invalid())?;
            sanitize::sanitize_pdf(bytes, &options).map(|(out, _)| out)
        }
        #[cfg(all(feature = "ocr", not(target_arch = "wasm32")))]
        "pdf_ocr" | "ocr" => {
            let op = ocr::OcrOperationImpl::new();
            let extracted = op.extracted_text.clone();
            let output = apply_op(bytes, op)?;
            let text = extracted
                .lock()
                .map_err(|_| PdfError::Other("Cannot inspect OCR result".into()))?;
            if !text.as_deref().is_some_and(|t| !t.trim().is_empty()) {
                return Err(PdfError::Other("OCR produced no text; check local models and whether the document contains supported raster text".into()));
            }
            Ok(output)
        }
        #[cfg(not(all(feature = "ocr", not(target_arch = "wasm32"))))]
        "pdf_ocr" | "ocr" => Err(PdfError::UnsupportedOperation(
            "OCR is unavailable in this build; use a native OCR-enabled build with local models"
                .into(),
        )),
        "filter_page_count" => Ok(bytes.to_vec()),
        _ => Err(PdfError::UnsupportedOperation(format!(
            "Unsupported workflow operation '{}'",
            s.operation_id
        ))),
    }
}
