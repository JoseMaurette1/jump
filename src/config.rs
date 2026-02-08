use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub show_hidden: bool,
}

pub enum ParseResult {
    Config(Config),
    Exit,
}

pub fn parse_args() -> ParseResult {
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in &args {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return ParseResult::Exit;
            }
            "-v" | "--version" => {
                println!("{} {}", NAME, VERSION);
                return ParseResult::Exit;
            }
            _ => {}
        }
    }

    let show_hidden = args.iter().any(|a| a == "-a" || a == "--all");

    ParseResult::Config(Config { show_hidden })
}

fn print_help() {
    println!(
        "{} {} - Vim-inspired directory navigation

USAGE:
    {} [OPTIONS]

OPTIONS:
    -a, --all       Show hidden directories
    -h, --help      Print help information
    -v, --version   Print version information

KEYBINDINGS:
    A-Z             Select label
    Backspace       Reset selection
    Esc / Ctrl+C    Cancel",
        NAME, VERSION, NAME
    );
}
