//! Shared AcroForm filling and appearance flattening. No filesystem or network IO.

use lopdf::{Dictionary, Document, Object, ObjectId, Stream, StringFormat, dictionary};
use paperpilot_core::error::{OperationResult, PdfError};
use std::collections::{HashMap, HashSet};

fn invalid(message: impl Into<String>) -> PdfError {
    PdfError::InvalidInput(message.into())
}

fn pdf_error(error: lopdf::Error) -> PdfError {
    PdfError::ParseError(error.to_string())
}

fn dict<'a>(doc: &'a Document, object: &'a Object) -> OperationResult<&'a Dictionary> {
    doc.dereference(object)
        .and_then(|(_, o)| o.as_dict())
        .map_err(pdf_error)
}

fn array<'a>(doc: &'a Document, object: &'a Object) -> OperationResult<&'a Vec<Object>> {
    doc.dereference(object)
        .and_then(|(_, o)| o.as_array())
        .map_err(pdf_error)
}

pub(super) fn text(object: &Object) -> String {
    match object {
        Object::Name(bytes) => String::from_utf8_lossy(bytes).into_owned(),
        // lopdf's PDFDocEncoding lookup drops ASCII line breaks. Preserve the
        // literal whitespace in ordinary text values (notably multiline CSV).
        Object::String(bytes, _)
            if bytes
                .iter()
                .all(|byte| byte.is_ascii_graphic() || byte.is_ascii_whitespace()) =>
        {
            String::from_utf8_lossy(bytes).into_owned()
        }
        _ => lopdf::decode_text_string(object).unwrap_or_default(),
    }
}

#[derive(Clone)]
struct Field {
    name: String,
    id: ObjectId,
    kind: Vec<u8>,
    flags: i64,
    widgets: Vec<ObjectId>,
}

fn fields(doc: &Document) -> OperationResult<Vec<Field>> {
    fn visit(
        doc: &Document,
        id: ObjectId,
        prefix: &str,
        kind: &[u8],
        flags: i64,
        active: &mut HashSet<ObjectId>,
        output: &mut Vec<Field>,
    ) -> OperationResult<()> {
        if active.len() >= 64 || !active.insert(id) {
            return Err(invalid("Cyclic or excessively nested form field tree"));
        }
        let field = doc.get_dictionary(id).map_err(pdf_error)?;
        let part = field.get(b"T").map(text).unwrap_or_default();
        let name = if prefix.is_empty() {
            part
        } else if part.is_empty() {
            prefix.to_string()
        } else {
            format!("{prefix}.{part}")
        };
        let kind = field.get(b"FT").and_then(Object::as_name).unwrap_or(kind);
        let flags = field.get(b"Ff").and_then(Object::as_i64).unwrap_or(flags);
        let mut widgets = Vec::new();
        let mut children = Vec::new();
        if let Ok(kids) = field.get(b"Kids") {
            for kid in array(doc, kids)? {
                let child_id = kid.as_reference().map_err(pdf_error)?;
                let child = doc.get_dictionary(child_id).map_err(pdf_error)?;
                if child.has(b"T") || child.has(b"Kids") {
                    children.push(child_id);
                } else {
                    widgets.push(child_id);
                }
            }
        }
        if field.get(b"Subtype").and_then(Object::as_name).ok() == Some(b"Widget") {
            widgets.push(id);
        }
        if children.is_empty() && !name.is_empty() {
            output.push(Field {
                name,
                id,
                kind: kind.to_vec(),
                flags,
                widgets,
            });
        } else {
            for child in children {
                visit(doc, child, &name, kind, flags, active, output)?;
            }
        }
        active.remove(&id);
        Ok(())
    }
    let catalog = doc.catalog().map_err(pdf_error)?;
    let Ok(acroform) = catalog.get(b"AcroForm") else {
        return Ok(Vec::new());
    };
    let acroform = dict(doc, acroform)?;
    if acroform.has(b"XFA") {
        return Err(invalid(
            "XFA forms are not supported; use an AcroForm template",
        ));
    }
    let mut output = Vec::new();
    for field in array(doc, acroform.get(b"Fields").map_err(pdf_error)?)? {
        visit(
            doc,
            field.as_reference().map_err(pdf_error)?,
            "",
            b"Tx",
            0,
            &mut HashSet::new(),
            &mut output,
        )?;
    }
    let mut names = HashSet::new();
    if output.iter().any(|field| !names.insert(field.name.clone())) {
        return Err(invalid("Duplicate fully qualified form field names"));
    }
    Ok(output)
}

