impl PdfOperation for PdfToDocxOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        use docx_rs::{
            Docx, Paragraph, Pic, Run, RunFonts, Table, TableBorders, TableCell, TableRow,
            WidthType,
        };

        let mut docx = Docx::new().page_margin(
            docx_rs::PageMargin::new()
                .top(720)
                .bottom(720)
                .left(720)
                .right(720),
        );

        let doc = &mut lopdf_doc.inner;
        let mut all_pages: Vec<u32> = doc.get_pages().keys().copied().collect();
        all_pages.sort_unstable();

        for (i, page_id) in all_pages.iter().enumerate() {
            let mut images_on_page = Vec::new();

            if let Ok(page_dict) = doc.get_dictionary((*page_id, 0))
                && let Ok(resources) = page_dict.get(b"Resources")
                && let Ok(resources_dict) = doc
                    .get_object(resources.as_reference().unwrap_or((0, 0)))
                    .and_then(|o| o.as_dict())
                    .or_else(|_| resources.as_dict())
                && let Ok(xobjects) = resources_dict.get(b"XObject")
                && let Ok(xobjects_dict) = doc
                    .get_object(xobjects.as_reference().unwrap_or((0, 0)))
                    .and_then(|o| o.as_dict())
                    .or_else(|_| xobjects.as_dict())
            {
                for (_, obj) in xobjects_dict.iter() {
                    if let Ok(obj_ref) = obj.as_reference()
                        && let Ok(xobject) = doc.get_object(obj_ref)
                        && let Ok(stream) = xobject.as_stream()
                        && let Ok(subtype) = stream.dict.get(b"Subtype").and_then(|s| s.as_name())
                        && subtype == b"Image"
                        && let Ok(content) = stream.decompressed_content() {
                            let mut filter = "";
                            if let Ok(f) = stream.dict.get(b"Filter")
                                && let Ok(n) = f.as_name()
                            {
                                filter = std::str::from_utf8(n).unwrap_or("");
                            }
                            images_on_page.push((filter.to_string(), stream.dict.clone(), content));
                        }
                }
            }

            // Extract fonts cleanly
            let mut fonts_on_page = std::collections::HashMap::new();
            if let Ok(page_dict) = doc.get_dictionary((*page_id, 0))
                && let Ok(resources) = page_dict.get(b"Resources")
                    && let Ok(res_dict) = doc.get_object(resources.as_reference().unwrap_or((0, 0))).and_then(|o| o.as_dict()).or_else(|_| resources.as_dict())
                        && let Ok(fonts) = res_dict.get(b"Font").and_then(|o| doc.get_object(o.as_reference().unwrap_or((0,0))).and_then(|o| o.as_dict()).or_else(|_| o.as_dict())) {
                            for (font_name, font_obj) in fonts.iter() {
                                if let Ok(font_dict) = doc.get_object(font_obj.as_reference().unwrap_or((0,0))).and_then(|o| o.as_dict()).or_else(|_| font_obj.as_dict())
                                    && let Ok(base_font) = font_dict.get(b"BaseFont").and_then(|n| n.as_name()) {
                                        let name = std::str::from_utf8(base_font).unwrap_or("Arial");
                                        fonts_on_page.insert(font_name.to_vec(), sanitize_font_name(name));
                                    }
                            }
                        }

            let page_text = doc.extract_text(&[*page_id]).unwrap_or_default();
            let mut text_blocks = Vec::new();
            for line in page_text.lines() {
                let trimmed = sanitize_xml_text(line.trim());
                if !trimmed.is_empty() {
                    text_blocks.push(trimmed);
                }
            }

            // Default font
            let default_font = if !fonts_on_page.is_empty() {
                fonts_on_page.values().next().unwrap().clone()
            } else {
                "Arial".to_string()
            };

            // Heuristic for 2-column layout
            if !images_on_page.is_empty() && !text_blocks.is_empty() {
                let mut right_cell = TableCell::new().width(5000, WidthType::Dxa);

                let heading = &text_blocks[0];
                let heading_run = Run::new()
                    .add_text(heading)
                    .bold()
                    .size(24)
                    .fonts(RunFonts::new().ascii(default_font.clone()).cs(default_font.clone()).hi_ansi(default_font.clone()));
                right_cell = right_cell.add_paragraph(Paragraph::new().add_run(heading_run));

                for block in text_blocks.into_iter().skip(1) {
                    let run = Run::new()
                        .add_text(block)
                        .size(20)
                        .fonts(RunFonts::new().ascii(default_font.clone()).cs(default_font.clone()).hi_ansi(default_font.clone()));
                    right_cell = right_cell.add_paragraph(Paragraph::new().add_run(run));
                }

                let (filter, dict, img_bytes) = images_on_page[0].clone();
                let mut final_png_bytes = Vec::new();

                if filter == "DCTDecode" {
                    final_png_bytes = img_bytes.clone(); // docx_rs accepts JPEG directly
                    if let Ok(dynamic_image) = image::load_from_memory(&final_png_bytes) {
                        let mut png_data = Vec::new();
                        let mut cursor = std::io::Cursor::new(&mut png_data);
                        if dynamic_image.write_to(&mut cursor, image::ImageFormat::Png).is_ok() {
                            final_png_bytes = png_data;
                        }
                    }
                } else if filter == "FlateDecode" || filter.is_empty() {
                    // Raw decompressed pixels! Need to reconstruct using Width, Height, ColorSpace
                    if let (Ok(width), Ok(height)) = (dict.get(b"Width").and_then(|w| w.as_i64()), dict.get(b"Height").and_then(|h| h.as_i64())) {
                        let width = width as u32;
                        let height = height as u32;
                        let mut is_rgb = false;
                        if let Ok(cs) = dict.get(b"ColorSpace").and_then(|c| c.as_name())
                            && cs == b"DeviceRGB" {
                                is_rgb = true;
                            }

                        let color_type = if is_rgb { image::ColorType::Rgb8 } else { image::ColorType::L8 };
                        let mut png_data = Vec::new();
                        let mut cursor = std::io::Cursor::new(&mut png_data);
                        let encoder = image::codecs::png::PngEncoder::new(&mut cursor);
                        if image::ImageEncoder::write_image(encoder, &img_bytes, width, height, color_type.into()).is_ok() {
                            final_png_bytes = png_data;
                        }
                    }
                }

                if final_png_bytes.is_empty() && !img_bytes.is_empty() {
                    // Fallback
                    if let Ok(dynamic_image) = image::load_from_memory(&img_bytes) {
                        let mut png_data = Vec::new();
                        let mut cursor = std::io::Cursor::new(&mut png_data);
                        if dynamic_image.write_to(&mut cursor, image::ImageFormat::Png).is_ok() {
                            final_png_bytes = png_data;
                        }
                    }
                }

                if final_png_bytes.is_empty() {
                    final_png_bytes = img_bytes; // Use raw as last resort (docx might corrupt)
                }

                let table = Table::new(vec![TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(
                            Paragraph::new().add_run(Run::new().add_image(Pic::new(&final_png_bytes))),
                        )
                        .width(5000, WidthType::Dxa),
                    right_cell,
                ])])
                .margins(docx_rs::TableCellMargins::new())
                .set_borders(TableBorders::new().clear_all());

                docx = docx.add_table(table);
            } else {
                for block in text_blocks {
                    let run = Run::new()
                        .add_text(block)
                        .fonts(RunFonts::new().ascii(default_font.clone()).cs(default_font.clone()).hi_ansi(default_font.clone()));
                    docx = docx.add_paragraph(Paragraph::new().add_run(run));
                }
            }

            if i < all_pages.len() - 1 {
                docx = docx.add_paragraph(Paragraph::new().page_break_before(true));
            }
        }

        let file = std::fs::File::create(&self.output_path)
            .map_err(|e| PdfError::Other(format!("Failed to create DOCX file: {e}")))?;
        docx.build()
            .pack(file)
            .map_err(|e| PdfError::Other(format!("Failed to pack DOCX file: {e}")))?;

        Ok(())
    }
}

pub struct PdfToXlsxOperation {
    pub output_path: std::path::PathBuf,
}

impl PdfToXlsxOperation {
    pub fn new(output_path: std::path::PathBuf) -> Self {
        Self { output_path }
    }
}

impl PdfOperation for PdfToXlsxOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
