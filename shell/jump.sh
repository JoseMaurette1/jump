#!/bin/bash
# Shell integration for jump
# Add to .bashrc or .zshrc:
#   source /path/to/jump.sh

jump() {
    local target
    target="$(command jump "$@")"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return 1
    fi
}

j() { jump "$@"; }
