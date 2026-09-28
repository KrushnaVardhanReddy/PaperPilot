use crate::schema::OperationResult;
use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, ListToolsResult, Tool,
};
use rmcp::service::{MaybeSendFuture, RequestContext};
use rmcp::{ErrorData, RoleServer};
use std::future::Future;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct PaperPilotMcpServer;

impl PaperPilotMcpServer {
    pub fn new() -> Self {
        Self
    }

    // Extracted logic to allow unit testing without rmcp RequestContext
    pub fn execute_list_tools() -> Result<ListToolsResult, ErrorData> {
        let mut schema = serde_json::Map::new();
        schema.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );

        let mut properties = serde_json::Map::new();
        let mut input_prop = serde_json::Map::new();
        input_prop.insert(
            "type".to_string(),
            serde_json::Value::String("string".to_string()),
        );
        input_prop.insert(
            "description".to_string(),
            serde_json::Value::String("The string to echo back.".to_string()),
        );
        properties.insert("input".to_string(), serde_json::Value::Object(input_prop));

        schema.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties),
        );

        schema.insert(
            "required".to_string(),
            serde_json::Value::Array(vec![serde_json::Value::String("input".to_string())]),
        );

        let mut tool = Tool::default();
        tool.name = "hello_world".into();
        tool.description = Some("A simple echo tool for testing the MCP connection.".into());
        tool.input_schema = Arc::new(schema);

        let result = ListToolsResult {
            tools: vec![tool],
            ..Default::default()
        };

        Ok(result)
    }

    pub fn execute_call_tool(
        request: CallToolRequestParams,
    ) -> Result<CallToolResponse, ErrorData> {
        match request.name.as_ref() {
            "hello_world" => {
                let input = request
                    .arguments
                    .and_then(|args| args.get("input").cloned())
                    .and_then(|val| val.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "World".to_string());

                let result = OperationResult {
                    success: true,
                    message: format!("Echo: {}", input),
                    output_path: None,
                };

                Ok(CallToolResponse::Complete(CallToolResult::success(vec![
                    ContentBlock::text(serde_json::to_string(&result).unwrap()),
                ])))
            }
            _ => Err(ErrorData::invalid_params("Unknown tool", None)),
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
        assert_eq!(res.tools.len(), 1);
        assert_eq!(res.tools[0].name, "hello_world");
    }

    #[test]
    fn test_execute_call_tool_success() {
        let mut args = serde_json::Map::new();
        args.insert(
            "input".to_string(),
            serde_json::Value::String("Test".to_string()),
        );
        let mut request = CallToolRequestParams::default();
        request.name = "hello_world".into();
        request.arguments = Some(args);

        let res = PaperPilotMcpServer::execute_call_tool(request).unwrap();
        if let CallToolResponse::Complete(result) = res {
            assert_eq!(result.content.len(), 1);
            if let ContentBlock::Text(text_block) = &result.content[0] {
                let parsed: OperationResult = serde_json::from_str(&text_block.text).unwrap();
                assert!(parsed.success);
                assert_eq!(parsed.message, "Echo: Test");
            } else {
                panic!("Expected Text content block");
            }
        } else {
            panic!("Expected Complete response");
        }
    }

    #[test]
    fn test_execute_call_tool_unknown() {
        let mut request = CallToolRequestParams::default();
        request.name = "unknown_tool".into();
        request.arguments = None;
        let res = PaperPilotMcpServer::execute_call_tool(request);
        assert!(res.is_err());
    }
}
