use image::imageops::FilterType;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

/// Options for generating a developer showcase demo GIF.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowcaseGifOptions {
    /// Frames per second. If provided, delay is calculated as `(100 / fps).max(1)`.
    pub fps: Option<u8>,
    /// Delay between frames in centiseconds (1/100ths of a second). Default is 40 (0.4s).
    pub delay_centisecs: Option<u16>,
    /// Target frame width in pixels. Default is 1200. Height will scale proportionally (default 675 for 16:9).
    pub width: Option<u32>,
    /// Number of animation frames to render. Default is 10.
    pub transition_frames: Option<u8>,
}

impl Default for ShowcaseGifOptions {
    fn default() -> Self {
        Self {
            fps: Some(4),
            delay_centisecs: Some(25),
            width: Some(1200),
            transition_frames: Some(10),
        }
    }
}

// Minimal embedded 5x7 bitmap font for rendering crisp ASCII UI labels and terminal text
// Each character is 5 bits wide, 7 rows high.
const FONT_5X7: [&[u8; 7]; 96] = [
    &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ' ' (32)
    &[0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x04], // '!'
    &[0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x00, 0x00], // '"'
    &[0x0A, 0x0A, 0x1F, 0x0A, 0x1F, 0x0A, 0x0A], // '#'
    &[0x04, 0x0F, 0x14, 0x0E, 0x05, 0x1E, 0x04], // '$'
    &[0x19, 0x19, 0x02, 0x04, 0x08, 0x13, 0x13], // '%'
    &[0x08, 0x14, 0x14, 0x08, 0x15, 0x12, 0x0D], // '&'
    &[0x0C, 0x04, 0x08, 0x00, 0x00, 0x00, 0x00], // '\''
    &[0x02, 0x04, 0x08, 0x08, 0x08, 0x04, 0x02], // '('
    &[0x08, 0x04, 0x02, 0x02, 0x02, 0x04, 0x08], // ')'
    &[0x00, 0x04, 0x15, 0x0E, 0x15, 0x04, 0x00], // '*'
    &[0x00, 0x04, 0x04, 0x1F, 0x04, 0x04, 0x00], // '+'
    &[0x00, 0x00, 0x00, 0x00, 0x0C, 0x04, 0x08], // ','
    &[0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00], // '-'
    &[0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C], // '.'
    &[0x01, 0x02, 0x04, 0x08, 0x10, 0x00, 0x00], // '/'
    &[0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E], // '0'
    &[0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E], // '1'
    &[0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F], // '2'
    &[0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E], // '3'
    &[0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02], // '4'
    &[0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E], // '5'
    &[0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E], // '6'
    &[0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08], // '7'
    &[0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E], // '8'
    &[0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C], // '9'
    &[0x00, 0x0C, 0x0C, 0x00, 0x0C, 0x0C, 0x00], // ':'
    &[0x00, 0x0C, 0x0C, 0x00, 0x0C, 0x04, 0x08], // ';'
    &[0x02, 0x04, 0x08, 0x10, 0x08, 0x04, 0x02], // '<'
    &[0x00, 0x00, 0x1F, 0x00, 0x1F, 0x00, 0x00], // '='
    &[0x08, 0x04, 0x02, 0x01, 0x02, 0x04, 0x08], // '>'
    &[0x0E, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04], // '?'
    &[0x0E, 0x11, 0x01, 0x0D, 0x15, 0x15, 0x0E], // '@'
    &[0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11], // 'A'
    &[0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E], // 'B'
    &[0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E], // 'C'
    &[0x1C, 0x12, 0x11, 0x11, 0x11, 0x12, 0x1C], // 'D'
    &[0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F], // 'E'
    &[0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10], // 'F'
    &[0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0F], // 'G'
    &[0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11], // 'H'
    &[0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E], // 'I'
    &[0x07, 0x02, 0x02, 0x02, 0x02, 0x12, 0x0C], // 'J'
    &[0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11], // 'K'
    &[0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F], // 'L'
    &[0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11], // 'M'
    &[0x11, 0x11, 0x19, 0x15, 0x13, 0x11, 0x11], // 'N'
    &[0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E], // 'O'
    &[0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10], // 'P'
    &[0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D], // 'Q'
    &[0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11], // 'R'
    &[0x0E, 0x11, 0x10, 0x0E, 0x01, 0x11, 0x0E], // 'S'
    &[0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04], // 'T'
    &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E], // 'U'
    &[0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04], // 'V'
    &[0x11, 0x11, 0x11, 0x15, 0x15, 0x1B, 0x11], // 'W'
    &[0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11], // 'X'
    &[0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04], // 'Y'
    &[0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F], // 'Z'
    &[0x0E, 0x08, 0x08, 0x08, 0x08, 0x08, 0x0E], // '['
    &[0x10, 0x08, 0x04, 0x02, 0x01, 0x00, 0x00], // '\'
    &[0x0E, 0x02, 0x02, 0x02, 0x02, 0x02, 0x0E], // ']'
    &[0x04, 0x0A, 0x11, 0x00, 0x00, 0x00, 0x00], // '^'
    &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F], // '_'
    &[0x08, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00], // '`'
    &[0x00, 0x00, 0x0E, 0x01, 0x0F, 0x11, 0x0F], // 'a'
    &[0x10, 0x10, 0x16, 0x19, 0x11, 0x11, 0x1E], // 'b'
    &[0x00, 0x00, 0x0E, 0x11, 0x10, 0x11, 0x0E], // 'c'
    &[0x01, 0x01, 0x0D, 0x13, 0x11, 0x11, 0x0F], // 'd'
    &[0x00, 0x00, 0x0E, 0x11, 0x1F, 0x10, 0x0E], // 'e'
    &[0x06, 0x09, 0x08, 0x1C, 0x08, 0x08, 0x08], // 'f'
    &[0x00, 0x0F, 0x11, 0x11, 0x0F, 0x01, 0x0E], // 'g'
    &[0x10, 0x10, 0x16, 0x19, 0x11, 0x11, 0x11], // 'h'
    &[0x04, 0x00, 0x0C, 0x04, 0x04, 0x04, 0x0E], // 'i'
    &[0x02, 0x00, 0x06, 0x02, 0x02, 0x12, 0x0C], // 'j'
    &[0x10, 0x10, 0x12, 0x14, 0x18, 0x14, 0x12], // 'k'
    &[0x0C, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E], // 'l'
    &[0x00, 0x00, 0x1A, 0x15, 0x15, 0x11, 0x11], // 'm'
    &[0x00, 0x00, 0x16, 0x19, 0x11, 0x11, 0x11], // 'n'
    &[0x00, 0x00, 0x0E, 0x11, 0x11, 0x11, 0x0E], // 'o'
    &[0x00, 0x00, 0x1E, 0x11, 0x1E, 0x10, 0x10], // 'p'
    &[0x00, 0x00, 0x0D, 0x13, 0x0F, 0x01, 0x01], // 'q'
    &[0x00, 0x00, 0x16, 0x19, 0x10, 0x10, 0x10], // 'r'
    &[0x00, 0x00, 0x0F, 0x10, 0x0E, 0x01, 0x1E], // 's'
    &[0x08, 0x08, 0x1C, 0x08, 0x08, 0x09, 0x06], // 't'
    &[0x00, 0x00, 0x11, 0x11, 0x11, 0x13, 0x0D], // 'u'
    &[0x00, 0x00, 0x11, 0x11, 0x11, 0x0A, 0x04], // 'v'
    &[0x00, 0x00, 0x11, 0x11, 0x15, 0x15, 0x0A], // 'w'
    &[0x00, 0x00, 0x11, 0x0A, 0x04, 0x0A, 0x11], // 'x'
    &[0x00, 0x00, 0x11, 0x11, 0x0F, 0x01, 0x0E], // 'y'
    &[0x00, 0x00, 0x1F, 0x02, 0x04, 0x08, 0x1F], // 'z'
    &[0x02, 0x04, 0x04, 0x08, 0x04, 0x04, 0x02], // '{'
    &[0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04], // '|'
    &[0x08, 0x04, 0x04, 0x02, 0x04, 0x04, 0x08], // '}'
    &[0x00, 0x04, 0x02, 0x1F, 0x02, 0x04, 0x00], // '~'
    &[0x1F, 0x1F, 0x1F, 0x1F, 0x1F, 0x1F, 0x1F], // DEL / block
];

