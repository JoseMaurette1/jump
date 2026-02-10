use crate::scoring::Score;

#[derive(Debug, Clone, PartialEq)]
pub struct DirEntry {
    pub path: String,
    pub name: String,
    pub score: Score,
    pub access_count: u32,
    pub last_accessed: i64,
    pub is_bookmark: bool,
    pub bookmark_key: Option<String>,
}

impl DirEntry {
    pub fn new(path: String, name: String) -> Self {
        Self {
            path,
            name,
            score: Score::default(),
            access_count: 0,
            last_accessed: 0,
            is_bookmark: false,
            bookmark_key: None,
        }
    }

    pub fn with_bookmark(path: String, name: String, key: String) -> Self {
        Self {
            path,
            name,
            score: Score::default(),
            access_count: 0,
            last_accessed: 0,
            is_bookmark: true,
            bookmark_key: Some(key),
        }
    }

    pub fn increment_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = chrono::Utc::now().timestamp();
        self.score = self
            .score
            .recalculate(self.access_count, self.last_accessed);
    }
}
