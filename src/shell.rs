//! Shell integration for jump
//!
//! Provides shell initialization scripts and completions for bash, zsh, and fish.

use std::io;

/// Supported shell types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    /// Detect the current shell from environment
    pub fn detect() -> Option<Self> {
        let shell = std::env::var("SHELL").ok()?;
        if shell.contains("zsh") {
            Some(Shell::Zsh)
        } else if shell.contains("fish") {
            Some(Shell::Fish)
        } else if shell.contains("bash") {
            Some(Shell::Bash)
        } else {
            None
        }
    }

}

/// Generate bash shell integration script
pub fn generate_bash_init() -> String {
    let script = r#"# jump shell integration
# Add this to ~/.bashrc

# Autojump-style function
jump() {
    local target
    target="$(command jump "$@")"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return 1
    fi
}

# Short alias
j() { jump "$@"; }

# Fuzzy find mode (Ctrl+F)
_bind_jump_fzf() {
    local target
    target="$(command jump --fuzzy)"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return 1
    fi
}
bind -x '"\C-f": _bind_jump_fzf'
"#;
    script.to_string()
}

/// Generate zsh shell integration script
pub fn generate_zsh_init() -> String {
    let script = r#"# jump shell integration
# Add this to ~/.zshrc

# Autojump-style function
jump() {
    local target
    target="$(command jump "$@")"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return 1
    fi
}

# Short alias
j() { jump "$@"; }

# Fuzzy find mode (Ctrl+F)
_bind_jump_fzf() {
    local target
    target="$(command jump --fuzzy)"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return 1
    fi
}
zle -N _bind_jump_fzf
bindkey '^F' _bind_jump_fzf
"#;
    script.to_string()
}

/// Generate fish shell integration script
pub fn generate_fish_init() -> String {
    let script = r#"# jump shell integration
# Add this to ~/.config/fish/config.fish

# Main jump function
function jump --description "Quick directory navigation"
    set -l target (command jump $argv)
    if test -n "$target" -a -d "$target"
        cd $target
    end
end

# Short alias
abbr -a j jump

# Fuzzy find mode (Ctrl+F)
function _jump_fish_fzf --description "Jump with fuzzy search"
    set -l target (command jump --fuzzy)
    if test -n "$target" -a -d "$target"
        cd $target
    end
    commandline -f repaint
end
bind \cf _jump_fish_fzf
"#;
    script.to_string()
}

/// Generate shell init script for a specific shell
pub fn generate_shell_init(shell: Shell) -> String {
    match shell {
        Shell::Bash => generate_bash_init(),
        Shell::Zsh => generate_zsh_init(),
        Shell::Fish => generate_fish_init(),
    }
}

/// Output shell init script to stdout
pub fn print_shell_init(shell: Shell) -> io::Result<()> {
    let script = generate_shell_init(shell);
    println!("{}", script);
    Ok(())
}

/// Generate bash completion script
pub fn generate_bash_completion(name: &str) -> String {
    format!(
        r#"# jump bash completion
# Source this file: source <(jump --completions bash)

_{name}_jump() {{
    local cur prev words cword
    _init_completion || return

    case "$prev" in
        --completions)
            COMPREPLY=(bash zsh fish)
            return
            ;;
        --shell-init)
            COMPREPLY=(bash zsh fish auto)
            return
            ;;
    esac

    if [[ "$cur" == -* ]]; then
        COMPREPLY=($(compgen -W "$({name} --help 2>/dev/null | grep -oP '(--\S+)' | tr '\n' ' ')" -- "$cur"))
        return
    fi
}} && complete -F _{name}_jump {name}
"#,
        name = name
    )
}

/// Generate zsh completion script
pub fn generate_zsh_completion(name: &str) -> String {
    format!(
        r#"#compdef {name}

_{name}() {{
    local -a args
    args=(
        "-h[Show help]"
        "-v[Show version]"
        "-a[Show hidden directories]"
        "--all[Show hidden directories]"
        "--bookmark[Bookmark management]"
        "--shell-init[Print shell initialization script]"
        "--completions[Print shell completion script]"
    )
    _arguments -s $args
}}

_{name} "$@"
"#,
        name = name
    )
}

/// Generate fish completion script
pub fn generate_fish_completion(name: &str) -> String {
    format!(
        r#"# fish completion for {name}

complete -c {name} -f -a "(
    echo --all
    echo --bookmark
    echo --shell-init
    echo --completions
)"
"#,
        name = name
    )
}

/// Generate completion script for a specific shell
pub fn generate_completion(shell: Shell, name: &str) -> String {
    match shell {
        Shell::Bash => generate_bash_completion(name),
        Shell::Zsh => generate_zsh_completion(name),
        Shell::Fish => generate_fish_completion(name),
    }
}

/// Output completion script to stdout
pub fn print_completion(shell: Shell, name: &str) -> io::Result<()> {
    let script = generate_completion(shell, name);
    println!("{}", script);
    Ok(())
}
