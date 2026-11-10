use paperpilot_core::document_source::DocumentSource;
use paperpilot_core::error::PdfError;
use paperpilot_pdf::operations::pdf_chunk::{ChunkOptions, PdfChunkOperation};
use std::path::{Path, PathBuf};

pub fn handle_pdf_chunk_command(
    input: &Path,
    target_tokens: usize,
    overlap: usize,
    strip_boilerplate: bool,
    break_on_headings: bool,
    output: Option<&PathBuf>,
) -> Result<(), PdfError> {
    let doc_source = DocumentSource::open(input, None).map_err(PdfError::IoError)?;

    let options = ChunkOptions {
        target_tokens,
        overlap_tokens: overlap,
        strip_boilerplate,
        break_on_headings,
    };

    let result = PdfChunkOperation::chunk_source(&doc_source, options)?;
    let json_bytes =
        serde_json::to_vec_pretty(&result).map_err(|e| PdfError::Other(e.to_string()))?;

    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(PdfError::IoError)?;
            }
        }
        std::fs::write(out_path, &json_bytes).map_err(PdfError::IoError)?;
        println!("PDF chunk result written to {:?}", out_path);
    } else {
        let json_str = String::from_utf8_lossy(&json_bytes);
        println!("{}", json_str);
    }

    Ok(())
}
