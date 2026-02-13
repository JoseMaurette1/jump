use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub enum InputEvent {
    Char(char),
    Escape,
    Backspace,
    Enter,
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    GoToStart,
    GoToEnd,
    StartSearch,
    NavigateIn,
    NavigateOut,
    None,
}

pub fn read_key(timeout_ms: u64) -> Result<InputEvent> {
    if event::poll(Duration::from_millis(timeout_ms))? {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                return Ok(InputEvent::Escape);
            }

            // Ctrl+D for page down
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('d') {
                return Ok(InputEvent::PageDown);
            }

            // Ctrl+U for page up
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('u') {
                return Ok(InputEvent::PageUp);
            }

            match code {
                KeyCode::Esc => return Ok(InputEvent::Escape),
                KeyCode::Backspace => return Ok(InputEvent::Backspace),
                KeyCode::Enter => return Ok(InputEvent::Enter),
                KeyCode::Char('/') => return Ok(InputEvent::StartSearch),
                KeyCode::Char('g') => {
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        return Ok(InputEvent::GoToEnd);
                    }
                    return Ok(InputEvent::GoToStart);
                }
                KeyCode::Char('G') => return Ok(InputEvent::GoToEnd),
                KeyCode::Char('j') => return Ok(InputEvent::ScrollDown),
                KeyCode::Char('k') => return Ok(InputEvent::ScrollUp),
                KeyCode::Char('l') => return Ok(InputEvent::NavigateIn),
                KeyCode::Char('h') => return Ok(InputEvent::NavigateOut),
                KeyCode::Char(c) => return Ok(InputEvent::Char(c)),
                _ => {}
            }
        }
    }
    Ok(InputEvent::None)
}
