use std::env;
use std::io::Write;

use crate::shell::{print_completion, print_shell_init, Shell};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub show_hidden: bool,
    pub fuzzy_mode: bool,
    pub query: Option<String>,
    pub mode: AppMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppMode {
    #[default]
    Browse, // Legacy TUI mode with labels
    Fuzzy,    // Fuzzy search mode
    Number,   // Number selection mode
    Bookmark, // Bookmark mode
}

pub enum ParseResult {
    Config(Config),
    Exit,
}

pub enum ShellAction {
    None,
    ShellInit(Shell),
    Completions(Shell),
}

#[derive(Debug)]
pub enum BookmarkAction {
    Add {
        key: String,
        name: Option<String>,
        path: Option<String>,
    },
    Remove {
        key: String,
    },
    List,
    Jump {
        key: String,
    },
    None,
}

pub fn parse_args() -> (ParseResult, ShellAction, BookmarkAction) {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut shell_action = ShellAction::None;
    let mut bookmark_action = BookmarkAction::None;

    let mut show_hidden = false;
    let mut fuzzy_mode = false;
    let mut mode = AppMode::default();
    let mut query: Option<String> = None;

    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::io::stdout().flush().ok();
                return (ParseResult::Exit, ShellAction::None, BookmarkAction::None);
            }
            "-v" | "--version" => {
                println!("{} {}", NAME, VERSION);
                std::io::stdout().flush().ok();
                std::process::exit(0);
            }
            "-a" | "--all" => {
                show_hidden = true;
            }
            "-f" | "--fuzzy" => {
                fuzzy_mode = true;
                mode = AppMode::Fuzzy;
            }
            "-n" | "--number" => {
                mode = AppMode::Number;
            }
            "-b" | "--bookmark" => {
                // Handle bookmark subcommand
                let subcommand = iter.next().map(|s| s.as_str()).unwrap_or("");
                bookmark_action = match subcommand {
                    "add" | "a" => {
                        let key = iter.next().expect("Bookmark key required").clone();
                        let name_or_path = iter.next().cloned();
                        // Try to determine if it's a name or path
                        let name = name_or_path.clone();
                        let path =
                            name_or_path.filter(|s| s.starts_with('/') || s.starts_with("~"));
                        BookmarkAction::Add { key, name, path }
                    }
                    "remove" | "rm" | "del" => {
                        let key = iter.next().expect("Bookmark key required").clone();
                        BookmarkAction::Remove { key }
                    }
                    "list" | "ls" => BookmarkAction::List,
                    "jump" | "j" => {
                        let key = iter.next().expect("Bookmark key required").clone();
                        BookmarkAction::Jump { key }
                    }
                    "" | "help" | "h" => {
                        print_bookmark_help();
                        std::io::stdout().flush().ok();
                        return (ParseResult::Exit, ShellAction::None, BookmarkAction::None);
                    }
                    _ => {
                        eprintln!("Unknown bookmark subcommand: {}", subcommand);
                        print_bookmark_help();
                        std::process::exit(1);
                    }
                };
            }
            "--shell-init" => {
                let shell = args
                    .iter()
                    .find(|a| *a != "--shell-init")
                    .and_then(|a| match a.as_str() {
                        "bash" => Some(Shell::Bash),
                        "zsh" => Some(Shell::Zsh),
                        "fish" => Some(Shell::Fish),
                        "auto" => Shell::detect(),
                        _ => None,
                    })
                    .unwrap_or_else(|| Shell::detect().unwrap_or(Shell::Bash));

                let _ = print_shell_init(shell);
                return (
                    ParseResult::Exit,
                    ShellAction::ShellInit(shell),
                    BookmarkAction::None,
                );
            }
            "--completions" => {
                let shell = args
                    .iter()
                    .find(|a| *a != "--completions")
                    .and_then(|a| match a.as_str() {
                        "bash" => Some(Shell::Bash),
                        "zsh" => Some(Shell::Zsh),
                        "fish" => Some(Shell::Fish),
                        _ => None,
                    })
                    .unwrap_or_else(|| Shell::detect().unwrap_or(Shell::Bash));

                let _ = print_completion(shell, NAME);
                return (
                    ParseResult::Exit,
                    ShellAction::Completions(shell),
                    BookmarkAction::None,
                );
            }
            // Handle query arguments
            arg if arg.starts_with('-') => {
                // Unknown flag, ignore for now
            }
            _ => {
                // Treat as query
                if query.is_none() {
                    query = Some(arg.clone());
                }
            }
        }
    }

    // Default to fuzzy mode if not specified but query is provided
    if query.is_some() && mode == AppMode::default() {
        mode = AppMode::Fuzzy;
        fuzzy_mode = true;
    }

    (
        ParseResult::Config(Config {
            show_hidden,
            fuzzy_mode,
            query,
            mode,
        }),
        shell_action,
        bookmark_action,
    )
}

fn print_help() {
    println!(
        "{} {} - Vim-inspired directory navigation

USAGE:
    {} [OPTIONS] [QUERY]

MODES:
    (default)        Interactive TUI with labels
    -f, --fuzzy     Enable fuzzy search mode
    -n, --number    Number selection mode (persistent ranking)
    -b, --bookmark  Bookmark management

OPTIONS:
    -a, --all           Show hidden directories
    -h, --help          Print help information
    -v, --version       Print version information
    --shell-init [SHELL] Print shell initialization script (bash/zsh/fish/auto)
    --completions SHELL Print shell completion script (bash/zsh/fish)

KEYBINDINGS (Browse Mode):
    A-Z             Select label
    Ctrl+H          Toggle hidden files
    Backspace       Reset selection
    Esc / Ctrl+C    Cancel

KEYBINDINGS (Fuzzy Mode):
    /               Start search
    j / k           Move selection down/up
    Ctrl+U/D        Page up/down
    g / G           Go to top/bottom
    Enter           Confirm selection
    Backspace       Delete character
    Ctrl+H          Toggle hidden files
    Esc / Ctrl+C    Cancel

KEYBINDINGS (Number Mode):
    0-9             Enter number to jump
    Enter           Confirm selection
    Esc / Ctrl+C    Cancel
    j / k           Scroll down/up

KEYBINDINGS (Bookmark Mode):
    1-9             Quick jump to bookmark
    j / k           Scroll bookmarks
    Esc / Ctrl+C    Cancel

BOOKMARK COMMANDS:
    -b, --bookmark add <key> [name|path]  Add bookmark
    -b, --bookmark remove <key>           Remove bookmark
    -b, --bookmark list                    List all bookmarks
    -b, --bookmark jump <key>              Jump to bookmark

SHELL INTEGRATION:
    # Add to your shell rc file:
    eval \"$({} --shell-init)\"",
        NAME, VERSION, NAME, NAME
    );
}

fn print_bookmark_help() {
    println!(
        "{} bookmark - Bookmark management

USAGE:
    {} --bookmark <subcommand> [arguments]

SUBCOMMANDS:
    add <key> [name|path]  Add bookmark for current dir or specified path
    remove <key>         Remove bookmark by key
    list                 List all bookmarks
    jump <key>           Jump directly to bookmark

EXAMPLES:
    {} --bookmark add w work           # Bookmark current dir as 'w'
    {} --bookmark add p ~/projects     # Bookmark ~/projects as 'p'
    {} --bookmark list                 # Show all bookmarks
    {} --bookmark jump w               # Jump to 'w' bookmark",
        NAME, NAME, NAME, NAME, NAME, NAME
    );
}
