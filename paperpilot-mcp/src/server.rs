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
        assert_eq!(res.tools.len(), 8);
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
                serde_json::Value::String("dummy2.pdf".to_string())
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
