use std::path::Path;

use crate::{
    app::{App, InputMode},
    document::Row,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

fn list_items<'a>(app: &'a App, index: usize, row: &'a Row) -> ListItem<'a> {
    let mut content: Vec<Span> = vec![];
    let file = Span::styled(
        {
            if row.file_name.len() > 20 {
                format!("...{}", &row.file_name[row.file_name.len() - 20..])
            } else {
                row.file_name.to_string()
            }
        },
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

pub fn render_search_page<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
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
        InputMode::Editing | InputMode::OpenFile => (
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
            InputMode::Normal | InputMode::OpenFile => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Query"));
    f.render_widget(input, chunks[1]);
    if let InputMode::Editing = app.input_mode {
        f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1);
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| list_items(app, i, m))
        .collect();

    let result_title = format!(
        " Result: {} from {} files ",
        app.messages.len(),
        app.doc.paths.len()
    );
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title(result_title));
    f.render_widget(messages, chunks[2]);
}

pub fn render_open_page<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let block = Block::default().style(Style::default());
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);
    let row = app.messages.get(app.index).unwrap();

    let mut text = vec![];
    let mut num = 0;
    for line in &app.file_contents {
        num += 1;
        text.push(Spans::from(vec![
            Span::styled(
                {
                    if num < 10 {
                        format!("{}   ", num)
                    } else if num < 100 {
                        format!("{}  ", num)
                    } else {
                        format!("{} ", num)
                    }
                },
                Style::default().add_modifier(Modifier::DIM),
            ),
            Span::styled(line, {
                if num == row.line {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                }
            }),
        ]));
    }

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let offset = (
        {
            if app.scroll.y >= chunks[0].y {
                app.scroll.y - chunks[0].y
            } else {
                app.scroll.y
            }
        },
        app.scroll.x + 1,
    );
    let name = Path::new(&row.file_name).file_name().unwrap();
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default())
        .block(create_block(format!(" {} ", name.to_string_lossy())))
        .wrap(Wrap { trim: false })
        .scroll(offset);
    f.render_widget(paragraph, chunks[0]);
}

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.input_mode {
        InputMode::Editing | InputMode::Normal => render_search_page(f, app),
        InputMode::OpenFile => render_open_page(f, app),
    }
}
