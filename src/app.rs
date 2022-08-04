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

#[derive(Clone)]
pub enum State {
    File,
    Dir,
    RelationDir,
    Content,
    None,
}

#[derive(Clone)]
pub struct Item {
    pub path: PathBuf,
    pub state: State,
}

impl Item {
    pub fn is_dir(&self) -> bool {
        matches!(self.state, State::Dir | State::RelationDir)
    }
    pub fn filename(&self) -> Option<String> {
        Some(self.path.file_name()?.to_string_lossy().to_string())
    }
}

pub struct App {
    pub child_items: Vec<Item>,
    pub items: StatefulList<Item>,
    pub parent_items: Vec<Item>,
    pub grandparent_items: Vec<Item>,
    pwd: PathBuf,
    grandparent_path: PathBuf,
}

impl App {
    fn create_items<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Item>> {
        Ok(if path.as_ref().to_string_lossy().is_empty() {
            vec![Item {
                path: PathBuf::new(),
                state: State::None,
            }]
        } else {
            items::read_dir(path)?
        })
    }

    fn get_parent_path<P: AsRef<Path>>(path: P) -> PathBuf {
        path.as_ref()
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf()
    }

    fn get_pwd_index(&self) -> usize {
        for (i, item) in self.parent_items.iter().enumerate() {
            if item.path == self.pwd {
                return i;
            }
        }

        0
    }

    pub fn get_pwd_str(&self) -> String {
        self.pwd.to_string_lossy().to_string()
    }

    fn move_child(&mut self) -> anyhow::Result<()> {
        // TODO: 直近のpwdを選択
        let i = self.items.state.selected().unwrap();
        let selected_item = self.items.items[i].clone();
        let pwd = if selected_item.is_dir() {
            selected_item.path
        } else {
            return Ok(());
        };

        let child_items = if self.child_items[0].is_dir() {
            Self::create_items(&self.child_items[0].path)?
        } else if let Ok(s) = fs::read_to_string(&self.child_items[0].path) {
            s.lines()
                .map(|s| Item {
                    path: PathBuf::from(s),
                    state: State::Content,
                })
                .collect()
        } else {
            vec![Item {
                path: PathBuf::new(),
                state: State::None,
            }]
        };

        *self = Self {
            child_items,
            items: StatefulList::with_items(self.child_items.clone()),
            parent_items: self.items.items.clone(),
            grandparent_items: self.parent_items.clone(),
            pwd,
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
        let grandparent_items = Self::create_items(&grandparent_path)?;

        *self = Self {
            child_items: self.items.items.clone(),
            items: StatefulList::with_items_select(self.parent_items.clone(), self.get_pwd_index()),
            parent_items: self.grandparent_items.clone(),
            grandparent_items,
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
        let child_items = Self::create_items(child_path)?;

        let parent_path = Self::get_parent_path(&pwd);
        let parent_items = Self::create_items(&parent_path)?;

        let grandparent_path = Self::get_parent_path(parent_path);
        let grandparent_items = Self::create_items(&grandparent_path)?;

        Ok(App {
            child_items,
            items: StatefulList::with_items(items),
            parent_items,
            grandparent_items,
            pwd,
            grandparent_path,
        })
    }

    fn update_child_items(&mut self) -> anyhow::Result<()> {
        let i = self.items.state.selected().unwrap_or(0);

        let selected_item = &self.items.items[i];
        let child_items = if selected_item.is_dir() {
            Self::create_items(&selected_item.path)?
        } else if let Ok(s) = fs::read_to_string(&selected_item.path) {
            s.lines()
                .map(|s| Item {
                    path: PathBuf::from(s),
                    state: State::Content,
                })
                .collect()
        } else {
            vec![Item {
                path: PathBuf::new(),
                state: State::None,
            }]
        };

        self.child_items = child_items;

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
                _ => {}
            }
        }
    }
}
