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

use config::{parse_args, ParseResult};
use database::Database;
use input::InputEvent;
use ui::FuzzyState;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Search,
    BookmarkInput(String),
    BookmarkRemove,
}

fn main() -> Result<()> {
    let (result, bookmark_action) = parse_args();

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

    run_fuzzy_mode(
        &mut terminal,
        &current_dir,
        config.show_hidden,
        config.query.as_deref(),
    )?;

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
    let db = Database::new().ok();
    let mut fuzzy_state = FuzzyState::new_in_dir(start_dir, show_hidden);

    // Load bookmarks from DB
    if let Some(ref db) = db {
        if let Ok(bookmarks) = db.get_all_bookmarks() {
            fuzzy_state.set_bookmarks(bookmarks);
        }
    }

    if let Some(q) = query {
        fuzzy_state.set_query(q);
    }

    let mut mode = Mode::Normal;

    loop {
        terminal.draw(|f| ui::draw_fuzzy(f, &fuzzy_state, &mode))?;

        match input::read_key(100)? {
            InputEvent::StartSearch => match mode {
                Mode::Normal => mode = Mode::Search,
                Mode::Search => fuzzy_state.add_char('/'),
                Mode::BookmarkInput(ref mut alias) => alias.push('/'),
                Mode::BookmarkRemove => {}
            },
            InputEvent::Escape => match mode {
                Mode::Search => {
                    mode = Mode::Normal;
                    fuzzy_state.clear_query();
                }
                Mode::BookmarkInput(_) | Mode::BookmarkRemove => {
                    mode = Mode::Normal;
                }
                Mode::Normal => {
                    return Ok(());
                }
            },
            InputEvent::Enter => match mode {
                Mode::BookmarkInput(ref alias) => {
                    if !alias.is_empty() {
                        if let Some(item) = fuzzy_state.selected_item() {
                            let path = item.path();
                            let name = item.entry.name.clone();
                            if let Some(ref db) = db {
                                let _ = db.set_bookmark(&path, &name, alias);
                                if let Ok(bookmarks) = db.get_all_bookmarks() {
                                    fuzzy_state.set_bookmarks(bookmarks);
                                    fuzzy_state.refilter();
                                }
                            }
                        }
                    }
                    mode = Mode::Normal;
                }
                Mode::BookmarkRemove => {
                    if let Some(item) = fuzzy_state.selected_item() {
                        if let Some(ref key) = item.bookmark_key {
                            if let Some(ref db) = db {
                                let _ = db.remove_bookmark(key);
                                if let Ok(bookmarks) = db.get_all_bookmarks() {
                                    fuzzy_state.set_bookmarks(bookmarks);
                                    fuzzy_state.refilter();
                                }
                            }
                        }
                    }
                    mode = Mode::Normal;
                }
                _ => {
                    if let Some(item) = fuzzy_state.selected_item() {
                        println!("{}", item.path());
                        return Ok(());
                    }
                }
            },
            InputEvent::NavigateIn => match mode {
                Mode::Search => fuzzy_state.add_char('l'),
                Mode::BookmarkInput(ref mut alias) => alias.push('l'),
                Mode::Normal => fuzzy_state.navigate_into(),
                Mode::BookmarkRemove => {}
            },
            InputEvent::NavigateOut => match mode {
                Mode::Search => fuzzy_state.add_char('h'),
                Mode::BookmarkInput(ref mut alias) => alias.push('h'),
                Mode::Normal => fuzzy_state.navigate_back(),
                Mode::BookmarkRemove => {}
            },
            InputEvent::Bookmark => match mode {
                Mode::Normal => {
                    if fuzzy_state.selected_item().is_some() {
                        mode = Mode::BookmarkInput(String::new());
                    }
                }
                Mode::Search => fuzzy_state.add_char('b'),
                Mode::BookmarkInput(ref mut alias) => alias.push('b'),
                Mode::BookmarkRemove => {}
            },
            InputEvent::RemoveBookmark => match mode {
                Mode::Normal => {
                    if let Some(item) = fuzzy_state.selected_item() {
                        if item.is_bookmark {
                            mode = Mode::BookmarkRemove;
                        }
                    }
                }
                Mode::Search => fuzzy_state.add_char('x'),
                Mode::BookmarkInput(ref mut alias) => alias.push('x'),
                Mode::BookmarkRemove => {}
            },
            InputEvent::Char(c) => match mode {
                Mode::Search => fuzzy_state.add_char(c),
                Mode::BookmarkInput(ref mut alias) => alias.push(c),
                Mode::Normal => {
                    if c == '/' {
                        mode = Mode::Search;
                    } else if c.is_ascii_digit() {
                        let digit = c.to_digit(10).unwrap() as usize;
                        let new_count = fuzzy_state.motion_count.unwrap_or(0) * 10 + digit;
                        fuzzy_state.set_motion_count(new_count);
                    }
                }
                Mode::BookmarkRemove => {}
            },
            InputEvent::Backspace => match mode {
                Mode::Search => {
                    fuzzy_state.pop_char();
                    if fuzzy_state.search_query.is_empty() {
                        mode = Mode::Normal;
                    }
                }
                Mode::BookmarkInput(ref mut alias) => {
                    alias.pop();
                }
                Mode::Normal | Mode::BookmarkRemove => {}
            },
            InputEvent::ScrollUp => match mode {
                Mode::Search => fuzzy_state.add_char('k'),
                Mode::BookmarkInput(ref mut alias) => alias.push('k'),
                Mode::Normal => {
                    let count = fuzzy_state.take_motion_count();
                    for _ in 0..count {
                        fuzzy_state.move_up();
                    }
                }
                Mode::BookmarkRemove => {}
            },
            InputEvent::ScrollDown => match mode {
                Mode::Search => fuzzy_state.add_char('j'),
                Mode::BookmarkInput(ref mut alias) => alias.push('j'),
                Mode::Normal => {
                    let count = fuzzy_state.take_motion_count();
                    for _ in 0..count {
                        fuzzy_state.move_down();
                    }
                }
                Mode::BookmarkRemove => {}
            },
            InputEvent::GoToStart => match mode {
                Mode::Search => fuzzy_state.add_char('g'),
                Mode::BookmarkInput(ref mut alias) => alias.push('g'),
                Mode::Normal => fuzzy_state.go_to_start(),
                Mode::BookmarkRemove => {}
            },
            InputEvent::GoToEnd => match mode {
                Mode::Search => fuzzy_state.add_char('G'),
                Mode::BookmarkInput(ref mut alias) => alias.push('G'),
                Mode::Normal => fuzzy_state.go_to_end(),
                Mode::BookmarkRemove => {}
            },
            InputEvent::PageUp => match mode {
                Mode::Normal | Mode::Search => fuzzy_state.page_up(),
                Mode::BookmarkInput(_) | Mode::BookmarkRemove => {}
            },
            InputEvent::PageDown => match mode {
                Mode::Normal | Mode::Search => fuzzy_state.page_down(),
                Mode::BookmarkInput(_) | Mode::BookmarkRemove => {}
            },
            InputEvent::ToggleHidden => match mode {
                Mode::Search => fuzzy_state.add_char('.'),
                Mode::BookmarkInput(ref mut alias) => alias.push('.'),
                Mode::Normal => fuzzy_state.toggle_hidden(),
                Mode::BookmarkRemove => {}
            },
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
