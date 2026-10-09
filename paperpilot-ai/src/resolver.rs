use paperpilot_nlp::entities::ExtractedEntities;
use paperpilot_nlp::intent::Intent;
use paperpilot_nlp::traits::{NlpError, NlpResolver, OperationPlan, ResolverContext};
use serde_json::json;

use crate::client::{ChatMessage, FunctionDefinition, ToolDefinition, UniversalLlmClient};

pub struct LlmNlpResolver {
    client: UniversalLlmClient,
}

impl LlmNlpResolver {
    pub fn new(client: UniversalLlmClient) -> Self {
        Self { client }
    }

    fn get_tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: "pdf_merge".to_string(),
                    description: "Merge multiple PDF documents into one.".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "input_files": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "The PDF files to merge."
                            },
                            "output_file": {
                                "type": "string",
                                "description": "The resulting merged PDF file."
                            }
                        },
                        "required": ["input_files"]
                    }),
                },
            },
            ToolDefinition {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: "pdf_rotate".to_string(),
                    description: "Rotate pages in a PDF document.".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "input_files": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "The PDF files to rotate."
                            },
                            "angle": {
                                "type": "integer",
                                "description": "The angle to rotate by (e.g. 90, 180, 270)."
                            }
                        },
                        "required": ["input_files", "angle"]
                    }),
                },
            },
            ToolDefinition {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: "pdf_redact".to_string(),
                    description: "Redact sensitive information from a document.".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "input_files": {
                                "type": "array",
                                "items": { "type": "string" }
                            }
                        },
                        "required": ["input_files"]
                    }),
                },
            },
            ToolDefinition {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: "pdf_bates".to_string(),
                    description: "Apply bates stamping to a document.".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "input_files": {
                                "type": "array",
                                "items": { "type": "string" }
                            }
                        },
                        "required": ["input_files"]
                    }),
                },
            },
            ToolDefinition {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: "pdf_watermark".to_string(),
                    description: "Apply watermark to a document.".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "input_files": {
                                "type": "array",
                                "items": { "type": "string" }
                            }
                        },
                        "required": ["input_files"]
                    }),
                },
            },
             ToolDefinition {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: "pdf_hash".to_string(),
                    description: "Hash a document.".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "input_files": {
                                "type": "array",
                                "items": { "type": "string" }
                            }
                        },
                        "required": ["input_files"]
                    }),
                },
            },
            ToolDefinition {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: "pdf_linearize".to_string(),
                    description: "Linearize a document.".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "input_files": {
                                "type": "array",
                                "items": { "type": "string" }
                            }
                        },
                        "required": ["input_files"]
                    }),
                },
            },
        ]
    }

    fn map_tool_to_plan(&self, name: &str, arguments: &str, raw_query: &str) -> Result<OperationPlan, NlpError> {
        let args: serde_json::Value = serde_json::from_str(arguments).map_err(|e| {
            NlpError::InternalError(format!("Failed to parse tool arguments: {}", e))
        })?;

        let intent = match name {
            "pdf_merge" => Intent::Merge,
            "pdf_rotate" => Intent::Rotate,
            "pdf_redact" => Intent::Redact,
            "pdf_bates" => Intent::Bates,
            "pdf_watermark" => Intent::Watermark,
            "pdf_hash" => Intent::Hash,
            "pdf_linearize" => Intent::Linearize,
            _ => return Err(NlpError::AmbiguousIntent(format!("Unknown tool: {}", name))),
        };

        let mut entities = ExtractedEntities::default();

        if let Some(input_files) = args.get("input_files").and_then(|v| v.as_array()) {
            for file in input_files {
                if let Some(s) = file.as_str() {
                    entities.files.push(s.to_string());
                }
            }
        }

        if let Some(angle) = args.get("angle").and_then(|v| v.as_i64()) {
            entities.angles.push(angle as i32);
        }

        let plan = OperationPlan::new(intent, entities, raw_query.to_string());

        // OperationPlan::new might pop the output file for Merge intent but what if LLM specified output_file explicitly?
        // Let's explicitly set it if LLM gave it and the heuristic missed it or we want to trust LLM.
        let mut final_plan = plan;
        if let Some(output_file) = args.get("output_file").and_then(|v| v.as_str()) {
             // If heuristic already popped the last input file to use as output, we need to put it back
             // into input_files because the LLM explicitly provided the output_file.
             if final_plan.intent == Intent::Merge {
                  // Wait, OperationPlan::new split it. Let's just reconstruct input_files
                  // from our original entities to bypass the heuristic completely.
                  if let Some(input_files) = args.get("input_files").and_then(|v| v.as_array()) {
                      let mut full_inputs = Vec::new();
                      for file in input_files {
                          if let Some(s) = file.as_str() {
                              full_inputs.push(s.to_string());
                          }
                      }
                      final_plan.input_files = full_inputs;
                  }
             }
             final_plan.output_file = Some(output_file.to_string());
        }

        Ok(final_plan)
    }
}

