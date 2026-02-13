mod config;
mod database;
mod fs;
mod fuzzy;
mod input;
mod shell;
mod ui;

use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, fs::File, io, panic};

use config::{parse_args, AppMode, ParseResult};
use database::Database;
use input::InputEvent;
use ui::FuzzyState;

fn main() -> Result<()> {
    let (result, _, bookmark_action) = parse_args();

    match bookmark_action {
        config::BookmarkAction::None => {}
        action => {
            handle_bookmark_action(action)?;
            return Ok(());
        }
    }

    match result {
        ParseResult::Exit => Ok(()),
        ParseResult::Config(config) => run(config),
    }
}

fn handle_bookmark_action(action: config::BookmarkAction) -> Result<()> {
    let db = Database::new()?;

    match action {
        config::BookmarkAction::Add { key, name, path } => {
            let current_dir = env::current_dir()?;
            let target_path = path.unwrap_or_else(|| current_dir.to_string_lossy().into_owned());
            let target_name = name.unwrap_or_else(|| {
                std::path::Path::new(&target_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&target_path)
                    .to_string()
            });
            db.set_bookmark(&target_path, &target_name, &key)?;
            println!("Added bookmark '{}' -> {}", key, target_path);
        }
        config::BookmarkAction::Remove { key } => {
            db.remove_bookmark(&key)?;
            println!("Removed bookmark '{}'", key);
        }
        config::BookmarkAction::List => {
            let bookmarks = db.get_all_bookmarks()?;
            if bookmarks.is_empty() {
                println!("No bookmarks set. Use 'jump --bookmark add <key> <path>' to add one.");
            } else {
                println!("Bookmarks:");
                for bookmark in bookmarks {
                    let key = bookmark.bookmark_key.as_deref().unwrap_or("?");
                    println!("  [{}] {} ({})", key, bookmark.name, bookmark.path);
                }
            }
        }
        config::BookmarkAction::Jump { key } => {
            if let Some(entry) = db.get_by_bookmark_key(&key)? {
                println!("{}", entry.path);
            } else {
                eprintln!("Bookmark '{}' not found", key);
                std::process::exit(1);
            }
        }
        config::BookmarkAction::None => {}
    }

    Ok(())
}

fn run(config: config::Config) -> Result<()> {
    let current_dir = env::current_dir()?;

    setup_panic_hook();

    let tty = File::options()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .context("Failed to open /dev/tty - are you running in a terminal?")?;
    let mut tty_output = tty.try_clone()?;

    enable_raw_mode()?;
    execute!(tty_output, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(tty_output);
    let mut terminal = Terminal::new(backend)?;

    match config.mode {
        AppMode::Fuzzy => run_fuzzy_mode(&mut terminal, &current_dir, config.show_hidden, config.query.as_deref())?,
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_fuzzy_mode(
    terminal: &mut Terminal<CrosstermBackend<File>>,
    start_dir: &std::path::PathBuf,
    show_hidden: bool,
    query: Option<&str>,
) -> Result<()> {
    let mut fuzzy_state = FuzzyState::new_in_dir(start_dir, show_hidden);

    if let Some(q) = query {
        fuzzy_state.set_query(q);
    }

    let mut searching = false;

    loop {
        terminal.draw(|f| ui::draw_fuzzy(f, &fuzzy_state))?;

        match input::read_key(100)? {
            InputEvent::StartSearch => searching = true,
            InputEvent::Escape => {
                if searching {
                    searching = false;
                    fuzzy_state.clear_query();
                } else {
                    return Ok(());
                }
            }
            InputEvent::Enter => {
                if let Some(item) = fuzzy_state.selected_item() {
                    println!("{}", item.path());
                    return Ok(());
                }
            }
            InputEvent::NavigateIn => {
                if searching {
                    fuzzy_state.add_char('l');
                } else {
                    fuzzy_state.navigate_into();
                }
            }
            InputEvent::NavigateOut => {
                if searching {
                    fuzzy_state.add_char('h');
                } else {
                    fuzzy_state.navigate_back();
                }
            }
            InputEvent::Char(c) => {
                if searching {
                    fuzzy_state.add_char(c);
                } else if c == '/' {
                    searching = true;
                }
            }
            InputEvent::Backspace => {
                if searching {
                    fuzzy_state.pop_char();
                    if fuzzy_state.search_query.is_empty() {
                        searching = false;
                    }
                }
            }
            InputEvent::ScrollUp => fuzzy_state.move_up(),
            InputEvent::ScrollDown => fuzzy_state.move_down(),
            InputEvent::PageUp => fuzzy_state.page_up(),
            InputEvent::PageDown => fuzzy_state.page_down(),
            InputEvent::GoToStart => fuzzy_state.go_to_start(),
            InputEvent::GoToEnd => fuzzy_state.go_to_end(),
            InputEvent::ToggleHidden => {}
            InputEvent::None => {}
        }
    }
}

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}
