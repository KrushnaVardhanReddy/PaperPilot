#![cfg(feature = "pdf")]
use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use std::path::PathBuf;

pub fn handle_extract_text(
    input: &std::path::Path,
    output: &std::path::Path,
    format: &str,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::extract_text::ExtractTextOperation::new(None);
    op.execute(&mut doc)?;

    let text_lock = op.extracted_text.lock().unwrap();
    let text_pages = text_lock.as_ref().unwrap();

    let result_str = if format.eq_ignore_ascii_case("json") {
        serde_json::to_string_pretty(text_pages).unwrap()
    } else {
        text_pages.join("\n")
    };

    std::fs::write(output, result_str).map_err(paperpilot_core::error::PdfError::IoError)?;

    Ok(())
}

pub fn handle_extract_images(
    input: &std::path::Path,
    output: &std::path::Path,
) -> OperationResult<()> {
    std::fs::create_dir_all(output).map_err(paperpilot_core::error::PdfError::IoError)?;
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::extract_images::ExtractImagesOperation::new(
        output.to_path_buf(),
    );
    op.execute(&mut doc)
}

pub fn handle_search(input: &std::path::Path, query: &str, output: Option<&std::path::Path>) -> OperationResult<()> {
    let mut matches = Vec::new();
    if input.is_dir() {
        if let Ok(entries) = std::fs::read_dir(input) {
            for entry in entries.flatten() {
                let ep = entry.path();
                if ep.extension().and_then(|ext| ext.to_str()).map(|s| s.eq_ignore_ascii_case("pdf")).unwrap_or(false) {
                    if let Ok(mut doc) = LopdfDocument::load(&ep) {
                        let op = paperpilot_pdf::operations::search::SearchOperation::new(query.to_string());
                        if op.execute(&mut doc).is_ok() {
                            let p = op.match_pages.lock().unwrap().clone();
                            matches.extend(p);
                        }
                    }
                }
            }
        }
    } else {
        let mut doc = LopdfDocument::load(input)?;
        let op = paperpilot_pdf::operations::search::SearchOperation::new(query.to_string());
        op.execute(&mut doc)?;
        matches = op.match_pages.lock().unwrap().clone();
    }

    if matches.is_empty() {
        println!("No matches found for '{}'.", query);
    } else {
        println!("Found '{}' on page(s): {:?}", query, matches);
    }

    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let res_json = serde_json::json!({
            "query": query,
            "matches": matches,
        });
        let _ = std::fs::write(out_path, serde_json::to_string_pretty(&res_json).unwrap_or_default());
    }

    Ok(())
}

pub fn handle_render(input: &std::path::Path, output: &std::path::Path) -> OperationResult<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).map_err(paperpilot_core::error::PdfError::IoError)?;
    }
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::render::RenderOperation {
        page: Some(1),
        dpi: 150.0,
        output_path: Some(output.to_path_buf()),
        rendered_images: std::sync::Mutex::new(Vec::new()),
    };
    op.execute(&mut doc)?;
    let images = op.rendered_images.lock().map_err(|_| paperpilot_core::error::PdfError::Other("Mutex poisoned".to_string()))?;
    if let Some(first_img) = images.first() {
        std::fs::write(output, first_img).map_err(paperpilot_core::error::PdfError::IoError)?;
    }
    Ok(())
}

pub fn handle_compare(input: &std::path::Path, input_b: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let mut op = paperpilot_pdf::operations::compare::CompareOperation::new();
    op.input_b = Some(input_b.to_string_lossy().to_string());
    op.execute(&mut doc)
}

pub fn handle_ocr(
    input: &std::path::Path,
    output: Option<&std::path::Path>,
    format: &str,
    clipboard: bool,
    tier: Option<&str>,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let mut op = paperpilot_pdf::operations::ocr::OcrOperation::new();

    if let Some(t) = tier {
        log::info!("OCR Tier selected: {}", t);
    }

    // Determine target output path if specified or if format dictates a file
    let resolved_output = if let Some(out) = output {
        Some(out.to_path_buf())
    } else if format.eq_ignore_ascii_case("text") || format.eq_ignore_ascii_case("txt") {
        let mut p = input.to_path_buf();
        p.set_extension("txt");
        Some(p)
    } else if format.eq_ignore_ascii_case("pdf") && !clipboard {
        let file_stem = input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("document");
        let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
        Some(parent.join(format!("{}_ocr.pdf", file_stem)))
    } else {
        None
    };

    if let Some(ref out) = resolved_output {
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent).map_err(paperpilot_core::error::PdfError::IoError)?;
        }
        op.output_path = Some(out.clone());
    }

    op.execute(&mut doc)?;

    let extracted = op
        .extracted_text
        .lock()
        .unwrap()
        .clone()
        .unwrap_or_default();

    // If clipboard is requested or format is 'clipboard', write to system clipboard
    if clipboard || format.eq_ignore_ascii_case("clipboard") {
        copy_text_to_clipboard(&extracted);
        println!(
            "OCR text copied to clipboard ({} characters).",
            extracted.len()
        );
    }

    if let Some(ref out) = resolved_output {
        println!("OCR output written to: {}", out.display());
    }

    Ok(())
}

fn copy_text_to_clipboard(text: &str) {
    // Try wl-copy (Wayland)
    if let Ok(mut child) = std::process::Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        use std::io::Write;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
        return;
    }

    // Try xclip (X11)
    if let Ok(mut child) = std::process::Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        use std::io::Write;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
        return;
    }

    // Try xsel (X11)
    if let Ok(mut child) = std::process::Command::new("xsel")
        .arg("--clipboard")
        .arg("--input")
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        use std::io::Write;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
        return;
    }

    // Fallback: OSC 52 terminal clipboard escape sequence
    let b64 = {
        // Simple base64 encode using std/core
        const B64_CHARS: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let bytes = text.as_bytes();
        let mut buf = String::with_capacity((bytes.len() * 4) / 3 + 4);
        for chunk in bytes.chunks(3) {
            let b0 = chunk[0];
            let b1 = chunk.get(1).copied().unwrap_or(0);
            let b2 = chunk.get(2).copied().unwrap_or(0);
            let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
            buf.push(B64_CHARS[((n >> 18) & 63) as usize] as char);
            buf.push(B64_CHARS[((n >> 12) & 63) as usize] as char);
            if chunk.len() > 1 {
                buf.push(B64_CHARS[((n >> 6) & 63) as usize] as char);
            } else {
                buf.push('=');
            }
            if chunk.len() > 2 {
                buf.push(B64_CHARS[(n & 63) as usize] as char);
            } else {
                buf.push('=');
            }
        }
        buf
    };
    print!("\x1b]52;c;{}\x07", b64);
}

pub fn handle_bookmarks(input: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::bookmarks::BookmarksOperation::new();
    op.execute(&mut doc)
}

pub fn handle_images_to_pdf(images: &[PathBuf], output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::new();
    let op = paperpilot_pdf::operations::images_to_pdf::ImagesToPdfOperation::new(
        images.iter().map(|p| p.to_path_buf()).collect(),
    );
    op.execute(&mut doc)?;
    doc.save(output)
}
