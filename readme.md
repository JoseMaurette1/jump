# Jump

A minimal, Vim-inspired directory navigation tool for the terminal. Jump provides multiple navigation modes—fuzzy search, bookmarks, and a legacy browse mode—all designed for speed and muscle memory.

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![Rust](https://img.shields.io/badge/rust-2021-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Fuzzy Search Mode** - Type to filter directories with real-time fuzzy matching
- **Bookmark System** - Persistent shortcuts with custom keys (e.g., `w` for work)
- **Legacy Browse Mode** - Two-character label navigation for backwards compatibility
- **Vim Keybindings** - `j/k` to scroll, `g/G` for top/bottom, `Ctrl+U/D` to page
- **Persistent History** - SQLite database tracks your most-visited directories
- **Shell Integration** - Works with Bash, Zsh, and Fish with auto-completion

## Installation

### From Source

```bash
git clone https://github.com/JoseMaurette1/jump
cd jump
cargo build --release
sudo cp target/release/jump /usr/local/bin/
```

### Using Install Script

```bash
curl -fsSL https://raw.githubusercontent.com/JoseMaurette1/jump/main/install.sh | bash
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

## Usage

### Fuzzy Search Mode (Default)

```bash
# Interactive fuzzy search
jump

# Direct jump to directory matching "work"
jump work

# Show hidden directories
jump -a
```

**Keybindings:**
| Key | Action |
|-----|--------|
| `/` | Start search |
| `j/k` | Move selection down/up |
| `Ctrl+U/D` | Page up/down |
| `g/G` | Go to top/bottom |
| `Enter` | Confirm selection |
| `Esc` | Cancel |

### Bookmark Management

```bash
# Add bookmark for current directory
jump --bookmark add w

# Add bookmark for specific path
jump --bookmark add p ~/projects

# List all bookmarks
jump --bookmark list

# Jump to bookmark
jump --bookmark jump w

# Remove bookmark
jump --bookmark remove w
```

### Legacy Browse Mode

```bash
jump --browse
```

Navigate using two-character labels (AA, AS, AD, etc.)

## Commands

```
jump [OPTIONS] [QUERY]

Options:
    -h, --help          Print help information
    -v, --version       Print version information
    -f, --fuzzy         Fuzzy search mode (default)
    -b, --bookmark      Bookmark management
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

## How It Works

### Scoring Algorithm

Jump uses a weighted frequency algorithm to rank directories:

```
score = (access_count * recency_weight) + recency_bonus
```

- **Recency weight:** 0.1 (diminishes old access counts)
- **Recency bonus:** Exponential bonus for recent access
- **Weekly decay:** Unused directories lose 5% score per week

### Database Schema

SQLite database with the following schema:

```sql
CREATE TABLE entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    score REAL NOT NULL DEFAULT 0.0,
    access_count INTEGER NOT NULL DEFAULT 0,
    last_accessed INTEGER NOT NULL DEFAULT 0,
    is_bookmark INTEGER NOT NULL DEFAULT 0,
    bookmark_key TEXT
);
```

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Project Structure

```
jump/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── cli.rs            # Clap CLI definitions
│   ├── config.rs         # Configuration parsing
│   ├── app.rs            # Browse mode app state
│   ├── fs.rs             # Directory scanning
│   ├── input.rs          # Input handling
│   ├── labels.rs         # Label generation
│   ├── scoring.rs        # Frequency algorithm
│   ├── shell.rs          # Shell integration
│   ├── database/         # Database module
│   │   ├── db.rs         # SQLite operations
│   │   └── entry.rs      # DirEntry struct
│   ├── fuzzy/            # Fuzzy search module
│   │   └── matcher.rs    # Fuzzy matching engine
│   └── ui/               # UI components
│       ├── browse.rs     # Legacy browse TUI
│       ├── fuzzy.rs      # Fuzzy search TUI
│       └── number.rs     # Bookmark TUI
└── Cargo.toml
```

## Dependencies

- `crossterm` — Terminal I/O
- `ratatui` — TUI framework
- `walkdir` — Directory scanning
- `anyhow` — Error handling
- `rusqlite` — SQLite database
- `serde` + `serde_json` — Serialization
- `clap` — CLI argument parsing
- `chrono` — Date/time handling
- `fuzzy-matcher` — Fuzzy matching
- `directories` — Platform directories

## License

MIT License - See [LICENSE](LICENSE) for details.

## Acknowledgments

Inspired by:
- [zoxide](https://github.com/ajeetdsouza/zoxide) — The smarter cd command
- [fzf](https://github.com/junegunn/fzf) — Command-line fuzzy finder
- [autojump](https://github.com/wting/autojump) — Original directory jumper
- [z](https://github.com/rupa/z) — Directory jumping tool

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Roadmap

- [ ] Import/export from zoxide
- [ ] Directory exclusions configuration
- [ ] Performance metrics display
- [ ] Custom keybinding configuration

---

**Made with Rust** 🦀
