#!/bin/bash
# Shell integration for jump
# Add to .bashrc or .zshrc:
#   source /path/to/jump.sh

jump() {
    local target
    target="$(command jump "$@")"
    # Debug: uncomment to see what path is returned
    # echo "DEBUG: target='$target'" >&2

    if [[ -n "$target" ]]; then
        if [[ -d "$target" ]]; then
            cd "$target" || return 1
        elif [[ -f "$target" ]]; then
            # Open file in neovim (vim alias should point to nvim)
            vim "$target"
        else
            # Debug: path check failed
            # echo "DEBUG: target is not a file or directory" >&2
            # echo "DEBUG: ls -la '$target'" >&2
            # ls -la "$target" 2>&1 | head -5 >&2
            :
        fi
    fi
}

j() { jump "$@"; }
