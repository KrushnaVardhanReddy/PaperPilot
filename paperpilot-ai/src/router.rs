use paperpilot_nlp::resolver::OfflineNlpResolver;
use paperpilot_nlp::traits::{NlpError, NlpResolver, OperationPlan, ResolverContext};

use crate::resolver::LlmNlpResolver;

pub enum RouterMode {
    OfflineOnly,
    LlmWithFallback(LlmNlpResolver),
}

pub struct DynamicRouter {
    mode: RouterMode,
    offline_resolver: OfflineNlpResolver,
}

impl DynamicRouter {
    pub fn new(mode: RouterMode) -> Self {
        Self {
            mode,
            offline_resolver: OfflineNlpResolver::new(),
        }
    }
}

impl NlpResolver for DynamicRouter {
    fn resolve_with_context(
        &self,
        query: &str,
        context: &ResolverContext,
    ) -> Result<OperationPlan, NlpError> {
        match &self.mode {
            RouterMode::OfflineOnly => self.offline_resolver.resolve_with_context(query, context),
            RouterMode::LlmWithFallback(llm_resolver) => {
                match llm_resolver.resolve_with_context(query, context) {
                    Ok(plan) => Ok(plan),
                    Err(err) => {
                        // Fallback to offline
                        match self.offline_resolver.resolve_with_context(query, context) {
                            Ok(plan) => Ok(plan),
                            Err(_) => Err(err), // Return original LLM error if both fail
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_offline_mode() {
        let router = DynamicRouter::new(RouterMode::OfflineOnly);
        let ctx = ResolverContext::default();
        let plan = router.resolve_with_context("merge a.pdf b.pdf to out.pdf", &ctx).unwrap();
        assert_eq!(plan.intent, paperpilot_nlp::intent::Intent::Merge);
    }
}