pub fn read_fields(doc: &Document) -> OperationResult<HashMap<String, String>> {
    fields(doc)?
        .into_iter()
        .map(|field| {
            let dictionary = doc.get_dictionary(field.id).map_err(pdf_error)?;
            let value = dictionary
                .get(b"V")
                .or_else(|_| dictionary.get(b"AS"))
                .map(text)
                .unwrap_or_default();
            Ok((field.name, value))
        })
        .collect()
}

/// Parse the template once; each record gets a fresh, independent document clone.
pub struct FormFillTemplate {
    document: Document,
    fields: Vec<Field>,
}

impl FormFillTemplate {
    pub fn from_bytes(bytes: &[u8]) -> OperationResult<Self> {
        let document = Document::load_mem(bytes).map_err(pdf_error)?;
        if document.is_encrypted() {
            return Err(invalid("Decrypt the form template before batch filling"));
        }
        let fields = fields(&document)?;
        if fields.is_empty() {
            return Err(invalid("Template contains no AcroForm fields"));
        }
        let mut attached = HashSet::new();
        for (_, page_id) in document.get_pages() {
            if let Ok(annotations) = document
                .get_dictionary(page_id)
                .map_err(pdf_error)?
                .get(b"Annots")
            {
                for annotation in array(&document, annotations)? {
                    if let Ok(id) = annotation.as_reference() {
                        attached.insert(id);
                    }
                }
            }
        }
        for field in &fields {
            if field.flags & 1 != 0
                || !matches!(field.kind.as_slice(), b"Tx" | b"Btn")
                || (field.kind == b"Btn" && field.flags & ((1 << 15) | (1 << 16)) != 0)
            {
                return Err(invalid(format!(
                    "Unsupported or read-only field: {} (text and checkboxes only)",
                    field.name
                )));
            }
            if field.widgets.is_empty() {
                return Err(invalid(format!("Field {} has no page widget", field.name)));
            }
            for widget in &field.widgets {
                if !attached.contains(widget) {
                    return Err(invalid(format!(
                        "Field {} has an unattached widget",
                        field.name
                    )));
                }
                rectangle(
                    document
                        .get_dictionary(*widget)
                        .map_err(pdf_error)?
                        .get(b"Rect")
                        .map_err(pdf_error)?,
                )?;
            }
        }
        Ok(Self { document, fields })
    }

    pub fn field_names(&self) -> Vec<&str> {
        self.fields
            .iter()
            .map(|field| field.name.as_str())
            .collect()
    }

    pub fn fill(
        &self,
        values: &HashMap<String, String>,
        flatten: bool,
    ) -> OperationResult<Vec<u8>> {
        for field in &self.fields {
            if !values.contains_key(&field.name) {
                return Err(invalid(format!("Missing field: {}", field.name)));
            }
        }
        if values
            .keys()
            .any(|name| !self.fields.iter().any(|field| &field.name == name))
        {
            return Err(invalid("Values contain an unknown form field"));
        }
        let mut document = self.document.clone();
        fill_fields(&mut document, values)?;
        if flatten {
            flatten_widgets(&mut document)?;
        }
        let mut output = Vec::new();
        document.save_to(&mut output).map_err(PdfError::IoError)?;
        Ok(output)
    }
}

pub struct FillFormConfig {
    pub values: HashMap<String, String>,
    pub flatten: bool,
}

pub fn run_operation_bytes(bytes: &[u8], config: &FillFormConfig) -> OperationResult<Vec<u8>> {
    FormFillTemplate::from_bytes(bytes)?.fill(&config.values, config.flatten)
}

pub(super) fn fill_fields(
    doc: &mut Document,
    values: &HashMap<String, String>,
) -> OperationResult<()> {
    fill_fields_impl(doc, values, false)
}

