#[derive(Debug, Clone, PartialEq)]
pub struct DirEntry {
    pub path: String,
    pub name: String,
    pub is_bookmark: bool,
    pub bookmark_key: Option<String>,
}
