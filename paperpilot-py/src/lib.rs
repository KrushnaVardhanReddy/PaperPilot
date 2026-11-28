use std::io::Write;
use std::path::PathBuf;

use paperpilot_core::error::PdfError;
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::{
    compress::CompressOperation,
    encrypt::EncryptOperation,
    extract_text::run_extract_text_bytes,
    info::InfoOperation,
    merge::MergeOperation,
    pdf_chunk::{ChunkOptions, chunk_pdf_bytes},
    redact::RedactOperation,
    repair::RepairOperation,
    rotate::RotatePagesOperation,
    split::SplitOperation,
    watermark::WatermarkOperation,
};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};

fn map_pdf_err(err: PdfError) -> PyErr {
    match err {
        PdfError::InvalidInput(msg) | PdfError::ParseError(msg) => PyValueError::new_err(msg),
        other => PyRuntimeError::new_err(other.to_string()),
    }
}

fn parse_page_ranges(ranges: &str) -> PyResult<Vec<u32>> {
    let mut points = Vec::new();
    for part in ranges.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let val: u32 = part
            .parse()
            .map_err(|_| PyValueError::new_err(format!("Invalid split page number: {part}")))?;
        if val == 0 {
            return Err(PyValueError::new_err("Page numbers must be >= 1"));
        }
        points.push(val);
    }
    if points.is_empty() {
        return Err(PyValueError::new_err(
            "ranges must contain at least one 1-based page number",
        ));
    }
    Ok(points)
}

/// Merge multiple PDF byte buffers into a single PDF.
#[pyfunction]
fn merge<'py>(py: Python<'py>, pdfs: Vec<Vec<u8>>) -> PyResult<Bound<'py, PyBytes>> {
    if pdfs.is_empty() {
        return Err(PyValueError::new_err("pdfs list must not be empty"));
    }

    let out_bytes = py.allow_threads(|| -> Result<Vec<u8>, PdfError> {
        let mut temp_files = Vec::with_capacity(pdfs.len());
        let mut paths = Vec::with_capacity(pdfs.len());

        // Write all inputs to temp files (skipping the first which is loaded as base doc)
        let mut base_doc = LopdfDocument::from_bytes(&pdfs[0])?;
        for pdf_bytes in pdfs.iter().skip(1) {
            let mut tf = tempfile::NamedTempFile::new().map_err(PdfError::IoError)?;
            tf.write_all(pdf_bytes).map_err(PdfError::IoError)?;
            paths.push(tf.path().to_path_buf());
            temp_files.push(tf);
        }

        if !paths.is_empty() {
            let op = MergeOperation::new(paths);
            op.execute(&mut base_doc)?;
        }

        let mut out = Vec::new();
        base_doc
            .inner
            .save_to(&mut out)
            .map_err(|e| PdfError::Other(format!("Failed to serialize merged PDF: {e}")))?;
        Ok(out)
    });

    let bytes = out_bytes.map_err(map_pdf_err)?;
    Ok(PyBytes::new(py, &bytes))
}

/// Split a PDF at 1-based page numbers (e.g. "2,5") and return a list of PDF byte buffers.
#[pyfunction]
fn split<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    ranges: &str,
) -> PyResult<Vec<Bound<'py, PyBytes>>> {
    let split_points = parse_page_ranges(ranges)?;

    let parts = py.allow_threads(|| -> Result<Vec<Vec<u8>>, PdfError> {
        let mut doc = LopdfDocument::from_bytes(&pdf)?;
        let temp_dir = tempfile::tempdir().map_err(PdfError::IoError)?;
        let op = SplitOperation::new(temp_dir.path().to_path_buf(), split_points);
        op.execute(&mut doc)?;

        let mut entries: Vec<PathBuf> = std::fs::read_dir(temp_dir.path())
            .map_err(PdfError::IoError)?
            .filter_map(|e| e.ok().map(|entry| entry.path()))
            .filter(|p| p.extension().is_some_and(|ext| ext == "pdf"))
            .collect();
        entries.sort();

        let mut result = Vec::with_capacity(entries.len());
        for path in entries {
            result.push(std::fs::read(path).map_err(PdfError::IoError)?);
        }
        Ok(result)
    });

    let part_list = parts.map_err(map_pdf_err)?;
    Ok(part_list
        .into_iter()
        .map(|b| PyBytes::new(py, &b))
        .collect())
}

