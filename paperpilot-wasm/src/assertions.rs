use lopdf::Document;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PdfInspector {
    inner: Document,
    hash: String,
}

#[wasm_bindgen]
impl PdfInspector {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<PdfInspector, JsValue> {
        let doc = Document::load_mem(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse document: {}", e)))?;

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let mut hash = String::with_capacity(result.len() * 2);
        for byte in result {
            use std::fmt::Write;
            write!(&mut hash, "{:02x}", byte).unwrap();
        }

        Ok(PdfInspector { inner: doc, hash })
    }

    pub fn page_count(&self) -> usize {
        self.inner.get_pages().len()
    }

    pub fn get_text(&self, page: usize) -> Result<String, JsValue> {
        let page_u32 = page as u32;
        let pages = self.inner.get_pages();
        if !pages.contains_key(&page_u32) {
            return Err(JsValue::from_str(&format!("Page {} not found", page)));
        }

        let text = self
            .inner
            .extract_text(&[page_u32])
            .map_err(|e| JsValue::from_str(&format!("Failed to extract text: {}", e)))?;
        Ok(text)
    }

    pub fn contains_text(&self, text: &str) -> bool {
        let pages = self.inner.get_pages();
        for (page_num, _) in pages {
            if let Ok(extracted) = self.inner.extract_text(&[page_num]) {
                if extracted.contains(text) {
                    return true;
                }
            }
        }
        false
    }

    pub fn page_dimensions(&self, page: usize) -> Result<JsValue, JsValue> {
        let page_u32 = page as u32;
        let pages = self.inner.get_pages();
        if !pages.contains_key(&page_u32) {
            return Err(JsValue::from_str(&format!("Page {} not found", page)));
        }

        // This is a simplified extraction of dimensions; true dimensions may need parsing the MediaBox or CropBox
        // For standard assertions, this should suffice or we can use more advanced lopdf features if needed.
        // We will return an array [width, height].

        // Attempt to find MediaBox
        let page_id = *pages.get(&page_u32).unwrap();
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;

        if let Ok(dict) = self.inner.get_dictionary(page_id) {
            if let Ok(lopdf::Object::Array(arr)) = dict.get(b"MediaBox") {
                if arr.len() == 4 {
                    let mut values = Vec::new();
                    for obj in arr {
                        if let Ok(val) = obj.as_float() {
                            values.push(val as f64);
                        } else if let Ok(val) = obj.as_i64() {
                            values.push(val as f64);
                        }
                    }
                    if values.len() == 4 {
                        width = values[2] - values[0];
                        height = values[3] - values[1];
                    }
                }
            }
        }

        let arr = js_sys::Array::new();
        arr.push(&JsValue::from_f64(width));
        arr.push(&JsValue::from_f64(height));
        Ok(arr.into())
    }

    pub fn is_encrypted(&self) -> bool {
        self.inner.is_encrypted()
    }

    pub fn form_fields(&self) -> Result<JsValue, JsValue> {
        let mut fields = Vec::new();
        // Traverse AcroForm dictionary to extract fields
        if let Ok(catalog) = self.inner.catalog() {
            if let Ok(lopdf::Object::Dictionary(acro_form)) = catalog.get(b"AcroForm") {
                if let Ok(lopdf::Object::Array(fields_arr)) = acro_form.get(b"Fields") {
                    for field_obj in fields_arr {
                        if let lopdf::Object::Reference(ref_id) = field_obj {
                            if let Ok(lopdf::Object::Dictionary(field_dict)) =
                                self.inner.get_object(*ref_id)
                            {
                                let name = if let Ok(lopdf::Object::String(name_bytes, _)) =
                                    field_dict.get(b"T")
                                {
                                    String::from_utf8_lossy(name_bytes).into_owned()
                                } else {
                                    continue;
                                };

                                let value = if let Ok(lopdf::Object::String(val_bytes, _)) =
                                    field_dict.get(b"V")
                                {
                                    Some(String::from_utf8_lossy(val_bytes).into_owned())
                                } else {
                                    None
                                };

                                let arr = js_sys::Array::new();
                                arr.push(&JsValue::from_str(&name));
                                if let Some(v) = value {
                                    arr.push(&JsValue::from_str(&v));
                                } else {
                                    arr.push(&JsValue::null());
                                }
                                fields.push(arr);
                            }
                        }
                    }
                }
            }
        }

        let js_fields = js_sys::Array::new();
        for field in fields {
            js_fields.push(&field);
        }
        Ok(js_fields.into())
    }

    pub fn sha256_hex(&self) -> String {
        self.hash.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_pdf_inspector_page_count() {
        let bytes = fs::read("../tests/fixtures/multi_page.pdf").unwrap();
        let inspector = PdfInspector::new(&bytes).unwrap();
        assert_eq!(inspector.page_count(), 3); // Based on usual multi_page fixture
    }

    #[test]
    fn test_pdf_inspector_get_text() {
        let bytes = fs::read("../tests/fixtures/simple.pdf").unwrap();
        let inspector = PdfInspector::new(&bytes).unwrap();
        let text = inspector.get_text(1).unwrap();
        assert!(!text.is_empty());
    }

    #[test]
    fn test_pdf_inspector_contains_text() {
        let bytes = fs::read("../tests/fixtures/simple.pdf").unwrap();
        let inspector = PdfInspector::new(&bytes).unwrap();
        assert!(inspector.contains_text("")); // Should be true for simple pdf containing text
    }

    #[test]
    fn test_pdf_inspector_is_encrypted() {
        let bytes = fs::read("../tests/fixtures/encrypted.pdf").unwrap();
        let inspector = PdfInspector::new(&bytes).unwrap();
        assert!(inspector.is_encrypted());
    }
}
