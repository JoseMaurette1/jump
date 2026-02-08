use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::labels::MAX_LABELS;

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub name: String,
}

pub fn scan_directories(dir: &Path, show_hidden: bool) -> Result<Vec<DirEntry>> {
    let mut entries: Vec<DirEntry> = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .filter(|e| {
            if show_hidden {
                true
            } else {
                e.file_name()
                    .to_str()
                    .map(|s| !s.starts_with('.'))
                    .unwrap_or(false)
            }
        })
        .map(|e| DirEntry {
            name: e.file_name().to_string_lossy().into_owned(),
            path: e.path().to_path_buf(),
        })
        .collect();

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    entries.truncate(MAX_LABELS);

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_scan_current_dir() {
        let current = env::current_dir().unwrap();
        let result = scan_directories(&current, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scan_with_hidden() {
        let current = env::current_dir().unwrap();
        let result = scan_directories(&current, true);
        assert!(result.is_ok());
    }
}
