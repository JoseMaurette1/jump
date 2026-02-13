use std::sync::Arc;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Clone)]
pub struct FuzzyMatchEngine {
    matcher: Arc<SkimMatcherV2>,
}

impl std::fmt::Debug for FuzzyMatchEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FuzzyMatchEngine").finish()
    }
}

impl FuzzyMatchEngine {
    pub fn new() -> Self {
        Self {
            matcher: Arc::new(SkimMatcherV2::default()),
        }
    }

    pub fn get_score(&self, pattern: &str, text: &str) -> Option<i64> {
        (*self.matcher).fuzzy_match(text, pattern)
    }

    pub fn get_indices(&self, pattern: &str, text: &str) -> Option<Vec<usize>> {
        (*self.matcher)
            .fuzzy_indices(text, pattern)
            .map(|(_, indices)| indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_engine_new() {
        let matcher = FuzzyMatchEngine::new();
        let score = matcher.get_score("abc", "axbycz");

        assert!(score.is_some());
        assert!(score.unwrap() > 0);
    }

    #[test]
    fn test_get_indices() {
        let matcher = FuzzyMatchEngine::new();
        let indices = matcher.get_indices("abc", "axbycz");

        assert_eq!(indices, Some(vec![0, 2, 4]));
    }
}