impl NlpResolver for LlmNlpResolver {
    fn resolve_with_context(
        &self,
        query: &str,
        context: &ResolverContext,
    ) -> Result<OperationPlan, NlpError> {
        let mut system_prompt = "You are a PDF processing assistant. Your task is to select the appropriate tool to fulfill the user's request. You MUST use one of the provided tools.".to_string();

        if let Some(active) = &context.active_document {
            system_prompt.push_str(&format!("\nThe active document is '{}'. If the user does not specify a document, assume they mean this one.", active));
        }

        if !context.open_documents.is_empty() {
             system_prompt.push_str(&format!("\nOpen documents are: {:?}.", context.open_documents));
        }

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: query.to_string(),
            },
        ];

        let tools = self.get_tools();

        // Since NlpResolver is synchronous, we need to spawn a new thread with a basic runtime
        // or just use tokio::task::block_in_place if we are in a tokio runtime.
        // However, a simple thread with block_on works even without an existing runtime.
        let client = &self.client;
        let messages_clone = messages.clone();
        let tools_clone = Some(tools.clone());

        // Wait, self doesn't implement Clone and UniversalLlmClient doesn't either, so we can't easily move it into a new thread without references.
        // `tokio::runtime::Handle::current().block_on` fails if we're already inside a runtime.
        // `tokio::task::block_in_place` works if we are inside a multi-thread tokio runtime.
        // The safest approach is to create a new thread and build a current-thread runtime there,
        // but we'd need a cloned client. Let's make LlmConfig cloneable (it is) and we can just create a new client,
        // OR we can make UniversalLlmClient implement Clone by cloning its config and reqwest::Client (which is just an Arc).

        let client_clone = client.clone();
        let response = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async {
                client_clone.chat_completion(messages_clone, tools_clone).await
            })
        }).join().map_err(|_| NlpError::InternalError("Thread panicked".to_string()))?
        .map_err(|e| NlpError::InternalError(format!("LLM Request failed: {:?}", e)))?;

        if let Some(choice) = response.choices.first()
            && let Some(tool_call) = choice.message.tool_calls.first()
        {
            return self.map_tool_to_plan(
                &tool_call.function.name,
                &tool_call.function.arguments,
                query,
            );
        }

        Err(NlpError::AmbiguousIntent("Model did not return a valid tool call".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::LlmConfig;

    #[test]
    fn test_map_tool_to_plan_merge() {
        let config = LlmConfig {
            endpoint_url: "http://localhost:11434/v1".to_string(),
            model: "llama".to_string(),
            api_key: None,
            temperature: None,
        };
        let client = UniversalLlmClient::new(config);
        let resolver = LlmNlpResolver::new(client);

        let args = r#"{"input_files": ["a.pdf", "b.pdf"], "output_file": "out.pdf"}"#;
        let plan = resolver.map_tool_to_plan("pdf_merge", args, "merge a and b").unwrap();

        assert_eq!(plan.intent, Intent::Merge);
        assert_eq!(plan.input_files, vec!["a.pdf", "b.pdf"]);
        assert_eq!(plan.output_file, Some("out.pdf".to_string()));
    }

    #[test]
    fn test_map_tool_to_plan_rotate() {
        let config = LlmConfig {
            endpoint_url: "http://localhost:11434/v1".to_string(),
            model: "llama".to_string(),
            api_key: None,
            temperature: None,
        };
        let client = UniversalLlmClient::new(config);
        let resolver = LlmNlpResolver::new(client);

        let args = r#"{"input_files": ["doc.pdf"], "angle": 90}"#;
        let plan = resolver.map_tool_to_plan("pdf_rotate", args, "rotate doc by 90").unwrap();

        assert_eq!(plan.intent, Intent::Rotate);
        assert_eq!(plan.input_files, vec!["doc.pdf"]);
        assert_eq!(plan.angles, vec![90]);
    }
}
