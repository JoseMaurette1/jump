use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("Directory does not exist: {0}")]
    NotFound(PathBuf),
    #[error("Not a directory: {0}")]
    NotDirectory(PathBuf),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn scan_directories(dir: &Path, show_hidden: bool) -> Result<Vec<DirEntry>, ScanError> {
    if !dir.exists() {
        return Err(ScanError::NotFound(dir.to_path_buf()));
    }

    if !dir.is_dir() {
        return Err(ScanError::NotDirectory(dir.to_path_buf()));
    }

    let mut entries: Vec<DirEntry> = Vec::new();

    for entry in WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .follow_links(false)
        .contents_first(false)
    {
        match entry {
            Ok(e) => {
                if !e.file_type().is_dir() {
                    continue;
                }

                if !show_hidden {
                    if let Some(name) = e.file_name().to_str() {
                        if name.starts_with('.') {
                            continue;
                        }
                    }
                }

                entries.push(DirEntry {
                    name: e.file_name().to_string_lossy().into_owned(),
                    path: e.path().to_path_buf(),
                });
            }
            Err(e) => {
                if e.io_error().is_some() {
                    continue;
                }
            }
        }
    }

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(entries)
}

/// Safe way to get parent directory with fallbacks
pub fn get_safe_parent(dir: &Path) -> Option<PathBuf> {
    if dir == Path::new("/") {
        return None;
    }
    dir.parent()
        .map(|p| p.to_path_buf())
        .filter(|pb| !pb.as_os_str().is_empty())
}

/// Check if a path is accessible (exists and readable)
pub fn is_accessible(dir: &Path) -> bool {
    dir.exists() && dir.is_dir()
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

    #[test]
    fn test_scan_nonexistent() {
        let result = scan_directories(Path::new("/nonexistent/path"), false);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ScanError::NotFound(_)));
    }

    #[test]
    fn test_scan_not_directory() {
        let temp_file = std::env::temp_dir().join("jump_test_file");
        std::fs::write(&temp_file, "test").unwrap();
        let result = scan_directories(&temp_file, false);
        std::fs::remove_file(&temp_file).ok();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ScanError::NotDirectory(_)));
    }

    #[test]
    fn test_get_safe_parent() {
        let path = Path::new("/home/user/projects");
        assert_eq!(get_safe_parent(path), Some(PathBuf::from("/home/user")));

        let root = Path::new("/");
        assert_eq!(get_safe_parent(root), None);

        let single = Path::new("projects");
        assert_eq!(get_safe_parent(single), None);
    }

    #[test]
    fn test_is_accessible() {
        assert!(is_accessible(Path::new("/tmp")));
        assert!(!is_accessible(Path::new("/nonexistent")));
    }
}
