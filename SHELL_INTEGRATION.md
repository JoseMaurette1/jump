# Shell Integration

Jump provides seamless shell integration for bash, zsh, and fish shells. This enables auto-jumping to directories and fuzzy search keybindings.

## Quick Setup

### Bash/Zsh

```bash
# Add to ~/.bashrc or ~/.zshrc
eval "$(jump --shell-init)"
```

### Fish

```fish
# Add to ~/.config/fish/config.fish
eval (jump --shell-init fish)
```

## Manual Installation

### Bash

```bash
# Add to ~/.bashrc
jump() {
    local target
    target="$(command jump "$@")"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return 1
    fi
}
j() { jump "$@"; }
```

### Zsh

```zsh
# Add to ~/.zshrc
jump() {
    local target
    target="$(command jump "$@")"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return 1
    fi
}
j() { jump "$@"; }
```

### Fish

```fish
# Add to ~/.config/fish/config.fish
function jump --description "Quick directory navigation"
    set -l target (command jump $argv)
    if test -n "$target" -a -d "$target"
        cd $target
    end
end
```

## Shell Completion

Enable tab completion for jump commands:

### Bash

```bash
# Source completion in ~/.bashrc
source <(jump --completions bash)
```

### Zsh

```zsh
# Add to ~/.zshrc
source <(jump --completions zsh)
```

Or install completions manually:
```zsh
jump --completions zsh > ~/.zsh/completion/_jump
```

### Fish

```fish
# Add to ~/.config/fish/config.fish
source (jump --completions fish | psub)
```

## Keybindings

### Bash/Zsh

| Keybinding | Action |
|------------|--------|
| `Ctrl+F` | Fuzzy search jump |

### Fish

| Keybinding | Action |
|------------|--------|
| `Ctrl+F` | Fuzzy search jump |

## Commands

### Shell Init

```bash
# Auto-detect shell
jump --shell-init

# Explicit shell
jump --shell-init bash
jump --shell-init zsh
jump --shell-init fish
```

### Shell Completions

```bash
# Auto-detect shell
jump --completions

# Explicit shell
jump --completions bash
jump --completions zsh
jump --completions fish
```

## How It Works

The shell integration works by:

1. **Wrapper function**: `jump()` calls the binary and cds to the result
2. **Short alias**: `j` is a convenience alias for `jump`
3. **Keybindings**: `Ctrl+F` triggers fuzzy search mode

## Troubleshooting

### "command not found: jump"

Ensure jump is in your PATH:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.local/bin:$PATH"
```

### Completion not working

1. Restart your shell after adding completion
2. For zsh, ensure compinit is loaded:

```zsh
# In ~/.zshrc
autoload -Uz compinit
compinit
```

## Integration with Fuzzy Mode

The shell integration supports fuzzy search mode when using `Ctrl+F`:

```bash
# Fuzzy mode flow:
# 1. Press Ctrl+F
# 2. Type search query
# 3. Press Enter to select
# 4. Automatically cd to selected directory
```
