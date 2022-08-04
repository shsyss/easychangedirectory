use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::ListItem,
    widgets::{Block, Borders, List},
    Frame,
};

use crate::{
    app::{Item, State},
    App,
};

struct Standard;

impl Standard {
    fn block<'a>() -> Block<'a> {
        Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(Color::Gray))
    }

    fn highlight_style() -> Style {
        Style::default().fg(Color::Green)
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Overall style
    f.render_widget(
        Block::default()
            .title(Span::styled(
                app.generate_pwd_str(),
                Style::default().fg(Color::Yellow),
            ))
            .style(Style::default().bg(Color::Rgb(0, 0, 40))),
        f.size(),
    );

    // layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Max(100),
            Constraint::Percentage(30),
        ])
        .margin(1)
        .split(f.size());

    // grandparent
    let grandparent_items = set_items(&app.grandparent_items.items);
    let grandparent_items = List::new(grandparent_items)
        .block(Standard::block())
        .highlight_style(Standard::highlight_style());
    f.render_stateful_widget(
        grandparent_items,
        chunks[0],
        &mut app.grandparent_items.state,
    );

    // parent
    let parent_items = set_items(&app.parent_items.items);
    let parent_items = List::new(parent_items)
        .block(Standard::block())
        .highlight_style(Standard::highlight_style());
    f.render_stateful_widget(parent_items, chunks[1], &mut app.parent_items.state);

    // current
    let items: Vec<ListItem> = set_items(&app.items.items);
    let items = List::new(items)
        .block(Standard::block())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(items, chunks[2], &mut app.items.state);

    // child
    let child_items = set_items(&app.child_items.items);
    let child_items = List::new(child_items).highlight_style(Standard::highlight_style());
    f.render_stateful_widget(child_items, chunks[3], &mut app.child_items.state);
}

fn set_items(items: &[Item]) -> Vec<ListItem> {
    items
        .iter()
        .filter_map(|item| {
            let filename = item.filename()?;
            let style = match item.state {
                State::Content | State::None | State::File => Style::default().fg(Color::Gray),
                State::Dir => Style::default().fg(Color::Blue),
            };
            Some(ListItem::new(Span::styled(filename, style)))
        })
        .collect()
}
