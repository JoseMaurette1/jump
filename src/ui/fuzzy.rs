use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    Frame,
};

use crate::fs::DirEntry;
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
            .title(" /=search  Esc=cancel  Enter=confirm  j/k=scroll  Ctrl+U/D=page "),
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
    pub matched_indices: Vec<usize>,
}

impl FuzzyItem {
    pub fn new(entry: DirEntry, match_score: i64, matched_indices: Vec<usize>) -> Self {
        Self {
            entry,
            match_score,
            matched_indices,
        }
    }

    pub fn path(&self) -> String {
        self.entry.path.to_string_lossy().into_owned()
    }

    pub fn name(&self) -> &str {
        &self.entry.name
    }
}

/// State for the fuzzy search TUI
#[derive(Debug, Clone)]
pub struct FuzzyState {
    pub search_query: String,
    pub items: Vec<FuzzyItem>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub matcher: FuzzyMatchEngine,
}

impl Default for FuzzyState {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzyState {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            items: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            matcher: FuzzyMatchEngine::new(),
        }
    }

    /// Initialize with a list of entries
    pub fn with_entries(entries: Vec<DirEntry>) -> Self {
        let items: Vec<FuzzyItem> = entries
            .into_iter()
            .map(|e| FuzzyItem::new(e, 0, Vec::new()))
            .collect();

        Self {
            search_query: String::new(),
            items,
            selected_index: 0,
            scroll_offset: 0,
            matcher: FuzzyMatchEngine::new(),
        }
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
            return;
        }

        let pattern = &self.search_query;
        let matcher = &self.matcher;

        self.items
            .retain_mut(|item| match matcher.get_score(pattern, &item.entry.name) {
                Some(score) => {
                    item.match_score = score;
                    item.matched_indices = matcher
                        .get_indices(pattern, &item.entry.name)
                        .unwrap_or_default();
                    true
                }
                None => false,
            });

        self.items.sort_by(|a, b| {
            b.match_score
                .cmp(&a.match_score)
                .then_with(|| a.entry.name.cmp(&b.entry.name))
        });

        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    /// Get list of visible items (respecting scroll offset)
    pub fn visible_items(&self) -> &[FuzzyItem] {
        &self.items[self.scroll_offset..]
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

    /// Check if we have results
    pub fn has_results(&self) -> bool {
        !self.items.is_empty()
    }

    /// Get number of results
    pub fn result_count(&self) -> usize {
        self.items.len()
    }
}

/// Render the fuzzy search widget
pub fn render_fuzzy(frame: &mut Frame, area: Rect, state: &FuzzyState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Results list
        ])
        .split(area);

    // Render search bar
    render_search_bar(frame, chunks[0], state);

    // Render results list
    render_results_list(frame, chunks[1], state);
}

/// Render the search input bar
fn render_search_bar(frame: &mut Frame, area: Rect, state: &FuzzyState) {
    let search_text = format!(" Search: {} ", state.search_query);

    let input_style = if state.search_query.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let cursor_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::REVERSED);

    let paragraph = Paragraph::new(search_text).style(input_style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" /=search  Esc=cancel  Enter=confirm  j/k=scroll  Ctrl+U/D=page "),
    );

    frame.render_widget(paragraph, area);
}

/// Render the results list with highlighting
fn render_results_list(frame: &mut Frame, area: Rect, state: &FuzzyState) {
    let result_count = state.result_count();
    let selected = state.selected_index;

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
                let is_selected = global_idx == selected;

                // Create highlighted name with matched characters
                let name_spans = highlight_match(&item.entry.name, &item.matched_indices);

                let line = Line::from(vec![
                    Span::raw("  "),
                    Span::styled(&item.entry.name, Style::default().fg(Color::White)),
                    Span::styled("/", Style::default().fg(Color::DarkGray)),
                ]);

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else if item.match_score == i64::MIN {
                    Style::default().fg(Color::DarkGray)
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

    frame.render_widget(list, area);
}

/// Highlight matched characters in the name
fn highlight_match<'a>(name: &'a str, indices: &'a [usize]) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let mut last_end = 0;

    for &idx in indices {
        if idx > last_end {
            // Add unmatched portion
            spans.push(Span::raw(&name[last_end..idx]));
        }
        // Add matched character with highlight
        let c = name.chars().nth(idx).unwrap_or(' ');
        spans.push(Span::styled(
            c.to_string(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
        last_end = idx + 1;
    }

    // Add remaining portion
    if last_end < name.len() {
        spans.push(Span::raw(&name[last_end..]));
    }

    spans
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
        let state = FuzzyState::new();
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
    fn test_fuzzy_state_clear_query() {
        let entries = vec![test_entry("apple")];
        let mut state = FuzzyState::with_entries(entries);

        state.add_char('a');
        state.add_char('p');
        state.clear_query();
        assert!(state.search_query.is_empty());
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
        assert_eq!(state.selected_item().unwrap().name(), "banana");
    }

    #[test]
    fn test_fuzzy_state_has_results() {
        let entries = vec![test_entry("apple")];
        let mut state = FuzzyState::with_entries(entries);

        assert!(state.has_results());

        state.set_query("xyz");
        assert!(!state.has_results());
    }

    #[test]
    fn test_highlight_match() {
        let spans = highlight_match("apple", &[0, 1, 2]);
        // All characters matched, should have highlights
        assert!(!spans.is_empty());
    }
}
