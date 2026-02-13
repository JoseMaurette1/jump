use std::env;
use std::io::Write;

use crate::shell::{print_completion, print_shell_init, Shell};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub show_hidden: bool,
    pub query: Option<String>,
}

pub enum ParseResult {
    Config(Config),
    Exit,
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

pub fn parse_args() -> (ParseResult, BookmarkAction) {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut bookmark_action = BookmarkAction::None;

    let mut show_hidden = false;
    let mut query: Option<String> = None;

    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::io::stdout().flush().ok();
                return (ParseResult::Exit, BookmarkAction::None);
            }
            "-v" | "--version" => {
                println!("{} {}", NAME, VERSION);
                std::io::stdout().flush().ok();
                std::process::exit(0);
            }
            "-a" | "--all" => {
                show_hidden = true;
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
                        return (ParseResult::Exit, BookmarkAction::None);
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
                return (ParseResult::Exit, BookmarkAction::None);
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
                return (ParseResult::Exit, BookmarkAction::None);
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

    (
        ParseResult::Config(Config { show_hidden, query }),
        bookmark_action,
    )
}

fn print_help() {
    println!(
        "{} {} - Vim-inspired directory navigation

USAGE:
    {} [OPTIONS] [QUERY]

OPTIONS:
    -a, --all           Show hidden directories
    -h, --help          Print help information
    -v, --version       Print version information
    -b, --bookmark      Bookmark management
    --shell-init [SHELL] Print shell initialization script (bash/zsh/fish/auto)
    --completions SHELL Print shell completion script (bash/zsh/fish)

KEYBINDINGS:
    /               Start search
    j / k           Move selection down/up
    h / l           Navigate to parent/child directory
    Ctrl+U/D        Page up/down
    g / G           Go to top/bottom
    Enter           Confirm selection
    Backspace       Delete character
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
