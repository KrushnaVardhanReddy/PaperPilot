#![allow(clippy::collapsible_if)]
use lopdf::{Dictionary, Document, Object, Stream, StringFormat};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentInfo {
    pub name: String,
    pub size_bytes: usize,
    pub mime_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfAttachmentsOptions {
    #[serde(default = "default_action")]
    pub action: String, // "list", "extract", "add", "remove"
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub file_data_base64: Option<String>,
}

fn default_action() -> String {
    "list".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentsReport {
    pub attachments: Vec<AttachmentInfo>,
    pub total_count: usize,
}

pub fn pdf_attachments_bytes(
    input_bytes: &[u8],
    options: &PdfAttachmentsOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Empty PDF document",
        )));
    }
    let mut doc = Document::load_mem(input_bytes).map_err(|e| PdfError::Other(e.to_string()))?;

    match options.action.as_str() {
        "list" => {
            let mut attachments = Vec::new();
            if let Ok(catalog) = doc.catalog() {
                if let Ok(names_id) = catalog.get(b"Names").and_then(|obj| obj.as_reference()) {
                    if let Ok(names_dict) = doc.get_dictionary(names_id) {
                        if let Ok(embedded_files_id) = names_dict
                            .get(b"EmbeddedFiles")
                            .and_then(|obj| obj.as_reference())
                        {
                            if let Ok(embedded_files_dict) = doc.get_dictionary(embedded_files_id) {
                                // Extract from /Names array
                                if let Ok(names_array) = embedded_files_dict
                                    .get(b"Names")
                                    .and_then(|obj| obj.as_array())
                                {
                                    for i in (0..names_array.len()).step_by(2) {
                                        if let Ok(name_str) = names_array[i].as_str() {
                                            if let Ok(file_spec_id) =
                                                names_array[i + 1].as_reference()
                                            {
                                                if let Ok(file_spec) =
                                                    doc.get_dictionary(file_spec_id)
                                                {
                                                    let description = file_spec
                                                        .get(b"Desc")
                                                        .and_then(|obj| obj.as_str())
                                                        .ok()
                                                        .map(|s| {
                                                            String::from_utf8_lossy(s).to_string()
                                                        });
                                                    let mut size = 0;
                                                    let mut mime_type = None;
                                                    if let Ok(ef) = file_spec
                                                        .get(b"EF")
                                                        .and_then(|obj| obj.as_dict())
                                                    {
                                                        if let Ok(stream_id) =
                                                            ef.get(b"F").and_then(|obj: &Object| {
                                                                obj.as_reference()
                                                            })
                                                        {
                                                            if let Ok(stream) = doc
                                                                .get_object(stream_id)
                                                                .and_then(|obj| obj.as_stream())
                                                            {
                                                                size = stream.content.len();
                                                                if let Ok(subtype) = stream
                                                                    .dict
                                                                    .get(b"Subtype")
                                                                    .and_then(|obj| obj.as_name())
                                                                {
                                                                    mime_type = Some(
                                                                        String::from_utf8_lossy(
                                                                            subtype,
                                                                        )
                                                                        .to_string(),
                                                                    );
                                                                }
                                                                if let Ok(params) = stream
                                                                    .dict
                                                                    .get(b"Params")
                                                                    .and_then(|obj| obj.as_dict())
                                                                {
                                                                    if let Ok(sz) = params
                                                                        .get(b"Size")
                                                                        .and_then(|obj: &Object| {
                                                                            obj.as_i64()
                                                                        })
                                                                    {
                                                                        size = sz as usize;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    attachments.push(AttachmentInfo {
                                                        name: String::from_utf8_lossy(name_str)
                                                            .to_string(),
                                                        size_bytes: size,
                                                        mime_type,
                                                        description,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let report = AttachmentsReport {
                total_count: attachments.len(),
                attachments,
            };
            serde_json::to_vec_pretty(&report).map_err(|e| PdfError::Other(e.to_string()))
        }
        "add" => {
            let file_name = options
                .file_name
                .as_ref()
                .ok_or_else(|| PdfError::Other("Missing file_name for add action".to_string()))?;
            let b64_data = options.file_data_base64.as_ref().ok_or_else(|| {
                PdfError::Other("Missing file_data_base64 for add action".to_string())
            })?;

            // decode
            use base64::{Engine, engine::general_purpose::STANDARD as b64_standard};
            let decoded_data = b64_standard
                .decode(b64_data.trim())
                .map_err(|e| PdfError::Other(e.to_string()))?;

            let mut stream_dict = Dictionary::new();
            stream_dict.set("Type", Object::Name(b"EmbeddedFile".to_vec()));
            if let Some(mime) = &options.mime_type {
                stream_dict.set("Subtype", Object::Name(mime.as_bytes().to_vec()));
            }
            let mut params_dict = Dictionary::new();
            params_dict.set("Size", Object::Integer(decoded_data.len() as i64));
            stream_dict.set("Params", Object::Dictionary(params_dict));

            let mut stream = Stream::new(stream_dict, decoded_data);
            let _ = stream.compress();

            let stream_id = doc.add_object(stream);

            let mut file_spec = Dictionary::new();
            file_spec.set("Type", Object::Name(b"Filespec".to_vec()));
            file_spec.set(
                "F",
                Object::String(file_name.as_bytes().to_vec(), StringFormat::Literal),
            );
            file_spec.set(
                "UF",
                Object::String(file_name.as_bytes().to_vec(), StringFormat::Literal),
            );
            if let Some(desc) = &options.description {
                file_spec.set(
                    "Desc",
                    Object::String(desc.as_bytes().to_vec(), StringFormat::Literal),
                );
            }
            let mut ef_dict = Dictionary::new();
            ef_dict.set("F", Object::Reference(stream_id));
            file_spec.set("EF", Object::Dictionary(ef_dict));

            let file_spec_id = doc.add_object(file_spec);

            // Register in catalog
            let mut embedded_files_names: Vec<Object> = vec![];

            // Try to load existing
            let mut existing_embedded_files_id = None;
            let catalog_id = doc
                .trailer
                .get(b"Root")
                .and_then(|obj| obj.as_reference())
                .map_err(|_| PdfError::Other("No root".into()))?;

            let mut has_names = false;
            if let Ok(catalog) = doc.get_dictionary(catalog_id) {
                if let Ok(names_id) = catalog.get(b"Names").and_then(|obj| obj.as_reference()) {
                    has_names = true;
                    if let Ok(names_dict) = doc.get_dictionary(names_id) {
                        if let Ok(ef_id) = names_dict
                            .get(b"EmbeddedFiles")
                            .and_then(|obj| obj.as_reference())
                        {
                            existing_embedded_files_id = Some(ef_id);
                            if let Ok(ef_dict) = doc.get_dictionary(ef_id) {
                                if let Ok(names_array) =
                                    ef_dict.get(b"Names").and_then(|obj| obj.as_array())
                                {
                                    embedded_files_names = names_array.clone();
                                }
                            }
                        }
                    }
                }
            }

            embedded_files_names.push(Object::String(
                file_name.as_bytes().to_vec(),
                StringFormat::Literal,
            ));
            embedded_files_names.push(Object::Reference(file_spec_id));

            let mut new_ef_dict = Dictionary::new();
            new_ef_dict.set("Names", Object::Array(embedded_files_names));

            if let Some(ef_id) = existing_embedded_files_id {
                doc.set_object(ef_id, Object::Dictionary(new_ef_dict));
            } else {
                let ef_id = doc.add_object(Object::Dictionary(new_ef_dict));
                if has_names {
                    // update Names dictionary
                    if let Ok(catalog) = doc.get_dictionary(catalog_id) {
                        if let Ok(names_id) =
                            catalog.get(b"Names").and_then(|obj| obj.as_reference())
                        {
                            if let Ok(names_obj) = doc.get_object_mut(names_id) {
                                if let Ok(names_dict) = names_obj.as_dict_mut() {
                                    names_dict.set("EmbeddedFiles", Object::Reference(ef_id));
                                }
                            }
                        }
                    }
                } else {
                    let mut names_dict = Dictionary::new();
                    names_dict.set("EmbeddedFiles", Object::Reference(ef_id));
                    let names_id = doc.add_object(Object::Dictionary(names_dict));
                    if let Ok(catalog_obj) = doc.get_object_mut(catalog_id) {
                        if let Ok(catalog) = catalog_obj.as_dict_mut() {
                            catalog.set("Names", Object::Reference(names_id));
                        }
                    }
                }
            }

            let mut output = Vec::new();
            doc.save_to(&mut output)
                .map_err(|e| PdfError::Other(e.to_string()))?;
            Ok(output)
        }
        "extract" => {
            let target_name = options.file_name.as_ref().ok_or_else(|| {
                PdfError::Other("Missing file_name for extract action".to_string())
            })?;
            if let Ok(catalog) = doc.catalog() {
                if let Ok(names_id) = catalog.get(b"Names").and_then(|obj| obj.as_reference()) {
                    if let Ok(names_dict) = doc.get_dictionary(names_id) {
                        if let Ok(embedded_files_id) = names_dict
                            .get(b"EmbeddedFiles")
                            .and_then(|obj| obj.as_reference())
                        {
                            if let Ok(embedded_files_dict) = doc.get_dictionary(embedded_files_id) {
                                if let Ok(names_array) = embedded_files_dict
                                    .get(b"Names")
                                    .and_then(|obj| obj.as_array())
                                {
                                    for i in (0..names_array.len()).step_by(2) {
                                        if let Ok(name_str) = names_array[i].as_str() {
                                            let name =
                                                String::from_utf8_lossy(name_str).to_string();
                                            if name == *target_name {
                                                if let Ok(file_spec_id) =
                                                    names_array[i + 1].as_reference()
                                                {
                                                    if let Ok(file_spec) =
                                                        doc.get_dictionary(file_spec_id)
                                                    {
                                                        if let Ok(ef) = file_spec
                                                            .get(b"EF")
                                                            .and_then(|obj| obj.as_dict())
                                                        {
                                                            if let Ok(stream_id) = ef
                                                                .get(b"F")
                                                                .and_then(|obj: &Object| {
                                                                    obj.as_reference()
                                                                })
                                                            {
                                                                if let Ok(stream) = doc
                                                                    .get_object(stream_id)
                                                                    .and_then(|obj| obj.as_stream())
                                                                {
                                                                    let content = stream
                                                                        .decompressed_content()
                                                                        .map_err(|e| {
                                                                            PdfError::Other(
                                                                                e.to_string(),
                                                                            )
                                                                        })?;
                                                                    return Ok(content);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(PdfError::Other(format!(
                "Attachment not found: {}",
                target_name
            )))
        }
        "remove" => {
            let target_name = options.file_name.as_ref().ok_or_else(|| {
                PdfError::Other("Missing file_name for remove action".to_string())
            })?;
            let mut removed = false;
            if let Ok(catalog) = doc.catalog() {
                if let Ok(names_id) = catalog.get(b"Names").and_then(|obj| obj.as_reference()) {
                    if let Ok(names_dict) = doc.get_dictionary(names_id) {
                        if let Ok(embedded_files_id) = names_dict
                            .get(b"EmbeddedFiles")
                            .and_then(|obj| obj.as_reference())
                        {
                            let mut new_names_array = Vec::new();
                            if let Ok(embedded_files_dict) = doc.get_dictionary(embedded_files_id) {
                                if let Ok(names_array) = embedded_files_dict
                                    .get(b"Names")
                                    .and_then(|obj| obj.as_array())
                                {
                                    for i in (0..names_array.len()).step_by(2) {
                                        if let Ok(name_str) = names_array[i].as_str() {
                                            let name =
                                                String::from_utf8_lossy(name_str).to_string();
                                            if name == *target_name {
                                                removed = true;
                                            } else {
                                                new_names_array.push(names_array[i].clone());
                                                new_names_array.push(names_array[i + 1].clone());
                                            }
                                        }
                                    }
                                }
                            }
                            if removed {
                                if let Ok(ef_obj) = doc.get_object_mut(embedded_files_id) {
                                    if let Ok(ef_dict) = ef_obj.as_dict_mut() {
                                        ef_dict.set("Names", Object::Array(new_names_array));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !removed {
                return Err(PdfError::Other(format!(
                    "Attachment not found: {}",
                    target_name
                )));
            }

            let mut output = Vec::new();
            doc.save_to(&mut output)
                .map_err(|e| PdfError::Other(e.to_string()))?;
            Ok(output)
        }
        _ => Err(PdfError::UnsupportedOperation(format!(
            "Unknown action: {}",
            options.action
        ))),
    }
}

#[derive(Default)]
pub struct PdfAttachmentsOperation {
    _options: PdfAttachmentsOptions,
}

impl PdfAttachmentsOperation {
    pub fn new(options: PdfAttachmentsOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfAttachmentsOperation {
    fn execute(
        &self,
        _document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Use pdf_attachments_bytes instead".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine, engine::general_purpose::STANDARD as b64_standard};
    use lopdf::Document;

    fn create_blank_pdf() -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", "Font".into()),
            ("Subtype", "Type1".into()),
            ("BaseFont", "Courier".into()),
        ]));
        let resources_id = doc.add_object(Dictionary::from_iter(vec![(
            "Font",
            Dictionary::from_iter(vec![("F1", font_id.into())]).into(),
        )]));
        let content_id = doc.add_object(Stream::new(
            Dictionary::new(),
            b"BT /F1 12 Tf 100 700 Td (Test) Tj ET".to_vec(),
        ));
        let page_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", "Page".into()),
            ("Parent", pages_id.into()),
            ("Contents", content_id.into()),
            ("Resources", resources_id.into()),
            (
                "MediaBox",
                vec![0.into(), 0.into(), 595.into(), 842.into()].into(),
            ),
        ]));
        let pages = Dictionary::from_iter(vec![
            ("Type", "Pages".into()),
            ("Kids", vec![page_id.into()].into()),
            ("Count", 1.into()),
        ]);
        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", "Catalog".into()),
            ("Pages", pages_id.into()),
        ]));
        doc.trailer.set("Root", catalog_id);
        let mut out = Vec::new();
        doc.save_to(&mut out).unwrap();
        out
    }

    #[test]
    fn test_pdf_attachments_empty_document() {
        let options = PdfAttachmentsOptions::default();
        let bytes = vec![];
        let result = pdf_attachments_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_attachments_list_empty() {
        let mut options = PdfAttachmentsOptions::default();
        options.action = "list".to_string();
        let bytes = create_blank_pdf();
        let result = pdf_attachments_bytes(&bytes, &options).unwrap();
        let report: AttachmentsReport = serde_json::from_slice(&result).unwrap();
        assert_eq!(report.total_count, 0);
        assert_eq!(report.attachments.len(), 0);
    }

    #[test]
    fn test_pdf_attachments_add_and_list() {
        let mut options = PdfAttachmentsOptions {
            action: "add".to_string(),
            file_name: Some("test.txt".to_string()),
            description: Some("Test description".to_string()),
            mime_type: Some("text/plain".to_string()),
            file_data_base64: Some(b64_standard.encode(b"Hello PDF Attachment")),
        };

        let bytes = create_blank_pdf();
        let new_pdf_bytes = pdf_attachments_bytes(&bytes, &options).unwrap();

        options.action = "list".to_string();
        let list_bytes = pdf_attachments_bytes(&new_pdf_bytes, &options).unwrap();
        let report: AttachmentsReport = serde_json::from_slice(&list_bytes).unwrap();

        assert_eq!(report.total_count, 1);
        assert_eq!(report.attachments[0].name, "test.txt");
        assert_eq!(
            report.attachments[0].description,
            Some("Test description".to_string())
        );
        assert_eq!(
            report.attachments[0].mime_type,
            Some("text/plain".to_string())
        );
    }

    #[test]
    fn test_pdf_attachments_extract() {
        let mut options = PdfAttachmentsOptions {
            action: "add".to_string(),
            file_name: Some("secret.dat".to_string()),
            description: None,
            mime_type: None,
            file_data_base64: Some(b64_standard.encode(b"super secret byte data")),
        };

        let bytes = create_blank_pdf();
        let new_pdf_bytes = pdf_attachments_bytes(&bytes, &options).unwrap();

        options.action = "extract".to_string();
        let extracted_bytes = pdf_attachments_bytes(&new_pdf_bytes, &options).unwrap();
        assert_eq!(extracted_bytes, b"super secret byte data");
    }

    #[test]
    fn test_pdf_attachments_remove() {
        let mut options = PdfAttachmentsOptions {
            action: "add".to_string(),
            file_name: Some("removeme.bin".to_string()),
            description: None,
            mime_type: None,
            file_data_base64: Some(b64_standard.encode(b"data to remove")),
        };

        let bytes = create_blank_pdf();
        let new_pdf_bytes = pdf_attachments_bytes(&bytes, &options).unwrap();

        options.action = "remove".to_string();
        let removed_pdf_bytes = pdf_attachments_bytes(&new_pdf_bytes, &options).unwrap();

        options.action = "list".to_string();
        let list_bytes = pdf_attachments_bytes(&removed_pdf_bytes, &options).unwrap();
        let report: AttachmentsReport = serde_json::from_slice(&list_bytes).unwrap();

        assert_eq!(report.total_count, 0);
    }
}
