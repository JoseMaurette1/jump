# jump

A minimal, Vim-inspired directory navigation tool for the terminal — built in Rust.

`jump` lets you move through directories using short, deterministic key sequences instead of typing long `cd` paths or relying on fuzzy search.

---

## TL;DR

- **What:** Keyboard-driven directory jumping
- **Where:** Linux, macOS (Apple Silicon), WSL, Windows
- **How:** Rust + terminal UI
- **Why:** Faster navigation, fewer keystrokes, zero fluff
- **Philosophy:** Movement primitive, not a file manager

---

## Installation

Linux / WSL:
```bash
curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | bash
```

macOS (Apple Silicon):
```bash
curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | zsh
```

Windows (PowerShell):
```powershell
irm https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.ps1 | iex
```

After install, restart your shell so PATH updates apply.

---

## Troubleshooting

### "version `GLIBC_2.xx` not found"

If you see an error like `/lib/x86_64-linux-gnu/libc.so.6: version 'GLIBC_2.39' not found`, your system's `glibc` is older than the one used to build the release binary.

To fix this:

1. Remove the broken binary:
   ```bash
   rm ~/.local/bin/jump
   ```

2. Install from source using Cargo:
   ```bash
   cargo install --git https://github.com/JoseMaurette1/jump
   ```

## Command-Line Options

```bash
jump           # Start with hidden files hidden (default)
jump -a        # Start with hidden files visible
jump --all     # Same as -a
jump -h        # Show help
jump -v        # Show version
```

---