fn fill_fields_impl(
    doc: &mut Document,
    values: &HashMap<String, String>,
    allow_readonly: bool,
) -> OperationResult<()> {
    let bindings = fields(doc)?;
    let mut fonts = HashMap::new();
    for field in &bindings {
        let Some(value) = values.get(&field.name) else {
            continue;
        };
        if field.flags & 1 != 0 && !allow_readonly {
            return Err(invalid(format!("Field {} is read-only", field.name)));
        }
        if !matches!(field.kind.as_slice(), b"Tx" | b"Btn")
            || (field.kind == b"Btn" && field.flags & ((1 << 15) | (1 << 16)) != 0)
        {
            return Err(invalid(format!("Unsupported field type: {}", field.name)));
        }
        let on = if field.kind == b"Btn" {
            field
                .widgets
                .iter()
                .find_map(|id| {
                    let widget = doc.get_dictionary(*id).ok()?;
                    let ap = dict(doc, widget.get(b"AP").ok()?).ok()?;
                    let states = dict(doc, ap.get(b"N").ok()?).ok()?;
                    states
                        .iter()
                        .find(|(name, _)| name.as_slice() != b"Off")
                        .map(|(name, _)| String::from_utf8_lossy(name).into_owned())
                })
                .unwrap_or_else(|| "Yes".into())
        } else {
            String::new()
        };
        let state = if field.kind == b"Btn" {
            let off = matches!(
                value.to_ascii_lowercase().as_str(),
                "" | "false" | "0" | "off" | "no"
            );
            if off {
                "Off".to_string()
            } else {
                if value != &on
                    && !matches!(
                        value.to_ascii_lowercase().as_str(),
                        "true" | "1" | "yes" | "on"
                    )
                {
                    return Err(invalid(format!(
                        "Invalid checkbox value for {}",
                        field.name
                    )));
                }
                on.clone()
            }
        } else {
            String::new()
        };
        doc.get_object_mut(field.id)
            .and_then(Object::as_dict_mut)
            .map_err(pdf_error)?
            .set(
                "V",
                if field.kind == b"Btn" {
                    Object::Name(state.as_bytes().to_vec())
                } else {
                    lopdf::text_string(value)
                },
            );
        for widget in &field.widgets {
            let bounds = doc
                .get_dictionary(*widget)
                .map_err(pdf_error)?
                .get(b"Rect")
                .cloned()
                .map_err(pdf_error)?;
            let [x0, y0, x1, y1] = rectangle(&bounds)?;
            let normal = if field.kind == b"Btn" {
                let off = checkbox_appearance(doc, x1 - x0, y1 - y0, false)?;
                let checked = checkbox_appearance(doc, x1 - x0, y1 - y0, true)?;
                let mut states = dictionary! { "Off" => off };
                states.set(on.as_str(), checked);
                Object::Dictionary(states)
            } else {
                Object::Reference(text_appearance(
                    doc,
                    &mut fonts,
                    value,
                    x1 - x0,
                    y1 - y0,
                    field.flags & (1 << 12) != 0,
                )?)
            };
            let dictionary = doc
                .get_object_mut(*widget)
                .and_then(Object::as_dict_mut)
                .map_err(pdf_error)?;
            dictionary.set("AP", dictionary! { "N" => normal });
            if field.kind == b"Btn" {
                dictionary.set("AS", Object::Name(state.as_bytes().to_vec()));
            }
        }
    }
    let acroform = doc
        .catalog()
        .map_err(pdf_error)?
        .get(b"AcroForm")
        .ok()
        .cloned();
    if let Some(Object::Reference(id)) = acroform {
        doc.get_object_mut(id)
            .and_then(Object::as_dict_mut)
            .map_err(pdf_error)?
            .set("NeedAppearances", false);
    } else if matches!(acroform, Some(Object::Dictionary(_))) {
        let root = doc
            .trailer
            .get(b"Root")
            .and_then(Object::as_reference)
            .map_err(pdf_error)?;
        doc.get_object_mut(root)
            .and_then(Object::as_dict_mut)
            .map_err(pdf_error)?
            .get_mut(b"AcroForm")
            .and_then(Object::as_dict_mut)
            .map_err(pdf_error)?
            .set("NeedAppearances", false);
    }
    Ok(())
}

