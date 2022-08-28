use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::Span,
  widgets::{Block, Borders, List},
  widgets::{ListItem, ListState},
  Frame,
};

use super::{App, Config, Item, ItemType, Kind, Mode};

struct MyStyle;

impl MyStyle {
  fn right_border<'a>() -> Block<'a> {
    Block::default().borders(Borders::RIGHT).border_style(Style::default().fg(Color::Gray))
  }
  fn highlight_style() -> Style {
    Style::default().fg(Color::Magenta)
  }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  // Overall style
  if app.config.is_set_bg() {
    f.render_widget(Block::default().style(Style::default().bg(Color::Rgb(10, 10, 10))), f.size());
  }

  // layout
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(10), Constraint::Max(100)])
    .split(f.size());

  // top----------------------------------------------------------
  let top_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(80), Constraint::Length(1)])
    .split(chunks[0]);

  // show pwd
  f.render_widget(
    Block::default().title(Span::styled(app.generate_pwd_str(), Style::default().fg(Color::Yellow))),
    top_chunks[0],
  );

  // search
  let item = Item { item: ItemType::SearchText(app.search.text.clone()), kind: Kind::Search, index: Some(0) };
  let search_items = vec![item];
  let search_items = set_items(&search_items, app.config);
  let search_text = List::new(search_items).highlight_symbol("> ");
  let mut state = ListState::default();
  if app.mode == Mode::Normal {
    state.select(None);
  } else if app.mode == Mode::Search {
    state.select(Some(0));
  }
  f.render_stateful_widget(search_text, top_chunks[1], &mut state);

  // bottom------------------------------------------------------
  let bottom_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(20),
      Constraint::Percentage(20),
      Constraint::Max(100),
      Constraint::Percentage(30),
    ])
    .split(chunks[1]);

  // grandparent
  let grandparent_items = set_items(&app.grandparent_items.items, app.config);
  let grandparent_items =
    List::new(grandparent_items).block(MyStyle::right_border()).highlight_style(MyStyle::highlight_style());
  f.render_stateful_widget(grandparent_items, bottom_chunks[0], &mut app.grandparent_items.state);

  // parent
  let parent_items = set_items(&app.parent_items.items, app.config);
  let parent_items = List::new(parent_items).block(MyStyle::right_border()).highlight_style(MyStyle::highlight_style());
  f.render_stateful_widget(parent_items, bottom_chunks[1], &mut app.parent_items.state);

  // current
  let (items, state) = match app.judge_mode() {
    Mode::Normal => (set_items(&app.items.items, app.config), &mut app.items.state),
    Mode::Search => (set_items(&app.search.list, app.config), &mut app.search.state),
  };
  let items = List::new(items)
    .block(MyStyle::right_border())
    .highlight_style(Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED))
    .highlight_symbol("> ");
  f.render_stateful_widget(items, bottom_chunks[2], state);

  // child
  let child_items = set_items(&app.child_items.items, app.config);
  let child_items = List::new(child_items).highlight_style(MyStyle::highlight_style());
  f.render_stateful_widget(child_items, bottom_chunks[3], &mut app.child_items.state);
}

fn set_items(items: &[Item], config: Config) -> Vec<ListItem> {
  items
    .iter()
    .filter_map(|item| {
      let style = match item.kind {
        Kind::Content | Kind::None | Kind::File => Style::default().fg(Color::Gray),
        Kind::Dir => Style::default().fg(Color::Blue),
        Kind::Search => Style::default().fg(Color::Green),
      };

      let mut text = if let ItemType::SearchText(text) = &item.item {
        text.clone()
      } else if let ItemType::Content(text) = &item.item {
        text.clone()
      } else {
        item.generate_filename()?
      };

      if config.is_show_index(items) {
        text = format!("{} {}", item.index.unwrap_or(0) + 1, text);
      }

      Some(ListItem::new(Span::styled(text, style)))
    })
    .collect()
}
