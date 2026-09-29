use crate::schema::OperationResult;
use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, ListToolsResult, Tool,
};
use rmcp::service::{MaybeSendFuture, RequestContext};
use rmcp::{ErrorData, RoleServer};
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;

#[derive(Clone, Default)]
pub struct PaperPilotMcpServer;

impl PaperPilotMcpServer {
    pub fn new() -> Self {
        Self
    }
    pub fn execute_list_tools() -> Result<ListToolsResult, ErrorData> {
        let mut tools = Vec::new();

        // Tool: pdf_merge
        let mut schema_0 = serde_json::Map::new();
        schema_0.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties_0 = serde_json::Map::new();

        let mut prop_inputs_0 = serde_json::Map::new();
        prop_inputs_0.insert(
            "type".to_string(),
            serde_json::Value::String("array".to_string()),
        );
        prop_inputs_0.insert(
            "description".to_string(),
            serde_json::Value::String("Array of file paths to merge.".to_string()),
        );

        let mut items_inputs_0 = serde_json::Map::new();
        items_inputs_0.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_inputs_0.insert(
            "items".to_string(),
            serde_json::Value::Object(items_inputs_0),
        );

        properties_0.insert(
            "inputs".to_string(),
            serde_json::Value::Object(prop_inputs_0),
        );

        let mut prop_output_0 = serde_json::Map::new();
        prop_output_0.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_0.insert(
            "description".to_string(),
            serde_json::Value::String("File path for the merged output.".to_string()),
        );

        properties_0.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_0),
        );

        schema_0.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties_0),
        );

        let required_0: Vec<serde_json::Value> = vec![
            serde_json::Value::String("inputs".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_0.insert("required".to_string(), serde_json::Value::Array(required_0));

        let mut tool_0 = Tool::default();
        tool_0.name = "pdf_merge".into();
        tool_0.description = Some("Merges multiple PDFs into one.".into());
        tool_0.input_schema = Arc::new(schema_0);
        tools.push(tool_0);

        // Tool: pdf_split
        let mut schema_1 = serde_json::Map::new();
        schema_1.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties_1 = serde_json::Map::new();

        let mut prop_input_1 = serde_json::Map::new();
        prop_input_1.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_1.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );

        properties_1.insert("input".to_string(), serde_json::Value::Object(prop_input_1));

        let mut prop_output_dir_1 = serde_json::Map::new();
        prop_output_dir_1.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_dir_1.insert(
            "description".to_string(),
            serde_json::Value::String("The directory where split pages will be saved.".to_string()),
        );

        properties_1.insert(
            "output_dir".to_string(),
            serde_json::Value::Object(prop_output_dir_1),
        );

        schema_1.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties_1),
        );

        let required_1: Vec<serde_json::Value> = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("output_dir".to_string()),
        ];
        schema_1.insert("required".to_string(), serde_json::Value::Array(required_1));

        let mut tool_1 = Tool::default();
        tool_1.name = "pdf_split".into();
        tool_1.description = Some("Splits a PDF into multiple single-page PDFs.".into());
        tool_1.input_schema = Arc::new(schema_1);
        tools.push(tool_1);

        // Tool: pdf_extract_pages
        let mut schema_2 = serde_json::Map::new();
        schema_2.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties_2 = serde_json::Map::new();

        let mut prop_input_2 = serde_json::Map::new();
        prop_input_2.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_2.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );

        properties_2.insert("input".to_string(), serde_json::Value::Object(prop_input_2));

        let mut prop_pages_2 = serde_json::Map::new();
        prop_pages_2.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_pages_2.insert(
            "description".to_string(),
            serde_json::Value::String("The pages to extract, e.g., '1,3,5-7'.".to_string()),
        );

        properties_2.insert("pages".to_string(), serde_json::Value::Object(prop_pages_2));

        let mut prop_output_2 = serde_json::Map::new();
        prop_output_2.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_2.insert(
            "description".to_string(),
            serde_json::Value::String("File path for the extracted output.".to_string()),
        );

        properties_2.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_2),
        );

        schema_2.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties_2),
        );

        let required_2: Vec<serde_json::Value> = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("pages".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_2.insert("required".to_string(), serde_json::Value::Array(required_2));

        let mut tool_2 = Tool::default();
        tool_2.name = "pdf_extract_pages".into();
        tool_2.description = Some("Extracts specific pages into a new PDF.".into());
        tool_2.input_schema = Arc::new(schema_2);
        tools.push(tool_2);

        // Tool: pdf_delete_pages
        let mut schema_3 = serde_json::Map::new();
        schema_3.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties_3 = serde_json::Map::new();

        let mut prop_input_3 = serde_json::Map::new();
        prop_input_3.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_3.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );

        properties_3.insert("input".to_string(), serde_json::Value::Object(prop_input_3));

        let mut prop_pages_3 = serde_json::Map::new();
        prop_pages_3.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_pages_3.insert(
            "description".to_string(),
            serde_json::Value::String("The pages to delete, e.g., '1,3,5-7'.".to_string()),
        );

        properties_3.insert("pages".to_string(), serde_json::Value::Object(prop_pages_3));

        let mut prop_output_3 = serde_json::Map::new();
        prop_output_3.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_3.insert(
            "description".to_string(),
            serde_json::Value::String("File path for the modified output.".to_string()),
        );

        properties_3.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_3),
        );

        schema_3.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties_3),
        );

        let required_3: Vec<serde_json::Value> = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("pages".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_3.insert("required".to_string(), serde_json::Value::Array(required_3));

        let mut tool_3 = Tool::default();
        tool_3.name = "pdf_delete_pages".into();
        tool_3.description = Some("Deletes specific pages.".into());
        tool_3.input_schema = Arc::new(schema_3);
        tools.push(tool_3);

        // Tool: pdf_reorder_pages
        let mut schema_4 = serde_json::Map::new();
        schema_4.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties_4 = serde_json::Map::new();

        let mut prop_input_4 = serde_json::Map::new();
        prop_input_4.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_4.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );

        properties_4.insert("input".to_string(), serde_json::Value::Object(prop_input_4));

        let mut prop_order_4 = serde_json::Map::new();
        prop_order_4.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_order_4.insert(
            "description".to_string(),
            serde_json::Value::String("The new 1-based page order, e.g., '3,2,1'.".to_string()),
        );

        properties_4.insert("order".to_string(), serde_json::Value::Object(prop_order_4));

        let mut prop_output_4 = serde_json::Map::new();
        prop_output_4.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_4.insert(
            "description".to_string(),
            serde_json::Value::String("File path for the reordered output.".to_string()),
        );

        properties_4.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_4),
        );

        schema_4.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties_4),
        );

        let required_4: Vec<serde_json::Value> = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("order".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_4.insert("required".to_string(), serde_json::Value::Array(required_4));

        let mut tool_4 = Tool::default();
        tool_4.name = "pdf_reorder_pages".into();
        tool_4.description = Some("Reorders pages.".into());
        tool_4.input_schema = Arc::new(schema_4);
        tools.push(tool_4);

        // Tool: pdf_rotate
        let mut schema_5 = serde_json::Map::new();
        schema_5.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties_5 = serde_json::Map::new();

        let mut prop_input_5 = serde_json::Map::new();
        prop_input_5.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_5.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );

        properties_5.insert("input".to_string(), serde_json::Value::Object(prop_input_5));

        let mut prop_pages_5 = serde_json::Map::new();
        prop_pages_5.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_pages_5.insert(
            "description".to_string(),
            serde_json::Value::String(
                "The pages to rotate, either 'all' or specific pages like '1,3'.".to_string(),
            ),
        );

        properties_5.insert("pages".to_string(), serde_json::Value::Object(prop_pages_5));

        let mut prop_angle_5 = serde_json::Map::new();
        prop_angle_5.insert(
            "type".to_string(),
            serde_json::Value::String("number".to_string()),
        );
        prop_angle_5.insert(
            "description".to_string(),
            serde_json::Value::String(
                "The angle to rotate by, typically 90, 180, or 270.".to_string(),
            ),
        );

        properties_5.insert("angle".to_string(), serde_json::Value::Object(prop_angle_5));

        let mut prop_output_5 = serde_json::Map::new();
        prop_output_5.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_5.insert(
            "description".to_string(),
            serde_json::Value::String("File path for the rotated output.".to_string()),
        );

        properties_5.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_5),
        );

        schema_5.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties_5),
        );

        let required_5: Vec<serde_json::Value> = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("pages".to_string()),
            serde_json::Value::String("angle".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_5.insert("required".to_string(), serde_json::Value::Array(required_5));

        let mut tool_5 = Tool::default();
        tool_5.name = "pdf_rotate".into();
        tool_5.description = Some("Rotates pages in a PDF.".into());
        tool_5.input_schema = Arc::new(schema_5);
        tools.push(tool_5);

        // Tool: pdf_crop
        let mut schema_6 = serde_json::Map::new();
        schema_6.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties_6 = serde_json::Map::new();

        let mut prop_input_6 = serde_json::Map::new();
        prop_input_6.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_6.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );

        properties_6.insert("input".to_string(), serde_json::Value::Object(prop_input_6));

        let mut prop_box_6 = serde_json::Map::new();
        prop_box_6.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_box_6.insert(
            "description".to_string(),
            serde_json::Value::String(
                "The crop box in format 'left,bottom,right,top' e.g. '0,0,595,842'.".to_string(),
            ),
        );

        properties_6.insert("box".to_string(), serde_json::Value::Object(prop_box_6));

        let mut prop_output_6 = serde_json::Map::new();
        prop_output_6.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_6.insert(
            "description".to_string(),
            serde_json::Value::String("File path for the cropped output.".to_string()),
        );

        properties_6.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_6),
        );

        schema_6.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties_6),
        );

        let required_6: Vec<serde_json::Value> = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("box".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_6.insert("required".to_string(), serde_json::Value::Array(required_6));

        let mut tool_6 = Tool::default();
        tool_6.name = "pdf_crop".into();
        tool_6.description = Some("Crops pages to specific dimensions.".into());
        tool_6.input_schema = Arc::new(schema_6);
        tools.push(tool_6);

        // Tool: pdf_burst
        let mut schema_7 = serde_json::Map::new();
        schema_7.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties_7 = serde_json::Map::new();

        let mut prop_input_7 = serde_json::Map::new();
        prop_input_7.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_7.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );

        properties_7.insert("input".to_string(), serde_json::Value::Object(prop_input_7));

        let mut prop_output_dir_7 = serde_json::Map::new();
        prop_output_dir_7.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_dir_7.insert(
            "description".to_string(),
            serde_json::Value::String(
                "The directory where individual files will be saved.".to_string(),
            ),
        );

        properties_7.insert(
            "output_dir".to_string(),
            serde_json::Value::Object(prop_output_dir_7),
        );

        schema_7.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties_7),
        );

        let required_7: Vec<serde_json::Value> = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("output_dir".to_string()),
        ];
        schema_7.insert("required".to_string(), serde_json::Value::Array(required_7));

        let mut tool_7 = Tool::default();
        tool_7.name = "pdf_burst".into();
        tool_7.description =
            Some("Bursts a PDF into individual files, returning the path of each.".into());
        tool_7.input_schema = Arc::new(schema_7);
        tools.push(tool_7);

        // Tool: pdf_compress
        let mut schema_compress = serde_json::Map::new();
        schema_compress.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
        let mut prop_compress = serde_json::Map::new();
        let mut prop_input_c = serde_json::Map::new();
        prop_input_c.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_c.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );
        prop_compress.insert("input".to_string(), serde_json::Value::Object(prop_input_c));
        let mut prop_output_c = serde_json::Map::new();
        prop_output_c.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_c.insert(
            "description".to_string(),
            serde_json::Value::String("The output PDF file path.".to_string()),
        );
        prop_compress.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_c),
        );
        schema_compress.insert(
            "properties".to_string(),
            serde_json::Value::Object(prop_compress),
        );
        let req_compress = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_compress.insert(
            "required".to_string(),
            serde_json::Value::Array(req_compress),
        );
        let mut tool_compress = Tool::default();
        tool_compress.name = "pdf_compress".into();
        tool_compress.description = Some("Compresses a PDF to reduce file size.".into());
        tool_compress.input_schema = std::sync::Arc::new(schema_compress);
        tools.push(tool_compress);

        // Tool: pdf_extract_text
        let mut schema_ext_txt = serde_json::Map::new();
        schema_ext_txt.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
        let mut prop_ext_txt = serde_json::Map::new();
        let mut prop_input_ext = serde_json::Map::new();
        prop_input_ext.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_ext.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );
        prop_ext_txt.insert(
            "input".to_string(),
            serde_json::Value::Object(prop_input_ext),
        );

        let mut prop_pages_ext = serde_json::Map::new();
        prop_pages_ext.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_pages_ext.insert(
            "description".to_string(),
            serde_json::Value::String(
                "Pages to extract text from, e.g., '1,3,5-7' or 'all'.".to_string(),
            ),
        );
        prop_ext_txt.insert(
            "pages".to_string(),
            serde_json::Value::Object(prop_pages_ext),
        );

        let mut prop_output_ext = serde_json::Map::new();
        prop_output_ext.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_ext.insert(
            "description".to_string(),
            serde_json::Value::String("The output text file path.".to_string()),
        );
        prop_ext_txt.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_ext),
        );

        schema_ext_txt.insert(
            "properties".to_string(),
            serde_json::Value::Object(prop_ext_txt),
        );
        let req_ext_txt = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_ext_txt.insert(
            "required".to_string(),
            serde_json::Value::Array(req_ext_txt),
        );

        let mut tool_ext_txt = Tool::default();
        tool_ext_txt.name = "pdf_extract_text".into();
        tool_ext_txt.description = Some("Extracts raw text from a PDF.".into());
        tool_ext_txt.input_schema = std::sync::Arc::new(schema_ext_txt);
        tools.push(tool_ext_txt);

        // Tool: pdf_extract_images
        let mut schema_ext_img = serde_json::Map::new();
        schema_ext_img.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
        let mut prop_ext_img = serde_json::Map::new();

        let mut prop_input_img = serde_json::Map::new();
        prop_input_img.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_img.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );
        prop_ext_img.insert(
            "input".to_string(),
            serde_json::Value::Object(prop_input_img),
        );

        let mut prop_output_dir_img = serde_json::Map::new();
        prop_output_dir_img.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_dir_img.insert(
            "description".to_string(),
            serde_json::Value::String("The directory where images will be saved.".to_string()),
        );
        prop_ext_img.insert(
            "output_dir".to_string(),
            serde_json::Value::Object(prop_output_dir_img),
        );

        schema_ext_img.insert(
            "properties".to_string(),
            serde_json::Value::Object(prop_ext_img),
        );
        let req_ext_img = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("output_dir".to_string()),
        ];
        schema_ext_img.insert(
            "required".to_string(),
            serde_json::Value::Array(req_ext_img),
        );

        let mut tool_ext_img = Tool::default();
        tool_ext_img.name = "pdf_extract_images".into();
        tool_ext_img.description = Some("Extracts all embedded images from a PDF.".into());
        tool_ext_img.input_schema = std::sync::Arc::new(schema_ext_img);
        tools.push(tool_ext_img);

        // Tool: pdf_search
        let mut schema_search = serde_json::Map::new();
        schema_search.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
        let mut prop_search = serde_json::Map::new();

        let mut prop_input_s = serde_json::Map::new();
        prop_input_s.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_s.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );
        prop_search.insert("input".to_string(), serde_json::Value::Object(prop_input_s));

        let mut prop_query_s = serde_json::Map::new();
        prop_query_s.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_query_s.insert(
            "description".to_string(),
            serde_json::Value::String("The text to search for.".to_string()),
        );
        prop_search.insert("query".to_string(), serde_json::Value::Object(prop_query_s));

        schema_search.insert(
            "properties".to_string(),
            serde_json::Value::Object(prop_search),
        );
        let req_search = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("query".to_string()),
        ];
        schema_search.insert("required".to_string(), serde_json::Value::Array(req_search));

        let mut tool_search = Tool::default();
        tool_search.name = "pdf_search".into();
        tool_search.description = Some("Searches for a text query inside a PDF.".into());
        tool_search.input_schema = std::sync::Arc::new(schema_search);
        tools.push(tool_search);

        // Tool: pdf_watermark
        let mut schema_wm = serde_json::Map::new();
        schema_wm.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
        let mut prop_wm = serde_json::Map::new();

        let mut prop_input_w = serde_json::Map::new();
        prop_input_w.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_w.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );
        prop_wm.insert("input".to_string(), serde_json::Value::Object(prop_input_w));

        let mut prop_text_w = serde_json::Map::new();
        prop_text_w.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_text_w.insert(
            "description".to_string(),
            serde_json::Value::String("The text to use as watermark.".to_string()),
        );
        prop_wm.insert("text".to_string(), serde_json::Value::Object(prop_text_w));

        let mut prop_output_w = serde_json::Map::new();
        prop_output_w.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_w.insert(
            "description".to_string(),
            serde_json::Value::String("The output PDF file path.".to_string()),
        );
        prop_wm.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_w),
        );

        schema_wm.insert("properties".to_string(), serde_json::Value::Object(prop_wm));
        let req_wm = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("text".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_wm.insert("required".to_string(), serde_json::Value::Array(req_wm));

        let mut tool_wm = Tool::default();
        tool_wm.name = "pdf_watermark".into();
        tool_wm.description = Some("Applies a text watermark to a PDF.".into());
        tool_wm.input_schema = std::sync::Arc::new(schema_wm);
        tools.push(tool_wm);

        // Tool: pdf_encrypt
        let mut schema_enc = serde_json::Map::new();
        schema_enc.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
        let mut prop_enc = serde_json::Map::new();

        let mut prop_input_e = serde_json::Map::new();
        prop_input_e.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_e.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );
        prop_enc.insert("input".to_string(), serde_json::Value::Object(prop_input_e));

        let mut prop_pass_e = serde_json::Map::new();
        prop_pass_e.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_pass_e.insert(
            "description".to_string(),
            serde_json::Value::String("The user password.".to_string()),
        );
        prop_enc.insert(
            "password".to_string(),
            serde_json::Value::Object(prop_pass_e),
        );

        let mut prop_output_e = serde_json::Map::new();
        prop_output_e.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_e.insert(
            "description".to_string(),
            serde_json::Value::String("The output PDF file path.".to_string()),
        );
        prop_enc.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_e),
        );

        schema_enc.insert(
            "properties".to_string(),
            serde_json::Value::Object(prop_enc),
        );
        let req_enc = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("password".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_enc.insert("required".to_string(), serde_json::Value::Array(req_enc));

        let mut tool_enc = Tool::default();
        tool_enc.name = "pdf_encrypt".into();
        tool_enc.description = Some("Encrypts a PDF with a user password.".into());
        tool_enc.input_schema = std::sync::Arc::new(schema_enc);
        tools.push(tool_enc);

        // Tool: pdf_decrypt
        let mut schema_dec = serde_json::Map::new();
        schema_dec.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
        let mut prop_dec = serde_json::Map::new();

        let mut prop_input_d = serde_json::Map::new();
        prop_input_d.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_d.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );
        prop_dec.insert("input".to_string(), serde_json::Value::Object(prop_input_d));

        let mut prop_pass_d = serde_json::Map::new();
        prop_pass_d.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_pass_d.insert(
            "description".to_string(),
            serde_json::Value::String("The user password.".to_string()),
        );
        prop_dec.insert(
            "password".to_string(),
            serde_json::Value::Object(prop_pass_d),
        );

        let mut prop_output_d = serde_json::Map::new();
        prop_output_d.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_d.insert(
            "description".to_string(),
            serde_json::Value::String("The output PDF file path.".to_string()),
        );
        prop_dec.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_d),
        );

        schema_dec.insert(
            "properties".to_string(),
            serde_json::Value::Object(prop_dec),
        );
        let req_dec = vec![
            serde_json::Value::String("input".to_string()),
            serde_json::Value::String("password".to_string()),
            serde_json::Value::String("output".to_string()),
        ];
        schema_dec.insert("required".to_string(), serde_json::Value::Array(req_dec));

        let mut tool_dec = Tool::default();
        tool_dec.name = "pdf_decrypt".into();
        tool_dec.description = Some("Decrypts a PDF using the provided password.".into());
        tool_dec.input_schema = std::sync::Arc::new(schema_dec);
        tools.push(tool_dec);

        // Tool: pdf_metadata
        let mut schema_meta = serde_json::Map::new();
        schema_meta.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
        let mut prop_meta = serde_json::Map::new();

        let mut prop_input_m = serde_json::Map::new();
        prop_input_m.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_input_m.insert(
            "description".to_string(),
            serde_json::Value::String("The input PDF file path.".to_string()),
        );
        prop_meta.insert("input".to_string(), serde_json::Value::Object(prop_input_m));

        let mut prop_title_m = serde_json::Map::new();
        prop_title_m.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_title_m.insert(
            "description".to_string(),
            serde_json::Value::String("The new title.".to_string()),
        );
        prop_meta.insert("title".to_string(), serde_json::Value::Object(prop_title_m));

        let mut prop_author_m = serde_json::Map::new();
        prop_author_m.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_author_m.insert(
            "description".to_string(),
            serde_json::Value::String("The new author.".to_string()),
        );
        prop_meta.insert(
            "author".to_string(),
            serde_json::Value::Object(prop_author_m),
        );

        let mut prop_output_m = serde_json::Map::new();
        prop_output_m.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        prop_output_m.insert(
            "description".to_string(),
            serde_json::Value::String("The output PDF file path.".to_string()),
        );
        prop_meta.insert(
            "output".to_string(),
            serde_json::Value::Object(prop_output_m),
        );

        schema_meta.insert(
            "properties".to_string(),
            serde_json::Value::Object(prop_meta),
        );
        let req_meta = vec![serde_json::Value::String("input".to_string())]; // Output, title, author are optional
        schema_meta.insert("required".to_string(), serde_json::Value::Array(req_meta));

        let mut tool_meta = Tool::default();
        tool_meta.name = "pdf_metadata".into();
        tool_meta.description = Some("Reads (or writes) metadata to a PDF.".into());
        tool_meta.input_schema = std::sync::Arc::new(schema_meta);
        tools.push(tool_meta);
        let mut tool_docx = Tool::default();
        tool_docx.name = "pdf_to_docx".into();
        tool_docx.description =
            Some("Converts a PDF document to DOCX format by extracting its text.".into());
        let mut d_props = serde_json::Map::new();
        d_props.insert(
            "input".into(),
            serde_json::json!({ "type": "string" }),
        );
        d_props.insert(
            "output".into(),
            serde_json::json!({ "type": "string" }),
        );
        tool_docx.input_schema = std::sync::Arc::new(serde_json::json!({
            "type": "object",
            "properties": d_props,
            "required": ["input", "output"]
        }).as_object().unwrap().clone());
        tools.push(tool_docx);

        let mut tool_xlsx = Tool::default();
        tool_xlsx.name = "pdf_to_xlsx".into();
        tool_xlsx.description =
            Some("Converts a PDF document to XLSX format by using best effort table extraction.".into());
        let mut x_props = serde_json::Map::new();
        x_props.insert(
            "input".into(),
            serde_json::json!({ "type": "string" }),
        );
        x_props.insert(
            "output".into(),
            serde_json::json!({ "type": "string" }),
        );
        tool_xlsx.input_schema = std::sync::Arc::new(serde_json::json!({
            "type": "object",
            "properties": x_props,
            "required": ["input", "output"]
        }).as_object().unwrap().clone());
        tools.push(tool_xlsx);

        let mut tool_pptx = Tool::default();
        tool_pptx.name = "pdf_to_pptx".into();
        tool_pptx.description =
            Some("Converts a PDF document to PPTX format via best effort slide generation.".into());
        let mut p_props = serde_json::Map::new();
        p_props.insert(
            "input".into(),
            serde_json::json!({ "type": "string" }),
        );
        p_props.insert(
            "output".into(),
            serde_json::json!({ "type": "string" }),
        );
        tool_pptx.input_schema = std::sync::Arc::new(serde_json::json!({
            "type": "object",
            "properties": p_props,
            "required": ["input", "output"]
        }).as_object().unwrap().clone());
        tools.push(tool_pptx);

        let mut tool_class = Tool::default();
        tool_class.name = "pdf_classify_type".into();
        tool_class.description =
            Some("Classifies the PDF document type based on heuristics.".into());
        let mut c_props = serde_json::Map::new();
        c_props.insert(
            "input".into(),
            serde_json::json!({ "type": "string" }),
        );
        tool_class.input_schema = std::sync::Arc::new(serde_json::json!({
            "type": "object",
            "properties": c_props,
            "required": ["input"]
        }).as_object().unwrap().clone());
        tools.push(tool_class);


        Ok(ListToolsResult {
            tools,
            ..Default::default()
        })
    }

    pub fn execute_call_tool(
        request: CallToolRequestParams,
    ) -> Result<CallToolResponse, ErrorData> {
        let args = request.arguments.clone().unwrap_or_default();
        let get_string = |key: &str| -> Result<String, ErrorData> {
            args.get(key)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| {
                    ErrorData::invalid_params(
                        format!("Missing or invalid '{}' parameter", key),
                        None,
                    )
                })
        };

        let get_string_array = |key: &str| -> Result<Vec<String>, ErrorData> {
            args.get(key)
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .ok_or_else(|| {
                    ErrorData::invalid_params(
                        format!("Missing or invalid '{}' parameter", key),
                        None,
                    )
                })
        };

        let get_number = |key: &str| -> Result<f64, ErrorData> {
            args.get(key).and_then(|v| v.as_f64()).ok_or_else(|| {
                ErrorData::invalid_params(format!("Missing or invalid '{}' parameter", key), None)
            })
        };
        let ensure_parent_dir = |path_str: &str| -> Result<(), ErrorData> {
            if let Some(parent) = std::path::PathBuf::from(path_str).parent() {
                let os_str = parent.as_os_str();
                if !os_str.is_empty() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        ErrorData::invalid_params(
                            format!("Failed to create parent directory: {}", e),
                            None,
                        )
                    })?;
                }
            }
            Ok(())
        };

        let ensure_dir = |path_str: &str| -> Result<(), ErrorData> {
            std::fs::create_dir_all(path_str).map_err(|e| {
                ErrorData::invalid_params(format!("Failed to create directory: {}", e), None)
            })
        };

        let parse_pages = |pages_str: &str| -> Result<Vec<u32>, ErrorData> {
            if pages_str.to_lowercase() == "all" {
                return Ok(vec![]);
            }
            let mut pages = Vec::new();
            for part in pages_str.split(',') {
                if part.contains('-') {
                    let bounds: Vec<&str> = part.split('-').collect();
                    if bounds.len() == 2 {
                        let start = bounds[0]
                            .trim()
                            .parse::<u32>()
                            .map_err(|_| ErrorData::invalid_params("Invalid page range", None))?;
                        let end = bounds[1]
                            .trim()
                            .parse::<u32>()
                            .map_err(|_| ErrorData::invalid_params("Invalid page range", None))?;
                        for p in start..=end {
                            pages.push(p);
                        }
                    } else {
                        return Err(ErrorData::invalid_params("Invalid page range format", None));
                    }
                } else {
                    let p = part
                        .trim()
                        .parse::<u32>()
                        .map_err(|_| ErrorData::invalid_params("Invalid page number", None))?;
                    pages.push(p);
                }
            }
            Ok(pages)
        };

        let result = match request.name.as_ref() {
            "pdf_merge" => {
                let inputs = get_string_array("inputs")?;
                let output = get_string("output")?;
                ensure_parent_dir(&output)?;
                let paths: Vec<PathBuf> = inputs.into_iter().map(PathBuf::from).collect();

                let mut doc = LopdfDocument::new();
                let op = paperpilot_pdf::operations::merge::MergeOperation::new(paths);
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "PDFs merged successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_split" => {
                let input = get_string("input")?;
                let output_dir = get_string("output_dir")?;

                let mut doc = LopdfDocument::load(&PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;
                let total_pages = doc.page_count().map_err(crate::error::to_mcp_error)?;

                let split_points: Vec<u32> = (1..total_pages).collect();
                let op = paperpilot_pdf::operations::split::SplitOperation::new(
                    PathBuf::from(&output_dir),
                    split_points,
                );
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "PDF split successfully.".to_string(),
                    output_path: Some(output_dir),
                })
            }
            "pdf_extract_pages" => {
                let input = get_string("input")?;
                let pages = get_string("pages")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let page_indices = parse_pages(&pages)?;

                let mut doc = LopdfDocument::load(&PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;
                let op =
                    paperpilot_pdf::operations::extract::ExtractPagesOperation::new(page_indices);
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Pages extracted successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_delete_pages" => {
                let input = get_string("input")?;
                let pages = get_string("pages")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let page_indices = parse_pages(&pages)?;

                let mut doc = LopdfDocument::load(&PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;
                let op =
                    paperpilot_pdf::operations::delete::DeletePagesOperation::new(page_indices);
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Pages deleted successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_reorder_pages" => {
                let input = get_string("input")?;
                let order = get_string("order")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let new_order = parse_pages(&order)?;

                let mut doc = LopdfDocument::load(&PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;
                let op = paperpilot_pdf::operations::reorder::ReorderPagesOperation::new(new_order);
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Pages reordered successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_rotate" => {
                let input = get_string("input")?;
                let pages = get_string("pages")?;
                let angle = get_number("angle")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let page_indices = parse_pages(&pages)?;
                let opt_pages = if page_indices.is_empty() {
                    None
                } else {
                    Some(page_indices)
                };

                let mut doc = LopdfDocument::load(&PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;
                let op = paperpilot_pdf::operations::rotate::RotatePagesOperation::new(
                    angle as u16,
                    opt_pages,
                );
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Pages rotated successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_crop" => {
                let input = get_string("input")?;
                let box_str = get_string("box")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let bounds: Vec<&str> = box_str.split(',').collect();
                if bounds.len() != 4 {
                    return Err(ErrorData::invalid_params("Invalid crop box format", None));
                }
                let left = bounds[0]
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| ErrorData::invalid_params("Invalid crop box", None))?;
                let bottom = bounds[1]
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| ErrorData::invalid_params("Invalid crop box", None))?;
                let right = bounds[2]
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| ErrorData::invalid_params("Invalid crop box", None))?;
                let top = bounds[3]
                    .trim()
                    .parse::<f32>()
                    .map_err(|_| ErrorData::invalid_params("Invalid crop box", None))?;

                let crop_box = paperpilot_pdf::operations::crop::CropBox {
                    left,
                    bottom,
                    right,
                    top,
                };

                let mut doc = LopdfDocument::load(&PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;
                let op = paperpilot_pdf::operations::crop::CropPagesOperation::new(crop_box);
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Pages cropped successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_burst" => {
                let input = get_string("input")?;
                let output_dir = get_string("output_dir")?;

                let mut doc = LopdfDocument::load(&PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;
                let op = paperpilot_pdf::operations::burst::BurstOperation::new(PathBuf::from(
                    &output_dir,
                ));
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "PDF burst successfully.".to_string(),
                    output_path: Some(output_dir),
                })
            }

            "pdf_compress" => {
                let input = get_string("input")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;
                let op = paperpilot_pdf::operations::compress::CompressOperation::new();
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&std::path::PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "PDF compressed successfully.".to_string(),
                    output_path: Some(output),
                })
            }

            "pdf_extract_text" => {
                let input = get_string("input")?;
                let output = get_string("output")?;
                let pages = get_string("pages").ok();

                ensure_parent_dir(&output)?;

                let page_indices = if let Some(p) = pages {
                    let parsed = parse_pages(&p)?;
                    if parsed.is_empty() {
                        None
                    } else {
                        Some(parsed)
                    }
                } else {
                    None
                };

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::extract_text::ExtractTextOperation::new(
                    page_indices,
                );
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                let text_pages = op
                    .extracted_text
                    .lock()
                    .unwrap()
                    .clone()
                    .unwrap_or_default();
                let joined_text = text_pages.join("\n");
                std::fs::write(&output, joined_text)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

                Ok(OperationResult {
                    success: true,
                    message: "Text extracted successfully.".to_string(),
                    output_path: Some(output),
                })
            }

            "pdf_extract_images" => {
                let input = get_string("input")?;
                let output_dir = get_string("output_dir")?;

                ensure_dir(&output_dir)?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::extract_images::ExtractImagesOperation::new(
                    std::path::PathBuf::from(&output_dir),
                );
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Images extracted successfully.".to_string(),
                    output_path: Some(output_dir),
                })
            }

            "pdf_search" => {
                let input = get_string("input")?;
                let query = get_string("query")?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::search::SearchOperation::new(query);
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                let matches = op.match_pages.lock().unwrap().clone();
                let message = if matches.is_empty() {
                    "No matches found.".to_string()
                } else {
                    format!("Matches found on pages: {:?}", matches)
                };

                Ok(OperationResult {
                    success: true,
                    message,
                    output_path: None,
                })
            }

            "pdf_watermark" => {
                let input = get_string("input")?;
                let text = get_string("text")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::watermark::WatermarkOperation::new(text);
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&std::path::PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Watermark applied successfully.".to_string(),
                    output_path: Some(output),
                })
            }

            "pdf_encrypt" => {
                let input = get_string("input")?;
                let password = get_string("password")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let mut op = paperpilot_pdf::operations::encrypt::EncryptOperation::new();
                op.user_password = Some(password.clone());
                op.owner_password = Some(password);

                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&std::path::PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "PDF encrypted successfully.".to_string(),
                    output_path: Some(output),
                })
            }

            "pdf_decrypt" => {
                let input = get_string("input")?;
                let password = get_string("password")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::decrypt::DecryptOperation::new(Some(password));
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;
                doc.save(&std::path::PathBuf::from(&output))
                    .map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "PDF decrypted successfully.".to_string(),
                    output_path: Some(output),
                })
            }

            "pdf_metadata" => {
                let input = get_string("input")?;
                let title = get_string("title").ok();
                let author = get_string("author").ok();
                let output = get_string("output").ok();

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let mut op = paperpilot_pdf::operations::metadata::MetadataOperation::new();
                op.title = title;
                op.author = author;

                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                let mut is_update = false;
                if let Some(out_path) = &output {
                    ensure_parent_dir(out_path)?;
                    doc.save(&std::path::PathBuf::from(out_path))
                        .map_err(crate::error::to_mcp_error)?;
                    is_update = true;
                }

                let message = if is_update {
                    "Metadata updated successfully.".to_string()
                } else {
                    let map = op.retrieved_metadata.lock().unwrap().clone();
                    if map.is_empty() {
                        "No metadata found.".to_string()
                    } else {
                        format!("Metadata: {:?}", map)
                    }
                };

                Ok(OperationResult {
                    success: true,
                    message,
                    output_path: output,
                })
            }
                        "pdf_to_docx" => {
                let input = get_string("input")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::conversion::PdfToDocxOperation::new(std::path::PathBuf::from(&output));
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Converted to DOCX successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_to_xlsx" => {
                let input = get_string("input")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::conversion::PdfToXlsxOperation::new(std::path::PathBuf::from(&output));
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Converted to XLSX successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_to_pptx" => {
                let input = get_string("input")?;
                let output = get_string("output")?;

                ensure_parent_dir(&output)?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::conversion::PdfToPptxOperation::new(std::path::PathBuf::from(&output));
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                Ok(OperationResult {
                    success: true,
                    message: "Converted to PPTX successfully.".to_string(),
                    output_path: Some(output),
                })
            }
            "pdf_classify_type" => {
                let input = get_string("input")?;

                let mut doc = LopdfDocument::load(&std::path::PathBuf::from(&input))
                    .map_err(crate::error::to_mcp_error)?;

                let op = paperpilot_pdf::operations::classify::PdfClassifyOperation::new();
                op.execute(&mut doc).map_err(crate::error::to_mcp_error)?;

                let classification = op.classification.lock().unwrap().clone();
                let message = if let Some(c) = classification {
                    serde_json::to_string(&c).unwrap_or_else(|_| "Failed to serialize classification".to_string())
                } else {
                    "Classification failed".to_string()
                };

                Ok(OperationResult {
                    success: true,
                    message,
                    output_path: None,
                })
            }
            _ => Err(ErrorData::invalid_params("Unknown tool", None)),
        };

        match result {
            Ok(res) => Ok(CallToolResponse::Complete(CallToolResult::success(vec![
                ContentBlock::text(serde_json::to_string(&res).unwrap()),
            ]))),
            Err(e) => Err(e),
        }
    }
}

