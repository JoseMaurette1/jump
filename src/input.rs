use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub enum InputEvent {
    Char(char),
    Escape,
    Backspace,
    Enter,
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

            match code {
                KeyCode::Esc => return Ok(InputEvent::Escape),
                KeyCode::Backspace => return Ok(InputEvent::Backspace),
                KeyCode::Enter => return Ok(InputEvent::Enter),
                KeyCode::Char(c) => return Ok(InputEvent::Char(c)),
                _ => {}
            }
        }
    }
    Ok(InputEvent::None)
}
