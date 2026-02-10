pub mod matcher;

pub use matcher::FuzzyMatchEngine;

use crate::fs::DirEntry;
use crate::fuzzy::matcher::MatchResult;

/// State for fuzzy search mode
#[derive(Debug, Clone)]
pub struct FuzzyState {
    entries: Vec<DirEntry>,
    filtered: Vec<FilteredEntry>,
    search_query: String,
    selection_index: usize,
}

#[derive(Debug, Clone)]
pub struct FilteredEntry {
    pub entry: DirEntry,
    pub match_result: Option<MatchResult>,
}

impl FuzzyState {
    pub fn with_entries(entries: Vec<DirEntry>) -> Self {
        let filtered = entries
            .iter()
            .map(|e| FilteredEntry {
                entry: e.clone(),
                match_result: None,
            })
            .collect();

        Self {
            entries,
            filtered,
            search_query: String::new(),
            selection_index: 0,
        }
    }

    pub fn set_query(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.filter();
    }

    pub fn clear_query(&mut self) {
        self.search_query.clear();
        self.filter();
    }

    pub fn add_char(&mut self, c: char) {
        self.search_query.push(c);
        self.filter();
    }

    pub fn pop_char(&mut self) {
        self.search_query.pop();
        self.filter();
    }

    fn filter(&mut self) {
        let engine = FuzzyMatchEngine::new();
        let query = &self.search_query;

        let mut filtered: Vec<FilteredEntry> = self
            .entries
            .iter()
            .map(|e| {
                let match_result = if query.is_empty() {
                    Some(MatchResult {
                        text: e.name.clone(),
                        score: 0,
                        indices: Vec::new(),
                        is_match: true,
                    })
                } else {
                    engine.match_text(query, &e.name)
                };

                FilteredEntry {
                    entry: e.clone(),
                    match_result,
                }
            })
            .filter(|f| f.match_result.as_ref().map(|r| r.is_match).unwrap_or(false))
            .collect();

        filtered.sort_by(|a, b| {
            let score_a = a.match_result.as_ref().map(|r| r.score).unwrap_or(i64::MIN);
            let score_b = b.match_result.as_ref().map(|r| r.score).unwrap_or(i64::MIN);
            score_b.cmp(&score_a)
        });

        self.filtered = filtered;
        self.selection_index = 0;
    }

    pub fn move_up(&mut self) {
        if self.selection_index > 0 {
            self.selection_index = self.selection_index.saturating_sub(1);
        }
    }

    pub fn move_down(&mut self) {
        if self.selection_index + 1 < self.filtered.len() {
            self.selection_index = self.selection_index.saturating_add(1);
        }
    }

    pub fn page_up(&mut self) {
        self.selection_index = self.selection_index.saturating_sub(10);
    }

    pub fn page_down(&mut self) {
        self.selection_index = std::cmp::min(
            self.selection_index + 10,
            self.filtered.len().saturating_sub(1),
        );
    }

    pub fn go_to_start(&mut self) {
        self.selection_index = 0;
    }

    pub fn go_to_end(&mut self) {
        self.selection_index = self.filtered.len().saturating_sub(1);
    }

    pub fn selected_item(&self) -> Option<&DirEntry> {
        self.filtered.get(self.selection_index).map(|f| &f.entry)
    }

    pub fn is_searching(&self) -> bool {
        !self.search_query.is_empty()
    }

    pub fn query(&self) -> &str {
        &self.search_query
    }

    pub fn len(&self) -> usize {
        self.filtered.len()
    }

    pub fn is_empty(&self) -> bool {
        self.filtered.is_empty()
    }

    pub fn selection_index(&self) -> usize {
        self.selection_index
    }
}
