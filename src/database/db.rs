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

    pub fn in_memory() -> Result<Self> {
        let conn = rusqlite::Connection::open_in_memory()?;
        conn.execute(SCHEMA, [])?;
        Ok(Self { conn })
    }

    pub fn get_or_create(&self, path: &str, name: &str) -> Result<DirEntry> {
        let mut stmt = self.conn.prepare(
            "SELECT path, name, score, access_count, last_accessed, is_bookmark, bookmark_key
             FROM entries WHERE path = ?1",
        )?;

        let entry = stmt.query_row([path], |row| {
            Ok(DirEntry {
                path: row.get(0)?,
                name: row.get(1)?,
                score: Score::from_raw(row.get(2)?),
                access_count: row.get(3)?,
                last_accessed: row.get(4)?,
                is_bookmark: row.get(5)?,
                bookmark_key: row.get(6)?,
            })
        });

        match entry {
            Ok(e) => Ok(e),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let entry = DirEntry::new(path.to_string(), name.to_string());
                self.insert(&entry)?;
                Ok(entry)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn insert(&self, entry: &DirEntry) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO entries (path, name, score, access_count, last_accessed, is_bookmark, bookmark_key)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &entry.path,
                &entry.name,
                entry.score.to_raw(),
                entry.access_count,
                entry.last_accessed,
                if entry.is_bookmark { 1i32 } else { 0i32 },
                entry.bookmark_key.as_ref(),
            ),
        )?;
        Ok(())
    }

    pub fn update(&self, entry: &DirEntry) -> Result<()> {
        self.conn.execute(
            "UPDATE entries SET
                name = ?2, score = ?3, access_count = ?4,
                last_accessed = ?5, is_bookmark = ?6, bookmark_key = ?7
             WHERE path = ?1",
            (
                &entry.path,
                &entry.name,
                entry.score.to_raw(),
                entry.access_count,
                entry.last_accessed,
                if entry.is_bookmark { 1i32 } else { 0i32 },
                entry.bookmark_key.as_ref(),
            ),
        )?;
        Ok(())
    }

    pub fn record_access(&self, path: &str) -> Result<()> {
        let mut entry = self.get_or_create(
            path,
            &std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path),
        )?;

        entry.increment_access();
        self.update(&entry)?;
        Ok(())
    }

    pub fn get_top_entries(&self, limit: usize) -> Result<Vec<DirEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT path, name, score, access_count, last_accessed, is_bookmark, bookmark_key
             FROM entries ORDER BY score DESC LIMIT ?1",
        )?;

        let entries = stmt
            .query_map([limit], |row| {
                Ok(DirEntry {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    score: Score::from_raw(row.get(2)?),
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

    pub fn search_by_name(&self, query: &str, limit: usize) -> Result<Vec<DirEntry>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT path, name, score, access_count, last_accessed, is_bookmark, bookmark_key
             FROM entries WHERE name LIKE ?1 ORDER BY score DESC LIMIT ?2",
        )?;

        let entries = stmt
            .query_map((&pattern, limit), |row| {
                Ok(DirEntry {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    score: Score::from_raw(row.get(2)?),
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

    pub fn get_by_bookmark_key(&self, key: &str) -> Result<Option<DirEntry>> {
        use rusqlite::OptionalExtension;

        let mut stmt = self.conn.prepare(
            "SELECT path, name, score, access_count, last_accessed, is_bookmark, bookmark_key
             FROM entries WHERE bookmark_key = ?1",
        )?;

        stmt.query_row([key], |row| {
            Ok(DirEntry {
                path: row.get(0)?,
                name: row.get(1)?,
                score: Score::from_raw(row.get(2)?),
                access_count: row.get(3)?,
                last_accessed: row.get(4)?,
                is_bookmark: row.get(5)?,
                bookmark_key: row.get(6)?,
            })
        })
        .optional()
        .map_err(|e| e.into())
    }

    pub fn set_bookmark(&self, path: &str, name: &str, key: &str) -> Result<()> {
        let mut entry = self.get_or_create(path, name)?;
        entry.is_bookmark = true;
        entry.bookmark_key = Some(key.to_string());
        self.update(&entry)?;
        Ok(())
    }

    pub fn remove_bookmark(&self, key: &str) -> Result<()> {
        if let Some(mut entry) = self.get_by_bookmark_key(key)? {
            entry.is_bookmark = false;
            entry.bookmark_key = None;
            self.update(&entry)?;
        }
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
                    score: Score::from_raw(row.get(2)?),
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

    pub fn cleanup_low_score(&self, threshold: f64) -> Result<u64> {
        let count = self.conn.execute(
            "DELETE FROM entries WHERE score < ?1 AND is_bookmark = 0",
            [threshold],
        )?;
        Ok(count as u64)
    }

    pub fn count_entries(&self) -> Result<u64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM entries", [], |row| row.get(0))?;
        Ok(count as u64)
    }

    /// Batch insert entries efficiently using a transaction
    pub fn batch_insert(&mut self, entries: &[DirEntry]) -> Result<()> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO entries (path, name, score, access_count, last_accessed, is_bookmark, bookmark_key)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            )?;
            for entry in entries {
                stmt.execute((
                    &entry.path,
                    &entry.name,
                    entry.score.to_raw(),
                    entry.access_count,
                    entry.last_accessed,
                    if entry.is_bookmark { 1i32 } else { 0i32 },
                    entry.bookmark_key.as_ref(),
                ))?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Vacuum the database to reclaim space
    pub fn vacuum(&self) -> Result<()> {
        self.conn.execute("VACUUM", [])?;
        Ok(())
    }

    /// Get database size in bytes
    pub fn get_size(&self) -> Result<u64> {
        let binding = directories::ProjectDirs::from("com", "jump", "jump")
            .expect("Failed to get project directories");
        let data_dir = binding.data_dir();
        let db_path = data_dir.join(DB_NAME);
        std::fs::metadata(db_path)
            .map(|m| m.len())
            .context("Failed to get database size")
    }
}

use crate::scoring::Score;
