use std::path::PathBuf;

use crate::fs::{self, DirEntry};
use crate::labels::{self, Label};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Selecting,
    PartialMatch,
    Confirmed,
    Cancelled,
}

pub struct App {
    pub state: AppState,
    pub entries: Vec<DirEntry>,
    pub labels: Vec<Label>,
    pub first_char: Option<char>,
    pub current_dir: PathBuf,
    pub show_hidden: bool,
}

impl App {
    pub fn new(start_dir: PathBuf, show_hidden: bool) -> Self {
        let entries = fs::scan_directories(&start_dir, show_hidden).unwrap_or_default();
        let labels = labels::generate_labels(entries.len());
        Self {
            state: AppState::Selecting,
            entries,
            labels,
            first_char: None,
            current_dir: start_dir,
            show_hidden,
        }
    }

    pub fn handle_key(&mut self, c: char) {
        let c = c.to_ascii_uppercase();

        match self.state {
            AppState::Selecting => {
                let matching = labels::filter_by_first(&self.labels, c);
                if !matching.is_empty() {
                    self.first_char = Some(c);
                    self.state = AppState::PartialMatch;
                }
            }
            AppState::PartialMatch => {
                if let Some(first) = self.first_char {
                    // Check for hh toggle: only works if no HH label exists
                    if first == 'H' && c == 'H' {
                        let has_hh_label = labels::find_label(&self.labels, 'H', 'H').is_some();
                        if !has_hh_label {
                            self.toggle_hidden();
                            self.first_char = None;
                            self.state = AppState::Selecting;
                            return;
                        }
                    }
                    // Regular label navigation
                    if let Some(idx) = labels::find_label(&self.labels, first, c) {
                        self.navigate_to(idx);
                    } else {
                        self.first_char = None;
                        self.state = AppState::Selecting;
                    }
                } else {
                    self.state = AppState::Selecting;
                }
            }
            _ => {}
        }
    }

    fn navigate_to(&mut self, idx: usize) {
        if let Some(entry) = self.entries.get(idx) {
            let new_dir = entry.path.clone();
            self.current_dir = new_dir.clone();
            self.entries = fs::scan_directories(&new_dir, self.show_hidden).unwrap_or_default();
            self.labels = labels::generate_labels(self.entries.len());
            self.first_char = None;
            self.state = AppState::Selecting;
        }
    }

    pub fn go_up(&mut self) {
        if self.state == AppState::PartialMatch {
            self.first_char = None;
            self.state = AppState::Selecting;
        } else if let Some(parent) = self.current_dir.parent() {
            let parent_path = parent.to_path_buf();
            self.current_dir = parent_path.clone();
            self.entries = fs::scan_directories(&parent_path, self.show_hidden).unwrap_or_default();
            self.labels = labels::generate_labels(self.entries.len());
            self.first_char = None;
            self.state = AppState::Selecting;
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.entries =
            fs::scan_directories(&self.current_dir, self.show_hidden).unwrap_or_default();
        self.labels = labels::generate_labels(self.entries.len());
        self.first_char = None;
        self.state = AppState::Selecting;
    }

    pub fn confirm(&mut self) {
        self.state = AppState::Confirmed;
    }

    pub fn cancel(&mut self) {
        self.state = AppState::Cancelled;
    }

    pub fn selected_path(&self) -> Option<PathBuf> {
        if self.state == AppState::Confirmed {
            Some(self.current_dir.clone())
        } else {
            None
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self.state, AppState::Confirmed | AppState::Cancelled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new(PathBuf::from("/tmp"), false);
        assert_eq!(app.state, AppState::Selecting);
        assert_eq!(app.current_dir, PathBuf::from("/tmp"));
    }
}
