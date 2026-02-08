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

Linux / macOS (Apple Silicon) / WSL:
```bash
curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | bash
```

Windows (PowerShell):
```powershell
irm https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.ps1 | iex
```

After install, restart your shell so PATH updates apply.

---

## Problem Statement

Terminal navigation is slow and repetitive.

Typical workflow:
```bash
cd projects
ls
cd school
ls
cd cs

Even with tab completion, this costs attention and time.

Fuzzy finders help, but:

They require typing

They rely on ranking heuristics

They break muscle memory

Vim solved this problem for text.
jump applies the same idea to directories.

Core Idea

Instead of typing directory names:

Trigger jump mode

Directories are labeled with short key sequences

Type the label

Instantly move there

No searching. No guessing. No UI clutter.

Non-Goals (Very Important)

jump is intentionally limited.

It will not:

Replace ls

Act as a file manager

Preview files

Use fuzzy matching

Use the mouse

Animate anything

Constraints are a feature.

User Experience
Example

Current directory:

~/projects


Trigger jump mode:

ER → school/
UI → work/
OA → experiments/


Typing ER instantly jumps into school/.

Key Principles

Deterministic – same directory = same label

Fast – no indexing, no background daemons

Minimal UI – text only

Modal – inspired by Vim

Composable – works with your shell, not against it

Application Name
Name: jump

Reasons:

Short and memorable

Verb-based (Unix style)

Describes intent clearly

Looks good in commands and resumes

Binary:

jump


Repository:

jump

Platform Support
Supported

Linux

macOS (Apple Silicon only)

WSL (Windows Subsystem for Linux)

Windows (PowerShell)

Not Supported

macOS Intel

How It Works (High Level)

User runs jump

jump scans the current directory

Directories are assigned short key labels

A minimal overlay is rendered

User types a label

jump prints the selected path

Shell wrapper performs cd

Why Shell Integration Is Required

A standalone binary cannot change the parent shell’s directory.

Therefore jump:

Outputs a path to stdout

Is wrapped in a shell function that performs cd

This is standard practice (used by tools like zoxide, fzf, etc.).

Shell Integration
Bash / Zsh

Add this to .bashrc or .zshrc:

jump() {
  cd "$(command jump "$@")"
}


Now jump behaves like a built-in command.

PowerShell

Add this to your PowerShell profile:
```powershell
function jump { Set-Location (jump.exe @args) }
Set-Alias j jump
```

Frontend (install page)

The `frontend/` folder contains a small static page with copy-to-clipboard
install commands. Open `frontend/index.html` directly or run:

```bash
cd frontend
python3 -m http.server 5173
```

Technical Stack
Language

Rust

Safety

Performance

Cross-platform

Excellent terminal ecosystem

Core Crates
Crate	Purpose
crossterm	Terminal input/output
ratatui	Minimal TUI rendering
walkdir	Directory traversal
anyhow	Error handling

Optional later:

serde (config)

dirs (cache paths)

Architecture
jump
├── src
│   ├── main.rs        # entrypoint
│   ├── app.rs         # application state
│   ├── ui.rs          # rendering logic
│   ├── input.rs       # key handling
│   ├── labels.rs      # key label generation
│   ├── fs.rs          # filesystem scanning
│   └── config.rs      # optional config
├── shell
│   └── jump.sh        # shell wrapper example
├── frontend
│   ├── index.html     # small install page
│   └── app.js         # copy-to-clipboard logic
├── Cargo.toml
└── readme.md

Label Generation Strategy

Two-letter labels

Home-row–friendly keys

Deterministic order

No collisions

Example key pool:

A S D F G H J K L Q W E R U I O


Generated combinations:

AS AD AF ...


This guarantees:

Speed

Muscle memory

Predictability

UI Design
Appearance

Single overlay

No borders or decorations

Text only

High contrast

Behavior

Appears instantly

Disappears on selection or cancel

Never blocks shell after exit

Keybindings (Initial)
Key	Action
F	Enter jump mode
Esc	Cancel
A-Z	Select label

Configurable later.

Performance Goals

Startup < 10ms

No background processes

No persistent index

Minimal memory usage

Security & Safety

Read-only filesystem access

No shell execution

No network access

No telemetry

Future Roadmap (Optional)

Config file support

File jumping (not just directories)

Frequency-based label ordering

Git-root awareness

Neovim integration

Wayland / tmux friendliness
