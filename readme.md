# Jump

A Vim-inspired directory navigator for the terminal. Fuzzy search, bookmarks, file creation, and full tree navigation — all from a fast TUI with Vim keybindings.

![Version](https://img.shields.io/badge/version-1.2.0-blue)
![Rust](https://img.shields.io/badge/rust-2021-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Fuzzy Search** — Press `/` to filter with real-time fuzzy matching (SkimMatcherV2)
- **Tree Navigation** — `h/l` to traverse parent/child directories without leaving the TUI
- **Bookmark System** — Persist shortcuts with custom aliases (`b` to add, `x` to remove)
- **File Browsing** — Press `f` to toggle files alongside directories; selecting a file opens it in Neovim
- **File & Directory Creation** — Press `a`, type a name, end with `/` for a directory or omit for a file
- **Vim Keybindings** — `j/k`, `g/G`, `Ctrl+U/D`, motion counts (`3j`), relative line numbers
- **Help Overlay** — Press `?` for a full-screen keybinding reference
- **Shell Integration** — Works with Bash, Zsh, and Fish

## Installation

### Linux / WSL

```bash
curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | bash
```

### macOS (Apple Silicon)

```bash
curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | zsh
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.ps1 | iex
```

### From Source

```bash
git clone https://github.com/JoseMaurette1/jump
cd jump
cargo build --release
sudo cp target/release/jump /usr/local/bin/
```

### Shell Setup

The installer handles this automatically. For manual setup, add to your shell config:

**Bash/Zsh:**
```bash
jump() {
    local target
    target="$(command jump "$@")"
    if [[ -n "$target" ]]; then
        if [[ -d "$target" ]]; then
            cd "$target"
        elif [[ -f "$target" ]]; then
            vim "$target"
        fi
    fi
}
alias j="jump"
```

Or use the built-in generator:
```bash
eval "$(jump --shell-init)"
```

See [SHELL_INTEGRATION.md](SHELL_INTEGRATION.md) for full details.

## Usage

```bash
j              # Launch navigator (hidden files hidden, files hidden)
j -a           # Launch with hidden files visible
j --all        # Same as -a
j -h           # Show help
j -v           # Show version
```

## Keybindings

### Navigation

| Key | Action |
|-----|--------|
| `j` / `k` | Move selection down / up |
| `h` | Navigate to parent directory |
| `l` | Navigate into selected directory |
| `g` | Go to first item |
| `G` | Go to last item |
| `Ctrl+U` / `Ctrl+D` | Page up / down |
| `[0-9]` prefix | Motion count (e.g. `3j` moves down 3) |

### Search & Selection

| Key | Action |
|-----|--------|
| `/` | Enter search mode (fuzzy filter) |
| `Enter` | Select — `cd` into directory, or open file in Neovim |
| `Esc` | Cancel / quit |

### Bookmarks

| Key | Action |
|-----|--------|
| `b` | Bookmark selected directory (prompts for alias) |
| `x` | Remove bookmark from selected directory |

### Toggles & Creation

| Key | Action |
|-----|--------|
| `.` | Toggle hidden files on/off |
| `f` | Toggle file visibility (show files alongside dirs) |
| `a` | Create — type a name, end with `/` for directory, without for file |

### Other

| Key | Action |
|-----|--------|
| `?` | Show help overlay (press any key to return) |

## Bookmark Management (CLI)

```bash
jump --bookmark add w ~/projects/work   # Add bookmark with alias "w"
jump --bookmark list                    # List all bookmarks
jump --bookmark jump w                  # Jump to bookmark "w"
jump --bookmark remove w                # Remove bookmark "w"
```

## CLI Reference

```
jump [OPTIONS] [QUERY]

Options:
    -h, --help          Print help information
    -v, --version       Print version information
    -a, --all           Show hidden files/directories
    --shell-init        Generate shell initialization script
    --completions       Generate shell completion script

Bookmark Commands:
    jump --bookmark add <key> [path]     Add bookmark
    jump --bookmark remove <key>         Remove bookmark
    jump --bookmark list                 List bookmarks
    jump --bookmark jump <key>           Jump to bookmark
```

## Data Storage

Bookmarks are stored in a SQLite database (WAL mode):

- **Linux / WSL:** `~/.local/share/jump/jump.db`
- **macOS:** `~/Library/Application Support/jump/jump.db`
- **Windows:** `%APPDATA%\jump\data\jump.db`

## Project Structure

```
src/
├── main.rs              # Entry point, event loop, mode dispatch
├── config.rs            # CLI argument parsing (custom parser)
├── fs.rs                # Directory/file scanning (walkdir, depth=1)
├── input.rs             # Crossterm key event → InputEvent mapping
├── shell.rs             # Shell init & completion (bash/zsh/fish)
├── fuzzy/
│   └── matcher.rs       # SkimMatcherV2 fuzzy scoring wrapper
├── ui/
│   └── fuzzy.rs         # FuzzyState + ratatui renderer (draw_fuzzy, draw_help)
└── database/
    ├── db.rs            # SQLite operations (WAL mode)
    └── entry.rs         # DirEntry struct for persistence
```

## Dependencies

- `ratatui` (0.29) + `crossterm` (0.28) — TUI rendering and terminal control
- `rusqlite` (0.32, bundled) — SQLite database
- `fuzzy-matcher` (0.3) — SkimMatcherV2 scoring
- `walkdir` (2.5) — Directory traversal
- `directories` (5) — Platform-specific data paths
- `anyhow` + `thiserror` — Error handling

## Development

```bash
cargo build                # Debug build
cargo build --release      # Optimized release (LTO, strip, opt-level=3)
cargo test                 # Run all tests
cargo test -- --nocapture  # Run tests with stdout visible
```

## License

MIT — see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome. Fork the repo, create a feature branch, and open a Pull Request.
