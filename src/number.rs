use std::path::PathBuf;

/// Number selection mode for persistent ranking navigation
pub struct NumberMode {
    /// Current entered number (0-999)
    pub current_number: u32,
    /// Maximum selectable number (based on entries count)
    pub max_number: usize,
    /// Whether number entry is complete
    pub is_complete: bool,
}

impl NumberMode {
    /// Create a new number mode with max entries
    pub fn new(max_entries: usize) -> Self {
        Self {
            current_number: 0,
            max_number: max_entries.saturating_sub(1),
            is_complete: false,
        }
    }

    /// Reset the number input
    pub fn reset(&mut self) {
        self.current_number = 0;
        self.is_complete = false;
    }

    /// Add a digit to the number
    pub fn add_digit(&mut self, digit: u8) {
        if self.is_complete {
            return;
        }
        // Only allow up to 3 digits
        if self.current_number < 999 {
            self.current_number = self.current_number * 10 + digit as u32;
        }
    }

    /// Backspace: remove last digit
    pub fn backspace(&mut self) {
        if self.is_complete {
            self.is_complete = false;
            return;
        }
        self.current_number /= 10;
    }

    /// Get the selected index (0-based), returns None if out of range
    pub fn selected_index(&self) -> Option<usize> {
        let idx = self.current_number as usize;
        if idx > 0 && idx <= self.max_number + 1 {
            Some(idx.saturating_sub(1))
        } else if idx == 0 {
            // Pressing 0 alone selects the last entry
            if self.max_number > 0 {
                Some(self.max_number)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check if current selection is valid
    pub fn is_valid_selection(&self) -> bool {
        self.selected_index().is_some()
    }

    /// Confirm the selection
    pub fn confirm(&mut self) -> Option<usize> {
        self.is_complete = true;
        self.selected_index()
    }

    /// Get the display string for current number
    pub fn display_string(&self) -> String {
        if self.current_number == 0 {
            String::from("_")
        } else {
            self.current_number.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_mode_new() {
        let mode = NumberMode::new(10);
        assert_eq!(mode.current_number, 0);
        assert_eq!(mode.max_number, 9);
        assert!(!mode.is_complete);
    }

    #[test]
    fn test_add_digit() {
        let mut mode = NumberMode::new(10);
        mode.add_digit(5);
        assert_eq!(mode.current_number, 5);
        mode.add_digit(2);
        assert_eq!(mode.current_number, 52);
    }

    #[test]
    fn test_backspace() {
        let mut mode = NumberMode::new(10);
        mode.add_digit(5);
        mode.add_digit(2);
        assert_eq!(mode.current_number, 52);
        mode.backspace();
        assert_eq!(mode.current_number, 5);
        mode.backspace();
        assert_eq!(mode.current_number, 0);
    }

    #[test]
    fn test_selected_index() {
        let mut mode = NumberMode::new(10);
        mode.add_digit(1);
        assert_eq!(mode.selected_index(), Some(0));
        mode.reset();
        mode.add_digit(5);
        assert_eq!(mode.selected_index(), Some(4));
    }

    #[test]
    fn test_selected_index_zero() {
        let mut mode = NumberMode::new(10);
        assert_eq!(mode.selected_index(), Some(9)); // 0 selects last
    }

    #[test]
    fn test_selected_index_out_of_range() {
        let mut mode = NumberMode::new(5);
        mode.add_digit(9);
        assert!(mode.selected_index().is_none()); // 9 > max 4
    }

    #[test]
    fn test_confirm() {
        let mut mode = NumberMode::new(10);
        mode.add_digit(3);
        assert_eq!(mode.confirm(), Some(2));
        assert!(mode.is_complete);
    }

    #[test]
    fn test_display_string() {
        let mut mode = NumberMode::new(10);
        assert_eq!(mode.display_string(), "_");
        mode.add_digit(5);
        assert_eq!(mode.display_string(), "5");
    }
}