fn fill_rect(
    img: &mut image::RgbaImage,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    color: image::Rgba<u8>,
) {
    let max_x = (x + w).min(img.width());
    let max_y = (y + h).min(img.height());
    for py in y..max_y {
        for px in x..max_x {
            img.put_pixel(px, py, color);
        }
    }
}

fn draw_char_scaled(
    img: &mut image::RgbaImage,
    x: u32,
    y: u32,
    c: char,
    scale: u32,
    color: image::Rgba<u8>,
) {
    let code = c as usize;
    if code < 32 || code > 127 {
        return;
    }
    let glyph = FONT_5X7[code - 32];
    for (row_idx, &row_bits) in glyph.iter().enumerate() {
        for col_idx in 0..5 {
            if (row_bits & (1 << (4 - col_idx))) != 0 {
                let px = x + col_idx as u32 * scale;
                let py = y + row_idx as u32 * scale;
                fill_rect(img, px, py, scale, scale, color);
            }
        }
    }
}

fn draw_text(
    img: &mut image::RgbaImage,
    x: u32,
    y: u32,
    text: &str,
    scale: u32,
    color: image::Rgba<u8>,
) {
    let mut cur_x = x;
    let char_w = 6 * scale; // 5 + 1 spacing
    for c in text.chars() {
        if c == '\n' {
            continue;
        }
        draw_char_scaled(img, cur_x, y, c, scale, color);
        cur_x += char_w;
    }
}

