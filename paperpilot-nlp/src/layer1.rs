use aho_corasick::{AhoCorasick, MatchKind};
use crate::intent::Intent;

pub struct RuleEngine {
    ac: AhoCorasick,
    pattern_to_intent: Vec<Intent>,
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleEngine {
    pub fn new() -> Self {
        let mut patterns = Vec::new();
        let mut pattern_to_intent = Vec::new();

        for intent in Intent::all() {
            let def = intent.definition();
            for alias in def.aliases {
                patterns.push(*alias);
                pattern_to_intent.push(intent);
            }
        }

        let ac = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .match_kind(MatchKind::LeftmostLongest)
            .build(patterns)
            .expect("Failed to build AhoCorasick automaton");

        Self {
            ac,
            pattern_to_intent,
        }
    }

    /// Attempts to find a matching intent in the text.
    /// Returns Some(Intent) if a strong keyword match is found, otherwise None.
    pub fn predict(&self, text: &str) -> Option<Intent> {
        self.ac.find(text).map(|mat| self.pattern_to_intent[mat.pattern().as_usize()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_merge() {
        let engine = RuleEngine::new();
        assert_eq!(
            engine.predict("Please merge these files for me"),
            Some(Intent::Merge)
        );
    }

    #[test]
    fn test_predict_rotate() {
        let engine = RuleEngine::new();
        assert_eq!(
            engine.predict("rotate the document 90 degrees"),
            Some(Intent::Rotate)
        );
    }

    #[test]
    fn test_predict_none() {
        let engine = RuleEngine::new();
        assert_eq!(
            engine.predict("this string has absolutely no valid pdf commands"),
            None
        );
    }

    #[test]
    fn test_predict_case_insensitive() {
        let engine = RuleEngine::new();
        assert_eq!(
            engine.predict("MERGE THESE FILES"),
            Some(Intent::Merge)
        );
    }

    #[test]
    fn test_predict_leftmost_longest() {
        let engine = RuleEngine::new();
        // Intent::ExtractImages has alias "extract images", which should be matched over "extract" from Intent::Extract
        assert_eq!(
            engine.predict("please extract images from this pdf"),
            Some(Intent::ExtractImages)
        );
    }
}