fn rectangle(object: &Object) -> OperationResult<[f32; 4]> {
    let items = object.as_array().map_err(pdf_error)?;
    if items.len() != 4 {
        return Err(invalid("Widget rectangle must contain four coordinates"));
    }
    let mut values = [0.0; 4];
    for (i, item) in items.iter().enumerate() {
        values[i] = item.as_float().map_err(pdf_error)?;
    }
    if values.iter().any(|value| !value.is_finite())
        || values[2] <= values[0]
        || values[3] <= values[1]
    {
        return Err(invalid("Widget rectangle must have positive dimensions"));
    }
    Ok(values)
}

fn font(doc: &mut Document, unicode: bool) -> ObjectId {
    if !unicode {
        return doc.add_object(
            dictionary! { "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica" },
        );
    }
    // Standard Adobe-GB1 CJK font, with an explicit extraction map. Requires no
    // downloaded fonts; viewers provide their standard CJK fallback face.
    let descriptor = doc.add_object(dictionary! {
        "Type" => "FontDescriptor", "FontName" => "STSong-Light", "Flags" => 6,
        "FontBBox" => vec![0.into(), (-200).into(), 1000.into(), 900.into()],
        "Ascent" => 880, "Descent" => -120, "CapHeight" => 700, "StemV" => 80,
        "ItalicAngle" => 0,
    });
    let cid = doc.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "CIDFontType0", "BaseFont" => "STSong-Light",
        "CIDSystemInfo" => dictionary! { "Registry" => Object::string_literal("Adobe"), "Ordering" => Object::string_literal("GB1"), "Supplement" => 4 },
        "FontDescriptor" => descriptor, "DW" => 1000,
    });
    let cmap = b"/CIDInit /ProcSet findresource begin 12 dict begin begincmap /CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def /CMapName /PaperPilotUnicode def /CMapType 2 def 1 begincodespacerange <0000> <FFFF> endcodespacerange 1 beginbfrange <0000> <FFFF> <0000> endbfrange endcmap CMapName currentdict /CMap defineresource pop end end";
    let unicode_map = doc.add_object(Stream::new(Dictionary::new(), cmap.to_vec()));
    doc.add_object(dictionary! { "Type" => "Font", "Subtype" => "Type0", "BaseFont" => "STSong-Light", "Encoding" => "UniGB-UCS2-H", "DescendantFonts" => vec![cid.into()], "ToUnicode" => unicode_map })
}

fn appearance_stream(
    doc: &mut Document,
    width: f32,
    height: f32,
    resources: Dictionary,
    bytes: Vec<u8>,
) -> ObjectId {
    doc.add_object(Stream::new(dictionary! {
        "Type" => "XObject", "Subtype" => "Form", "FormType" => 1,
        "BBox" => vec![0.into(), 0.into(), width.into(), height.into()], "Resources" => resources,
    }, bytes))
}

