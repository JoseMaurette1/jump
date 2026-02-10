use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Fuzzy,
    Number,
    Browse,
}

#[derive(Debug, Clone, ValueEnum)]
enum Shell {
    Bash,
    Zsh,
    Fish,
}

#[derive(Parser)]
#[command(name = "jump")]
#[command(author = "JoseMaurette1")]
#[command(version = "1.0.0")]
#[command(about = "A minimal, Vim-inspired directory navigation tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, global = true)]
    pub all: bool,

    #[arg(short, long, global = true)]
    pub hidden: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Fuzzy search and jump to a directory
    #[command(name = "jump")]
    Jump {
        /// Query to match against directory names
        query: Option<String>,
    },

    /// Interactive number-based selection
    #[command(name = "num", alias = "n")]
    Number {
        /// Optional filter query
        query: Option<String>,
    },

    /// Interactive bookmark management
    #[command(name = "book", alias = "b", visible_alias = "bookmark")]
    Bookmark {
        #[command(subcommand)]
        command: Option<BookmarkCommands>,
    },

    /// Browse current directory with labels (legacy mode)
    #[command(name = "browse")]
    Browse {
        /// Start from a specific directory
        path: Option<PathBuf>,
    },

    /// Track the current directory in the database
    #[command(name = "track", alias = "t")]
    Track {
        /// Directory to track (defaults to current directory)
        path: Option<PathBuf>,
    },

    /// Show usage statistics
    #[command(name = "stats")]
    Stats,

    /// Clean old entries from the database
    #[command(name = "clean")]
    Clean {
        /// Remove entries below this score threshold
        #[arg(default_value = "1.0")]
        threshold: f64,
    },

    /// Generate shell integration script
    #[command(name = "shell")]
    Shell {
        /// Shell type to generate for
        #[arg(value_enum)]
        shell: Option<Shell>,
    },

    /// Import entries from another tool
    #[command(name = "import")]
    Import {
        /// File to import from (zoxide JSON, etc.)
        file: PathBuf,
    },

    /// Export entries to a file
    #[command(name = "export")]
    Export {
        /// Output file path
        file: PathBuf,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum BookmarkCommands {
    /// Add a bookmark for the current or specified directory
    #[command(name = "add")]
    Add {
        /// Bookmark key (shortcut)
        key: String,

        /// Directory to bookmark (defaults to current directory)
        path: Option<PathBuf>,
    },

    /// Remove a bookmark
    #[command(name = "remove", alias = "rm", alias = "del")]
    Remove {
        /// Bookmark key to remove
        key: String,
    },

    /// List all bookmarks
    #[command(name = "list", alias = "ls")]
    List,

    /// Jump to a bookmark directly
    #[command(name = "go")]
    Go {
        /// Bookmark key to jump to
        key: String,
    },
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn is_help(&self) -> bool {
        std::env::args().any(|arg| arg == "-h" || arg == "--help")
    }

    pub fn is_version(&self) -> bool {
        std::env::args().any(|arg| arg == "-v" || arg == "--version")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_jump_without_query() {
        let cli = Cli::parse_from(["jump"]);
        match cli.command {
            Some(Commands::Jump { query }) => assert!(query.is_none()),
            _ => panic!("Expected Jump command"),
        }
    }

    #[test]
    fn test_cli_jump_with_query() {
        let cli = Cli::parse_from(["jump", "work"]);
        match cli.command {
            Some(Commands::Jump { query }) => assert_eq!(query, Some("work".to_string())),
            _ => panic!("Expected Jump command"),
        }
    }

    #[test]
    fn test_cli_bookmark_add() {
        let cli = Cli::parse_from(["bookmark", "add", "w"]);
        match cli.command {
            Some(Commands::Bookmark {
                command: Some(BookmarkCommands::Add { key, path: _ }),
            }) => {
                assert_eq!(key, "w")
            }
            _ => panic!("Expected Bookmark Add command"),
        }
    }

    #[test]
    fn test_cli_all_flag() {
        let cli = Cli::parse_from(["jump", "-a", "work"]);
        assert!(cli.all);
    }
}
