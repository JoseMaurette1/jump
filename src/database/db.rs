use anyhow::{Context, Result};

use super::entry::DirEntry;

const DB_NAME: &str = "jump.db";
const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    score REAL NOT NULL DEFAULT 0.0,
    access_count INTEGER NOT NULL DEFAULT 0,
    last_accessed INTEGER NOT NULL DEFAULT 0,
    is_bookmark INTEGER NOT NULL DEFAULT 0,
    bookmark_key TEXT
);

CREATE INDEX IF NOT EXISTS idx_score ON entries(score DESC);
CREATE INDEX IF NOT EXISTS idx_path ON entries(path);
CREATE INDEX IF NOT EXISTS idx_bookmark_key ON entries(bookmark_key) WHERE bookmark_key IS NOT NULL;
"#;

pub struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let binding = directories::ProjectDirs::from("com", "jump", "jump")
            .expect("Failed to get project directories");
        let data_dir = binding.data_dir();

        std::fs::create_dir_all(data_dir).context("Failed to create data directory")?;
        let db_path = data_dir.join(DB_NAME);

        let conn = rusqlite::Connection::open(db_path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.execute(SCHEMA, [])?;

        Ok(Self { conn })
    }

    pub fn get_by_bookmark_key(&self, key: &str) -> Result<Option<DirEntry>> {
        use rusqlite::OptionalExtension;

        let mut stmt = self.conn.prepare(
            "SELECT path, name, score, access_count, last_accessed, is_bookmark, bookmark_key
             FROM entries WHERE bookmark_key = ?1",
        )?;

        let result: Option<DirEntry> = stmt
            .query_row([key], |row| {
                Ok(DirEntry {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    score: row.get(2)?,
                    access_count: row.get(3)?,
                    last_accessed: row.get(4)?,
                    is_bookmark: row.get(5)?,
                    bookmark_key: row.get(6)?,
                })
            })
            .optional()?;

        Ok(result)
    }

    pub fn set_bookmark(&self, path: &str, name: &str, key: &str) -> Result<()> {
        let exists: bool = self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM entries WHERE path = ?1)",
            [path],
            |row| row.get(0),
        )?;

        if exists {
            self.conn.execute(
                "UPDATE entries SET name = ?1, is_bookmark = 1, bookmark_key = ?2 WHERE path = ?3",
                (name, key, path),
            )?;
        } else {
            self.conn.execute(
                "INSERT INTO entries (path, name, score, access_count, last_accessed, is_bookmark, bookmark_key)
                 VALUES (?1, ?2, 0.0, 0, 0, 1, ?3)",
                (path, name, key),
            )?;
        }
        Ok(())
    }

    pub fn remove_bookmark(&self, key: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE entries SET is_bookmark = 0, bookmark_key = NULL WHERE bookmark_key = ?1",
            [key],
        )?;
        Ok(())
    }

    pub fn get_all_bookmarks(&self) -> Result<Vec<DirEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT path, name, score, access_count, last_accessed, is_bookmark, bookmark_key
             FROM entries WHERE is_bookmark = 1 ORDER BY bookmark_key",
        )?;

        let entries = stmt
            .query_map([], |row| {
                Ok(DirEntry {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    score: row.get(2)?,
                    access_count: row.get(3)?,
                    last_accessed: row.get(4)?,
                    is_bookmark: row.get(5)?,
                    bookmark_key: row.get(6)?,
                })
            })?
            .filter_map(|e| e.ok())
            .collect();

        Ok(entries)
    }
}