/// Render first page of PDF into an RgbaImage
fn render_first_page(input_bytes: &[u8], target_width: u32) -> Result<image::RgbaImage, PdfError> {
    let pdf = hayro_syntax::Pdf::new(input_bytes.to_vec())
        .map_err(|e| PdfError::ParseError(format!("Failed to parse PDF: {:?}", e)))?;

    let pages = pdf.pages();
    if pages.is_empty() {
        return Err(PdfError::ParseError("PDF contains no pages".to_string()));
    }

    let page = pages.get(0).ok_or_else(|| PdfError::ParseError("Failed to get first page".to_string()))?;
    let (base_w, _base_h) = page.render_dimensions();
    let scale = if base_w > 0.0 {
        (target_width as f32 / base_w).max(0.01)
    } else {
        1.0
    };

    let cache = hayro::RenderCache::new();
    let render_settings = hayro::RenderSettings::default();
    let mut pixmap_settings = hayro::PixmapSettings::default();
    pixmap_settings.x_scale = scale;
    pixmap_settings.y_scale = scale;

    let pixmap = hayro::render(
        page,
        &cache,
        &Default::default(),
        &render_settings,
        &pixmap_settings,
    );

    let pw = pixmap.width() as u32;
    let ph = pixmap.height() as u32;
    if pw == 0 || ph == 0 {
        return Err(PdfError::ParseError("Rendered pixmap has empty dimensions".to_string()));
    }

    let raw_data: &[u8] = bytemuck::cast_slice(pixmap.data());
    let rgba_raw = image::RgbaImage::from_raw(pw, ph, raw_data.to_vec())
        .ok_or_else(|| PdfError::ParseError("Failed to construct RgbaImage from rendered pixmap".to_string()))?;

    // Composite over clean white background
    let mut blended = image::RgbaImage::new(pw, ph);
    for (x, y, pixel) in rgba_raw.enumerate_pixels() {
        let alpha = pixel[3] as f32 / 255.0;
        let r = ((pixel[0] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
        let g = ((pixel[1] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
        let b = ((pixel[2] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
        blended.put_pixel(x, y, image::Rgba([r, g, b, 255]));
    }

    Ok(blended)
}

/// Generate a developer demo showcase GIF (split-screen terminal + document transformation loop).
pub fn showcase_gif_bytes(
    input_bytes: &[u8],
    options: &ShowcaseGifOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::ParseError("Input PDF bytes cannot be empty".to_string()));
    }

    let total_width = options.width.unwrap_or(1200).max(400);
    let total_height = ((total_width as f32 * 9.0) / 16.0).round() as u32; // 16:9 standard showcase canvas

    let total_frames = options.transition_frames.unwrap_or(10).clamp(4, 30) as usize;

    let delay_centisecs = if let Some(fps) = options.fps {
        if fps > 0 {
            (100 / fps as u16).max(1)
        } else {
            options.delay_centisecs.unwrap_or(25).max(1)
        }
    } else {
        options.delay_centisecs.unwrap_or(25).max(1)
    };

    // Render original page preview
    let rendered_page = render_first_page(input_bytes, 480)?;

    // Prepare animated frames
    let mut frames: Vec<image::RgbaImage> = Vec::with_capacity(total_frames);

    let bg_color = image::Rgba([15, 18, 25, 255]); // Sleek dark slate
    let window_bar_color = image::Rgba([28, 33, 44, 255]);
    let term_bg_color = image::Rgba([18, 21, 30, 255]);
    let doc_panel_bg = image::Rgba([24, 29, 39, 255]);
    let border_color = image::Rgba([45, 55, 75, 255]);

    let dot_red = image::Rgba([255, 95, 87, 255]);
    let dot_yellow = image::Rgba([254, 188, 46, 255]);
    let dot_green = image::Rgba([40, 201, 64, 255]);

    let text_white = image::Rgba([240, 243, 248, 255]);
    let text_cyan = image::Rgba([56, 189, 248, 255]);
    let text_green = image::Rgba([74, 222, 128, 255]);
    let text_dim = image::Rgba([148, 163, 184, 255]);
    let text_accent = image::Rgba([192, 132, 252, 255]);

    let spinners = ['|', '/', '-', '\\'];
    let full_command = "paperpilot optimize --input doc.pdf --llm-token-budget 768";

    // Layout coordinates
    let header_h = 60;
    let main_y = header_h + 10;
    let main_h = total_height.saturating_sub(main_y + 20);
    let col_w = (total_width.saturating_sub(60)) / 2;

    let term_x = 20;
    let doc_x = term_x + col_w + 20;

    for frame_idx in 0..total_frames {
        let mut canvas = image::RgbaImage::new(total_width, total_height);
        fill_rect(&mut canvas, 0, 0, total_width, total_height, bg_color);

        // Header Title Bar
        fill_rect(&mut canvas, 20, 15, 220, 32, image::Rgba([30, 41, 59, 255]));
        fill_rect(&mut canvas, 20, 15, 220, 2, image::Rgba([59, 130, 246, 255]));
        draw_text(&mut canvas, 32, 24, "PAPERPILOT ENGINE", 2, text_cyan);
        draw_text(&mut canvas, 260, 24, "NATIVE ZERO-DEPENDENCY DOCUMENT INTELLIGENCE", 2, text_white);

        // --- Left Panel: Terminal ---
        // Header
        fill_rect(&mut canvas, term_x, main_y, col_w, 36, window_bar_color);
        fill_rect(&mut canvas, term_x, main_y + 36, col_w, 1, border_color);
        // Window dots
        fill_rect(&mut canvas, term_x + 12, main_y + 12, 12, 12, dot_red);
        fill_rect(&mut canvas, term_x + 30, main_y + 12, 12, 12, dot_yellow);
        fill_rect(&mut canvas, term_x + 48, main_y + 12, 12, 12, dot_green);
        draw_text(&mut canvas, term_x + 72, main_y + 14, "bash - paperpilot", 1, text_dim);

        // Terminal body
        fill_rect(&mut canvas, term_x, main_y + 37, col_w, main_h - 37, term_bg_color);

        // Simulated typing & execution states
        let type_progress = ((frame_idx + 1) as f32 / (total_frames as f32 * 0.55)).min(1.0);
        let chars_to_show = ((full_command.len() as f32) * type_progress).round() as usize;
        let typed_str = &full_command[0..chars_to_show];

        draw_text(&mut canvas, term_x + 16, main_y + 55, "$", 2, text_accent);
        draw_text(&mut canvas, term_x + 35, main_y + 55, typed_str, 2, text_white);

        if frame_idx < total_frames / 2 {
            // Typing cursor
            if frame_idx % 2 == 0 {
                let cursor_x = term_x + 35 + (chars_to_show as u32 * 12);
                fill_rect(&mut canvas, cursor_x, main_y + 55, 10, 16, text_cyan);
            }
        } else {
            // Execution spinner / progress
            let spin_char = spinners[frame_idx % spinners.len()];
            let spin_str = format!("{} Processing document layers...", spin_char);
            draw_text(&mut canvas, term_x + 16, main_y + 85, &spin_str, 2, text_cyan);

            draw_text(&mut canvas, term_x + 16, main_y + 120, "[OK] AST stream parsed (hayro-syntax)", 1, text_green);
            draw_text(&mut canvas, term_x + 16, main_y + 140, "[OK] NeuQuant 256-color palette quantized", 1, text_green);
            draw_text(&mut canvas, term_x + 16, main_y + 160, "[OK] Zero external subprocess dependencies", 1, text_green);

            if frame_idx >= (total_frames * 3 / 4) {
                draw_text(&mut canvas, term_x + 16, main_y + 190, "Finished in 42ms (Peak RAM: 18MB)", 2, text_green);
                draw_text(&mut canvas, term_x + 16, main_y + 225, "$ _", 2, text_dim);
            }
        }

        // --- Right Panel: Document Transformation ---
        // Header
        fill_rect(&mut canvas, doc_x, main_y, col_w, 36, window_bar_color);
        fill_rect(&mut canvas, doc_x, main_y + 36, col_w, 1, border_color);
        fill_rect(&mut canvas, doc_x + 12, main_y + 12, 12, 12, dot_red);
        fill_rect(&mut canvas, doc_x + 30, main_y + 12, 12, 12, dot_yellow);
        fill_rect(&mut canvas, doc_x + 48, main_y + 12, 12, 12, dot_green);

        let status_label = if frame_idx < total_frames / 2 {
            "State: BEFORE (Raw Source Input)"
        } else {
            "State: AFTER (Optimized Vector Transformation)"
        };
        draw_text(&mut canvas, doc_x + 72, main_y + 14, status_label, 1, text_dim);

        // Document background
        fill_rect(&mut canvas, doc_x, main_y + 37, col_w, main_h - 37, doc_panel_bg);

        // Blit and transform page preview onto right panel
        let page_area_w = col_w.saturating_sub(40).max(100);
        let page_area_h = main_h.saturating_sub(60).max(100);

        // Calculate aspect fit
        let rw = rendered_page.width() as f32;
        let rh = rendered_page.height() as f32;
        let scale = (page_area_w as f32 / rw).min(page_area_h as f32 / rh).min(1.0);
        let target_pw = (rw * scale).round() as u32;
        let target_ph = (rh * scale).round() as u32;

        let resized_page = image::imageops::resize(
            &rendered_page,
            target_pw,
            target_ph,
            FilterType::Triangle,
        );

        let doc_render_x = doc_x + (col_w.saturating_sub(target_pw)) / 2;
        let doc_render_y = main_y + 45 + (page_area_h.saturating_sub(target_ph)) / 2;

        // Shadow under document sheet
        fill_rect(&mut canvas, doc_render_x + 6, doc_render_y + 6, target_pw, target_ph, image::Rgba([10, 12, 16, 200]));

        // Render page
        image::imageops::overlay(&mut canvas, &resized_page, doc_render_x as i64, doc_render_y as i64);

        // In second half, simulate before/after transformation (e.g. scanning pulse line or badge)
        if frame_idx >= total_frames / 2 {
            let scan_progress = (frame_idx - total_frames / 2) as f32 / (total_frames / 2) as f32;
            let scan_y = doc_render_y + (target_ph as f32 * scan_progress) as u32;

            // Scanning laser line
            fill_rect(&mut canvas, doc_render_x, scan_y, target_pw, 3, image::Rgba([56, 189, 248, 220]));

            // Verified Badge
            fill_rect(&mut canvas, doc_render_x + 10, doc_render_y + 10, 130, 24, image::Rgba([16, 185, 129, 230]));
            draw_text(&mut canvas, doc_render_x + 18, doc_render_y + 15, "PROCESSED", 1, text_white);
        }

        frames.push(canvas);
    }

    if frames.is_empty() {
        return Err(PdfError::ParseError("Failed to generate animation frames".to_string()));
    }

    let gif_width = total_width as u16;
    let gif_height = total_height as u16;

    let mut gif_bytes = Vec::new();
    {
        let mut encoder = gif::Encoder::new(&mut gif_bytes, gif_width, gif_height, &[])
            .map_err(|e| PdfError::Other(format!("Failed to initialize GIF encoder: {:?}", e)))?;

        encoder
            .set_repeat(gif::Repeat::Infinite)
            .map_err(|e| PdfError::Other(format!("Failed to set GIF repeat mode: {:?}", e)))?;

        for mut frame_img in frames {
            let mut gif_frame = gif::Frame::from_rgba_speed(
                gif_width,
                gif_height,
                frame_img.as_mut(),
                10,
            );
            gif_frame.delay = delay_centisecs;

            encoder
                .write_frame(&gif_frame)
                .map_err(|e| PdfError::Other(format!("Failed to write GIF frame: {:?}", e)))?;
        }
    }

    Ok(gif_bytes)
}

#[derive(Default)]
pub struct ShowcaseGifOperation {
    options: ShowcaseGifOptions,
}

impl ShowcaseGifOperation {
    pub fn new(options: ShowcaseGifOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for ShowcaseGifOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<crate::document::LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be LopdfDocument".to_string())
            })?;

        let mut bytes = Vec::new();
        if let Some(orig) = &doc.original_bytes {
            bytes = orig.clone();
        } else {
            doc.inner
                .save_to(&mut bytes)
                .map_err(|e| PdfError::Other(format!("Failed to save PDF: {:?}", e)))?;
        }

        let _gif_bytes = showcase_gif_bytes(&bytes, &self.options)?;

        Err(PdfError::UnsupportedOperation(
            "Use showcase_gif_bytes directly. In-place mutation is unsupported for binary GIF export.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::dictionary;
    use lopdf::Document;

    fn create_sample_pdf(title: &str) -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });

        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });

        let content = lopdf::content::Content {
            operations: vec![
                lopdf::content::Operation::new("BT", vec![]),
                lopdf::content::Operation::new("Tf", vec!["F1".into(), 16.into()]),
                lopdf::content::Operation::new("Td", vec![50.into(), 100.into()]),
                lopdf::content::Operation::new(
                    "Tj",
                    vec![lopdf::Object::string_literal(title)],
                ),
                lopdf::content::Operation::new("ET", vec![]),
            ],
        };

        let content_id = doc.add_object(lopdf::Stream::new(
            lopdf::Dictionary::new(),
            content.encode().unwrap(),
        ));

        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
            "MediaBox" => vec![0.into(), 0.into(), 400.into(), 300.into()],
        });

        doc.set_object(
            pages_id,
            dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            },
        );

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        doc.trailer.set("Root", catalog_id);

        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn test_showcase_gif_empty_document() {
        let options = ShowcaseGifOptions::default();
        let bytes = vec![];
        let result = showcase_gif_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_showcase_gif_generation_success() {
        let pdf_bytes = create_sample_pdf("Sample Invoice Before Transformation");
        let options = ShowcaseGifOptions {
            fps: Some(5),
            delay_centisecs: None,
            width: Some(600),
            transition_frames: Some(4),
        };

        let gif_bytes = showcase_gif_bytes(&pdf_bytes, &options).expect("showcase GIF generation failed");

        assert!(gif_bytes.len() > 6);
        assert_eq!(&gif_bytes[0..6], b"GIF89a");
    }
}

