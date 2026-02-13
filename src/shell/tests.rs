#[cfg(test)]
mod tests {
    use crate::shell::{generate_bash_init, generate_fish_init, generate_zsh_init};

    #[test]
    fn test_bash_init_contains_function() {
        let script = generate_bash_init();
        assert!(script.contains("jump()"));
        assert!(script.contains("j()"));
        assert!(script.contains("command jump"));
        assert!(script.contains("bind"));
    }

    #[test]
    fn test_zsh_init_contains_function() {
        let script = generate_zsh_init();
        assert!(script.contains("jump()"));
        assert!(script.contains("j()"));
        assert!(script.contains("command jump"));
        assert!(script.contains("bindkey"));
    }

    #[test]
    fn test_fish_init_contains_function() {
        let script = generate_fish_init();
        assert!(script.contains("function jump"));
        assert!(script.contains("command jump"));
        assert!(script.contains("abbr"));
        assert!(script.contains("bind"));
    }

    #[test]
    fn test_bash_init_has_cd() {
        let script = generate_bash_init();
        assert!(script.contains("cd \"$target\""));
    }

    #[test]
    fn test_zsh_init_has_cd() {
        let script = generate_zsh_init();
        assert!(script.contains("cd \"$target\""));
    }

    #[test]
    fn test_fish_init_has_cd() {
        let script = generate_fish_init();
        assert!(script.contains("cd $target"));
    }

    #[test]
    fn test_bash_init_has_fuzzy_binding() {
        let script = generate_bash_init();
        assert!(script.contains("Ctrl+F") || script.contains("\\C-f"));
    }

    #[test]
    fn test_zsh_init_has_fuzzy_binding() {
        let script = generate_zsh_init();
        assert!(script.contains("^F"));
    }

    #[test]
    fn test_fish_init_has_fuzzy_binding() {
        let script = generate_fish_init();
        assert!(script.contains("\\cf"));
    }

    #[test]
    fn test_bash_init_not_empty() {
        let script = generate_bash_init();
        assert!(!script.is_empty());
        assert!(script.len() > 100);
    }

    #[test]
    fn test_zsh_init_not_empty() {
        let script = generate_zsh_init();
        assert!(!script.is_empty());
        assert!(script.len() > 100);
    }

    #[test]
    fn test_fish_init_not_empty() {
        let script = generate_fish_init();
        assert!(!script.is_empty());
        assert!(script.len() > 100);
    }
}

#[cfg(test)]
mod completion_tests {
    use crate::shell::{generate_bash_completion, generate_fish_completion, generate_zsh_completion};

    #[test]
    fn test_bash_completion_contains_complete() {
        let script = generate_bash_completion("jump");
        assert!(script.contains("complete -F"));
        assert!(script.contains("_jump"));
    }

    #[test]
    fn test_zsh_completion_contains_compdef() {
        let script = generate_zsh_completion("jump");
        assert!(script.contains("#compdef"));
        assert!(script.contains("_jump"));
    }

    #[test]
    fn test_fish_completion_contains_complete() {
        let script = generate_fish_completion("jump");
        assert!(script.contains("complete -c"));
        assert!(script.contains("jump"));
    }

    #[test]
    fn test_bash_completion_has_help_option() {
        let script = generate_bash_completion("jump");
        assert!(script.contains("--help"));
    }

    #[test]
    fn test_zsh_completion_has_help_option() {
        let script = generate_zsh_completion("jump");
        assert!(script.contains("help"));
    }

    #[test]
    fn test_fish_completion_has_all_option() {
        let script = generate_fish_completion("jump");
        assert!(script.contains("--all"));
    }

    #[test]
    fn test_bash_completion_not_empty() {
        let script = generate_bash_completion("jump");
        assert!(!script.is_empty());
    }

    #[test]
    fn test_zsh_completion_not_empty() {
        let script = generate_zsh_completion("jump");
        assert!(!script.is_empty());
    }

    #[test]
    fn test_fish_completion_not_empty() {
        let script = generate_fish_completion("jump");
        assert!(!script.is_empty());
    }
}
