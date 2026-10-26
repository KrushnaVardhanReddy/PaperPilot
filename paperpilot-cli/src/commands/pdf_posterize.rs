use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_posterize_command(
    input: &PathBuf,
    output: &PathBuf,
    target_sheet: &String,
    grid: &Option<String>,
    overlap: &f32,
    scale: &f32,
    guides: &bool,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut grid_cols = None;
    let mut grid_rows = None;

    if let Some(g) = grid {
        let parts: Vec<&str> = g.split('x').collect();
        if parts.len() == 2 {
            if let (Ok(c), Ok(r)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                grid_cols = Some(c);
                grid_rows = Some(r);
            }
        }
    }

    let options = paperpilot_pdf::operations::pdf_posterize::PdfPosterizeOptions {
        target_sheet: target_sheet.clone(),
        grid_cols,
        grid_rows,
        overlap_pt: *overlap,
        scale: *scale,
        include_guides: *guides,
    };

    let output_bytes = paperpilot_pdf::operations::pdf_posterize::pdf_posterize_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfPosterize complete.");
    Ok(())
}
