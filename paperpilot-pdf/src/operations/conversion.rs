use crate::document::LopdfDocument;
use lopdf::dictionary;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use rust_xlsxwriter::Workbook;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

pub const CSS_PRESET_GITHUB: &str = r#"
@page { margin: 20mm; size: A4; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: 14px;
    line-height: 1.6;
    color: #24292e;
    background-color: #ffffff;
}
h1, h2, h3, h4, h5, h6 {
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
    line-height: 1.25;
    color: #1f2328;
}
h1 { font-size: 2em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
h2 { font-size: 1.5em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
code {
    background-color: rgba(175, 184, 193, 0.2);
    border-radius: 4px;
    padding: 0.2em 0.4em;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
    font-size: 85%;
}
pre {
    background-color: #f6f8fa;
    border-radius: 6px;
    padding: 16px;
    overflow: auto;
    line-height: 1.45;
}
table {
    border-collapse: collapse;
    width: 100%;
    margin: 16px 0;
}
table th, table td {
    padding: 8px 12px;
    border: 1px solid #d0d7de;
}
table th {
    background-color: #f6f8fa;
    font-weight: 600;
}
table tr:nth-child(2n) {
    background-color: #f6f8fa;
}
thead { display: table-header-group; }
tr { page-break-inside: avoid; }
"#;

pub const CSS_PRESET_ELEGANT: &str = r#"
@page { margin: 25mm; size: A4; }
body {
    font-family: "Georgia", "Merriweather", serif;
    font-size: 15px;
    line-height: 1.7;
    color: #2c3e50;
    background-color: #ffffff;
}
h1, h2, h3 {
    font-family: "Palatino Linotype", "Book Antiqua", Palatino, serif;
    color: #1a252f;
    letter-spacing: 0.02em;
}
h1 { font-size: 2.2em; text-align: center; margin-bottom: 1.5em; }
h2 { font-size: 1.6em; border-bottom: 2px solid #8e44ad; padding-bottom: 6px; margin-top: 1.8em; }
table {
    border-collapse: collapse;
    width: 100%;
    margin: 20px 0;
}
table th {
    border-bottom: 2px solid #2c3e50;
    padding: 10px 14px;
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}
table td {
    border-bottom: 1px solid #ecf0f1;
    padding: 10px 14px;
}
"#;

pub const CSS_PRESET_MINIMAL: &str = r#"
@page { margin: 18mm; size: A4; }
body {
    font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
    font-size: 13px;
    line-height: 1.5;
    color: #111111;
}
h1, h2, h3 { font-weight: 700; color: #000000; }
h1 { font-size: 1.8em; }
h2 { font-size: 1.4em; }
table { width: 100%; border-collapse: collapse; margin: 16px 0; }
table th, table td { padding: 6px 10px; border-bottom: 1px solid #000; text-align: left; }
table th { font-weight: 700; border-top: 1px solid #000; }
"#;

pub const CSS_PRESET_BRANDED: &str = r#"
@page { margin: 20mm; size: A4; }
body {
    font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 13.5px;
    line-height: 1.6;
    color: #1e293b;
}
h1 { font-size: 2em; color: #4361ee; border-left: 4px solid #4361ee; padding-left: 12px; }
h2 { font-size: 1.4em; color: #3a0ca3; margin-top: 1.5em; }
table { width: 100%; border-collapse: separate; border-spacing: 0; border-radius: 8px; overflow: hidden; border: 1px solid #e2e8f0; margin: 16px 0; }
table th { background: #4361ee; color: #ffffff; padding: 10px 14px; font-weight: 600; text-align: left; }
table td { padding: 8px 14px; border-bottom: 1px solid #f1f5f9; }
table tr:nth-child(even) td { background-color: #f8fafc; }
"#;

pub const CSS_PRESET_COMPACT: &str = r#"
@page { margin: 12mm; size: A4; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 11px;
    line-height: 1.35;
    color: #1f2937;
}
h1 { font-size: 1.4em; margin-bottom: 8px; }
h2 { font-size: 1.2em; margin-top: 12px; margin-bottom: 6px; }
table { width: 100%; border-collapse: collapse; margin: 8px 0; font-size: 10.5px; }
table th, table td { padding: 4px 6px; border: 1px solid #d1d5db; }
table th { background: #f3f4f6; font-weight: 600; }
thead { display: table-header-group; }
tr { page-break-inside: avoid; }
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlToPdfOptions {
    pub preset: Option<String>, // "github", "elegant", "minimal", "branded", "compact"
    pub custom_css: Option<String>, // inline CSS string
    pub page_size: Option<String>, // "A4", "Letter", etc.
    pub margin_mm: Option<f32>, // default 20mm
}

impl Default for HtmlToPdfOptions {
    fn default() -> Self {
        Self {
            preset: Some("github".to_string()),
            custom_css: None,
            page_size: Some("A4".to_string()),
            margin_mm: Some(20.0),
        }
    }
}

pub fn resolve_css(preset: Option<&str>, custom_css: Option<&str>) -> String {
    let preset_css = match preset.unwrap_or("github") {
        "elegant" => CSS_PRESET_ELEGANT,
        "minimal" => CSS_PRESET_MINIMAL,
        "branded" => CSS_PRESET_BRANDED,
        "compact" => CSS_PRESET_COMPACT,
        _ => CSS_PRESET_GITHUB,
    };

    if let Some(custom) = custom_css
        && !custom.trim().is_empty()
    {
        return format!("{}\n/* Custom CSS */\n{}", preset_css, custom);
    }
    preset_css.to_string()
}

pub fn inject_css_into_html(html: &str, css: &str) -> String {
    let style_tag = format!("<style id=\"paperpilot-injected-css\">\n{}\n</style>", css);
    if let Some(pos) = html.find("</head>") {
        let mut out = html[..pos].to_string();
        out.push_str(&style_tag);
        out.push_str(&html[pos..]);
        out
    } else {
        format!(
            "<!DOCTYPE html><html><head>{}</head><body>{}</body></html>",
            style_tag, html
        )
    }
}

pub struct HtmlToPdfOperation {
    pub input_html: String,
    pub output_path: PathBuf,
    pub options: HtmlToPdfOptions,
}

impl HtmlToPdfOperation {
    pub fn new(input_html: String, output_path: PathBuf, options: HtmlToPdfOptions) -> Self {
        Self {
            input_html,
            output_path,
            options,
        }
    }

    pub fn render(&self) -> OperationResult<()> {
        let css = resolve_css(
            self.options.preset.as_deref(),
            self.options.custom_css.as_deref(),
        );
        let styled_html = inject_css_into_html(&self.input_html, &css);

        let img_re = regex::Regex::new(r#"(?i)(<img\b[^>]*?\bsrc\s*=\s*['"])(.*?)(['"])"#).unwrap();

        let bundle_cell =
            std::sync::Arc::new(std::sync::Mutex::new(fulgur::asset::AssetBundle::new()));
        let counter_cell = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let new_styled_html = img_re
            .replace_all(&styled_html, |caps: &regex::Captures| {
                let prefix = &caps[1];
                let src = &caps[2];
                let suffix = &caps[3];

                let mut replaced_src = src.to_string();

                if src.starts_with("data:image/") {
                    if let Some(pos) = src.find(";base64,") {
                        let b64 = &src[pos + 8..];
                        use base64::{Engine as _, engine::general_purpose};
                        if let Ok(bytes) = general_purpose::STANDARD.decode(b64) {
                            let counter =
                                counter_cell.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            let asset_name = format!("inline_img_{}", counter);
                            if let Ok(mut b) = bundle_cell.lock() {
                                b.add_image(asset_name.clone(), bytes);
                            }
                            replaced_src = asset_name;
                        } else {
                            eprintln!("Warning: Failed to decode base64 image payload");
                        }
                    }
                } else if !src.starts_with("http://") && !src.starts_with("https://") {
                    let resolved_path = if let Some(parent) = self.output_path.parent() {
                        parent.join(src)
                    } else {
                        std::path::PathBuf::from(src)
                    };

                    if resolved_path.exists() {
                        match std::fs::read(&resolved_path) {
                            Ok(bytes) => {
                                let counter =
                                    counter_cell.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                                let asset_name = format!("local_img_{}", counter);
                                if let Ok(mut b) = bundle_cell.lock() {
                                    b.add_image(asset_name.clone(), bytes);
                                }
                                replaced_src = asset_name;
                            }
                            Err(e) => {
                                eprintln!(
                                    "Warning: Failed to read local image {}: {}",
                                    resolved_path.display(),
                                    e
                                );
                            }
                        }
                    } else {
                        eprintln!(
                            "Warning: Local image not found at {}",
                            resolved_path.display()
                        );
                    }
                }

                format!("{}{}{}", prefix, replaced_src, suffix)
            })
            .into_owned();

        let bundle = match std::sync::Arc::try_unwrap(bundle_cell) {
            Ok(mutex) => match mutex.into_inner() {
                Ok(b) => b,
                Err(_) => return Err(PdfError::Other("Mutex poisoned".to_string())),
            },
            Err(_) => {
                return Err(PdfError::Other(
                    "Arc still has multiple references".to_string(),
                ));
            }
        };

        let engine = fulgur::Engine::builder().assets(bundle).build();
        let pdf_data = engine
            .render(&new_styled_html)
            .map_err(|e| PdfError::Other(format!("Failed to render pdf: {:?}", e)))?;

        if pdf_data.is_empty() || !pdf_data.starts_with(b"%PDF-") {
            return Err(PdfError::Other(
                "Fulgur rendered invalid or empty PDF bytes".to_string(),
            ));
        }

        std::fs::write(&self.output_path, pdf_data)
            .map_err(|e| PdfError::Other(format!("Failed to write pdf file: {}", e)))?;

        Ok(())
    }
}

pub struct MarkdownToPdfOperation {
    pub input_md: String,
    pub output_path: PathBuf,
    pub options: HtmlToPdfOptions,
}

impl MarkdownToPdfOperation {
    pub fn new(input_md: String, output_path: PathBuf, options: HtmlToPdfOptions) -> Self {
        Self {
            input_md,
            output_path,
            options,
        }
    }

    pub fn render(&self) -> OperationResult<()> {
        use pulldown_cmark::{Parser, html};
        let parser = Parser::new(&self.input_md);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let full_html = format!(
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body>{}</body></html>",
            html_output
        );

        let html_op =
            HtmlToPdfOperation::new(full_html, self.output_path.clone(), self.options.clone());
        html_op.render()
    }
}

pub struct ExcelToStyledHtmlOperation {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub options: HtmlToPdfOptions,
}

impl ExcelToStyledHtmlOperation {
    pub fn new(input_path: PathBuf, output_path: PathBuf, options: HtmlToPdfOptions) -> Self {
        Self {
            input_path,
            output_path,
            options,
        }
    }

    pub fn render(&self) -> OperationResult<()> {
        use calamine::{Data, Reader, open_workbook_auto};

        let mut html =
            String::from("<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body>");

        if self.input_path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let mut rdr = csv::Reader::from_path(&self.input_path)
                .map_err(|e| PdfError::Other(format!("Failed to parse CSV: {}", e)))?;

            html.push_str("<table>");
            if let Ok(headers) = rdr.headers() {
                html.push_str("<tr>");
                for h in headers {
                    html.push_str(&format!("<th>{}</th>", html_escape::encode_text(h)));
                }
                html.push_str("</tr>");
            }

            for record in rdr.records().flatten() {
                html.push_str("<tr>");
                for field in record.iter() {
                    html.push_str(&format!("<td>{}</td>", html_escape::encode_text(field)));
                }
                html.push_str("</tr>");
            }
            html.push_str("</table>");
        } else {
            let mut workbook = open_workbook_auto(&self.input_path)
                .map_err(|e| PdfError::Other(format!("Failed to open workbook: {}", e)))?;

            let sheets = workbook.sheet_names().to_owned();
            for sheet_name in sheets {
                html.push_str(&format!(
                    "<h2>{}</h2>",
                    html_escape::encode_text(&sheet_name)
                ));
                if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                    html.push_str("<table>");
                    let mut is_first_row = true;
                    for row in range.rows() {
                        html.push_str("<tr>");
                        for cell in row {
                            let cell_tag = if is_first_row { "th" } else { "td" };
                            let cell_val = match cell {
                                Data::String(s) => s.to_string(),
                                Data::Float(f) => f.to_string(),
                                Data::Int(i) => i.to_string(),
                                Data::Bool(b) => b.to_string(),
                                Data::Error(e) => format!("Error: {:?}", e),
                                Data::Empty => String::new(),
                                Data::DateTime(d) => d.as_f64().to_string(),
                                Data::DateTimeIso(d) => d.to_string(),
                                Data::DurationIso(d) => d.to_string(),
                            };
                            html.push_str(&format!(
                                "<{}>{}</{}>",
                                cell_tag,
                                html_escape::encode_text(&cell_val),
                                cell_tag
                            ));
                        }
                        html.push_str("</tr>");
                        is_first_row = false;
                    }
                    html.push_str("</table>");
                }
            }
        }
        html.push_str("</body></html>");

        let html_op = HtmlToPdfOperation::new(html, self.output_path.clone(), self.options.clone());
        html_op.render()
    }
}

use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Ctm([f32; 6]);
impl Ctm {
    fn transform(&self, m: [f32; 6]) -> Self {
        Self([
            self.0[0] * m[0] + self.0[1] * m[2],
            self.0[0] * m[1] + self.0[1] * m[3],
            self.0[2] * m[0] + self.0[3] * m[2],
            self.0[2] * m[1] + self.0[3] * m[3],
            self.0[4] * m[0] + self.0[5] * m[2] + m[4],
            self.0[4] * m[1] + self.0[5] * m[3] + m[5],
        ])
    }
    fn point(&self, x: f32, y: f32) -> (f32, f32) {
        (
            x * self.0[0] + y * self.0[2] + self.0[4],
            x * self.0[1] + y * self.0[3] + self.0[5],
        )
    }
}

fn extract_page_media(
    lopdf_doc: &mut lopdf::Document,
    page_id: (u32, u16),
    original_bytes: &[u8],
    page_index: usize,
) -> OperationResult<Vec<(String, String, f32, f32)>> {
    let mut extracted = Vec::new();

    let page_dict = match lopdf_doc.get_object(page_id).and_then(|obj| obj.as_dict()) {
        Ok(dict) => dict,
        Err(_) => return Ok(extracted),
    };

    let resources_dict = match page_dict.get(b"Resources").and_then(|res| match res {
        lopdf::Object::Reference(res_id) => {
            lopdf_doc.get_object(*res_id).and_then(|obj| obj.as_dict())
        }
        lopdf::Object::Dictionary(dict) => Ok(dict),
        _ => Err(lopdf::Error::DictKey("Invalid resources".to_string())),
    }) {
        Ok(dict) => dict,
        Err(_) => return Ok(extracted),
    };

    let xobject_dict = match resources_dict.get(b"XObject").and_then(|xobj| match xobj {
        lopdf::Object::Reference(xobj_id) => {
            lopdf_doc.get_object(*xobj_id).and_then(|obj| obj.as_dict())
        }
        lopdf::Object::Dictionary(dict) => Ok(dict),
        _ => Err(lopdf::Error::DictKey("Invalid XObject".to_string())),
    }) {
        Ok(dict) => dict,
        Err(_) => return Ok(extracted),
    };

    let mut has_forms = false;
    let mut form_xobjects = Vec::new();

    for (name, obj) in xobject_dict.iter() {
        let stream_id = match obj.as_reference() {
            Ok(id) => id,
            Err(_) => continue,
        };

        let stream = match lopdf_doc
            .get_object(stream_id)
            .and_then(|obj| obj.as_stream())
        {
            Ok(stream) => stream,
            Err(_) => continue,
        };

        let subtype = match stream.dict.get(b"Subtype").and_then(|name| name.as_name()) {
            Ok(name) => name,
            Err(_) => continue,
        };

        let name_str = String::from_utf8_lossy(name).to_string();

        if subtype == b"Form" {
            has_forms = true;
            if let Ok(bbox_obj) = stream.dict.get(b"BBox").and_then(|o| o.as_array())
                && bbox_obj.len() == 4
            {
                let mut bbox = [0.0; 4];
                for i in 0..4 {
                    match bbox_obj[i] {
                        lopdf::Object::Real(f) => bbox[i] = f as f64,
                        lopdf::Object::Integer(i_val) => bbox[i] = i_val as f64,
                        _ => {}
                    }
                }
                form_xobjects.push((name_str, bbox));
            }
        }
    }

    if has_forms && let Ok(hayro_pdf) = hayro_syntax::Pdf::new(original_bytes.to_vec()) {
        let pages = hayro_pdf.pages();
        if let Some(page) = pages.get(page_index) {
            let cache = hayro::RenderCache::new();
            let render_settings = hayro::RenderSettings::default();
            let mut pixmap_settings = hayro::PixmapSettings::default();

            let dpi = 150.0;
            let scale = dpi / 72.0;
            pixmap_settings.x_scale = scale;
            pixmap_settings.y_scale = scale;

            let pixmap = hayro::render(
                page,
                &cache,
                &Default::default(),
                &render_settings,
                &pixmap_settings,
            );

            let width = pixmap.width();
            let height = pixmap.height();

            let data = bytemuck::cast_slice(pixmap.data());

            if let Some(mut skia_pixmap) = tiny_skia::Pixmap::new(width as u32, height as u32) {
                skia_pixmap.data_mut().copy_from_slice(data);

                let mut media_h = 0.0;
                if let Ok(media_box) = page_dict.get(b"MediaBox").and_then(|o| o.as_array())
                    && media_box.len() == 4
                {
                    let y0 = match media_box[1] {
                        lopdf::Object::Real(f) => f,
                        lopdf::Object::Integer(i) => i as f32,
                        _ => 0.0,
                    };
                    let y1 = match media_box[3] {
                        lopdf::Object::Real(f) => f,
                        lopdf::Object::Integer(i) => i as f32,
                        _ => 0.0,
                    };
                    media_h = (y1 - y0).abs();
                }

                let mut on_page_bboxes = HashMap::new();

                let content_bytes = lopdf_doc.get_page_content(page_id);
                if let Ok(content) = lopdf::content::Content::decode(&content_bytes) {
                    let mut ctm_stack = vec![Ctm([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])];

                    for op in content.operations {
                        match op.operator.as_str() {
                            "q" => {
                                let current = *ctm_stack.last().unwrap();
                                ctm_stack.push(current);
                            }
                            "Q" => {
                                if ctm_stack.len() > 1 {
                                    ctm_stack.pop();
                                }
                            }
                            "cm" => {
                                if op.operands.len() == 6 {
                                    let get_f32 = |obj: &lopdf::Object| -> f32 {
                                        match obj {
                                            lopdf::Object::Real(f) => *f,
                                            lopdf::Object::Integer(i) => *i as f32,
                                            _ => 0.0,
                                        }
                                    };
                                    let m = [
                                        get_f32(&op.operands[0]),
                                        get_f32(&op.operands[1]),
                                        get_f32(&op.operands[2]),
                                        get_f32(&op.operands[3]),
                                        get_f32(&op.operands[4]),
                                        get_f32(&op.operands[5]),
                                    ];
                                    let current = *ctm_stack.last().unwrap();
                                    let last = ctm_stack.last_mut().unwrap();
                                    *last = current.transform(m);
                                }
                            }
                            "Do" => {
                                if op.operands.len() == 1
                                    && let Ok(name) = op.operands[0].as_name()
                                {
                                    let name_str = String::from_utf8_lossy(name).to_string();
                                    let current = *ctm_stack.last().unwrap();
                                    on_page_bboxes.insert(name_str, current);
                                }
                            }
                            _ => {}
                        }
                    }
                }

                for (form_name, form_bbox) in form_xobjects {
                    if let Some(ctm) = on_page_bboxes.get(&form_name) {
                        let (x0, y0) = ctm.point(form_bbox[0] as f32, form_bbox[1] as f32);
                        let (x1, y1) = ctm.point(form_bbox[2] as f32, form_bbox[3] as f32);

                        let orig_y0 = y0.min(y1);
                        let orig_y1 = y0.max(y1);
                        let px_x0 = x0.min(x1);
                        let px_x1 = x0.max(x1);

                        let px_y0 = media_h - orig_y1;
                        let px_y1 = media_h - orig_y0;

                        let left = (px_x0 * scale).floor() as u32;
                        let top = (px_y0 * scale).floor() as u32;
                        let right = (px_x1 * scale).ceil() as u32;
                        let bottom = (px_y1 * scale).ceil() as u32;

                        let crop_rect = tiny_skia::IntRect::from_ltrb(
                            left.min(width as u32) as i32,
                            top.min(height as u32) as i32,
                            right.min(width as u32) as i32,
                            bottom.min(height as u32) as i32,
                        );

                        if let Some(rect) = crop_rect
                            && let Some(cropped) = skia_pixmap.clone_rect(rect)
                            && let Ok(png_bytes) = cropped.encode_png()
                        {
                            use base64::{Engine as _, engine::general_purpose};
                            let b64 = general_purpose::STANDARD.encode(&png_bytes);
                            extracted.push((
                                form_name,
                                format!("data:image/png;base64,{}", b64),
                                orig_y0,
                                orig_y1,
                            ));
                        }
                    }
                }
            }
        }
    }

    Ok(extracted)
}

#[derive(Default)]
pub struct PdfToHtmlOperation {
    pub extracted_html: Arc<Mutex<Option<String>>>,
}

impl PdfToHtmlOperation {
    pub fn new() -> Self {
        Self {
            extracted_html: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToHtmlOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let mut html = String::new();

        if let Some(bytes) = lopdf_doc.original_bytes.as_deref()
            && !bytes.is_empty()
            && let Ok(mut pdf) = pdf_oxide::api::Pdf::from_bytes(bytes.to_vec())
            && let Ok(page_count) = pdf.page_count()
        {
            let mut page_nums: Vec<u32> = lopdf_doc.inner.get_pages().keys().copied().collect();
            page_nums.sort();

            html.push_str(
                "<!DOCTYPE html>\n<html>\n<head><title>PDF Export</title></head>\n<body>\n",
            );
            for i in 0..page_count {
                let page_id = i + 1;

                let obj_id = if i < page_nums.len() {
                    let pid = page_nums[i];
                    *lopdf_doc.inner.get_pages().get(&pid).unwrap()
                } else {
                    (0, 0)
                };

                let page_media = if obj_id.0 != 0 {
                    extract_page_media(&mut lopdf_doc.inner, obj_id, bytes, i).unwrap_or_default()
                } else {
                    Vec::new()
                };

                html.push_str(&format!("<div class=\"page\" id=\"page-{}\">\n", page_id));
                html.push_str(&format!("<h2>Page {}</h2>\n", page_id));

                let mut page_content = String::new();
                if let Ok(page_html) = pdf.to_html(i) {
                    page_content = page_html;
                }

                let mut all_images: Vec<(String, String, f32, f32)> = page_media;
                if let Ok(pdf_images) = pdf.extract_images(i) {
                    for (img_idx, img) in pdf_images.iter().enumerate() {
                        if let Some(bbox) = img.bbox() {
                            let (ext, b64) = match img.data() {
                                pdf_oxide::extractors::ImageData::Jpeg(jpeg_data) => {
                                    use base64::{Engine as _, engine::general_purpose};
                                    ("jpeg", general_purpose::STANDARD.encode(jpeg_data))
                                }
                                pdf_oxide::extractors::ImageData::Raw { pixels, format } => {
                                    let color = match format {
                                        pdf_oxide::extractors::PixelFormat::RGB => {
                                            image::ColorType::Rgb8
                                        }
                                        pdf_oxide::extractors::PixelFormat::Grayscale => {
                                            image::ColorType::L8
                                        }
                                        pdf_oxide::extractors::PixelFormat::CMYK => {
                                            image::ColorType::Rgb8
                                        } // Fallback mapping, Image API handles basic conversions or fails gracefully.
                                    };
                                    let mut png_bytes = Vec::new();
                                    use image::ImageEncoder;
                                    if image::codecs::png::PngEncoder::new(&mut png_bytes)
                                        .write_image(
                                            pixels,
                                            img.width(),
                                            img.height(),
                                            color.into(),
                                        )
                                        .is_ok()
                                    {
                                        use base64::{Engine as _, engine::general_purpose};
                                        ("png", general_purpose::STANDARD.encode(&png_bytes))
                                    } else {
                                        ("png", String::new())
                                    }
                                }
                            };
                            if !b64.is_empty() {
                                all_images.push((
                                    format!("Image_{}", img_idx),
                                    format!("data:image/{};base64,{}", ext, b64),
                                    bbox.y,
                                    bbox.y + bbox.height,
                                ));
                            }
                        }
                    }
                }

                let spans = pdf.extract_spans(i).unwrap_or_default();
                let mut image_insertions: Vec<(usize, String)> = Vec::new();

                for (name, b64, _, y1) in all_images {
                    let markup = format!(
                        "<img src=\"{}\" alt=\"Illustration {}\" style=\"max-width: 100%; border-radius: 4px; margin-top: 10px;\" />\n",
                        b64, name
                    );
                    let mut closest_span = None;
                    let mut min_dist = f32::MAX;

                    for span in &spans {
                        let span_y0 = span.bbox.y;
                        let dist = span_y0 - y1;
                        if dist >= -5.0 && dist < min_dist {
                            min_dist = dist;
                            closest_span = Some(span);
                        }
                    }

                    if let Some(span) = closest_span {
                        if let Some(pos) = page_content.find(&span.text) {
                            let mut insert_pos = pos + span.text.len();
                            while insert_pos < page_content.len()
                                && page_content.as_bytes()[insert_pos] != b'\n'
                            {
                                insert_pos += 1;
                            }
                            image_insertions.push((insert_pos, markup));
                        } else {
                            image_insertions.push((page_content.len(), markup));
                        }
                    } else {
                        image_insertions.push((0, markup));
                    }
                }

                image_insertions.sort_by(|a, b| b.0.cmp(&a.0));
                let mut final_html = page_content;
                for (pos, markup) in image_insertions {
                    let insert_pos = if final_html.is_char_boundary(pos) {
                        pos
                    } else {
                        let mut p = pos;
                        while p > 0 && !final_html.is_char_boundary(p) {
                            p -= 1;
                        }
                        p
                    };
                    final_html.insert_str(insert_pos, &format!("\n{}\n", markup));
                }

                html.push_str(&final_html);

                html.push_str("</div>\n");
            }
            html.push_str("</body>\n</html>");
        }

        if html.is_empty() {
            let inner = &mut lopdf_doc.inner;
            let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

            html = String::from(
                "<!DOCTYPE html>\n<html>\n<head><title>PDF Export</title></head>\n<body>\n",
            );

            for page_id in all_pages {
                match inner.extract_text(&[page_id]) {
                    Ok(text) => {
                        html.push_str(&format!("<div class=\"page\" id=\"page-{}\">\n", page_id));
                        html.push_str(&format!("<h2>Page {}</h2>\n", page_id));
                        for line in text.lines() {
                            if !line.trim().is_empty() {
                                html.push_str(&format!("<p>{}</p>\n", line.trim()));
                            }
                        }
                        html.push_str("</div>\n");
                    }
                    Err(e) => {
                        return Err(PdfError::Other(format!(
                            "Failed to extract text from page {}: {}",
                            page_id, e
                        )));
                    }
                }
            }

            html.push_str("</body>\n</html>");
        }

        if let Ok(mut lock) = self.extracted_html.lock() {
            *lock = Some(html);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct PdfToMarkdownOperation {
    pub extracted_markdown: Arc<Mutex<Option<String>>>,
}

impl PdfToMarkdownOperation {
    pub fn new() -> Self {
        Self {
            extracted_markdown: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToMarkdownOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let mut md = String::new();

        if let Some(bytes) = lopdf_doc.original_bytes.as_deref()
            && !bytes.is_empty()
            && let Ok(mut pdf) = pdf_oxide::api::Pdf::from_bytes(bytes.to_vec())
            && let Ok(page_count) = pdf.page_count()
        {
            let mut page_nums: Vec<u32> = lopdf_doc.inner.get_pages().keys().copied().collect();
            page_nums.sort();

            for i in 0..page_count {
                let page_id = i + 1;

                let obj_id = if i < page_nums.len() {
                    let pid = page_nums[i];
                    *lopdf_doc.inner.get_pages().get(&pid).unwrap()
                } else {
                    (0, 0)
                };

                let page_media = if obj_id.0 != 0 {
                    extract_page_media(&mut lopdf_doc.inner, obj_id, bytes, i).unwrap_or_default()
                } else {
                    Vec::new()
                };

                let mut all_images: Vec<(String, String, f32, f32)> = page_media;
                if let Ok(pdf_images) = pdf.extract_images(i) {
                    for (img_idx, img) in pdf_images.iter().enumerate() {
                        if let Some(bbox) = img.bbox() {
                            let (ext, b64) = match img.data() {
                                pdf_oxide::extractors::ImageData::Jpeg(jpeg_data) => {
                                    use base64::{Engine as _, engine::general_purpose};
                                    ("jpeg", general_purpose::STANDARD.encode(jpeg_data))
                                }
                                pdf_oxide::extractors::ImageData::Raw { pixels, format } => {
                                    let color = match format {
                                        pdf_oxide::extractors::PixelFormat::RGB => {
                                            image::ColorType::Rgb8
                                        }
                                        pdf_oxide::extractors::PixelFormat::Grayscale => {
                                            image::ColorType::L8
                                        }
                                        pdf_oxide::extractors::PixelFormat::CMYK => {
                                            image::ColorType::Rgb8
                                        }
                                    };
                                    let mut png_bytes = Vec::new();
                                    use image::ImageEncoder;
                                    if image::codecs::png::PngEncoder::new(&mut png_bytes)
                                        .write_image(
                                            pixels,
                                            img.width(),
                                            img.height(),
                                            color.into(),
                                        )
                                        .is_ok()
                                    {
                                        use base64::{Engine as _, engine::general_purpose};
                                        ("png", general_purpose::STANDARD.encode(&png_bytes))
                                    } else {
                                        ("png", String::new())
                                    }
                                }
                            };
                            if !b64.is_empty() {
                                all_images.push((
                                    format!("Image_{}", img_idx),
                                    format!("data:image/{};base64,{}", ext, b64),
                                    bbox.y,
                                    bbox.y + bbox.height,
                                ));
                            }
                        }
                    }
                }

                if let Ok(page_md) = pdf.to_markdown(i) {
                    md.push_str(&format!("# Page {}\n\n", page_id));

                    let spans = pdf.extract_spans(i).unwrap_or_default();
                    let mut image_insertions: Vec<(usize, String)> = Vec::new();

                    for (name, b64, _, y1) in all_images {
                        let markup = format!("![Illustration {}]({})", name, b64);
                        let mut closest_span = None;
                        let mut min_dist = f32::MAX;

                        for span in &spans {
                            let span_y0 = span.bbox.y;
                            let dist = span_y0 - y1;
                            if dist >= -5.0 && dist < min_dist {
                                min_dist = dist;
                                closest_span = Some(span);
                            }
                        }

                        if let Some(span) = closest_span {
                            if let Some(pos) = page_md.find(&span.text) {
                                let mut insert_pos = pos + span.text.len();
                                while insert_pos < page_md.len()
                                    && page_md.as_bytes()[insert_pos] != b'\n'
                                {
                                    insert_pos += 1;
                                }
                                image_insertions.push((insert_pos, markup));
                            } else {
                                image_insertions.push((page_md.len(), markup));
                            }
                        } else {
                            image_insertions.push((0, markup));
                        }
                    }

                    image_insertions.sort_by(|a, b| b.0.cmp(&a.0));
                    let mut final_md = page_md;
                    for (pos, markup) in image_insertions {
                        let insert_pos = if final_md.is_char_boundary(pos) {
                            pos
                        } else {
                            let mut p = pos;
                            while p > 0 && !final_md.is_char_boundary(p) {
                                p -= 1;
                            }
                            p
                        };
                        final_md.insert_str(insert_pos, &format!("\n\n{}\n\n", markup));
                    }

                    md.push_str(final_md.trim());
                    md.push_str("\n\n");
                }
            }
        }

        if md.is_empty() {
            let inner = &mut lopdf_doc.inner;
            let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

            for page_id in all_pages {
                match inner.extract_text(&[page_id]) {
                    Ok(text) => {
                        md.push_str(&format!("# Page {}\n\n", page_id));
                        md.push_str(text.trim());
                        md.push_str("\n\n");
                    }
                    Err(e) => {
                        return Err(PdfError::Other(format!(
                            "Failed to extract text from page {}: {}",
                            page_id, e
                        )));
                    }
                }
            }
        }

        if let Ok(mut lock) = self.extracted_markdown.lock() {
            *lock = Some(md);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageMetadata {
    pub page_number: u32,
    pub char_count: usize,
    pub word_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmExportMetadata {
    pub total_pages: usize,
    pub total_chars: usize,
    pub total_words: usize,
    pub pages: Vec<PageMetadata>,
}

#[derive(Default)]
pub struct PdfToLlmExportOperation {
    pub extracted_markdown: Arc<Mutex<Option<String>>>,
    pub metadata: Arc<Mutex<Option<LlmExportMetadata>>>,
}

impl PdfToLlmExportOperation {
    pub fn new() -> Self {
        Self {
            extracted_markdown: Arc::new(Mutex::new(None)),
            metadata: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToLlmExportOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut md = String::new();
        let mut pages_meta = Vec::new();
        let mut total_chars = 0;
        let mut total_words = 0;

        for page_id in &all_pages {
            match inner.extract_text(&[*page_id]) {
                Ok(text) => {
                    let trimmed = text.trim();
                    let chars = trimmed.chars().count();
                    let words = trimmed.split_whitespace().count();

                    pages_meta.push(PageMetadata {
                        page_number: *page_id,
                        char_count: chars,
                        word_count: words,
                    });

                    total_chars += chars;
                    total_words += words;

                    md.push_str(&format!("## Page {}\n\n", page_id));
                    md.push_str(trimmed);
                    md.push_str("\n\n---\n\n");
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        let meta = LlmExportMetadata {
            total_pages: all_pages.len(),
            total_chars,
            total_words,
            pages: pages_meta,
        };

        if let Ok(mut lock) = self.extracted_markdown.lock() {
            *lock = Some(md);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        if let Ok(mut lock) = self.metadata.lock() {
            *lock = Some(meta);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonImage {
    pub id: String,
    pub page_number: u32,
    pub format: String, // "png" or "jpeg"
    pub width: u32,
    pub height: u32,
    pub bbox: Option<[f32; 4]>, // [x0, y0, x1, y1] coordinates on page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64_data: Option<String>, // "data:image/png;base64,..." or raw base64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonPage {
    pub page_number: u32,
    pub text: String,
    #[serde(default)]
    pub char_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<JsonImage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonDocument {
    pub document_name: String,
    pub pages: Vec<JsonPage>,
}

pub struct PdfToJsonOperation {
    pub document_name: String,
    pub extracted_json: Arc<Mutex<Option<String>>>,
}

impl PdfToJsonOperation {
    pub fn new(document_name: String) -> Self {
        Self {
            document_name,
            extracted_json: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToJsonOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut pages_data = Vec::new();

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    let chars = text.chars().count();
                    pages_data.push(JsonPage {
                        page_number: page_id,
                        text,
                        char_count: chars,
                        images: vec![], // Add an empty vector as pdf_to_json doesn't extract images yet in this mock or if we're not touching its impl here
                    });
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        let doc_struct = JsonDocument {
            document_name: self.document_name.clone(),
            pages: pages_data,
        };

        let json_str = serde_json::to_string_pretty(&doc_struct)
            .map_err(|e| PdfError::Other(format!("Failed to serialize JSON: {}", e)))?;

        if let Ok(mut lock) = self.extracted_json.lock() {
            *lock = Some(json_str);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

pub struct JsonToPdfOperation {
    pub json_content: String,
}

impl JsonToPdfOperation {
    pub fn new(json_content: String) -> Self {
        Self { json_content }
    }
}

impl PdfOperation for JsonToPdfOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let doc_struct: JsonDocument = serde_json::from_str(&self.json_content)
            .map_err(|e| PdfError::Other(format!("Failed to parse JSON: {}", e)))?;

        let mut inner = lopdf::Document::with_version("1.5");

        let font_id = inner.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });

        let pages_id = inner.new_object_id();
        let mut page_ids = vec![];

        for json_page in doc_struct.pages {
            let mut ops = vec![
                lopdf::content::Operation::new("BT", vec![]),
                lopdf::content::Operation::new("Tf", vec!["F1".into(), 12.into()]),
                lopdf::content::Operation::new("Td", vec![50.into(), 750.into()]),
            ];

            for line in json_page.text.lines() {
                ops.push(lopdf::content::Operation::new(
                    "Tj",
                    vec![lopdf::Object::string_literal(line)],
                ));
                ops.push(lopdf::content::Operation::new(
                    "Td",
                    vec![0.into(), (-15).into()],
                ));
            }

            ops.push(lopdf::content::Operation::new("ET", vec![]));

            let mut resources = dictionary! {
                "Font" => dictionary! {
                    "F1" => font_id,
                },
            };

            let mut xobjects = lopdf::Dictionary::new();

            // Handle images if any
            for (idx, img) in json_page.images.iter().enumerate() {
                let img_name = format!("Im{}", idx);
                let (width, height, image_bytes) = if let Some(b64) = &img.base64_data {
                    let b64_str = if b64.starts_with("data:image/") {
                        b64.split(',').nth(1).unwrap_or(b64)
                    } else {
                        b64
                    };

                    use base64::{Engine as _, engine::general_purpose};
                    let bytes = general_purpose::STANDARD
                        .decode(b64_str)
                        .map_err(|e| PdfError::Other(format!("Failed to decode base64: {}", e)))?;

                    let img_reader = image::load_from_memory(&bytes)
                        .map_err(|e| PdfError::Other(format!("Failed to parse image: {}", e)))?;

                    let img_rgb = img_reader.to_rgb8();
                    let (w, h) = img_rgb.dimensions();
                    (w, h, img_rgb.into_raw())
                } else if let Some(path) = &img.file_path {
                    let img_reader = image::open(path).map_err(|e| {
                        PdfError::Other(format!("Failed to load image from path: {}", e))
                    })?;
                    let img_rgb = img_reader.to_rgb8();
                    let (w, h) = img_rgb.dimensions();
                    (w, h, img_rgb.into_raw())
                } else {
                    continue;
                };

                let mut compressed = Vec::new();
                {
                    use flate2::Compression;
                    use flate2::write::ZlibEncoder;
                    use std::io::Write;
                    let mut encoder = ZlibEncoder::new(&mut compressed, Compression::default());
                    encoder
                        .write_all(&image_bytes)
                        .map_err(|e: std::io::Error| PdfError::Other(e.to_string()))?;
                }

                let image_dict = dictionary! {
                    "Type" => "XObject",
                    "Subtype" => "Image",
                    "Width" => width as i64,
                    "Height" => height as i64,
                    "ColorSpace" => "DeviceRGB",
                    "BitsPerComponent" => 8,
                    "Filter" => "FlateDecode",
                };

                let img_stream = lopdf::Stream::new(image_dict, compressed);
                let img_id = inner.add_object(img_stream);

                xobjects.set(img_name.clone(), img_id);

                if let Some(bbox) = img.bbox {
                    let (x0, y0, x1, y1) = (bbox[0], bbox[1], bbox[2], bbox[3]);
                    let w = x1 - x0;
                    let h = y1 - y0;

                    ops.push(lopdf::content::Operation::new("q", vec![]));
                    ops.push(lopdf::content::Operation::new(
                        "cm",
                        vec![w.into(), 0.into(), 0.into(), h.into(), x0.into(), y0.into()],
                    ));
                    ops.push(lopdf::content::Operation::new("Do", vec![img_name.into()]));
                    ops.push(lopdf::content::Operation::new("Q", vec![]));
                }
            }

            if !xobjects.is_empty() {
                resources.set("XObject", xobjects);
            }

            let content_stream = lopdf::content::Content { operations: ops };
            let content_id = inner.add_object(lopdf::Stream::new(
                dictionary! {},
                content_stream
                    .encode()
                    .map_err(|e| PdfError::Other(e.to_string()))?,
            ));

            let page_dict = dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
                "Contents" => content_id,
                "Resources" => resources,
            };

            let page_id = inner.add_object(page_dict);
            page_ids.push(page_id);
        }

        let pages_dict = dictionary! {
            "Type" => "Pages",
            "Kids" => page_ids.clone().into_iter().map(lopdf::Object::Reference).collect::<Vec<_>>(),
            "Count" => page_ids.len() as i32,
        };
        inner
            .objects
            .insert(pages_id, lopdf::Object::Dictionary(pages_dict));

        let catalog_id = inner.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        inner.trailer.set("Root", catalog_id);

        lopdf_doc.inner = inner;

        Ok(())
    }
}

pub struct PdfToDocxOperation {
    pub output_path: std::path::PathBuf,
}

impl PdfToDocxOperation {
    pub fn new(output_path: std::path::PathBuf) -> Self {
        Self { output_path }
    }
}

#[derive(Debug, Clone)]
pub struct TextSpan {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    pub is_bold: bool,
    pub is_italic: bool,
    pub color_hex: String,
}

#[allow(dead_code)]
fn sanitize_xml_text(s: &str) -> String {
    s.chars()
        .filter(|&c| {
            c == '\t'
                || c == '\n'
                || c == '\r'
                || ('\u{0020}'..='\u{D7FF}').contains(&c)
                || ('\u{E000}'..='\u{FFFD}').contains(&c)
        })
        .collect()
}

pub fn sanitize_font_name(raw_name: &str) -> String {
    // 1. Strip 6-char random subset prefix e.g. "JFGNNE+SourceSansPro-Bold" -> "SourceSansPro-Bold"
    let cleaned = if let Some((prefix, rest)) = raw_name.split_once('+') {
        if prefix.len() == 6 && prefix.chars().all(|c| c.is_ascii_uppercase()) {
            rest
        } else {
            raw_name
        }
    } else {
        raw_name
    };

    // 2. Separate font family from weight suffix
    let family = cleaned
        .trim_end_matches("-Bold")
        .trim_end_matches("-Regular")
        .trim_end_matches("-Italic")
        .trim_end_matches("-BoldItalic")
        .trim_end_matches(",Bold")
        .trim_end_matches(",Italic");

    // 3. Insert spaces between CamelCase words if needed (e.g., "SourceSansPro" -> "Source Sans Pro")
    let mut normalized = String::new();
    let chars: Vec<char> = family.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
        if i > 0 && ch.is_ascii_uppercase() && chars[i - 1].is_ascii_lowercase() {
            normalized.push(' ');
        }
        normalized.push(ch);
    }

    if normalized.is_empty() || (normalized.starts_with('F') && normalized.len() <= 3) {
        "Arial".to_string()
    } else {
        normalized
    }
}

#[derive(Debug, Clone)]
pub enum LayoutItem {
    Paragraph(Vec<TextSpan>),
    Table(Vec<Vec<Vec<TextSpan>>>),
}

/// EMU scaling constants for OpenXML DrawingML
pub const EMUS_PER_INCH: u32 = 914_400;
pub const MAX_DOCX_PAGE_WIDTH_EMUS: u32 = 4_500_000;

pub fn decode_data_uri(uri: &str) -> Option<(String, Vec<u8>)> {
    if !uri.starts_with("data:image/") {
        return None;
    }
    let parts: Vec<&str> = uri.splitn(2, ',').collect();
    if parts.len() != 2 {
        return None;
    }
    let mime = parts[0]
        .strip_prefix("data:image/")?
        .strip_suffix(";base64")?;
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    let bytes = STANDARD.decode(parts[1]).ok()?;
    Some((mime.to_string(), bytes))
}

impl PdfOperation for PdfToDocxOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let _lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        use docx_rs::{
            AlignmentType, Docx, Paragraph, Run, Table, TableBorders, TableCell, TableRow,
            WidthType,
        };
        use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

        let mut docx = Docx::new().page_margin(
            docx_rs::PageMargin::new()
                .top(720)
                .bottom(720)
                .left(720)
                .right(720),
        );

        let markdown_op = PdfToMarkdownOperation::new();
        markdown_op.execute(document)?;

        let markdown = {
            let guard = markdown_op.extracted_markdown.lock().unwrap();
            guard.clone().unwrap_or_default()
        };

        let parser = Parser::new_ext(&markdown, Options::all());

        let mut current_heading_level = 0;
        let mut current_paragraph = Paragraph::new();
        let mut has_run = false;

        let mut pending_image: Option<(Vec<u8>, u32, u32)> = None;
        let mut pending_image_paragraph: Option<Paragraph> = None;
        let mut current_card_title: Option<Paragraph> = None;

        let mut table_rows: Vec<TableRow> = Vec::new();
        let mut current_row_cells: Vec<TableCell> = Vec::new();
        let mut current_cell_paragraphs: Vec<Paragraph> = Vec::new();

        for event in parser {
            match event {
                Event::Start(Tag::Image { dest_url, .. }) => {
                    if let Some((_, bytes)) = decode_data_uri(&dest_url)
                        && let Ok(img) = image::load_from_memory(&bytes)
                    {
                        let width_px = img.width() as f64;
                        let height_px = img.height() as f64;
                        // 150 DPI => 6096 EMUs per pixel
                        let scale = 914_400.0 / 150.0;
                        let mut width_emu = (width_px * scale) as u32;
                        let mut height_emu = (height_px * scale) as u32;

                        if width_emu > MAX_DOCX_PAGE_WIDTH_EMUS {
                            let ratio = MAX_DOCX_PAGE_WIDTH_EMUS as f64 / width_emu as f64;
                            width_emu = MAX_DOCX_PAGE_WIDTH_EMUS;
                            height_emu = (height_emu as f64 * ratio) as u32;
                        }
                        pending_image = Some((bytes, width_emu, height_emu));
                    }
                }
                Event::End(TagEnd::Image) => {
                    if let Some((bytes, width, height)) = pending_image.take() {
                        use docx_rs::Pic;
                        let pic = Pic::new(&bytes).size(width, height);
                        let pic_run = Run::new().add_image(pic);
                        let pic_para = Paragraph::new()
                            .align(AlignmentType::Center)
                            .add_run(pic_run);
                        pending_image_paragraph = Some(pic_para);
                    }
                }
                Event::Start(Tag::Heading { level, .. }) => {
                    let level_num = match level {
                        pulldown_cmark::HeadingLevel::H1 => 1,
                        pulldown_cmark::HeadingLevel::H2 => 2,
                        pulldown_cmark::HeadingLevel::H3 => 3,
                        pulldown_cmark::HeadingLevel::H4 => 4,
                        pulldown_cmark::HeadingLevel::H5 => 5,
                        pulldown_cmark::HeadingLevel::H6 => 6,
                    };
                    current_heading_level = level_num;
                    current_paragraph = Paragraph::new();
                    if current_heading_level == 1 || current_heading_level == 3 {
                        current_paragraph = current_paragraph.align(AlignmentType::Center);
                    }
                    has_run = false;
                }
                Event::End(TagEnd::Heading(_)) => {
                    if has_run {
                        if current_heading_level == 3 || current_heading_level == 4 {
                            current_card_title = Some(current_paragraph.clone());
                        } else {
                            if let Some(pic_para) = pending_image_paragraph.take() {
                                docx = docx.add_paragraph(pic_para);
                            }
                            docx = docx.add_paragraph(current_paragraph.clone());
                        }
                    }
                    current_heading_level = 0;
                    current_paragraph = Paragraph::new();
                    has_run = false;
                }
                Event::Start(Tag::Paragraph) => {
                    current_paragraph = Paragraph::new();
                    has_run = false;
                }
                Event::End(TagEnd::Paragraph) => {
                    if has_run {
                        if let Some(title_para) = current_card_title.take() {
                            if let Some(illus_para) = pending_image_paragraph.take() {
                                let illus_cell = TableCell::new()
                                    .add_paragraph(illus_para)
                                    .width(3168, WidthType::Dxa);

                                let text_cell = TableCell::new()
                                    .add_paragraph(title_para)
                                    .add_paragraph(current_paragraph.clone())
                                    .width(6912, WidthType::Dxa);

                                let row_cells = vec![illus_cell, text_cell];

                                let table = Table::new(vec![TableRow::new(row_cells)])
                                    .set_borders(TableBorders::new().clear_all())
                                    .layout(docx_rs::TableLayoutType::Fixed);
                                docx = docx.add_table(table);
                            } else {
                                docx = docx.add_paragraph(title_para);
                                docx = docx.add_paragraph(current_paragraph.clone());
                            }
                        } else {
                            if let Some(pic_para) = pending_image_paragraph.take() {
                                docx = docx.add_paragraph(pic_para);
                            }
                            docx = docx.add_paragraph(current_paragraph.clone());
                        }
                    }
                    current_paragraph = Paragraph::new();
                    has_run = false;
                }
                Event::Text(text) => {
                    let mut run = Run::new().add_text(text.into_string());

                    if current_heading_level == 1 {
                        run = run.bold().size(44).color("1F242E");
                    } else if current_heading_level == 3 || current_heading_level == 4 {
                        run = run.bold().size(24).color("1F242E");
                    } else {
                        run = run.size(19).color("525C68");
                    }

                    current_paragraph = current_paragraph.add_run(run);
                    has_run = true;
                }
                Event::Start(Tag::Table(_)) => {
                    table_rows = Vec::new();
                }
                Event::End(TagEnd::Table) => {
                    if !table_rows.is_empty() {
                        let table = Table::new(table_rows.clone());
                        docx = docx.add_table(table);
                    }
                    table_rows = Vec::new();
                }
                Event::Start(Tag::TableRow) => {
                    current_row_cells = Vec::new();
                }
                Event::End(TagEnd::TableRow) => {
                    if !current_row_cells.is_empty() {
                        table_rows.push(TableRow::new(current_row_cells.clone()));
                    }
                    current_row_cells = Vec::new();
                }
                Event::Start(Tag::TableCell) => {
                    current_cell_paragraphs = Vec::new();
                    current_paragraph = Paragraph::new();
                    has_run = false;
                }
                Event::End(TagEnd::TableCell) => {
                    if has_run {
                        current_cell_paragraphs.push(current_paragraph.clone());
                    }
                    let mut cell = TableCell::new();
                    for p in current_cell_paragraphs.clone() {
                        cell = cell.add_paragraph(p);
                    }
                    current_row_cells.push(cell);
                    current_cell_paragraphs = Vec::new();
                    current_paragraph = Paragraph::new();
                    has_run = false;
                }
                _ => {}
            }
        }

        // flush
        if let Some(pic_para) = pending_image_paragraph.take() {
            docx = docx.add_paragraph(pic_para);
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
        let mut all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();
        all_pages.sort_unstable();

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        let mut current_row: u32 = 0;
        let ws_regex = regex::Regex::new(r"\s{3,}").unwrap();

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    for line in text.lines() {
                        let line_trimmed = line.trim();
                        if line_trimmed.is_empty() {
                            continue;
                        }

                        let cols: Vec<&str> = ws_regex.split(line_trimmed).collect();

                        for (col_idx, val) in cols.iter().enumerate() {
                            worksheet
                                .write_string(current_row, col_idx as u16, *val)
                                .map_err(|e| {
                                    PdfError::Other(format!("Failed to write XLSX cell: {}", e))
                                })?;
                        }
                        current_row += 1;
                    }
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        workbook
            .save(&self.output_path)
            .map_err(|e| PdfError::Other(format!("Failed to save XLSX: {}", e)))?;

        Ok(())
    }
}

pub struct PdfToPptxOperation {
    pub output_path: std::path::PathBuf,
}

impl PdfToPptxOperation {
    pub fn new(output_path: std::path::PathBuf) -> Self {
        Self { output_path }
    }
}

impl PdfOperation for PdfToPptxOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let mut all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();
        all_pages.sort_unstable();

        let file = std::fs::File::create(&self.output_path)
            .map_err(|e| PdfError::Other(format!("Failed to create PPTX file: {}", e)))?;
        let mut zip = ZipWriter::new(file);

        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        for (i, page_id) in all_pages.iter().enumerate() {
            let slide_name = format!("ppt/slides/slide{}.xml", i + 1);
            zip.start_file(&slide_name, options)
                .map_err(|e| PdfError::Other(format!("Failed to start ZIP file: {}", e)))?;

            let text = inner.extract_text(&[*page_id]).unwrap_or_default();
            let slide_xml = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
    <p:cSld>
        <p:spTree>
            <p:sp>
                <p:txBody>
                    <a:p>
                        <a:r>
                            <a:t>{}</a:t>
                        </a:r>
                    </a:p>
                </p:txBody>
            </p:sp>
        </p:spTree>
    </p:cSld>
</p:sld>"#,
                text.replace("&", "&amp;")
                    .replace("<", "&lt;")
                    .replace(">", "&gt;")
            );

            zip.write_all(slide_xml.as_bytes())
                .map_err(|e| PdfError::Other(format!("Failed to write slide XML: {}", e)))?;
        }

        zip.start_file("[Content_Types].xml", options)
            .map_err(|e| PdfError::Other(format!("Failed to start ZIP file: {}", e)))?;
        let content_types = r#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
    <Default Extension="xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
</Types>"#;
        zip.write_all(content_types.as_bytes())
            .map_err(|e| PdfError::Other(format!("Failed to write content types: {}", e)))?;

        zip.finish()
            .map_err(|e| PdfError::Other(format!("Failed to finish ZIP file: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Document, Object};

    fn create_dummy_doc(pages: u32, content_str: Option<&str>) -> LopdfDocument {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut page_ids = vec![];

        for _ in 0..pages {
            let mut page_dict = Dictionary::new();
            page_dict.set("Type", Object::Name(b"Page".to_vec()));
            page_dict.set("Parent", Object::Reference(pages_id));

            let content = content_str.unwrap_or("BT /F1 12 Tf 0 0 Td (Test Content) Tj ET");
            let content_stream = lopdf::Stream::new(Dictionary::new(), content.as_bytes().to_vec());
            let content_id = doc.add_object(Object::Stream(content_stream));
            page_dict.set("Contents", Object::Reference(content_id));

            let mut resources_dict = Dictionary::new();
            let mut font_dict = Dictionary::new();
            let mut f1_dict = Dictionary::new();
            f1_dict.set("Type", Object::Name(b"Font".to_vec()));
            f1_dict.set("Subtype", Object::Name(b"Type1".to_vec()));
            f1_dict.set("BaseFont", Object::Name(b"Helvetica".to_vec()));
            let f1_id = doc.add_object(Object::Dictionary(f1_dict));
            font_dict.set("F1", Object::Reference(f1_id));
            resources_dict.set("Font", Object::Dictionary(font_dict));

            page_dict.set("Resources", Object::Dictionary(resources_dict));

            let page_id = doc.add_object(page_dict);
            page_ids.push(Object::Reference(page_id));
        }

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(pages as i64));
        pages_dict.set("Kids", Object::Array(page_ids));

        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
        doc.trailer
            .set("Root", Object::Dictionary(Dictionary::new()));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();
        LopdfDocument {
            inner: doc,
            original_bytes: Some(bytes),
        }
    }

    #[test]
    fn test_css_preset_resolution() {
        let css = resolve_css(Some("elegant"), None);
        assert!(css.contains("Georgia"));
        assert!(css.contains("margin: 25mm"));

        let css2 = resolve_css(Some("github"), None);
        assert!(css2.contains("-apple-system"));
        assert!(css2.contains("margin: 20mm"));

        let css3 = resolve_css(None, None);
        assert!(css3.contains("-apple-system"));
    }

    #[test]
    fn test_custom_css_concatenation() {
        let css = resolve_css(Some("minimal"), Some("body { font-size: 20px; }"));
        assert!(css.contains("margin: 18mm"));
        assert!(css.contains("/* Custom CSS */"));
        assert!(css.contains("body { font-size: 20px; }"));
    }

    #[test]
    fn test_inject_css_into_html() {
        let html = "<!DOCTYPE html><html><head><title>Test</title></head><body><h1>Hello</h1></body></html>";
        let css = "body { color: red; }";
        let injected = inject_css_into_html(html, css);

        assert!(
            injected
                .contains("<style id=\"paperpilot-injected-css\">\nbody { color: red; }\n</style>")
        );
        assert!(injected.contains("</head>"));

        let html_no_head = "<body><h1>Hello</h1></body>";
        let injected_no_head = inject_css_into_html(html_no_head, css);
        assert!(
            injected_no_head
                .starts_with("<!DOCTYPE html><html><head><style id=\"paperpilot-injected-css\">")
        );
        assert!(injected_no_head.contains("<body><body><h1>Hello</h1></body></body></html>"));
    }

    #[test]
    fn test_pdf_to_html() {
        let mut doc = create_dummy_doc(2, None);
        let op = PdfToHtmlOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_html.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("<!DOCTYPE html>"));
        assert!(extracted.contains("<div class=\"page\" id=\"page-1\">"));
    }

    #[test]
    fn test_pdf_to_markdown() {
        let mut doc = create_dummy_doc(2, None);
        let op = PdfToMarkdownOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_markdown.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("# Page 1"));
    }

    #[test]
    fn test_pdf_to_llm_export() {
        let mut doc = create_dummy_doc(1, None);
        let op = PdfToLlmExportOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_markdown.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("## Page 1"));

        let meta = op.metadata.lock().unwrap();
        assert!(meta.is_some());
        let extracted_meta = meta.as_ref().unwrap();
        assert_eq!(extracted_meta.total_pages, 1);
        assert_eq!(extracted_meta.pages.len(), 1);
    }

    #[test]
    fn test_pdf_to_json() {
        let mut doc = create_dummy_doc(1, None);
        let op = PdfToJsonOperation::new("test_doc.pdf".to_string());
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_json.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("\"document_name\": \"test_doc.pdf\""));
    }

    #[test]
    fn test_pdf_to_docx_with_images() {
        use crate::document::LopdfDocument;
        let fixture_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("fixtures")
            .join("Get_Started_With_Smallpdf.pdf");

        if !fixture_path.exists() {
            return;
        }

        let mut doc = LopdfDocument::load(&fixture_path).unwrap();

        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("output.docx");

        let op = PdfToDocxOperation::new(out_path.clone());
        assert!(op.execute(&mut doc).is_ok());
        assert!(out_path.exists());

        let file = std::fs::File::open(&out_path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut image_count = 0;
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            println!("DOCX File: {}", file.name());
        }
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            if file.name().starts_with("word/media/")
                && (file.name().ends_with(".png") || file.name().ends_with(".jpeg"))
            {
                image_count += 1;
                assert!(file.size() > 0);
            }
        }
        assert!(image_count > 0, "No images found in generated DOCX");
    }

    #[test]
    fn test_pdf_to_docx() {
        let mut doc = create_dummy_doc(1, None);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out_test.docx");

        let op = PdfToDocxOperation::new(path.clone());
        assert!(op.execute(&mut doc).is_ok());

        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_sanitize_font_name() {
        assert_eq!(
            super::sanitize_font_name("JFGNNE+SourceSansPro-Bold"),
            "Source Sans Pro"
        );
        assert_eq!(super::sanitize_font_name("Arial-Italic"), "Arial");
        assert_eq!(
            super::sanitize_font_name("TimesNewRoman,Bold"),
            "Times New Roman"
        );
        assert_eq!(super::sanitize_font_name("F1"), "Arial");
        assert_eq!(super::sanitize_font_name("OpenSans"), "Open Sans");
    }

    #[test]
    fn test_pdf_to_xlsx() {
        let mut doc = create_dummy_doc(1, None);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out_test.xlsx");

        let op = PdfToXlsxOperation::new(path.clone());
        assert!(op.execute(&mut doc).is_ok());

        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_pdf_to_pptx() {
        let mut doc = create_dummy_doc(1, None);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out_test.pptx");

        let op = PdfToPptxOperation::new(path.clone());
        assert!(op.execute(&mut doc).is_ok());

        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_excel_to_html_csv() {
        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("test.csv");
        let out_path = dir.path().join("out.pdf");
        std::fs::write(&csv_path, "Name,Age\nJohn,30\nJane,25").unwrap();

        let options = HtmlToPdfOptions {
            preset: None,
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };

        let op = ExcelToStyledHtmlOperation::new(csv_path.clone(), out_path.clone(), options);
        assert!(op.render().is_ok());

        assert!(out_path.exists());
        assert!(out_path.metadata().unwrap().len() > 0);
        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_html_to_pdf_with_base64_image() {
        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("out_html_img.pdf");
        let options = HtmlToPdfOptions {
            preset: Some("minimal".to_string()),
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };

        let html = r#"
            <html><body>
            <h1>Test HTML</h1>
            <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==" width="50" height="50">
            </body></html>
        "#.to_string();

        let op = HtmlToPdfOperation::new(html, out_path.clone(), options);
        let res = op.render();

        assert!(res.is_ok());
        assert!(out_path.exists());
        assert!(out_path.metadata().unwrap().len() > 0);

        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_md_to_pdf_with_base64_image() {
        let dir = tempfile::tempdir().unwrap();
        let md = "![Pixel](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==)".to_string();

        let out_path = dir.path().join("out_md_img.pdf");
        let options = HtmlToPdfOptions {
            preset: Some("github".to_string()),
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };
        let op = MarkdownToPdfOperation::new(md, out_path.clone(), options);
        let res = op.render();
        assert!(res.is_ok());
        assert!(out_path.exists());

        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_html_to_pdf_graceful_missing_image() {
        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("out_html_missing_img.pdf");
        let options = HtmlToPdfOptions {
            preset: Some("minimal".to_string()),
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };

        let html = r#"
            <html><body>
            <h1>Test Graceful HTML</h1>
            <img src="data:image/png;base64,INVALIDBASE64$$$" width="50">
            <img src="path/does/not/exist.png">
            </body></html>
        "#
        .to_string();

        let op = HtmlToPdfOperation::new(html, out_path.clone(), options);
        let res = op.render();

        assert!(res.is_ok());
        assert!(out_path.exists());
        assert!(out_path.metadata().unwrap().len() > 0);

        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_html_to_pdf_operation_fulgur() {
        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("out_fulgur.pdf");
        let options = HtmlToPdfOptions {
            preset: Some("minimal".to_string()),
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };

        let html =
            "<html><body><h1>Test HTML</h1><p>Rendering via fulgur.</p></body></html>".to_string();
        let op = HtmlToPdfOperation::new(html, out_path.clone(), options);
        let res = op.render();

        assert!(res.is_ok());
        assert!(out_path.exists());
        assert!(out_path.metadata().unwrap().len() > 0);

        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_markdown_to_pdf_operation_fulgur() {
        let dir = tempfile::tempdir().unwrap();

        let md = "# Test Markdown\n\nRendering via **fulgur**.".to_string();
        std::fs::write(dir.path().join("in.md"), md).unwrap();
        let out_path = dir.path().join("out_md_fulgur.pdf");
        let options = HtmlToPdfOptions {
            preset: Some("github".to_string()),
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };
        let op = MarkdownToPdfOperation::new(
            dir.path().join("in.md").to_string_lossy().to_string(),
            out_path.clone(),
            options,
        );
        let res = op.render();
        assert!(res.is_ok());
        assert!(out_path.exists());

        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_json_to_pdf_operation_text() {
        use crate::document::LopdfDocument;
        use paperpilot_core::traits::PdfOperation;
        let mut doc = LopdfDocument::new();
        let json = r#"{
            "document_name": "test.pdf",
            "pages": [
                {
                    "page_number": 1,
                    "text": "Hello World",
                    "char_count": 11,
                    "images": []
                }
            ]
        }"#;
        let op = super::JsonToPdfOperation::new(json.to_string());
        assert!(op.execute(&mut doc).is_ok());
        let mut bytes = Vec::new();
        doc.inner.save_to(&mut bytes).unwrap();
        assert!(!bytes.is_empty());
        assert!(bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_fidelity_round_trip() {
        use crate::document::LopdfDocument;
        use paperpilot_core::traits::PdfOperation;
        let mut doc = LopdfDocument::new();

        let initial_json = r#"{
            "document_name": "test_rt.pdf",
            "pages": [
                {
                    "page_number": 1,
                    "text": "Round trip testing",
                    "char_count": 18,
                    "images": []
                }
            ]
        }"#;

        let op_synth = super::JsonToPdfOperation::new(initial_json.to_string());
        assert!(op_synth.execute(&mut doc).is_ok());

        let op_extract = super::PdfToJsonOperation::new("extracted_rt.pdf".to_string());
        assert!(op_extract.execute(&mut doc).is_ok());

        let extracted_str = op_extract.extracted_json.lock().unwrap().clone().unwrap();
        let extracted: super::JsonDocument = serde_json::from_str(&extracted_str).unwrap();

        assert_eq!(extracted.pages.len(), 1);
        assert!(extracted.pages[0].text.contains("Round trip testing"));
    }

    #[test]
    fn test_json_to_pdf_operation_image() {
        use crate::document::LopdfDocument;
        use paperpilot_core::traits::PdfOperation;
        let mut doc = LopdfDocument::new();

        let b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
        let json = format!(
            r#"{{
            "document_name": "test.pdf",
            "pages": [
                {{
                    "page_number": 1,
                    "text": "Image test",
                    "char_count": 10,
                    "images": [
                        {{
                            "id": "img1",
                            "page_number": 1,
                            "format": "png",
                            "width": 1,
                            "height": 1,
                            "bbox": [10.0, 10.0, 100.0, 100.0],
                            "base64_data": "data:image/png;base64,{}"
                        }}
                    ]
                }}
            ]
        }}"#,
            b64
        );

        let op = super::JsonToPdfOperation::new(json);
        assert!(op.execute(&mut doc).is_ok());
        let mut bytes = Vec::new();
        doc.inner.save_to(&mut bytes).unwrap();
        assert!(!bytes.is_empty());
        assert!(bytes.starts_with(b"%PDF-"));
    }
}
