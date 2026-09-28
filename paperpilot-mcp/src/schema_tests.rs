use crate::server::PaperPilotMcpServer;

#[test]
fn test_schema_validation() {
    let list_tools_result =
        PaperPilotMcpServer::execute_list_tools().expect("Failed to list tools");

    assert!(
        !list_tools_result.tools.is_empty(),
        "Tools list should not be empty"
    );

    for tool in list_tools_result.tools {
        let schema = tool.input_schema;

        let type_val = schema.get("type").expect("Schema must have a 'type'");
        assert_eq!(
            type_val.as_str().unwrap(),
            "object",
            "Schema type must be 'object'"
        );

        assert!(
            schema.get("properties").is_some(),
            "Schema must have 'properties'"
        );
        assert!(
            schema.get("required").is_some(),
            "Schema must have 'required'"
        );
    }
}
