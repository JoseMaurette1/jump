# Jump

A minimal, Vim-inspired directory navigation tool for the terminal. Jump provides fuzzy search, bookmarks, and full directory tree navigation — all with vim keybindings.

![Version](https://img.shields.io/badge/version-1.1.0-blue)
![Rust](https://img.shields.io/badge/rust-2021-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Fuzzy Search** - Type `/` to filter directories with real-time fuzzy matching
- **Bookmark System** - Persistent shortcuts with custom keys (e.g., `b` to bookmark, `x` to remove)
- **Tree Navigation** - `h/l` to navigate parent/child directories without leaving the TUI
- **Vim Keybindings** - `j/k` scroll, `g/G` top/bottom, `Ctrl+U/D` page, `3j` motion counts
- **Relative Line Numbers** - Vim-style relative numbering for quick counted motions
- **Shell Integration** - Works with Bash, Zsh, and Fish with auto-completion

## Installation

### From Source

```bash
git clone https://github.com/JoseMaurette1/jump
cd jump
cargo build --release
sudo cp target/release/jump /usr/local/bin/
```

### Shell Setup

Add to your shell configuration:

**Bash/Zsh:**
```bash
eval "$(jump --shell-init)"
```

**Fish:**
```fish
eval (jump --shell-init fish)
```

See [SHELL_INTEGRATION.md](SHELL_INTEGRATION.md) for full details.

## Usage

### Interactive Mode (Default)

```bash
# Launch interactive fuzzy navigator
jump

# Show hidden directories
jump -a
```

### Keybindings

| Key | Mode | Action |
|-----|------|--------|
| `/` | Normal | Enter search mode |
| `j` / `k` | Normal | Move selection down/up |
| `h` / `l` | Normal | Navigate to parent/child directory |
| `g` / `G` | Normal | Go to top/bottom |
| `Ctrl+U` / `Ctrl+D` | Normal | Page up/down |
| `[0-9]` | Normal | Motion count prefix (e.g. `3j` moves down 3) |
| `b` | Normal | Bookmark selected directory |
| `x` | Normal | Remove bookmark |
| `.` | Normal | Toggle hidden files |
| `Enter` | Any | Select directory |
| `Esc` | Any | Cancel / exit |
| `Backspace` | Search | Delete character |

### Bookmark Management (CLI)

```bash
# Add bookmark
jump --bookmark add w ~/projects/work

# List all bookmarks
jump --bookmark list

# Jump to bookmark
jump --bookmark jump w

# Remove bookmark
jump --bookmark remove w
```

## Commands

```
jump [OPTIONS] [QUERY]

Options:
    -h, --help          Print help information
    -v, --version       Print version information
    -a, --all           Show hidden directories
    --shell-init        Generate shell initialization script
    --completions       Generate shell completion script

Bookmark Commands:
    jump --bookmark add <key> [path]     Add bookmark
    jump --bookmark remove <key>         Remove bookmark
    jump --bookmark list                 List bookmarks
    jump --bookmark jump <key>           Jump to bookmark
```

## Configuration

Jump stores its database in the platform's data directory:

- **Linux:** `~/.local/share/jump/jump.db`
- **macOS:** `~/Library/Application Support/jump/jump.db`
- **Windows:** `%APPDATA%\jump\data\jump.db`

## Project Structure

```
src/
├── main.rs              # Entry point, event loop, mode dispatch
├── config.rs            # CLI argument parsing (custom parser)
├── fs.rs                # Directory scanning (walkdir, depth=1)
├── input.rs             # Crossterm key event → InputEvent mapping
├── shell.rs             # Shell init & completion (bash/zsh/fish)
├── fuzzy/
│   └── matcher.rs       # SkimMatcherV2 fuzzy scoring wrapper
├── ui/
│   └── fuzzy.rs         # FuzzyState state machine + ratatui renderer
└── database/
    ├── db.rs            # SQLite operations (WAL mode)
    └── entry.rs         # DirEntry struct for persistence
```

## Dependencies

- `ratatui` + `crossterm` — TUI rendering and terminal control
- `rusqlite` (bundled) — SQLite database
- `fuzzy-matcher` — SkimMatcherV2 scoring
- `walkdir` — Directory traversal
- `directories` — Platform-specific data paths
- `anyhow` + `thiserror` — Error handling

## Development

```bash
cargo build                # Debug build
cargo build --release      # Optimized release (LTO, strip)
cargo test                 # Run all tests
```

## License

MIT License - See [LICENSE](LICENSE) for details.

## Acknowledgments

Inspired by [zoxide](https://github.com/ajeetdsouza/zoxide), [fzf](https://github.com/junegunn/fzf), and [autojump](https://github.com/wting/autojump).

## Contributing

Contributions are welcome! Fork the repo, create a feature branch, and open a Pull Request.