/// Compress a PDF using the specified quality preset ("low", "medium", "high", or "1"-"100").
#[pyfunction]
#[pyo3(signature = (pdf, level="medium"))]
fn compress<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    level: &str,
) -> PyResult<Bound<'py, PyBytes>> {
    let level_owned = level.to_string();
    let out_bytes = py.allow_threads(|| -> Result<Vec<u8>, PdfError> {
        let mut doc = LopdfDocument::from_bytes(&pdf)?;
        let op = CompressOperation::new(Some(level_owned));
        op.execute(&mut doc)?;

        let mut out = Vec::new();
        doc.inner
            .save_to(&mut out)
            .map_err(|e| PdfError::Other(format!("Failed to serialize compressed PDF: {e}")))?;
        Ok(out)
    });

    let bytes = out_bytes.map_err(map_pdf_err)?;
    Ok(PyBytes::new(py, &bytes))
}

/// Convert a PDF to layout-aware Markdown.
#[pyfunction]
fn to_markdown(py: Python<'_>, pdf: Vec<u8>) -> PyResult<String> {
    let pages = py
        .allow_threads(|| run_extract_text_bytes(&pdf, None))
        .map_err(map_pdf_err)?;
    Ok(pages.join("\n\n---\n\n"))
}

/// Extract text from a PDF, optionally restricted to specific 1-based page numbers.
#[pyfunction]
#[pyo3(signature = (pdf, pages=None))]
fn extract_text(py: Python<'_>, pdf: Vec<u8>, pages: Option<Vec<u32>>) -> PyResult<String> {
    let extracted = py
        .allow_threads(|| run_extract_text_bytes(&pdf, pages.as_deref()))
        .map_err(map_pdf_err)?;
    Ok(extracted.join("\n\n"))
}

/// Chunk a PDF into RAG-ready semantic chunks with token estimates and page metadata.
#[pyfunction]
#[pyo3(signature = (pdf, max_tokens=512, overlap=50))]
fn chunk_rag<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    max_tokens: usize,
    overlap: usize,
) -> PyResult<Bound<'py, PyList>> {
    let chunk_result = py
        .allow_threads(|| {
            let mut opts = ChunkOptions::default();
            opts.target_tokens = max_tokens;
            opts.overlap_tokens = overlap;
            chunk_pdf_bytes(&pdf, opts)
        })
        .map_err(map_pdf_err)?;

    let list = PyList::empty(py);
    for chunk in chunk_result.chunks {
        let dict = PyDict::new(py);
        dict.set_item("index", chunk.chunk_index)?;
        dict.set_item("text", chunk.text)?;
        dict.set_item("token_count", chunk.estimated_tokens)?;
        dict.set_item("page_start", chunk.page_start)?;
        dict.set_item("page_end", chunk.page_end)?;
        dict.set_item("headings", chunk.headings_context)?;
        list.append(dict)?;
    }
    Ok(list)
}

/// Redact matching text patterns from a PDF.
#[pyfunction]
#[pyo3(signature = (pdf, terms, overlay_text=None))]
fn redact<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    terms: Vec<String>,
    overlay_text: Option<String>,
) -> PyResult<Bound<'py, PyBytes>> {
    if terms.is_empty() {
        return Err(PyValueError::new_err("terms list must not be empty"));
    }

    let out_bytes = py.allow_threads(|| -> Result<Vec<u8>, PdfError> {
        let mut doc = LopdfDocument::from_bytes(&pdf)?;
        let op = RedactOperation::from_patterns(terms, overlay_text);
        op.execute(&mut doc)?;

        let mut out = Vec::new();
        doc.inner
            .save_to(&mut out)
            .map_err(|e| PdfError::Other(format!("Failed to serialize redacted PDF: {e}")))?;
        Ok(out)
    });

    let bytes = out_bytes.map_err(map_pdf_err)?;
    Ok(PyBytes::new(py, &bytes))
}

/// Encrypt a PDF with user and optional owner passwords.
#[pyfunction]
#[pyo3(signature = (pdf, user_password, owner_password=None))]
fn encrypt<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    user_password: String,
    owner_password: Option<String>,
) -> PyResult<Bound<'py, PyBytes>> {
    let out_bytes = py.allow_threads(|| -> Result<Vec<u8>, PdfError> {
        let mut doc = LopdfDocument::from_bytes(&pdf)?;
        let op = EncryptOperation {
            user_password: Some(user_password),
            owner_password,
        };
        op.execute(&mut doc)?;

        let mut out = Vec::new();
        doc.inner
            .save_to(&mut out)
            .map_err(|e| PdfError::Other(format!("Failed to serialize encrypted PDF: {e}")))?;
        Ok(out)
    });

    let bytes = out_bytes.map_err(map_pdf_err)?;
    Ok(PyBytes::new(py, &bytes))
}

