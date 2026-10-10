pub mod assertions;
pub mod operations;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPdfEngine;

#[wasm_bindgen]
impl WasmPdfEngine {
    /// Merge multiple PDF buffers (Array of Uint8Array) into a single PDF Uint8Array
    #[wasm_bindgen]
    pub fn merge(buffers: js_sys::Array) -> Result<js_sys::Uint8Array, JsValue> {
        let mut rust_buffers = Vec::new();
        for i in 0..buffers.length() {
            let item = buffers.get(i);
            let u8_array = js_sys::Uint8Array::new(&item);
            rust_buffers.push(u8_array.to_vec());
        }

        let result =
            operations::merge(rust_buffers).map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    /// Rotate specific pages ("all", "1,2", or "1") by angle (90, 180, 270)
    #[wasm_bindgen]
    pub fn rotate(
        input_bytes: &[u8],
        angle: u16,
        pages: &str,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::rotate(input_bytes, angle, pages)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    /// Split PDF by page ranges (e.g. "1-2, 3-4") returning an Array of Uint8Arrays
    #[wasm_bindgen]
    pub fn split(input_bytes: &[u8], ranges: &str) -> Result<js_sys::Array, JsValue> {
        let result_buffers = operations::split(input_bytes, ranges)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let js_array = js_sys::Array::new();
        for buffer in result_buffers {
            let u8_array = js_sys::Uint8Array::from(&buffer[..]);
            js_array.push(&u8_array);
        }

        Ok(js_array)
    }

    /// Compress PDF by removing unneeded metadata, deflating streams, and cleaning cross-references
    #[wasm_bindgen]
    pub fn compress(
        input_bytes: &[u8],
        quality: Option<String>,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let quality_str = quality.as_deref();
        let result = operations::compress(input_bytes, quality_str)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    /// Encrypt PDF with user password (standard 128/256-bit)
    #[wasm_bindgen]
    pub fn encrypt(input_bytes: &[u8], password: &str) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::encrypt(input_bytes, password)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    /// Add watermark text across pages
    #[wasm_bindgen]
    pub fn watermark(input_bytes: &[u8], text: &str) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::watermark(input_bytes, text)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn delete_pages(input_bytes: &[u8], pages: &str) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::delete_pages(input_bytes, pages)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn extract_pages(input_bytes: &[u8], pages: &str) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::extract_pages(input_bytes, pages)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn reorder_pages(
        input_bytes: &[u8],
        new_order: Vec<u32>,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::reorder_pages(input_bytes, &new_order)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn crop(
        input_bytes: &[u8],
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::crop(input_bytes, left, bottom, right, top)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn flatten(input_bytes: &[u8]) -> Result<js_sys::Uint8Array, JsValue> {
        let result =
            operations::flatten(input_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn set_metadata(
        input_bytes: &[u8],
        title: Option<String>,
        author: Option<String>,
        subject: Option<String>,
        keywords: Option<String>,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::set_metadata(input_bytes, title, author, subject, keywords)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn images_to_pdf(image_buffers: js_sys::Array) -> Result<js_sys::Uint8Array, JsValue> {
        let mut rust_buffers = Vec::new();
        for i in 0..image_buffers.length() {
            let item = image_buffers.get(i);
            let u8_array = js_sys::Uint8Array::new(&item);
            rust_buffers.push(u8_array.to_vec());
        }

        let refs: Vec<&[u8]> = rust_buffers.iter().map(|v| v.as_slice()).collect();

        let result =
            operations::images_to_pdf(&refs).map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn extract_images(input_bytes: &[u8]) -> Result<js_sys::Array, JsValue> {
        let result_buffers = operations::extract_images(input_bytes)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let js_array = js_sys::Array::new();
        for buffer in result_buffers {
            let u8_array = js_sys::Uint8Array::from(&buffer[..]);
            js_array.push(&u8_array);
        }

        Ok(js_array)
    }

    #[wasm_bindgen]
    pub fn pdf_hash(input_bytes: &[u8]) -> Result<String, JsValue> {
        let result =
            operations::pdf_hash(input_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(result)
    }

    #[wasm_bindgen]
    pub fn render_page(
        input_bytes: &[u8],
        page_index: u32,
        scale: f32,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::render_page(input_bytes, page_index, scale)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn extract_text(input_bytes: &[u8]) -> Result<String, JsValue> {
        let result =
            operations::extract_text(input_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(result)
    }

    #[wasm_bindgen]
    pub fn decrypt(input_bytes: &[u8], password: &str) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::decrypt(input_bytes, password)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn page_numbers(
        input_bytes: &[u8],
        format: &str,
        position: &str,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::page_numbers(input_bytes, format, position)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn header_footer(
        input_bytes: &[u8],
        header: &str,
        footer: &str,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let result = operations::header_footer(input_bytes, header, footer)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn pdf_info(input_bytes: &[u8]) -> Result<String, JsValue> {
        let result =
            operations::pdf_info(input_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(result)
    }

    #[wasm_bindgen]
    pub fn ocr(image_or_pdf_bytes: &[u8]) -> Result<String, JsValue> {
        let result =
            operations::ocr(image_or_pdf_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(result)
    }

    /// Convert JSON document into a PDF Uint8Array
    #[wasm_bindgen]
    pub fn json_to_pdf(json_str: &str) -> Result<js_sys::Uint8Array, JsValue> {
        let result =
            operations::json_to_pdf(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(js_sys::Uint8Array::from(&result[..]))
    }

    #[wasm_bindgen]
    pub fn pdf_to_docx(input_bytes: &[u8]) -> Result<js_sys::Uint8Array, JsValue> {
        let result_bytes = operations::pdf_to_docx(input_bytes)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let array = js_sys::Uint8Array::new_with_length(result_bytes.len() as u32);
        array.copy_from(&result_bytes);
        Ok(array)
    }
}
