use anyhow::{bail, Context};
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
  env, fs, io,
  path::{Path, PathBuf},
};
use tui::{
  backend::{Backend, CrosstermBackend},
  widgets::ListState,
  Terminal,
};

use crate::{items, ui};

#[derive(Debug)]
pub struct StatefulList<T> {
  pub state: ListState,
  pub items: Vec<T>,
}

impl<T> StatefulList<T> {
  fn next(&mut self) -> usize {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
    i
  }
  fn previous(&mut self) -> usize {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.items.len() - 1
        } else {
          i - 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
    i
  }
  fn select(&mut self, index: usize) {
    self.state.select(Some(index));
  }
  fn selected(&self) -> usize {
    self.state.selected().unwrap()
  }
  fn unselect(&mut self) {
    self.state.select(None);
  }
  fn with_items(items: Vec<T>) -> StatefulList<T> {
    let mut state = ListState::default();
    state.select(Some(0));
    StatefulList { state, items }
  }
  fn with_items_option(items: Vec<T>, index: Option<usize>) -> StatefulList<T> {
    let mut state = ListState::default();
    state.select(index);
    StatefulList { state, items }
  }
  fn with_items_select(items: Vec<T>, index: usize) -> StatefulList<T> {
    let mut state = ListState::default();
    state.select(Some(index));
    StatefulList { state, items }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
  File,
  Dir,
  Content,
  None,
}

// for identification
pub enum Family {
  Grandparent,
  Parent,
  Oneself,
  Child,
}

#[derive(Debug, Clone)]
pub enum TypeItem {
  Path(PathBuf),
  Content(String),
}

impl TypeItem {
  fn new_path() -> Self {
    Self::Path(PathBuf::new())
  }
  fn from_path(s: &str) -> Self {
    Self::Path(PathBuf::from(s))
  }
}

#[derive(Debug, Clone)]
pub struct Item {
  pub item: TypeItem,
  pub state: State,
}

impl Item {
  pub fn default() -> Self {
    Self { item: TypeItem::new_path(), state: State::None }
  }
  fn generate_child_items(&self) -> anyhow::Result<Vec<Item>> {
    if self.is_symlink() {
      if let TypeItem::Path(path) = &self.item {
        return App::make_items(path.read_link()?);
      }
    }
    Ok(if self.is_dir() {
      App::make_items(&self.get_path().unwrap())?
    } else if let Ok(s) = fs::read_to_string(&self.get_path().context("Non-string files are being read.")?) {
      s.lines().map(|s| Item { item: TypeItem::from_path(s), state: State::Content }).collect()
    } else {
      vec![Item::default()]
    })
  }
  pub fn generate_filename(&self) -> Option<String> {
    Some(self.get_path()?.file_name()?.to_string_lossy().to_string())
  }
  pub fn is_dir(&self) -> bool {
    matches!(self.state, State::Dir)
  }
  fn is_file(&self) -> bool {
    matches!(self.state, State::File)
  }
  fn is_symlink(&self) -> bool {
    self.get_path().unwrap().is_symlink()
  }
  pub fn get_path(&self) -> Option<PathBuf> {
    if let TypeItem::Path(path) = &self.item {
      Some(path.clone())
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct App {
  pub child_items: StatefulList<Item>,
  pub items: StatefulList<Item>,
  pub parent_items: StatefulList<Item>,
  pub grandparent_items: StatefulList<Item>,
  pwd: PathBuf,
  grandparent_path: PathBuf,
}

const JUMP: usize = 4;
impl App {
  fn generate_parent_path<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref().parent().unwrap_or_else(|| Path::new("")).to_path_buf()
  }
  pub fn generate_pwd_str(&self) -> String {
    self.pwd.to_string_lossy().to_string()
  }
  pub fn get_child_items(&self) -> Vec<Item> {
    self.child_items.items.clone()
  }
  pub fn get_grandparent_items(&self) -> Vec<Item> {
    self.grandparent_items.items.clone()
  }
  fn get_index(&self, family: Family) -> usize {
    let items = match family {
      Family::Grandparent => &self.grandparent_items,
      Family::Parent => &self.parent_items,
      Family::Oneself => &self.items,
      Family::Child => &self.child_items,
    };

    items.state.selected().unwrap_or(0)
  }
  pub fn get_items(&self) -> Vec<Item> {
    self.items.items.clone()
  }
  pub fn get_parent_items(&self) -> Vec<Item> {
    self.parent_items.items.clone()
  }
  /// If the working block is "content" `true`
  fn is_contents_in_working_block(&self) -> bool {
    let i = self.parent_items.selected();
    self.get_parent_items()[i].is_file()
  }
  fn generate_index<P: AsRef<Path>>(items: &[Item], path: P) -> usize {
    let generate_item = items.iter().enumerate().find(|(_, item)| item.get_path().unwrap() == path.as_ref());
    if let Some((i, _)) = generate_item {
      i
    } else {
      0
    }
  }
  fn make_items<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Item>> {
    Ok(if path.as_ref().to_string_lossy().is_empty() { vec![Item::default()] } else { items::read_dir(path)? })
  }
  fn move_child(&mut self) -> anyhow::Result<()> {
    let i = self.items.state.selected().unwrap();
    let selected_item = self.items.items[i].clone();
    let pwd = if selected_item.is_dir() {
      selected_item.get_path().unwrap()
    } else if selected_item.is_file() {
      self.move_content(selected_item)?;
      return Ok(());
    } else {
      return Ok(());
    };

    let old_child_items = self.get_child_items();
    if old_child_items.is_empty() {
      return Ok(());
    }

    let selected_ci = self.get_index(Family::Child);

    // The index of `items` is set to the index of `child_items` if it is selected. If not, it is set to `0`.
    let (child_items, i) = if let Some(items) = self.get_child_items().get(selected_ci) {
      (items.generate_child_items()?, self.get_index(Family::Child))
    } else {
      (self.get_child_items()[0].generate_child_items()?, 0)
    };

    let ci = None;
    let pi = self.get_index(Family::Oneself);
    let gi = self.get_index(Family::Parent);

    *self = Self {
      child_items: StatefulList::with_items_option(child_items, ci),
      items: StatefulList::with_items_select(old_child_items, i),
      parent_items: StatefulList::with_items_select(self.get_items(), pi),
      grandparent_items: StatefulList::with_items_select(self.get_parent_items(), gi),
      pwd,
      grandparent_path: Self::generate_parent_path(&self.pwd),
    };
    Ok(())
  }
  fn move_content(&mut self, selected_item: Item) -> anyhow::Result<()> {
    let pi = self.get_index(Family::Oneself);
    let gi = self.get_index(Family::Parent);

    *self = Self {
      child_items: StatefulList::with_items(vec![Item::default()]),
      items: StatefulList::with_items(self.get_child_items()),
      parent_items: StatefulList::with_items_select(self.get_items(), pi),
      grandparent_items: StatefulList::with_items_select(self.get_parent_items(), gi),
      pwd: selected_item.get_path().unwrap(),
      grandparent_path: Self::generate_parent_path(&self.pwd),
    };
    Ok(())
  }
  fn move_end(&mut self) {
    self.items.select(self.items.items.len() - 1);
  }
  fn move_home(&mut self) {
    self.items.select(0);
  }
  fn move_next(&mut self) -> anyhow::Result<()> {
    let i = self.items.next();
    self.update_child_items(i)?;
    Ok(())
  }
  fn move_page_down(&mut self) {
    let last = self.items.items.len() - 1;
    let i = self.get_index(Family::Oneself);
    self.items.select(if i > last - JUMP { last } else { i + JUMP });
  }
  fn move_page_up(&mut self) {
    let i = self.get_index(Family::Oneself);
    self.items.select(if i < JUMP { 0 } else { i - JUMP });
  }
  fn move_parent(&mut self) -> anyhow::Result<()> {
    let pwd = if let Some(pwd) = self.pwd.parent() {
      pwd.to_path_buf()
    } else {
      return Ok(());
    };

    let grandparent_path = Self::generate_parent_path(&self.grandparent_path);
    let grandparent_items = Self::make_items(&grandparent_path)?;

    let ci = if self.is_contents_in_working_block() { None } else { Some(self.get_index(Family::Oneself)) };
    let i = self.get_index(Family::Parent);
    let pi = self.get_index(Family::Grandparent);
    let gi = Self::generate_index(&grandparent_items, &self.grandparent_path);

    *self = Self {
      child_items: StatefulList::with_items_option(self.get_items(), ci),
      items: StatefulList::with_items_select(self.get_parent_items(), i),
      parent_items: StatefulList::with_items_select(self.get_grandparent_items(), pi),
      grandparent_items: StatefulList::with_items_select(grandparent_items, gi),
      pwd,
      grandparent_path,
    };

    Ok(())
  }
  fn move_previous(&mut self) -> anyhow::Result<()> {
    let i = self.items.previous();
    self.update_child_items(i)?;
    Ok(())
  }
  fn new() -> anyhow::Result<App> {
    let pwd = env::current_dir()?;
    let items = items::read_dir(&pwd)?;

    // Initial selection is 0
    let child_path = if items[0].is_dir() { items[0].get_path().unwrap() } else { PathBuf::new() };
    let parent_path = Self::generate_parent_path(&pwd);
    let grandparent_path = Self::generate_parent_path(&parent_path);
    let parent_items = Self::make_items(&parent_path)?;
    let grandparent_items = Self::make_items(&grandparent_path)?;
    let pi = Self::generate_index(&parent_items, &pwd);
    let gi = Self::generate_index(&grandparent_items, &parent_path);

    let mut app = App {
      child_items: StatefulList::with_items_option(Self::make_items(child_path)?, None),
      items: StatefulList::with_items(items),
      parent_items: StatefulList::with_items(parent_items),
      grandparent_items: StatefulList::with_items(grandparent_items),
      pwd,
      grandparent_path,
    };

    app.parent_items.select(pi);
    app.grandparent_items.select(gi);

    Ok(app)
  }
  fn update_child_items(&mut self, index: usize) -> anyhow::Result<()> {
    let ci = self.child_items.state.selected();

    self.child_items = StatefulList::with_items_option(self.get_items()[index].generate_child_items()?, ci);
    if self.get_items()[index].is_file() {
      self.child_items.unselect();
    }

    Ok(())
  }
}

pub fn app() -> anyhow::Result<PathBuf> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let app = App::new()?;
  let path = match self::run(&mut terminal, app) {
    Ok(path) => path,
    Err(e) => {
      // restore terminal
      disable_raw_mode()?;
      execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
      terminal.show_cursor()?;

      bail!(e)
    }
  };

  // restore terminal
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  Ok(path)
}

fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> anyhow::Result<PathBuf> {
  let current = env::current_dir()?;
  loop {
    terminal.draw(|f| ui(f, &mut app))?;
    if let Event::Key(key) = event::read()? {
      match key.code {
        // finish
        KeyCode::Backspace => return Ok(current),
        KeyCode::Esc => return Ok(current),
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(current),
        // TODO: change directory
        KeyCode::Enter => return Ok(app.pwd),
        // next
        KeyCode::Char('j') => app.move_next()?,
        KeyCode::Down => app.move_next()?,
        // previous
        KeyCode::Char('k') => app.move_previous()?,
        KeyCode::Up => app.move_previous()?,
        // parent
        KeyCode::Char('h') => app.move_parent()?,
        KeyCode::Left => app.move_parent()?,
        // right move
        KeyCode::Char('l') => app.move_child()?,
        KeyCode::Right => app.move_child()?,
        // home end pageUp pageDown
        KeyCode::Home => app.move_home(),
        KeyCode::End => app.move_end(),
        KeyCode::PageUp => app.move_page_up(),
        KeyCode::PageDown => app.move_page_down(),
        _ => {}
      }
    }
  }
}
