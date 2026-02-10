use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::database::DirEntry as DbDirEntry;
use crate::fs::DirEntry as FsDirEntry;
use crate::number::NumberMode;

/// Draw the number selection mode TUI
pub fn draw_number(frame: &mut Frame, entries: &[FsDirEntry], number_mode: &NumberMode) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Number input display
            Constraint::Length(3), // Info bar
            Constraint::Min(0),    // Directory list
        ])
        .split(frame.area());

    // Draw number input display
    let num_display = format!(" [ {} ] ", number_mode.display_string());
    let num_style = if number_mode.is_complete {
        Style::default().fg(Color::Green)
    } else if number_mode.current_number == 0 {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let input_block = Paragraph::new(num_display).style(num_style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Enter number (0-9, then Enter)  Esc=cancel  j/k=scroll "),
    );

    frame.render_widget(input_block, chunks[0]);

    // Draw info bar
    let info_text = if entries.is_empty() {
        Line::from(vec![Span::styled(
            " (no directories found) ",
            Style::default().fg(Color::DarkGray),
        )])
    } else {
        let info_str = format!(" {} directories available ", entries.len());
        Line::from(vec![Span::styled(
            info_str,
            Style::default().fg(Color::Blue),
        )])
    };

    let info_block = Paragraph::new(info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Press number to jump, Enter to confirm "),
    );

    frame.render_widget(info_block, chunks[1]);

    // Draw directory list with numbers
    let items: Vec<ListItem> = if entries.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "  (empty directory)",
            Style::default().fg(Color::DarkGray),
        )]))]
    } else {
        entries
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let display_num = idx + 1;
                let is_selected = Some(idx) == number_mode.selected_index();

                let num_str = display_num.to_string();
                let num_style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                };

                let line = Line::from(vec![
                    Span::styled(num_str.clone(), num_style),
                    Span::raw("  "),
                    Span::styled(entry.name.clone(), Style::default().fg(Color::White)),
                    Span::styled("/", Style::default().fg(Color::DarkGray)),
                ]);

                if is_selected {
                    ListItem::new(line).style(
                        Style::default()
                            .add_modifier(Modifier::REVERSED)
                            .bg(Color::Blue),
                    )
                } else {
                    ListItem::new(line)
                }
            })
            .collect()
    };

    let title = format!(" directories ({}) ", entries.len());

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .bg(Color::Blue),
        );

    frame.render_widget(list, chunks[2]);
}

/// Draw bookmark selection mode TUI
pub fn draw_bookmarks(frame: &mut Frame, bookmarks: &[DbDirEntry], selected_idx: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    // Draw header
    let header_block = Paragraph::new(Line::from(vec![Span::styled(
        " Bookmarks ",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    )]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Press 1-9 to jump, j/k=scroll, Esc=cancel "),
    );

    frame.render_widget(header_block, chunks[0]);

    // Draw bookmarks list
    let items: Vec<ListItem> = if bookmarks.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "  (no bookmarks set)",
            Style::default().fg(Color::DarkGray),
        )]))]
    } else {
        bookmarks
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let is_selected = idx == selected_idx;
                let key = entry
                    .bookmark_key
                    .as_ref()
                    .map(|s: &String| s.as_str())
                    .unwrap_or("?");

                let key_style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                };

                let key_span = format!("[{}]", key);
                let path_span = format!(" ({})", &entry.path);
                let line = Line::from(vec![
                    Span::styled(key_span, key_style),
                    Span::raw("  "),
                    Span::styled(entry.name.clone(), Style::default().fg(Color::White)),
                    Span::styled(path_span, Style::default().fg(Color::DarkGray)),
                ]);

                if is_selected {
                    ListItem::new(line).style(
                        Style::default()
                            .add_modifier(Modifier::REVERSED)
                            .bg(Color::Magenta),
                    )
                } else {
                    ListItem::new(line)
                }
            })
            .collect()
    };

    let title = format!(" bookmarks ({}) ", bookmarks.len());

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .bg(Color::Magenta),
        );

    frame.render_widget(list, chunks[1]);
}
