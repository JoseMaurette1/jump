# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-02-10

### Added

- **Phase 1: Foundation**
  - SQLite database for persistent storage
  - Weighted frequency scoring algorithm
  - Directory scanner with walkdir integration
  - Database operations: insert, update, search, bookmarks

- **Phase 2: Fuzzy Search**
  - Fuzzy matching using skim v2 algorithm (fzf-compatible)
  - Real-time result reordering by score
  - Interactive fuzzy search TUI

- **Phase 3: Numbers & Bookmarks**
  - Number selection mode for quick access
  - Bookmark CRUD operations
  - Custom key bindings for bookmarks
  - Statistics command

- **Phase 4: Shell Integration**
  - Shell init script generation (Bash, Zsh, Fish)
  - Multi-platform release builds (Linux, macOS, Windows)
  - Import/Export functionality

- **Phase 5: Polish** (this release)
  - Performance optimizations (cached prepared statements, batch operations)
  - Better error handling and edge cases
  - Comprehensive test suite
  - CI/CD pipeline with multi-platform testing

### Changed

- Improved scoring algorithm with recency decay
- Optimized database queries with WAL mode
- Enhanced directory scanning with better error handling

### Fixed

- Handle permission errors gracefully
- Handle non-existent paths
- Handle empty directories

### Performance

- Pre-compiled SQL statements for faster queries
- Batch insert operations for imports
- WAL mode for better concurrent access

### Documentation

- Updated README with new features
- Added inline documentation
- Created this changelog

## [0.1.7] - Previous versions

Initial development versions with basic TUI functionality.
