use anyhow::bail;
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
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(0));
        StatefulList { state, items }
    }
    fn with_items_select(items: Vec<T>, index: usize) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(index));
        StatefulList { state, items }
    }
    fn next(&mut self) {
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
        self.state.select(Some(i))
    }
    fn previous(&mut self) {
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
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    File,
    Dir,
    RelationalDir,
    Content,
    None,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub path: PathBuf,
    pub state: State,
}

impl Item {
    fn change_state(&mut self, state: State) -> Self {
        self.state = state;
        self.clone()
    }
    pub fn filename(&self) -> Option<String> {
        Some(self.path.file_name()?.to_string_lossy().to_string())
    }
    fn generate_child_items(&self) -> anyhow::Result<Vec<Item>> {
        Ok(if self.is_dir() {
            App::generate_items(&self.path)?
        } else if let Ok(s) = fs::read_to_string(&self.path) {
            s.lines()
                .map(|s| Item {
                    path: PathBuf::from(s),
                    state: State::Content,
                })
                .collect()
        } else {
            vec![Item::default()]
        })
    }
    pub fn is_dir(&self) -> bool {
        matches!(self.state, State::Dir | State::RelationalDir)
    }
    fn is_file(&self) -> bool {
        matches!(self.state, State::File)
    }
    pub fn default() -> Self {
        Self {
            path: PathBuf::new(),
            state: State::None,
        }
    }
}

// TODO: itemsは全部StatefulListにする
#[derive(Debug)]
pub struct App {
    pub child_items: StatefulList<Item>,
    pub items: StatefulList<Item>,
    pub parent_items: StatefulList<Item>,
    pub grandparent_items: StatefulList<Item>,
    pwd: PathBuf,
    grandparent_path: PathBuf,
}

impl App {
    fn generate_items<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Item>> {
        Ok(if path.as_ref().to_string_lossy().is_empty() {
            vec![Item::default()]
        } else {
            items::read_dir(path)?
        })
    }
    pub fn get_child_items(&self) -> Vec<Item> {
        self.child_items.items.clone()
    }
    pub fn get_grandparent_items(&self) -> Vec<Item> {
        self.grandparent_items.items.clone()
    }
    fn get_index<P: AsRef<Path>>(items: &[Item], path: P) -> usize {
        for (i, item) in items.iter().enumerate() {
            if item.path == path.as_ref() {
                return i;
            }
        }
        0
    }
    fn get_index_child(&self) -> usize {
        for (i, item) in self.get_child_items().iter().enumerate() {
            if item.state == State::RelationalDir {
                return i;
            }
        }
        0
    }
    fn get_index_parent(&self) -> usize {
        for (i, item) in self.get_parent_items().iter().enumerate() {
            if item.path == self.pwd {
                return i;
            }
        }
        0
    }
    pub fn get_items(&self) -> Vec<Item> {
        self.items.items.clone()
    }
    pub fn get_parent_items(&self) -> Vec<Item> {
        self.parent_items.items.clone()
    }
    fn get_parent_path<P: AsRef<Path>>(path: P) -> PathBuf {
        path.as_ref()
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf()
    }
    pub fn get_pwd_str(&self) -> String {
        self.pwd.to_string_lossy().to_string()
    }
    fn move_child(&mut self) -> anyhow::Result<()> {
        let i = self.items.state.selected().unwrap();
        let selected_item = self.items.items[i].clone();
        let pwd = if selected_item.is_dir() {
            self.items.items[i].change_state(State::RelationalDir);
            selected_item.path
        } else if selected_item.is_file() {
            self.move_content(selected_item)?;
            return Ok(());
        } else {
            return Ok(());
        };
        *self = Self {
            child_items: StatefulList::with_items(
                self.get_child_items()[0].generate_child_items()?,
            ),
            items: StatefulList::with_items_select(self.get_child_items(), self.get_index_child()),
            parent_items: StatefulList::with_items(self.get_items()),
            grandparent_items: StatefulList::with_items(self.get_parent_items()),
            pwd,
            grandparent_path: Self::get_parent_path(&self.pwd),
        };
        Ok(())
    }
    fn move_content(&mut self, selected_item: Item) -> anyhow::Result<()> {
        *self = Self {
            child_items: StatefulList::with_items(vec![Item::default()]),
            items: StatefulList::with_items(self.get_child_items()),
            parent_items: StatefulList::with_items(self.get_items()),
            grandparent_items: StatefulList::with_items(self.get_parent_items()),
            pwd: selected_item.path,
            grandparent_path: Self::get_parent_path(&self.pwd),
        };
        Ok(())
    }
    fn move_down(&mut self) -> anyhow::Result<()> {
        self.items.next();
        self.update_child_items()?;
        Ok(())
    }
    fn move_parent(&mut self) -> anyhow::Result<()> {
        let pwd = if let Some(pwd) = self.pwd.parent() {
            pwd.to_path_buf()
        } else {
            return Ok(());
        };

        let grandparent_path = Self::get_parent_path(&self.grandparent_path);
        let mut grandparent_items = Self::generate_items(&grandparent_path)?;
        let i = Self::get_index(&grandparent_items, &self.grandparent_path);
        grandparent_items[i].change_state(State::RelationalDir);

        *self = Self {
            child_items: StatefulList::with_items(self.get_items()),
            items: StatefulList::with_items_select(
                self.get_parent_items(),
                self.get_index_parent(),
            ),
            parent_items: StatefulList::with_items(self.get_grandparent_items()),
            grandparent_items: StatefulList::with_items(grandparent_items),
            pwd,
            grandparent_path,
        };

        Ok(())
    }
    fn move_up(&mut self) -> anyhow::Result<()> {
        self.items.previous();
        self.update_child_items()?;
        Ok(())
    }
    fn new() -> anyhow::Result<App> {
        let pwd = env::current_dir()?;
        let items = items::read_dir(&pwd)?;

        let child_path = if items[0].is_dir() {
            items[0].path.clone()
        } else {
            PathBuf::new()
        };
        let parent_path = Self::get_parent_path(&pwd);
        let grandparent_path = Self::get_parent_path(&parent_path);
        let mut parent_items = Self::generate_items(&parent_path)?;
        let mut grandparent_items = Self::generate_items(&grandparent_path)?;
        let (i, j) = (
            Self::get_index(&parent_items, &pwd),
            Self::get_index(&grandparent_items, &parent_path),
        );
        parent_items[i].change_state(State::RelationalDir);
        grandparent_items[j].change_state(State::RelationalDir);

        Ok(App {
            child_items: StatefulList::with_items(Self::generate_items(child_path)?),
            items: StatefulList::with_items(items),
            parent_items: StatefulList::with_items(parent_items),
            grandparent_items: StatefulList::with_items(grandparent_items),
            pwd,
            grandparent_path,
        })
    }
    fn update_child_items(&mut self) -> anyhow::Result<()> {
        let i = self.items.state.selected().unwrap_or(0);
        self.child_items = StatefulList::with_items(self.get_items()[i].generate_child_items()?);
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
        Err(e) => bail!(e),
    };

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
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
                KeyCode::Char('j') => app.move_down()?,
                KeyCode::Down => app.move_down()?,
                // previous
                KeyCode::Char('k') => app.move_up()?,
                KeyCode::Up => app.move_up()?,
                // parent
                KeyCode::Char('h') => app.move_parent()?,
                KeyCode::Left => app.move_parent()?,
                // right move
                KeyCode::Char('l') => app.move_child()?,
                KeyCode::Right => app.move_child()?,
                // TODO: home,end pageUp,pageDown
                _ => {}
            }
        }
    }
}
