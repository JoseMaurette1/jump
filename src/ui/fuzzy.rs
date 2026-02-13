use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use std::path::{Path, PathBuf};

use crate::fs::{self, DirEntry};
use crate::fuzzy::FuzzyMatchEngine;

/// Draw the fuzzy search TUI
pub fn draw_fuzzy(frame: &mut Frame, state: &FuzzyState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    // Draw search input
    let search_display = format!(" Search: {} ", state.search_query);
    let search_style = if state.search_query.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let search_block = Paragraph::new(search_display).style(search_style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", state.current_dir.display())),
    );

    frame.render_widget(search_block, chunks[0]);

    // Draw results
    let result_count = state.result_count();
    let title = if state.search_query.is_empty() {
        format!(" all directories ({}) ", result_count)
    } else {
        format!(" results ({}) ", result_count)
    };

    let items: Vec<ListItem> = if state.items.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "  (no matches)",
            Style::default().fg(Color::DarkGray),
        )]))]
    } else {
        state
            .items
            .iter()
            .skip(state.scroll_offset)
            .enumerate()
            .map(|(idx, item)| {
                let global_idx = state.scroll_offset + idx;
                let is_selected = global_idx == state.selected_index;

                let line = Line::from(vec![
                    Span::raw("  "),
                    Span::styled(&item.entry.name, Style::default().fg(Color::White)),
                    Span::styled("/", Style::default().fg(Color::DarkGray)),
                ]);

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(line).style(style)
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .bg(Color::Blue),
        );

    frame.render_widget(list, chunks[1]);
}

/// A fuzzy search result item with matched path
#[derive(Debug, Clone)]
pub struct FuzzyItem {
    pub entry: DirEntry,
    pub match_score: i64,
}

impl FuzzyItem {
    pub fn new(entry: DirEntry, match_score: i64) -> Self {
        Self {
            entry,
            match_score,
        }
    }

    pub fn path(&self) -> String {
        self.entry.path.to_string_lossy().into_owned()
    }
}

/// State for the fuzzy search TUI
#[derive(Debug, Clone)]
pub struct FuzzyState {
    pub search_query: String,
    pub all_items: Vec<FuzzyItem>,
    pub items: Vec<FuzzyItem>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub matcher: FuzzyMatchEngine,
    pub current_dir: PathBuf,
    pub show_hidden: bool,
}

impl FuzzyState {
    /// Create a new FuzzyState by scanning the given directory
    pub fn new_in_dir(dir: &Path, show_hidden: bool) -> Self {
        let entries = fs::scan_directories(dir, show_hidden).unwrap_or_default();
        let items: Vec<FuzzyItem> = entries
            .into_iter()
            .map(|e| FuzzyItem::new(e, 0))
            .collect();

        Self {
            search_query: String::new(),
            all_items: items.clone(),
            items,
            selected_index: 0,
            scroll_offset: 0,
            matcher: FuzzyMatchEngine::new(),
            current_dir: dir.to_path_buf(),
            show_hidden,
        }
    }

    /// Initialize with a list of entries (for tests)
    #[cfg(test)]
    pub fn with_entries(entries: Vec<DirEntry>) -> Self {
        let items: Vec<FuzzyItem> = entries
            .into_iter()
            .map(|e| FuzzyItem::new(e, 0))
            .collect();

        Self {
            search_query: String::new(),
            all_items: items.clone(),
            items,
            selected_index: 0,
            scroll_offset: 0,
            matcher: FuzzyMatchEngine::new(),
            current_dir: PathBuf::from("/"),
            show_hidden: false,
        }
    }

    /// Navigate into the currently selected directory
    pub fn navigate_into(&mut self) {
        if let Some(item) = self.selected_item() {
            let target = item.entry.path.clone();
            if fs::is_accessible(&target) {
                self.load_dir(&target);
            }
        }
    }

    /// Navigate to the parent directory
    pub fn navigate_back(&mut self) {
        if let Some(parent) = fs::get_safe_parent(&self.current_dir) {
            self.load_dir(&parent);
        }
    }

    fn load_dir(&mut self, dir: &Path) {
        let entries = fs::scan_directories(dir, self.show_hidden).unwrap_or_default();
        let items: Vec<FuzzyItem> = entries
            .into_iter()
            .map(|e| FuzzyItem::new(e, 0))
            .collect();

        self.current_dir = dir.to_path_buf();
        self.all_items = items.clone();
        self.items = items;
        self.search_query.clear();
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    /// Update search query and re-filter results
    pub fn set_query(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.filter_results();
    }

    /// Add character to search query
    pub fn add_char(&mut self, c: char) {
        self.search_query.push(c);
        self.filter_results();
    }

    /// Remove last character from search query
    pub fn pop_char(&mut self) {
        self.search_query.pop();
        self.filter_results();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.search_query.clear();
        self.filter_results();
    }

    fn filter_results(&mut self) {
        if self.search_query.is_empty() {
            self.items = self.all_items.clone();
            self.selected_index = 0;
            self.scroll_offset = 0;
            return;
        }

        let pattern = &self.search_query;
        let matcher = &self.matcher;

        let mut filtered: Vec<FuzzyItem> = self
            .all_items
            .iter()
            .filter_map(|item| {
                matcher.get_score(pattern, &item.entry.name).map(|score| {
                    FuzzyItem::new(item.entry.clone(), score)
                })
            })
            .collect();

        filtered.sort_by(|a, b| {
            b.match_score
                .cmp(&a.match_score)
                .then_with(|| a.entry.name.cmp(&b.entry.name))
        });

        self.items = filtered;
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    /// Get currently selected item
    pub fn selected_item(&self) -> Option<&FuzzyItem> {
        self.items.get(self.selected_index)
    }

    /// Move selection up by one
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.update_scroll();
        }
    }

    /// Move selection down by one
    pub fn move_down(&mut self) {
        if self.selected_index + 1 < self.items.len() {
            self.selected_index += 1;
            self.update_scroll();
        }
    }

    /// Scroll up by one page
    pub fn page_up(&mut self) {
        let page_size = 10;
        if self.selected_index >= page_size {
            self.selected_index -= page_size;
        } else {
            self.selected_index = 0;
        }
        self.update_scroll();
    }

    /// Scroll down by one page
    pub fn page_down(&mut self) {
        let page_size = 10;
        let max_index = self.items.len().saturating_sub(1);
        if self.selected_index + page_size <= max_index {
            self.selected_index += page_size;
        } else {
            self.selected_index = max_index;
        }
        self.update_scroll();
    }

    /// Go to first item
    pub fn go_to_start(&mut self) {
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    /// Go to last item
    pub fn go_to_end(&mut self) {
        self.selected_index = self.items.len().saturating_sub(1);
        self.update_scroll();
    }

    /// Update scroll offset based on selected index
    fn update_scroll(&mut self) {
        let visible_height = 15; // Approximate visible items
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index - visible_height + 1;
        }
    }

    /// Get number of results
    pub fn result_count(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_entry(name: &str) -> DirEntry {
        DirEntry {
            path: PathBuf::from(format!("/test/{}", name)),
            name: name.to_string(),
        }
    }

    #[test]
    fn test_fuzzy_state_new() {
        let state = FuzzyState::with_entries(vec![]);
        assert!(state.search_query.is_empty());
        assert!(state.items.is_empty());
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_fuzzy_state_with_entries() {
        let entries = vec![
            test_entry("apple"),
            test_entry("apricot"),
            test_entry("banana"),
        ];
        let state = FuzzyState::with_entries(entries);

        assert_eq!(state.items.len(), 3);
        assert_eq!(state.result_count(), 3);
    }

    #[test]
    fn test_fuzzy_state_add_char() {
        let entries = vec![
            test_entry("apple"),
            test_entry("apricot"),
            test_entry("banana"),
        ];
        let mut state = FuzzyState::with_entries(entries);

        state.add_char('a');
        state.add_char('p');
        assert_eq!(state.search_query, "ap");
        assert_eq!(state.result_count(), 2);
    }

    #[test]
    fn test_fuzzy_state_pop_char() {
        let entries = vec![test_entry("apple")];
        let mut state = FuzzyState::with_entries(entries);

        state.add_char('a');
        state.add_char('p');
        state.pop_char();
        assert_eq!(state.search_query, "a");
    }

    #[test]
    fn test_fuzzy_state_clear_query_restores_all_items() {
        let entries = vec![
            test_entry("apple"),
            test_entry("apricot"),
            test_entry("banana"),
        ];
        let mut state = FuzzyState::with_entries(entries);

        // Filter down to "ap" matches
        state.add_char('a');
        state.add_char('p');
        assert_eq!(state.result_count(), 2);

        // Clearing query should restore all 3 items
        state.clear_query();
        assert!(state.search_query.is_empty());
        assert_eq!(state.result_count(), 3);
    }

    #[test]
    fn test_fuzzy_state_move_up() {
        let entries = vec![test_entry("a"), test_entry("b"), test_entry("c")];
        let mut state = FuzzyState::with_entries(entries);

        state.move_down(); // Select b
        state.move_down(); // Select c
        state.move_up(); // Select b

        assert_eq!(state.selected_index, 1);
    }

    #[test]
    fn test_fuzzy_state_move_down() {
        let entries = vec![test_entry("a"), test_entry("b")];
        let mut state = FuzzyState::with_entries(entries);

        assert_eq!(state.selected_index, 0);
        state.move_down();
        assert_eq!(state.selected_index, 1);
    }

    #[test]
    fn test_fuzzy_state_move_up_from_start() {
        let entries = vec![test_entry("a")];
        let mut state = FuzzyState::with_entries(entries);

        state.move_up(); // Should not go below 0
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_fuzzy_state_go_to_start() {
        let entries = vec![test_entry("a"), test_entry("b"), test_entry("c")];
        let mut state = FuzzyState::with_entries(entries);

        state.move_down();
        state.move_down();
        state.go_to_start();
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_fuzzy_state_go_to_end() {
        let entries = vec![test_entry("a"), test_entry("b"), test_entry("c")];
        let mut state = FuzzyState::with_entries(entries);

        state.go_to_end();
        assert_eq!(state.selected_index, 2);
    }

    #[test]
    fn test_fuzzy_state_selected_item() {
        let entries = vec![test_entry("apple"), test_entry("banana")];
        let mut state = FuzzyState::with_entries(entries);

        state.move_down();
        assert_eq!(state.selected_item().unwrap().entry.name, "banana");
    }
}
