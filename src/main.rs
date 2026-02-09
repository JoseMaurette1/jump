mod app;
mod config;
mod fs;
mod input;
mod labels;
mod ui;

use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, fs::File, io, panic};

use app::App;
use config::{parse_args, ParseResult};
use input::InputEvent;

fn main() -> Result<()> {
    match parse_args() {
        ParseResult::Exit => Ok(()),
        ParseResult::Config(config) => run(config),
    }
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

    let mut app = App::new(current_dir, config.show_hidden);

    while !app.is_done() {
        terminal.draw(|f| ui::draw(f, &app))?;

        match input::read_key(100)? {
            InputEvent::Char(c) => app.handle_key(c),
            InputEvent::Escape => app.cancel(),
            InputEvent::Backspace => app.go_up(),
            InputEvent::Enter => app.confirm(),
            InputEvent::ToggleHidden => app.toggle_hidden(),
            InputEvent::ScrollUp => app.scroll_up(),
            InputEvent::ScrollDown => app.scroll_down(),
            InputEvent::None => {}
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Some(path) = app.selected_path() {
        println!("{}", path.display());
    }

    Ok(())
}

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}
