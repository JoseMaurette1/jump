# Jump Redesign: Design Document

## Executive Summary

Redesign jump to solve the core UX problem: **current 2-character label system adds indirection without memorability**. The new design will provide multiple navigation modes, persistent ranking, and vim-inspired efficiency.

---

## Current State Analysis

### What Works
- Vim keybindings (`j/k` for scroll, `Esc`/`Ctrl+C`)
- TUI with ratatui is clean and minimal
- Hidden file toggle
- Simple, single-level directory scanning
- No external dependencies beyond std + 4 crates

### What Doesn't Work
- **Position-based labels change every directory** — no muscle memory possible
- **Two-character codes require reading all options** — slower than `cd` + tab
- **No persistence** — frequency data is lost on exit
- **Pure recognition, zero recall benefit**

### Root Cause
The original goal was "tired of typing cd/ls", but the solution added a mapping layer (code → directory) that provides no advantage over seeing actual directory names.

---

## Redesign Goals

1. **Speed**: Beat `cd` + tab for common directories
2. **Memorability**: Build muscle memory over time
3. **Flexibility**: Support multiple navigation patterns
4. **Vim-familiar**: Keep vim keybindings, enhance them
5. **Persistence**: Track frequency/recency

---

## Proposed Architecture

### Core Components

```
┌─────────────────────────────────────────────────────┐
│                    Jump CLI                         │
├─────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │
│  │  Database   │  │   Scanner   │  │   UI/TUI    │  │
│  │ (freq/rec)  │  │ (directory) │  │  (ratatui)  │  │
│  └─────────────┘  └─────────────┘  └─────────────┘  │
│         │                │                │         │
│         └────────────────┼────────────────┘         │
│                          ▼                          │
│                 ┌─────────────────┐                 │
│                 │    Router       │                 │
│                 │ (mode dispatch)│                 │
│                 └─────────────────┘                 │
└─────────────────────────────────────────────────────┘
```

### Database Schema (JSON/SQLite)

```json
{
  "version": 1,
  "last_updated": "2026-02-10T...",
  "entries": [
    {
      "path": "/home/user/projects/work",
      "name": "work",
      "score": 85.3,
      "access_count": 42,
      "last_accessed": "2026-02-10T...",
      "bookmark": false,
      "bookmark_key": null
    }
  ]
}
```

---

## Navigation Modes

### Mode 1: Fuzzy Jump (Primary)
```
Usage: j <query>        # Interactive fuzzy search
       j --jump work     # Direct jump to highest-scored "work"
```

- Type characters to filter directories
- Results reorder by frequency score in real-time
- `Enter` to confirm, `Esc` to cancel
- Vim bindings: `j/k` to move selection, `Ctrl+U/D` for page up/down

### Mode 2: Bookmark System
```
Usage: jump --bookmark add work     # Add "work" as bookmark
       jump --bookmark work         # Jump directly
       jump --bookmark list         # Show all bookmarks
```

- User-defined persistent shortcuts
- Custom keys: `jump -b w` → jumps to bookmark "w"
- Global or per-project bookmarks

### Mode 4: Current Behavior (Legacy)
```
Usage: jump --browse     # Current TUI with labels
       A S               # Old 2-char selection
```

- Kept for backwards compatibility
- Optional flag to enable
- May be deprecated in v2.0

---

## Scoring Algorithm

### Weighted Frequency Score

```
score = (access_count * recency_weight) + recency_bonus
```

Where:
- `recency_weight`: 0.1 (diminishes old access counts)
- `recency_bonus`: Exponential bonus for recent access
- Base score + incremental bonuses for repeated access

### Update Rules
- On each jump: increment `access_count`, update `last_accessed`
- Weekly decay: reduce scores of unused directories by 5%
- On `jump --clean`: remove entries with score < 1.0

---

## Vim Keybindings (Expanded)

| Key | Action |
|-----|--------|
| `j` / `k` | Move selection down/up |
| `Ctrl+D` / `Ctrl+U` | Page down/up |
| `Ctrl+F` | Fuzzy find mode |
| `Ctrl+B` | Bookmark mode |
| `g` / `G` | Go to top/bottom |
| `/` | Start search |
| `Enter` | Confirm selection |
| `Esc` / `Ctrl+C` | Cancel |
| `Ctrl+H` | Toggle hidden (in browse mode) |
| `Backspace` | Go up one directory (in browse mode) |

---

## Shell Integration

### Bash/Zsh
```bash
# Auto-eval on jump completion
eval "$(jump --shell-init)"

# Alias for quick jump
alias jj='jump --fuzzy'
```

### Fish
```fish
# In config.fish
eval (jump --shell-init fish)
```

### Vim/Neovim
```lua
-- In init.lua
vim.api.nvim_create_autocmd("DirChanged", {
  pattern = "*",
  callback = function()
    vim.fn.jobstart("jump --track")
  end
})
```

---

## CLI Interface