/// Return basic metadata (page_count, version) for a PDF.
#[pyfunction]
fn info<'py>(py: Python<'py>, pdf: Vec<u8>) -> PyResult<Bound<'py, PyDict>> {
    let res = py
        .allow_threads(|| -> Result<paperpilot_pdf::operations::info::PdfInfoResult, PdfError> {
            let mut doc = LopdfDocument::from_bytes(&pdf)?;
            let op = InfoOperation::new();
            op.execute(&mut doc)?;
            let guard = op
                .result
                .lock()
                .map_err(|_| PdfError::Other("Info mutex poisoned".into()))?;
            guard
                .clone()
                .ok_or_else(|| PdfError::Other("InfoOperation produced no result".into()))
        })
        .map_err(map_pdf_err)?;

    let dict = PyDict::new(py);
    dict.set_item("page_count", res.page_count)?;
    dict.set_item("version", res.version)?;
    Ok(dict)
}

/// Attempt to repair a malformed or truncated PDF.
#[pyfunction]
fn repair<'py>(py: Python<'py>, pdf: Vec<u8>) -> PyResult<Bound<'py, PyBytes>> {
    let bytes = py
        .allow_threads(|| RepairOperation::repair_bytes(&pdf))
        .map_err(map_pdf_err)?;
    Ok(PyBytes::new(py, &bytes))
}

/// Rotate pages in a PDF by a multiple of 90 degrees.
#[pyfunction]
#[pyo3(signature = (pdf, degrees, pages=None))]
fn rotate<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    degrees: u16,
    pages: Option<Vec<u32>>,
) -> PyResult<Bound<'py, PyBytes>> {
    let out_bytes = py.allow_threads(|| -> Result<Vec<u8>, PdfError> {
        let mut doc = LopdfDocument::from_bytes(&pdf)?;
        let op = RotatePagesOperation::new(degrees, pages);
        op.execute(&mut doc)?;

        let mut out = Vec::new();
        doc.inner
            .save_to(&mut out)
            .map_err(|e| PdfError::Other(format!("Failed to serialize rotated PDF: {e}")))?;
        Ok(out)
    });

    let bytes = out_bytes.map_err(map_pdf_err)?;
    Ok(PyBytes::new(py, &bytes))
}

/// Add a diagonal text watermark to a PDF.
#[pyfunction]
#[pyo3(signature = (pdf, text, angle=45.0, opacity=0.2, font_size=None, color=None, pages=None))]
fn watermark<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    text: String,
    angle: f32,
    opacity: f32,
    font_size: Option<f32>,
    color: Option<String>,
    pages: Option<Vec<u32>>,
) -> PyResult<Bound<'py, PyBytes>> {
    let out_bytes = py.allow_threads(|| -> Result<Vec<u8>, PdfError> {
        let mut doc = LopdfDocument::from_bytes(&pdf)?;
        let op = WatermarkOperation {
            text,
            angle,
            opacity,
            font_size,
            color,
            pages,
        };
        op.execute(&mut doc)?;

        let mut out = Vec::new();
        doc.inner
            .save_to(&mut out)
            .map_err(|e| PdfError::Other(format!("Failed to serialize watermarked PDF: {e}")))?;
        Ok(out)
    });

    let bytes = out_bytes.map_err(map_pdf_err)?;
    Ok(PyBytes::new(py, &bytes))
}

/// In-memory bytes alias for `merge` (for RAG/FastAPI pipelines).
#[pyfunction]
fn merge_bytes<'py>(py: Python<'py>, pdfs: Vec<Vec<u8>>) -> PyResult<Bound<'py, PyBytes>> {
    merge(py, pdfs)
}

/// In-memory bytes alias for `compress` (for RAG/FastAPI pipelines).
#[pyfunction]
#[pyo3(signature = (pdf, level="medium"))]
fn compress_bytes<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    level: &str,
) -> PyResult<Bound<'py, PyBytes>> {
    compress(py, pdf, level)
}

/// In-memory bytes alias for `to_markdown` (for RAG/FastAPI pipelines).
#[pyfunction]
fn to_markdown_bytes(py: Python<'_>, pdf: Vec<u8>) -> PyResult<String> {
    to_markdown(py, pdf)
}

