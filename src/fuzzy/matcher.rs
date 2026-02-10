use std::cmp::Ordering;
use std::sync::Arc;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

/// A fuzzy-matched result with score and indices
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub text: String,
    pub score: i64,
    pub indices: Vec<usize>,
    pub is_match: bool,
}

impl PartialEq for MatchResult {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Eq for MatchResult {}

impl PartialOrd for MatchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MatchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

/// Fuzzy matcher using the skim v2 algorithm (fzf-compatible)
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

    pub fn match_text(&self, pattern: &str, text: &str) -> Option<MatchResult> {
        if pattern.is_empty() {
            return Some(MatchResult {
                text: text.to_string(),
                score: 0,
                indices: Vec::new(),
                is_match: true,
            });
        }

        let result = (*self.matcher).fuzzy_indices(text, pattern);

        match result {
            Some((score, indices)) => Some(MatchResult {
                text: text.to_string(),
                score,
                indices,
                is_match: true,
            }),
            None => Some(MatchResult {
                text: text.to_string(),
                score: i64::MIN,
                indices: Vec::new(),
                is_match: false,
            }),
        }
    }

    pub fn match_list<'a>(
        &self,
        pattern: &str,
        texts: impl IntoIterator<Item = &'a str>,
    ) -> Vec<MatchResult> {
        let mut results: Vec<MatchResult> = texts
            .into_iter()
            .filter_map(|text| {
                let result = self.match_text(pattern, text);
                if pattern.is_empty()
                    || result.as_ref().map(|r| r.score).unwrap_or(i64::MIN) > i64::MIN
                {
                    result
                } else {
                    None
                }
            })
            .collect();

        results.sort();
        results
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
        let result = matcher.match_text("abc", "axbycz").unwrap();

        assert!(result.is_match);
        assert!(result.score > 0);
        assert_eq!(result.indices, vec![0, 2, 4]);
    }

    #[test]
    fn test_match_text_no_match() {
        let matcher = FuzzyMatchEngine::new();
        let result = matcher.match_text("xyz", "abc").unwrap();

        assert!(!result.is_match);
        assert_eq!(result.score, i64::MIN);
    }

    #[test]
    fn test_match_list() {
        let matcher = FuzzyMatchEngine::new();
        let texts = vec!["apple", "apricot", "banana", "cherry"];
        let results = matcher.match_list("ap", texts.iter().copied());

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].text, "apple");
        assert_eq!(results[1].text, "apricot");
    }
}
