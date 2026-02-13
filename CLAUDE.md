# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Jump is a Vim-inspired directory navigation CLI tool written in Rust. It replaces `cd` with fuzzy search, bookmarks, and shell integration. Current version: 1.0.0.

## Build & Test Commands

```bash
cargo build                # Debug build
cargo build --release      # Optimized release build (LTO, strip enabled)
cargo test                 # Run all tests
cargo test <test_name>     # Run a single test by name
cargo test -- --nocapture  # Run tests with stdout visible
```

## Architecture

The binary entry point is `src/main.rs`, which parses args via `src/config.rs` (custom parser, not clap subcommands) and dispatches to either fuzzy mode or bookmark operations.

**Module structure:**

- `config.rs` — CLI argument parsing, returns `(ParseResult, ShellAction, BookmarkAction)`. Custom hand-rolled parser using `env::args()`.
- `fs.rs` — Directory scanning with `walkdir` (depth=1, no symlink following). Defines its own `DirEntry` (path + name) used by the UI.
- `database/db.rs` — SQLite persistence (WAL mode) for bookmarks and directory entries. DB path via `directories` crate (`~/.local/share/jump/jump.db` on Linux).
- `database/entry.rs` — `DirEntry` struct for database records (with score, access_count, bookmark fields). Distinct from `fs::DirEntry`.
- `fuzzy.rs` → `fuzzy/matcher.rs` — Wrapper around `fuzzy_matcher::skim::SkimMatcherV2`.
- `ui/fuzzy.rs` — Core TUI: `FuzzyState` state machine + `draw_fuzzy()` ratatui renderer. Largest file, contains filtering/navigation/selection logic.
- `input.rs` — Terminal key event handling via crossterm. Maps raw key events to `InputEvent` enum.
- `shell.rs` — Shell init script and completion generation for bash/zsh/fish. Tests in `shell/tests.rs`.
- `cli.rs` — Clap derive structs (used alongside custom parser in `config.rs`).

**Data flow in fuzzy mode:**
1. `config::parse_args()` → determines mode
2. `fs::scan_directories()` → collects immediate child directories
3. `FuzzyState::with_entries()` → initializes UI state
4. Event loop in `main.rs`: reads keys → mutates `FuzzyState` → renders via `draw_fuzzy()`
5. On Enter: prints selected path to stdout (shell wrapper does the `cd`)

**Two DirEntry types exist:** `fs::DirEntry` (simple path+name for UI display) and `database::entry::DirEntry` (full record with score, access_count, bookmark_key for persistence).

**Vim keybindings:** `/` to search, `j/k` navigation, `g/G` top/bottom, `Ctrl+U/D` page up/down, `Enter` select, `Esc` quit.

## Key Dependencies

- `ratatui` + `crossterm` — TUI rendering and terminal control
- `rusqlite` (bundled) — SQLite database
- `fuzzy-matcher` — SkimMatcherV2 scoring
- `walkdir` — Directory traversal
- `directories` — Platform-specific data paths
- `clap` (derive) — CLI struct definitions
- `anyhow` + `thiserror` — Error handling

## Testing

Tests are co-located in each module behind `#[cfg(test)]`. Key test areas:
- `ui/fuzzy.rs` — FuzzyState navigation, filtering, selection
- `shell/tests.rs` — Shell script generation
- `fs.rs` — Directory scanning and error handling
- `fuzzy/matcher.rs` — Scoring

## Release Profile

Release builds use aggressive optimization: `opt-level = 3`, `lto = true`, `codegen-units = 1`, `strip = true`.

## Workflow Orchestration

### 1. Plan Mode Default
- Enter plan mode for **ANY** non-trivial task (3+ steps or architectural decisions)
- If something goes sideways, **STOP and re-plan immediately** — don't keep pushing
- Use plan mode for verification steps, not just building
- Write detailed specs upfront to reduce ambiguity

### 2. Subagent Strategy
- Use subagents liberally to keep main context window clean
- Offload research, exploration, and parallel analysis to subagents
- For complex problems, throw more compute at it via subagents
- One task per subagent for focused execution

### 3. Self-Improvement Loop
- After **ANY** correction from the user: update `tasks/lessons.md` with the pattern
- Write rules for yourself that prevent the same mistake
- Ruthlessly iterate on these lessons until mistake rate drops
- Review lessons at session start for relevant project

### 4. Verification Before Done
- Never mark a task complete without proving it works
- Diff behavior between main and your changes when relevant
- Ask yourself: *"Would a staff engineer approve this?"*
- Run tests, check logs, demonstrate correctness

### 5. Demand Elegance (Balanced)
- For non-trivial changes: pause and ask *"Is there a more elegant way?"*
- If a fix feels hacky: *"Knowing everything I know now, implement the elegant solution"*
- Skip this for simple, obvious fixes — don't over-engineer
- Challenge your own work before presenting it

### 6. Autonomous Bug Fixing
- When given a bug report: just fix it. Don't ask for hand-holding
- Point at logs, errors, failing tests — then resolve them
- Zero context switching required from the user
- Go fix failing CI tests without being told how

---

## Task Management

1. **Plan First**: Write plan to `tasks/todo.md` with checkable items
2. **Verify Plan**: Check in before starting implementation
3. **Track Progress**: Mark items complete as you go
4. **Explain Changes**: High-level summary at each step
5. **Document Results**: Add review section to `tasks/todo.md`
6. **Capture Lessons**: Update `tasks/lessons.md` after corrections

---

## Core Principles
- **Simplicity First**: Make every change as simple as possible. Impact minimal code.
- **No Laziness**: Find root causes. No temporary fixes. Senior developer standards.
- **Minimal Impact**: Changes should only touch what's necessary. Avoid introducing bugs.