/// In-memory bytes alias for `extract_text` (for RAG/FastAPI pipelines).
#[pyfunction]
#[pyo3(signature = (pdf, pages=None))]
fn extract_text_bytes(py: Python<'_>, pdf: Vec<u8>, pages: Option<Vec<u32>>) -> PyResult<String> {
    extract_text(py, pdf, pages)
}

/// In-memory bytes alias for `chunk_rag` (for RAG/FastAPI pipelines).
#[pyfunction]
#[pyo3(signature = (pdf, max_tokens=512, overlap=50))]
fn chunk_rag_bytes<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    max_tokens: usize,
    overlap: usize,
) -> PyResult<Bound<'py, PyList>> {
    chunk_rag(py, pdf, max_tokens, overlap)
}

/// In-memory bytes alias for `info` (for RAG/FastAPI pipelines).
#[pyfunction]
fn info_bytes<'py>(py: Python<'py>, pdf: Vec<u8>) -> PyResult<Bound<'py, PyDict>> {
    info(py, pdf)
}

/// In-memory bytes alias for `rotate` (for RAG/FastAPI pipelines).
#[pyfunction]
#[pyo3(signature = (pdf, degrees, pages=None))]
fn rotate_bytes<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    degrees: u16,
    pages: Option<Vec<u32>>,
) -> PyResult<Bound<'py, PyBytes>> {
    rotate(py, pdf, degrees, pages)
}

/// In-memory bytes alias for `watermark` (for RAG/FastAPI pipelines).
#[pyfunction]
#[pyo3(signature = (pdf, text, angle=45.0, opacity=0.2, font_size=None, color=None, pages=None))]
fn watermark_bytes<'py>(
    py: Python<'py>,
    pdf: Vec<u8>,
    text: String,
    angle: f32,
    opacity: f32,
    font_size: Option<f32>,
    color: Option<String>,
    pages: Option<Vec<u32>>,
) -> PyResult<Bound<'py, PyBytes>> {
    watermark(py, pdf, text, angle, opacity, font_size, color, pages)
}

