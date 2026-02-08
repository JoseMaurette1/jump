use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, AppState};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let path_display = app.current_dir.display().to_string();

    // Hidden files status indicator
    let hidden_status = if app.show_hidden {
        "[HIDDEN: ON]"
    } else {
        "[HIDDEN: OFF]"
    };
    let hidden_style = if app.show_hidden {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let path_block = Paragraph::new(Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(
            &path_display,
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(hidden_status, hidden_style),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Enter=confirm  Backspace=up  Esc=cancel  Ctrl+H=toggle hidden "),
    );

    frame.render_widget(path_block, chunks[0]);

    let items: Vec<ListItem> = if app.entries.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "  (empty directory)",
            Style::default().fg(Color::DarkGray),
        )]))]
    } else {
        app.entries
            .iter()
            .zip(app.labels.iter())
            .map(|(entry, label)| {
                let label_style = match app.state {
                    AppState::PartialMatch if app.first_char == Some(label.chars[0]) => {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    }
                    AppState::Selecting => Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                    _ => Style::default().fg(Color::DarkGray),
                };

                let second_char_style = if app.state == AppState::PartialMatch
                    && app.first_char == Some(label.chars[0])
                {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    label_style
                };

                let line = Line::from(vec![
                    Span::styled(label.chars[0].to_ascii_lowercase().to_string(), label_style),
                    Span::styled(
                        label.chars[1].to_ascii_lowercase().to_string(),
                        second_char_style,
                    ),
                    Span::raw("  "),
                    Span::styled(&entry.name, Style::default().fg(Color::White)),
                    Span::styled("/", Style::default().fg(Color::DarkGray)),
                ]);

                ListItem::new(line)
            })
            .collect()
    };

    let title = match app.state {
        AppState::Selecting => " type label ",
        AppState::PartialMatch => " type second key ",
        _ => " jump ",
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_widget(list, chunks[1]);
}
