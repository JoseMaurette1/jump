use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use std::path::{Path, PathBuf};

use crate::database::entry::DirEntry as DbDirEntry;
use crate::fs::{self, DirEntry};
use crate::fuzzy::FuzzyMatchEngine;
use crate::Mode;

/// Draw the fuzzy search TUI
pub fn draw_fuzzy(frame: &mut Frame, state: &FuzzyState, mode: &Mode) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    // Draw help bar
    let hidden_indicator = if state.show_hidden { "on" } else { "off" };
    let help_spans = vec![
        Span::styled(
            "j/k",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" nav [3j/6k]  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "h/l",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" in/out  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "/",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" search  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "b/x",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" bookmark/remove  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "g/G",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" top/bot  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            ".",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" hidden [{}]  ", hidden_indicator),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            "Enter",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" select  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "Esc",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" quit", Style::default().fg(Color::DarkGray)),
    ];
    let help_line = Paragraph::new(Line::from(help_spans));
    frame.render_widget(help_line, chunks[0]);

    // Draw search/bookmark input
    match mode {
        Mode::BookmarkInput(alias) => {
            let selected_name = state
                .selected_item()
                .map(|item| item.entry.name.as_str())
                .unwrap_or("");
            let display = format!(" Bookmark '{}' as: {} ", selected_name, alias);
            let input_block = Paragraph::new(display)
                .style(Style::default().fg(Color::Blue))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Blue))
                        .title(" BOOKMARK "),
                );
            frame.render_widget(input_block, chunks[1]);
        }
        Mode::BookmarkRemove => {
            let selected_name = state
                .selected_item()
                .map(|item| item.entry.name.as_str())
                .unwrap_or("");
            let bookmark_key = state
                .selected_item()
                .and_then(|item| item.bookmark_key.as_deref())
                .unwrap_or("");
            let display = format!(
                " Remove bookmark '{}' -> {}? Press Enter to confirm ",
                bookmark_key, selected_name
            );
            let input_block = Paragraph::new(display)
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Red))
                        .title(" REMOVE BOOKMARK "),
                );
            frame.render_widget(input_block, chunks[1]);
        }
        _ => {
            let search_display = format!(" Search: {} ", state.search_query);
            let (search_style, border_style, title) = if *mode == Mode::Search {
                (
                    Style::default().fg(Color::Yellow),
                    Style::default().fg(Color::Blue),
                    format!(" SEARCH: {} ", state.current_dir.display()),
                )
            } else if state.search_query.is_empty() {
                (
                    Style::default().fg(Color::DarkGray),
                    Style::default(),
                    format!(" {} ", state.current_dir.display()),
                )
            } else {
                (
                    Style::default().fg(Color::Yellow),
                    Style::default(),
                    format!(" {} ", state.current_dir.display()),
                )
            };

            let search_block = Paragraph::new(search_display).style(search_style).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(title),
            );
            frame.render_widget(search_block, chunks[1]);
        }
    }

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
                let rel_num = if global_idx > state.selected_index {
                    global_idx - state.selected_index
                } else {
                    state.selected_index - global_idx
                };
                let rel_num_str = if rel_num == 0 {
                    "  0 ".to_string()
                } else {
                    format!("{:>3} ", rel_num)
                };
                let num_style = if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else if *mode == Mode::Search {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::Gray)
                };
                let num_span = Span::styled(rel_num_str, num_style);

                if item.is_bookmark {
                    let alias = item.bookmark_key.as_deref().unwrap_or("");
                    let (prefix, star_style, alias_style, arrow_style, name_style) = if is_selected
                    {
                        let selection_color = if *mode == Mode::Search {
                            Color::DarkGray
                        } else {
                            Color::Blue
                        };
                        (
                            Span::styled(
                                ">",
                                Style::default()
                                    .fg(selection_color)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                            Style::default().fg(selection_color),
                            Style::default()
                                .fg(selection_color)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        (
                            Span::raw(" "),
                            Style::default().fg(Color::Yellow),
                            Style::default().fg(Color::Yellow),
                            Style::default().fg(Color::DarkGray),
                            Style::default().fg(Color::White),
                        )
                    };

                    let line = Line::from(vec![
                        num_span,
                        prefix,
                        Span::styled(" ★ ", star_style),
                        Span::styled(alias, alias_style),
                        Span::styled(" → ", arrow_style),
                        Span::styled(&item.entry.name, name_style),
                    ]);
                    ListItem::new(line)
                } else {
                    let (prefix, name_style, slash_style) = if is_selected {
                        let selection_color = if *mode == Mode::Search {
                            Color::DarkGray
                        } else {
                            Color::Blue
                        };
                        (
                            Span::styled(
                                ">",
                                Style::default()
                                    .fg(selection_color)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Style::default()
                                .fg(selection_color)
                                .add_modifier(Modifier::BOLD),
                            Style::default().fg(selection_color),
                        )
                    } else {
                        (
                            Span::raw(" "),
                            Style::default().fg(Color::White),
                            Style::default().fg(Color::DarkGray),
                        )
                    };

                    let line = Line::from(vec![
                        num_span,
                        prefix,
                        Span::styled(" ", name_style),
                        Span::styled(&item.entry.name, name_style),
                        Span::styled("/", slash_style),
                    ]);
                    ListItem::new(line)
                }
            })
            .collect()
    };

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(list, chunks[2]);
}

