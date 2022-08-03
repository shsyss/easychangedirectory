use std::path::PathBuf;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::ListItem,
    widgets::{Block, Borders, List},
    Frame,
};

use crate::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Overall style
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(0, 0, 40))),
        f.size(),
    );

    // layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Max(100),
            Constraint::Percentage(20),
        ])
        .split(f.size());

    // grandparent
    let grandparent_items = set_items(&app.grandparent_items);
    let grandparent_items = List::new(grandparent_items).block(
        Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(Color::Gray)),
    );
    f.render_widget(grandparent_items, chunks[0]);

    // parent
    let parent_items = set_items(&app.parent_items);
    let parent_items = List::new(parent_items).block(
        Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(Color::Gray)),
    );
    f.render_widget(parent_items, chunks[1]);

    // current
    let items: Vec<ListItem> = set_items(&app.items.items);
    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::RIGHT)
                .border_style(Style::default().fg(Color::Gray)),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(items, chunks[2], &mut app.items.state);

    // child
    // let child_items = set_items(&app.parent_items);
    // let child_items = List::new(child_items).block(
    //     Block::default()
    //         .borders(Borders::RIGHT)
    //         .border_style(Style::default().fg(Color::Gray)),
    // );
    // f.render_widget(child_items, chunks[3]);
}

fn set_items(items: &[PathBuf]) -> Vec<ListItem> {
    items
        .iter()
        .filter_map(|p| {
            let filename = p.file_name()?.to_string_lossy().to_string();
            let lines = if p.is_dir() {
                vec![Spans::from(Span::styled(
                    filename,
                    Style::default().fg(Color::Blue),
                ))]
            } else {
                vec![Spans::from(Span::styled(
                    filename,
                    Style::default().fg(Color::Gray),
                ))]
            };
            Some(ListItem::new(lines))
        })
        .collect()
}