fn text_appearance(
    doc: &mut Document,
    fonts: &mut HashMap<bool, ObjectId>,
    value: &str,
    width: f32,
    height: f32,
    multiline: bool,
) -> OperationResult<ObjectId> {
    use lopdf::content::{Content, Operation};
    if value
        .chars()
        .any(|c| c as u32 > 0xFFFF || (c.is_control() && !matches!(c, '\n' | '\r' | '\t')))
    {
        return Err(invalid(
            "Form appearances support printable BMP text; unsupported character in value",
        ));
    }
    let normalized = value
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\t', "    ");
    if !multiline && normalized.contains('\n') {
        return Err(invalid("Newline in a single-line form field"));
    }
    let lines: Vec<_> = normalized.split('\n').collect();
    let unicode = !value.is_ascii();
    let font_id = *fonts.entry(unicode).or_insert_with(|| font(doc, unicode));
    let max_width = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|_| if unicode { 1.0 } else { 0.6 })
                .sum::<f32>()
        })
        .fold(1.0, f32::max);
    let size = 12.0_f32
        .min((width - 4.0) / max_width)
        .min((height - 4.0) / (lines.len() as f32 * 1.2));
    if size < 4.0 {
        return Err(invalid(
            "Value is too long to fit the form field at readable size",
        ));
    }
    let mut operations = vec![
        Operation::new("q", vec![]),
        Operation::new("re", vec![0.into(), 0.into(), width.into(), height.into()]),
        Operation::new("W", vec![]),
        Operation::new("n", vec![]),
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec![Object::Name(b"PPF".to_vec()), size.into()]),
        Operation::new("g", vec![0.into()]),
    ];
    for (index, line) in lines.iter().enumerate() {
        let bytes = if unicode {
            line.encode_utf16().flat_map(u16::to_be_bytes).collect()
        } else {
            line.as_bytes().to_vec()
        };
        operations.push(Operation::new(
            "Tm",
            vec![
                1.into(),
                0.into(),
                0.into(),
                1.into(),
                2.into(),
                (height - 2.0 - size - index as f32 * size * 1.2).into(),
            ],
        ));
        operations.push(Operation::new(
            "Tj",
            vec![Object::String(
                bytes,
                if unicode {
                    StringFormat::Hexadecimal
                } else {
                    StringFormat::Literal
                },
            )],
        ));
    }
    operations.extend([Operation::new("ET", vec![]), Operation::new("Q", vec![])]);
    let bytes = Content { operations }.encode().map_err(pdf_error)?;
    Ok(appearance_stream(
        doc,
        width,
        height,
        dictionary! { "Font" => dictionary! { "PPF" => font_id } },
        bytes,
    ))
}

fn checkbox_appearance(
    doc: &mut Document,
    width: f32,
    height: f32,
    checked: bool,
) -> OperationResult<ObjectId> {
    let mut content = format!("q 0 G 1 w 1 1 {} {} re S\n", width - 2.0, height - 2.0);
    if checked {
        content += &format!(
            "2 2 m {} {} l S 2 {} m {} 2 l S\n",
            width - 2.0,
            height - 2.0,
            height - 2.0,
            width - 2.0
        );
    }
    content += "Q\n";
    Ok(appearance_stream(
        doc,
        width,
        height,
        Dictionary::new(),
        content.into_bytes(),
    ))
}