/// A fuzzy search result item with matched path
#[derive(Debug, Clone)]
pub struct FuzzyItem {
    pub entry: DirEntry,
    pub match_score: i64,
    pub is_bookmark: bool,
    pub bookmark_key: Option<String>,
}

impl FuzzyItem {
    pub fn new(entry: DirEntry, match_score: i64) -> Self {
        Self {
            entry,
            match_score,
            is_bookmark: false,
            bookmark_key: None,
        }
    }

    pub fn from_bookmark(db_entry: &DbDirEntry) -> Self {
        Self {
            entry: DirEntry {
                path: PathBuf::from(&db_entry.path),
                name: db_entry.name.clone(),
            },
            match_score: 0,
            is_bookmark: true,
            bookmark_key: db_entry.bookmark_key.clone(),
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
    pub bookmarks: Vec<DbDirEntry>,
    pub motion_count: Option<usize>,
}

impl FuzzyState {
    /// Create a new FuzzyState by scanning the given directory
    pub fn new_in_dir(dir: &Path, show_hidden: bool) -> Self {
        let entries = fs::scan_directories(dir, show_hidden).unwrap_or_default();
        let items: Vec<FuzzyItem> = entries.into_iter().map(|e| FuzzyItem::new(e, 0)).collect();

        Self {
            search_query: String::new(),
            all_items: items.clone(),
            items,
            selected_index: 0,
            scroll_offset: 0,
            matcher: FuzzyMatchEngine::new(),
            current_dir: dir.to_path_buf(),
            show_hidden,
            bookmarks: Vec::new(),
            motion_count: None,
        }
    }

    #[cfg(test)]
    pub fn with_entries(entries: Vec<DirEntry>) -> Self {
        let items: Vec<FuzzyItem> = entries.into_iter().map(|e| FuzzyItem::new(e, 0)).collect();

        Self {
            search_query: String::new(),
            all_items: items.clone(),
            items,
            selected_index: 0,
            scroll_offset: 0,
            matcher: FuzzyMatchEngine::new(),
            current_dir: PathBuf::from("/"),
            show_hidden: false,
            bookmarks: Vec::new(),
            motion_count: None,
        }
    }

    pub fn set_bookmarks(&mut self, bookmarks: Vec<DbDirEntry>) {
        self.bookmarks = bookmarks;
        self.refresh_bookmark_status();
    }

    fn refresh_bookmark_status(&mut self) {
        let update_item = |item: &mut FuzzyItem| {
            let path_str = item.entry.path.to_string_lossy().to_string();
            if let Some(bm) = self.bookmarks.iter().find(|b| b.path == path_str) {
                item.is_bookmark = true;
                item.bookmark_key = bm.bookmark_key.clone();
            } else {
                item.is_bookmark = false;
                item.bookmark_key = None;
            }
        };

        for item in &mut self.all_items {
            update_item(item);
        }
        for item in &mut self.items {
            update_item(item);
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
            .map(|e| {
                let path_str = e.path.to_string_lossy().to_string();
                if let Some(bm) = self.bookmarks.iter().find(|b| b.path == path_str) {
                    FuzzyItem {
                        entry: e,
                        match_score: 0,
                        is_bookmark: true,
                        bookmark_key: bm.bookmark_key.clone(),
                    }
                } else {
                    FuzzyItem::new(e, 0)
                }
            })
            .collect();

        self.current_dir = dir.to_path_buf();
        self.all_items = items.clone();
        self.items = items;
        self.search_query.clear();
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.motion_count = None;
    }

    pub fn set_motion_count(&mut self, count: usize) {
        self.motion_count = Some(count);
    }

    pub fn take_motion_count(&mut self) -> usize {
        self.motion_count.take().unwrap_or(1)
    }

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

    /// Re-run filtering with current query (e.g. after bookmarks change)
    pub fn refilter(&mut self) {
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

        // Filter directory items
        let mut filtered: Vec<FuzzyItem> = self
            .all_items
            .iter()
            .filter_map(|item| {
                matcher
                    .get_score(pattern, &item.entry.name)
                    .map(|score| FuzzyItem::new(item.entry.clone(), score))
            })
            .collect();

        // Also match bookmarks by name and alias
        for bm in &self.bookmarks {
            let name_score = matcher.get_score(pattern, &bm.name);
            let alias_score = bm
                .bookmark_key
                .as_deref()
                .and_then(|key| matcher.get_score(pattern, key));

            let best_score = match (name_score, alias_score) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            };

            if let Some(score) = best_score {
                // Don't add duplicate if bookmark path is already in directory items
                let already_present = filtered
                    .iter()
                    .any(|item| item.entry.path.to_string_lossy() == bm.path);
                if !already_present {
                    let mut bm_item = FuzzyItem::from_bookmark(bm);
                    bm_item.match_score = score;
                    filtered.push(bm_item);
                }
            }
        }

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

    /// Toggle hidden file visibility and reload directory
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        let dir = self.current_dir.clone();
        self.load_dir(&dir);
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

    #[test]
    fn test_bookmark_search_results() {
        let entries = vec![test_entry("projects"), test_entry("documents")];
        let mut state = FuzzyState::with_entries(entries);

        state.set_bookmarks(vec![DbDirEntry {
            path: "/home/user/work".to_string(),
            name: "work".to_string(),
            is_bookmark: true,
            bookmark_key: Some("w".to_string()),
        }]);

        // Search for "w" should find the bookmark
        state.add_char('w');
        assert!(state
            .items
            .iter()
            .any(|i| i.is_bookmark && i.entry.name == "work"));
    }
}
