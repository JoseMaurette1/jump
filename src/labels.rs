const KEYS: &[char] = &[
    'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Q', 'W', 'E', 'R', 'U', 'I', 'O',
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub chars: [char; 2],
}

impl Label {
    pub fn new(first: char, second: char) -> Self {
        Self {
            chars: [first, second],
        }
    }

    #[cfg(test)]
    pub fn as_str(&self) -> String {
        format!("{}{}", self.chars[0], self.chars[1])
    }

    pub fn matches_first(&self, c: char) -> bool {
        self.chars[0] == c.to_ascii_uppercase()
    }

    pub fn matches_second(&self, c: char) -> bool {
        self.chars[1] == c.to_ascii_uppercase()
    }

    pub fn matches(&self, first: char, second: char) -> bool {
        self.matches_first(first) && self.matches_second(second)
    }
}

pub const MAX_LABELS: usize = 256; // 16 * 16

pub fn generate_labels(count: usize) -> Vec<Label> {
    let capped_count = count.min(MAX_LABELS);
    let mut labels = Vec::with_capacity(capped_count);

    'outer: for &first in KEYS {
        for &second in KEYS {
            if labels.len() >= capped_count {
                break 'outer;
            }
            labels.push(Label::new(first, second));
        }
    }

    labels
}

pub fn find_label(labels: &[Label], first: char, second: char) -> Option<usize> {
    labels.iter().position(|l| l.matches(first, second))
}

pub fn filter_by_first(labels: &[Label], first: char) -> Vec<usize> {
    labels
        .iter()
        .enumerate()
        .filter(|(_, l)| l.matches_first(first))
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_labels() {
        let labels = generate_labels(5);
        assert_eq!(labels.len(), 5);
        assert_eq!(labels[0].as_str(), "AA");
        assert_eq!(labels[1].as_str(), "AS");
    }

    #[test]
    fn test_label_matching() {
        let label = Label::new('A', 'S');
        assert!(label.matches('a', 's'));
        assert!(label.matches('A', 'S'));
        assert!(!label.matches('A', 'D'));
    }

    #[test]
    fn test_find_label() {
        let labels = generate_labels(10);
        let idx = find_label(&labels, 'A', 'S');
        assert_eq!(idx, Some(1));
    }
}