/// Native Python bindings for PaperPilot.
#[pymodule]
fn paperpilot(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(merge, m)?)?;
    m.add_function(wrap_pyfunction!(split, m)?)?;
    m.add_function(wrap_pyfunction!(compress, m)?)?;
    m.add_function(wrap_pyfunction!(to_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(extract_text, m)?)?;
    m.add_function(wrap_pyfunction!(chunk_rag, m)?)?;
    m.add_function(wrap_pyfunction!(redact, m)?)?;
    m.add_function(wrap_pyfunction!(encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(repair, m)?)?;
    m.add_function(wrap_pyfunction!(rotate, m)?)?;
    m.add_function(wrap_pyfunction!(watermark, m)?)?;
    m.add_function(wrap_pyfunction!(merge_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(compress_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(to_markdown_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(extract_text_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(chunk_rag_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(info_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(rotate_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(watermark_bytes, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_minimal_pdf(text: &str) -> Vec<u8> {
        let stream = format!("BT /F1 12 Tf 72 720 Td ({text}) Tj ET");
        let objs = vec![
            "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n".to_string(),
            "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n".to_string(),
            "3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n".to_string(),
            format!(
                "4 0 obj\n<< /Length {} >>\nstream\n{}\nendstream\nendobj\n",
                stream.len(),
                stream
            ),
            "5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n".to_string(),
        ];

        let mut out = b"%PDF-1.4\n".to_vec();
        let mut offsets = Vec::with_capacity(objs.len());
        for obj in &objs {
            offsets.push(out.len());
            out.extend_from_slice(obj.as_bytes());
        }
        let xref_pos = out.len();
        out.extend_from_slice(
            format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes(),
        );
        for off in offsets {
            out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
        }
        out.extend_from_slice(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_pos}\n%%EOF\n",
                objs.len() + 1
            )
            .as_bytes(),
        );
        out
    }

    #[test]
    fn test_info_operation() {
        let pdf = make_minimal_pdf("Info Test");
        let mut doc = LopdfDocument::from_bytes(&pdf).unwrap();
        let op = InfoOperation::new();
        op.execute(&mut doc).unwrap();
        let res = op.result.lock().unwrap().clone().unwrap();
        assert_eq!(res.page_count, 1);
        assert!(!res.version.is_empty());
    }

    #[test]
    fn test_extract_text_and_markdown() {
        let pdf = make_minimal_pdf("PaperPilot Rust Test");
        let pages = run_extract_text_bytes(&pdf, None).unwrap();
        assert_eq!(pages.len(), 1);
        assert!(pages[0].contains("PaperPilot"));

        let filtered = run_extract_text_bytes(&pdf, Some(&[1])).unwrap();
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].contains("PaperPilot"));
    }

    #[test]
    fn test_chunk_rag_bytes() {
        let pdf = make_minimal_pdf("Chunking test sentence for PaperPilot RAG.");
        let mut opts = ChunkOptions::default();
        opts.target_tokens = 128;
        opts.overlap_tokens = 16;
        let result = chunk_pdf_bytes(&pdf, opts).unwrap();
        assert!(!result.chunks.is_empty());
        assert!(result.chunks[0].text.contains("PaperPilot"));
    }

    #[test]
    fn test_merge_and_split_operations() {
        let p1 = make_minimal_pdf("Page One");
        let p2 = make_minimal_pdf("Page Two");

        let mut base_doc = LopdfDocument::from_bytes(&p1).unwrap();
        let mut tf = tempfile::NamedTempFile::new().unwrap();
        tf.write_all(&p2).unwrap();
        let merge_op = MergeOperation::new(vec![tf.path().to_path_buf()]);
        merge_op.execute(&mut base_doc).unwrap();

        let mut merged_bytes = Vec::new();
        base_doc.inner.save_to(&mut merged_bytes).unwrap();

        let mut merged_doc = LopdfDocument::from_bytes(&merged_bytes).unwrap();
        let info_op = InfoOperation::new();
        info_op.execute(&mut merged_doc).unwrap();
        assert_eq!(info_op.result.lock().unwrap().as_ref().unwrap().page_count, 2);

        let temp_dir = tempfile::tempdir().unwrap();
        let split_op = SplitOperation::new(temp_dir.path().to_path_buf(), vec![1]);
        split_op.execute(&mut merged_doc).unwrap();

        let mut parts: Vec<PathBuf> = std::fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok().map(|entry| entry.path()))
            .filter(|p| p.extension().is_some_and(|ext| ext == "pdf"))
            .collect();
        parts.sort();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_compress_redact_encrypt_repair() {
        let pdf = make_minimal_pdf("SecretToken123");

        let mut doc = LopdfDocument::from_bytes(&pdf).unwrap();
        CompressOperation::new(Some("medium".to_string()))
            .execute(&mut doc)
            .unwrap();
        let mut out = Vec::new();
        doc.inner.save_to(&mut out).unwrap();
        assert!(out.starts_with(b"%PDF-"));

        let mut doc = LopdfDocument::from_bytes(&pdf).unwrap();
        RedactOperation::from_patterns(vec!["SecretToken123".to_string()], None)
            .execute(&mut doc)
            .unwrap();
        let mut out = Vec::new();
        doc.inner.save_to(&mut out).unwrap();
        assert!(out.starts_with(b"%PDF-"));

        let mut doc = LopdfDocument::from_bytes(&pdf).unwrap();
        EncryptOperation {
            user_password: Some("pass".to_string()),
            owner_password: None,
        }
        .execute(&mut doc)
        .unwrap();
        let mut out = Vec::new();
        doc.inner.save_to(&mut out).unwrap();
        assert!(out.starts_with(b"%PDF-"));

        let repaired = RepairOperation::repair_bytes(&pdf).unwrap();
        assert!(repaired.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_rotate_and_watermark_operations() {
        let pdf = make_minimal_pdf("Rotate and Watermark");

        let mut doc = LopdfDocument::from_bytes(&pdf).unwrap();
        RotatePagesOperation::new(90, None)
            .execute(&mut doc)
            .unwrap();
        let mut out = Vec::new();
        doc.inner.save_to(&mut out).unwrap();
        assert!(out.starts_with(b"%PDF-"));

        let mut doc = LopdfDocument::from_bytes(&pdf).unwrap();
        WatermarkOperation {
            text: "CONFIDENTIAL".to_string(),
            angle: 45.0,
            opacity: 0.25,
            font_size: None,
            color: None,
            pages: None,
        }
        .execute(&mut doc)
        .unwrap();
        let mut out = Vec::new();
        doc.inner.save_to(&mut out).unwrap();
        assert!(out.starts_with(b"%PDF-"));
    }
}