#[allow(clippy::manual_async_fn)]
impl ServerHandler for PaperPilotMcpServer {
    fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, rmcp::ErrorData>> + MaybeSendFuture + '_ {
        async move { Self::execute_list_tools() }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResponse, rmcp::ErrorData>> + MaybeSendFuture + '_
    {
        async move { Self::execute_call_tool(request) }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_list_tools() {
        let res = PaperPilotMcpServer::execute_list_tools().unwrap();
        assert_eq!(res.tools.len(), 20);
        assert_eq!(res.tools[0].name, "pdf_merge");
        assert_eq!(res.tools[1].name, "pdf_split");
    }

    #[test]
    fn test_execute_call_tool_unknown() {
        let mut request = CallToolRequestParams::default();
        request.name = "unknown_tool".into();
        request.arguments = None;
        let res = PaperPilotMcpServer::execute_call_tool(request);
        assert!(res.is_err());
    }

    #[test]
    fn test_execute_call_tool_merge_validation() {
        let mut args = serde_json::Map::new();
        args.insert(
            "inputs".to_string(),
            serde_json::Value::Array(vec![
                serde_json::Value::String("dummy1.pdf".to_string()),
                serde_json::Value::String("dummy2.pdf".to_string()),
            ]),
        );
        args.insert(
            "output".to_string(),
            serde_json::Value::String("out.pdf".to_string()),
        );
        let mut request = CallToolRequestParams::default();
        request.name = "pdf_merge".into();
        request.arguments = Some(args);

        // It will fail execution because dummy files don't exist, but it should successfully route
        let res = PaperPilotMcpServer::execute_call_tool(request);
        assert!(res.is_err()); // Due to file not found, meaning it correctly hit the logic
    }

    #[test]
    fn test_execute_call_tool_split_validation() {
        let mut args = serde_json::Map::new();
        args.insert(
            "input".to_string(),
            serde_json::Value::String("dummy.pdf".to_string()),
        );
        args.insert(
            "output_dir".to_string(),
            serde_json::Value::String("out_dir".to_string()),
        );
        let mut request = CallToolRequestParams::default();
        request.name = "pdf_split".into();
        request.arguments = Some(args);

        let res = PaperPilotMcpServer::execute_call_tool(request);
        assert!(res.is_err()); // Due to file not found
    }

    #[test]
    fn test_execute_call_tool_extract_text() {
        let mut args = serde_json::Map::new();
        args.insert(
            "input".to_string(),
            serde_json::Value::String("dummy.pdf".to_string()),
        );
        args.insert(
            "output".to_string(),
            serde_json::Value::String("out.txt".to_string()),
        );
        let mut request = CallToolRequestParams::default();
        request.name = "pdf_extract_text".into();
        request.arguments = Some(args);

        let res = PaperPilotMcpServer::execute_call_tool(request);
        assert!(res.is_err());
    }

    #[test]
    fn test_execute_call_tool_metadata() {
        let mut args = serde_json::Map::new();
        args.insert(
            "input".to_string(),
            serde_json::Value::String("dummy.pdf".to_string()),
        );
        args.insert(
            "title".to_string(),
            serde_json::Value::String("My Title".to_string()),
        );
        let mut request = CallToolRequestParams::default();
        request.name = "pdf_metadata".into();
        request.arguments = Some(args);

        let res = PaperPilotMcpServer::execute_call_tool(request);
        assert!(res.is_err());
    }

    #[test]
    fn test_execute_call_tool_invalid_page_format() {
        let mut args = serde_json::Map::new();
        args.insert(
            "input".to_string(),
            serde_json::Value::String("dummy.pdf".to_string()),
        );
        args.insert(
            "pages".to_string(),
            serde_json::Value::String("1,x,3".to_string()), // Invalid
        );
        args.insert(
            "output".to_string(),
            serde_json::Value::String("out.pdf".to_string()),
        );
        let mut request = CallToolRequestParams::default();
        request.name = "pdf_extract_pages".into();
        request.arguments = Some(args);

        let res = PaperPilotMcpServer::execute_call_tool(request);
        assert!(res.is_err());
        if let Err(e) = res {
            assert_eq!(e.code, rmcp::model::ErrorCode(-32602)); // invalid params
        } else {
            panic!("Expected Err");
        }
    }
}
