use crate::traits::{NlpResolver, OperationPlan, NlpError};
use crate::layer1::RuleEngine;
use crate::entities::extract_entities;
use crate::ambiguity::check_completeness;

pub struct OfflineNlpResolver {
    engine: RuleEngine,
}

impl Default for OfflineNlpResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl OfflineNlpResolver {
    pub fn new() -> Self {
        OfflineNlpResolver { engine: RuleEngine::new() }
    }
}

impl NlpResolver for OfflineNlpResolver {
    fn resolve(&self, query: &str) -> Result<OperationPlan, NlpError> {
        // 1. Classify the intent
        let intent = self.engine.predict(query)
            .ok_or_else(|| NlpError::AmbiguousIntent(
                format!("Could not identify a PDF operation in: '{}'", query)
            ))?;

        // 2. Extract entities
        let entities = extract_entities(query);

        // 3. Check completeness
        check_completeness(&intent, &entities)?;

        // 4. Build and return the plan
        Ok(OperationPlan::new(intent, entities, query.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::Intent;

    #[test]
    fn test_resolve_merge() {
        let resolver = OfflineNlpResolver::new();
        let plan = resolver.resolve("merge a.pdf and b.pdf into output.pdf").unwrap();
        assert_eq!(plan.intent, Intent::Merge);
        assert_eq!(plan.input_files.len(), 2);
        assert_eq!(plan.input_files[0], "a.pdf");
        assert_eq!(plan.input_files[1], "b.pdf");
        assert_eq!(plan.output_file, Some("output.pdf".to_string()));
    }

    #[test]
    fn test_resolve_rotate() {
        let resolver = OfflineNlpResolver::new();
        let plan = resolver.resolve("rotate doc.pdf 90 degrees").unwrap();
        assert_eq!(plan.intent, Intent::Rotate);
        assert_eq!(plan.input_files.len(), 1);
        assert_eq!(plan.input_files[0], "doc.pdf");
        assert_eq!(plan.angles, vec![90]);
        assert_eq!(plan.output_file, None);
    }

    #[test]
    fn test_resolve_ambiguous() {
        let resolver = OfflineNlpResolver::new();
        let res = resolver.resolve("do something unrelated");
        assert!(res.is_err());
        if let Err(NlpError::AmbiguousIntent(msg)) = res {
            assert!(msg.contains("Could not identify a PDF operation in"));
        } else {
            panic!("Expected AmbiguousIntent error");
        }
    }

    #[test]
    fn test_resolve_missing_parameters() {
        let resolver = OfflineNlpResolver::new();
        let res = resolver.resolve("merge");
        assert!(res.is_err());
        if let Err(NlpError::MissingParameters(msg)) = res {
            assert!(msg.contains("requires at least one PDF file path"));
        } else {
            panic!("Expected MissingParameters error");
        }
    }
}
