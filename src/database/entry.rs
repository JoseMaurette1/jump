#[derive(Debug, Clone, PartialEq)]
pub struct DirEntry {
    pub path: String,
    pub name: String,
    pub score: f64,
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
            score: 0.0,
            access_count: 0,
            last_accessed: 0,
            is_bookmark: false,
            bookmark_key: None,
        }
    }
}