pub(super) fn flatten_widgets(doc: &mut Document) -> OperationResult<()> {
    let mut missing = HashMap::new();
    for field in fields(doc)? {
        if field.widgets.iter().any(|id| {
            doc.get_dictionary(*id)
                .ok()
                .and_then(|widget| widget.get(b"AP").ok())
                .and_then(|ap| dict(doc, ap).ok())
                .is_none_or(|ap| !ap.has(b"N"))
        }) {
            let value = doc
                .get_dictionary(field.id)
                .map_err(pdf_error)?
                .get(b"V")
                .map(text)
                .unwrap_or_default();
            missing.insert(field.name, value);
        }
    }
    fill_fields_impl(doc, &missing, true)?;
    for (_, page_id) in doc.get_pages() {
        let page = doc.get_dictionary(page_id).map_err(pdf_error)?.clone();
        let Ok(annots) = page.get(b"Annots") else {
            continue;
        };
        let annotations = array(doc, annots)?.clone();
        let mut retained = Vec::new();
        let mut drawings = String::new();
        let mut changed = false;
        let mut resources = Dictionary::new();
        let mut current = page_id;
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(current) || seen.len() > 64 {
                return Err(invalid("Cyclic page resource inheritance"));
            }
            let dictionary = doc.get_dictionary(current).map_err(pdf_error)?;
            if let Ok(value) = dictionary.get(b"Resources") {
                resources = dict(doc, value)?.clone();
                break;
            }
            let Ok(parent) = dictionary.get(b"Parent").and_then(Object::as_reference) else {
                break;
            };
            current = parent;
        }
        let mut xobjects = resources
            .get(b"XObject")
            .ok()
            .map(|o| dict(doc, o).cloned())
            .transpose()?
            .unwrap_or_default();
        for annotation in annotations {
            let widget = dict(doc, &annotation)?.clone();
            if widget.get(b"Subtype").and_then(Object::as_name).ok() != Some(b"Widget") {
                retained.push(annotation);
                continue;
            }
            changed = true;
            let flags = widget.get(b"F").and_then(Object::as_i64).unwrap_or(0);
            if flags & (1 | 2 | 32) != 0 {
                continue;
            }
            let normal = dict(doc, widget.get(b"AP").map_err(pdf_error)?)?
                .get(b"N")
                .cloned()
                .map_err(pdf_error)?;
            let (_, normal) = doc.dereference(&normal).map_err(pdf_error)?;
            let appearance = if let Object::Dictionary(states) = normal {
                let state = widget
                    .get(b"AS")
                    .and_then(Object::as_name)
                    .unwrap_or(b"Off");
                states.get(state).cloned().map_err(pdf_error)?
            } else {
                normal.clone()
            };
            let (appearance_id, appearance_object) =
                doc.dereference(&appearance).map_err(pdf_error)?;
            let stream = appearance_object.as_stream().map_err(pdf_error)?.clone();
            let [x0, y0, x1, y1] = rectangle(widget.get(b"Rect").map_err(pdf_error)?)?;
            let bbox = rectangle(stream.dict.get(b"BBox").map_err(pdf_error)?)?;
            let matrix = stream.dict.get(b"Matrix").and_then(Object::as_array).ok();
            let mut transform = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
            if let Some(matrix) = matrix {
                if matrix.len() != 6 {
                    return Err(invalid("Invalid appearance matrix"));
                }
                for (i, item) in matrix.iter().enumerate() {
                    transform[i] = item.as_float().map_err(pdf_error)?;
                }
            }
            if transform.iter().any(|value| !value.is_finite()) {
                return Err(invalid("Non-finite appearance matrix"));
            }
            let [a, b, c, d, e, f] = transform;
            let corners = [
                (bbox[0], bbox[1]),
                (bbox[2], bbox[1]),
                (bbox[0], bbox[3]),
                (bbox[2], bbox[3]),
            ]
            .map(|(x, y)| (a * x + c * y + e, b * x + d * y + f));
            let min_x = corners.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
            let max_x = corners
                .iter()
                .map(|p| p.0)
                .fold(f32::NEG_INFINITY, f32::max);
            let min_y = corners.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
            let max_y = corners
                .iter()
                .map(|p| p.1)
                .fold(f32::NEG_INFINITY, f32::max);
            if max_x <= min_x || max_y <= min_y {
                return Err(invalid("Degenerate appearance matrix"));
            }
            let sx = (x1 - x0) / (max_x - min_x);
            let sy = (y1 - y0) / (max_y - min_y);
            let appearance_id = appearance_id.unwrap_or_else(|| doc.add_object(stream));
            let mut name = format!("PPForm{}", appearance_id.0);
            while xobjects.has(name.as_bytes()) {
                name.push('_');
            }
            xobjects.set(name.as_str(), appearance_id);
            drawings += &format!(
                "q {sx} 0 0 {sy} {} {} cm /{name} Do Q\n",
                x0 - sx * min_x,
                y0 - sy * min_y
            );
        }
        if !drawings.is_empty() {
            let prefix = doc.add_object(Stream::new(Dictionary::new(), b"q\n".to_vec()));
            let suffix = doc.add_object(Stream::new(
                Dictionary::new(),
                format!("Q\n{drawings}").into_bytes(),
            ));
            let mut contents = vec![Object::Reference(prefix)];
            contents.extend(
                doc.get_page_contents(page_id)
                    .into_iter()
                    .map(Object::Reference),
            );
            contents.push(suffix.into());
            resources.set("XObject", xobjects);
            let page = doc
                .get_object_mut(page_id)
                .and_then(Object::as_dict_mut)
                .map_err(pdf_error)?;
            page.set("Contents", contents);
            page.set("Resources", resources);
        }
        if changed {
            let page = doc
                .get_object_mut(page_id)
                .and_then(Object::as_dict_mut)
                .map_err(pdf_error)?;
            if retained.is_empty() {
                page.remove(b"Annots");
            } else {
                page.set("Annots", retained);
            }
        }
    }
    let catalog = doc
        .trailer
        .get(b"Root")
        .and_then(Object::as_reference)
        .map_err(pdf_error)?;
    doc.get_object_mut(catalog)
        .and_then(Object::as_dict_mut)
        .map_err(pdf_error)?
        .remove(b"AcroForm");
    Ok(())
}
