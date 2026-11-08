use paperpilot_core::error::PdfError;
use paperpilot_pdf::operations::pdf_extract_patterns::{
    PdfExtractPatternsOptions, pdf_extract_patterns_bytes,
};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn handle_pdf_extract_patterns_command(
    input: &PathBuf,
    output: &Option<PathBuf>,
    keywords: &Option<Vec<String>>,
    patterns_arg: &Option<Vec<String>>,
    case_sensitive: bool,
    context_len: Option<usize>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut patterns = HashMap::new();
    if let Some(list) = patterns_arg {
        for entry in list {
            if let Some((k, v)) = entry.split_once('=') {
                patterns.insert(k.trim().to_string(), v.trim().to_string());
            } else {
                patterns.insert(entry.trim().to_string(), entry.trim().to_string());
            }
        }
    }

    let options = PdfExtractPatternsOptions {
        keywords: keywords.clone().unwrap_or_default(),
        patterns,
        case_sensitive,
        context_len: context_len.unwrap_or(30),
    };

    let output_bytes = pdf_extract_patterns_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(PdfError::IoError)?;
            }
        }
        std::fs::write(out_path, &output_bytes).map_err(PdfError::IoError)?;
        println!("PdfExtractPatterns report written to {:?}", out_path);
    } else {
        let json_str = String::from_utf8_lossy(&output_bytes);
        println!("{}", json_str);
    }

    Ok(())
}
