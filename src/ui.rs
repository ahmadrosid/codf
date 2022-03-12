use crate::{
    app::{App, InputMode},
    document::Row,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

fn list_items<'a>(app: &'a App, index: usize, row: &'a Row) -> ListItem<'a> {
    let mut content: Vec<Span> = vec![];
    let file = Span::styled(
        format!("{}", row.file_name),
        Style::default().add_modifier(Modifier::DIM),
    );
    content.push(file);
    content.push(Span::raw(format!(":{}: ", row.line)));
    if app.input.is_empty() {
        content.push(Span::raw(&row.raw));
    } else {
        let split = row.raw.split(&app.input).collect::<Vec<_>>();
        let len = split.len() - 1;
        for (index, text) in split.iter().enumerate() {
            content.push(Span::raw(*text));
            if index < len {
                content.push(Span::styled(
                    &app.input,
                    Style::default()
                        .bg(Color::Green)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                ));
            }
        }
    }
    if index == app.index {
        return ListItem::new(Spans::from(content))
            .style(Style::default().add_modifier(Modifier::REVERSED));
    }
    ListItem::new(Spans::from(content))
}

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw(" Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start searching."),
            ],
            Style::default().add_modifier(Modifier::DIM),
        ),
        InputMode::Editing => (
            vec![
                Span::raw(" Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop searching."),
            ],
            Style::default().add_modifier(Modifier::DIM),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Query"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1);
        }
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| list_items(app, i, m))
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Result"));
    f.render_widget(messages, chunks[2]);
}