```
USAGE:
    jump [OPTIONS] [QUERY]

OPTIONS:
    -h, --help          Print help information
    -v, --version       Print version information
    -f, --fuzzy         Fuzzy search mode (default)
    -b, --bookmark      Bookmark management
    -a, --all           Show hidden directories
    --track             Track current directory (internal)
    --clean             Clean old entries from database
    --import <file>     Import from zoxide/bm
    --export <file>     Export database
    --stats             Show usage statistics
```

---

## Implementation Phases

### Phase 1: Foundation (Week 1) ✅ COMPLETE
- [x] Add database module (SQLite)
- [x] Implement scoring algorithm
- [x] Add persistent storage
- [x] Refactor directory scanner

### Phase 2: Fuzzy Search (Week 2) ✅ COMPLETE
- [x] Integrate fuzzy matching library (fuzzy-matcher)
- [x] Implement real-time filtering by score
- [x] Build fuzzy TUI with search input

### Phase 3: Bookmarks (Week 3) ✅ COMPLETE
- [x] Add bookmark CRUD operations
- [x] Custom key bindings for bookmarks

### Phase 4: Shell Integration (Week 4) ✅ COMPLETE
- [x] Generate shell init scripts (bash, zsh, fish)
- [x] Add shell completion
- [x] Write documentation (SHELL_INTEGRATION.md)

### Phase 5: Polish (Week 5) ✅ COMPLETE
- [x] Performance optimization (release profile)
- [x] Edge case handling
- [x] Tests and CI/CD (38 tests, GitHub Actions)
- [x] Release v1.0

---

## Dependencies

### Current (Keep)
- `crossterm` — Terminal I/O
- `ratatui` — TUI framework
- `walkdir` — Directory scanning
- `anyhow` — Error handling

### New (Add)
- `rusqlite` — SQLite database OR
- `serde` + `serde_json` — JSON storage (simpler)
- `fzf-rs` or `skim` — Fuzzy matching
- `clap` — CLI argument parsing (replace manual parsing)

### Potential Replacements
- Replace manual arg parsing with `clap`
- Keep existing TUI, enhance with search input

---

## Migration Strategy

### From Current (v0.1.x) to New (v1.0)
1. Auto-import not needed (current version stores no data
2. Backward compatibility: `--browse` flag for old TUI mode
3. Deprecation notice for 6 months, then remove `--browse`

### From zoxide/bm
```bash
jump --import ~/.config/zoxide/db.json
```

---

## File Structure

```
jump/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry point
│   ├── cli.rs            # Arg parsing (clap)
│   ├── config.rs         # Configuration
│   ├── database/
│   │   ├── mod.rs
│   │   ├── entry.rs      # DirEntry struct
│   │   └── db.rs         # SQLite/JSON operations
│   ├── scoring.rs        # Frequency algorithm
│   ├── scanner/
│   │   ├── mod.rs
│   │   └── walk.rs       # Directory scanning
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── app.rs        # App state
│   │   ├── fuzzy.rs      # Fuzzy search widget
│   │   ├── number.rs     # Bookmark widget
│   │   ├── browse.rs     # Legacy browse widget
│   │   └── components/   # Reusable UI components
│   ├── fuzzy/
│   │   └── matcher.rs    # Fuzzy matching logic
│   ├── bookmarks.rs      # Bookmark management
│   └── shell.rs         # Shell integration scripts
├── tests/
├── scripts/
│   └── install.sh
└── data/
    └── schema.sql
```

---

## Success Metrics

### Performance
- Fuzzy search: < 50ms for 10,000 entries
- TUI render: 60 FPS
- Database query: < 5ms

### UX
- 80% of jumps should be 3 keystrokes or fewer
- Muscle memory: users remember top 10 directories after 1 week
- Preference: 70%+ of users prefer jump over cd+tab

### Compatibility
- Linux: Tested on Ubuntu 22.04+, Fedora 38+
- macOS: Tested on Ventura+, Apple Silicon + Intel
- Windows: WSL2 fully supported
- Shells: Bash, Zsh, Fish

---

## Open Questions

1. **SQLite vs JSON?** SQLite is faster but adds compile time. JSON is simpler but slower at scale. Recommendation: SQLite for better performance at 1,000+ entries.

2. **Bookmarks format?** YAML for readability, or binary in DB? Recommendation: Binary in DB, export to YAML/JSON.

3. **Shell integration approach?** Generate shell scripts vs Rust binary emitting eval-able output? Recommendation: `--shell-init` that outputs correct format for shell detection.

4. **Scope creep?** Should we add file navigation too? Recommendation: No. Directory-only is a feature, not a limitation. Files come later in v2.0 if at all.

---

## Appendix: Reference Tools

### zoxide
- https://github.com/ajeetdsouza/zoxide
- Most similar tool, Rust-based, battle-tested
- Uses `z <query>` pattern

### fzf
- https://github.com/junegunn/fzf
- Fuzzy finder, can be used for dir navigation
- Template: `cd $(find . -type d | fzf)`

### bm (bookmarks)
- https://github.com/huyvohcmc/bm
- Bookmark-only approach

### autojump
- https://github.com/wting/autojump
- Original frequency-based directory jumper
