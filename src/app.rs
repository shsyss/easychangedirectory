use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    env, io,
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

pub struct App {
    pub child_items: Vec<PathBuf>,
    pub items: StatefulList<PathBuf>,
    pub parent_items: Vec<PathBuf>,
    pub grandparent_items: Vec<PathBuf>,
    pwd: PathBuf,
    grandparent_path: PathBuf,
}

impl App {
    fn new() -> anyhow::Result<App> {
        let pwd = env::current_dir()?;
        let items = items::read_dir(&pwd)?;

        let child_path = if items[0].is_dir() {
            items[0].clone()
        } else {
            PathBuf::new()
        };
        let child_items = Self::get_items(child_path)?;

        let parent_path = Self::get_parent_path(&pwd);
        let parent_items = Self::get_items(&parent_path)?;

        let grandparent_path = Self::get_parent_path(parent_path);
        let grandparent_items = Self::get_items(&grandparent_path)?;

        Ok(App {
            child_items,
            items: StatefulList::with_items(items),
            parent_items,
            grandparent_items,
            pwd,
            grandparent_path,
        })
    }

    fn next(&mut self) -> anyhow::Result<()> {
        self.items.next();

        self.update_child_items()?;

        Ok(())
    }

    fn previous(&mut self) -> anyhow::Result<()> {
        self.items.previous();

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
        let grandparent_items = Self::get_items(&grandparent_path)?;

        *self = Self {
            child_items: self.items.items.clone(),
            items: StatefulList::with_items(self.parent_items.clone()),
            parent_items: self.grandparent_items.clone(),
            grandparent_items,
            pwd,
            grandparent_path,
        };

        Ok(())
    }

    fn move_child(&mut self) -> anyhow::Result<()> {
        let i = self.items.state.selected().unwrap();
        let selected_path = self.items.items[i].clone();
        let pwd = if selected_path.is_dir() {
            selected_path
        } else {
            return Ok(());
        };

        let child_items = Self::get_items(&self.child_items[0])?;

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

    fn get_parent_path<P: AsRef<Path>>(path: P) -> PathBuf {
        path.as_ref()
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf()
    }

    fn get_items<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
        Ok(if path.as_ref().to_string_lossy().is_empty() {
            vec![PathBuf::new()]
        } else {
            items::read_dir(path)?
        })
    }

    fn update_child_items(&mut self) -> anyhow::Result<()> {
        let i = self.items.state.selected().unwrap_or(0);

        let selected_path = self.items.items[i].clone();
        let child_path = if selected_path.is_dir() {
            selected_path
        } else {
            PathBuf::new()
        };
        let child_items = Self::get_items(child_path)?;

        self.child_items = child_items;

        Ok(())
    }
}

pub fn app() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new()?;
    self::run(&mut terminal, app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                // finish
                KeyCode::Backspace => return Ok(()),
                KeyCode::Esc => return Ok(()),
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(()),
                // TODO: change directory
                KeyCode::Enter => return Ok(()),
                // next
                KeyCode::Char('j') => app.next()?,
                KeyCode::Down => app.next()?,
                // previous
                KeyCode::Char('k') => app.previous()?,
                KeyCode::Up => app.previous()?,
                // parent
                KeyCode::Char('h') => app.move_parent()?,
                KeyCode::Left => app.move_parent()?,
                // TODO: right move
                KeyCode::Char('l') => app.move_child()?,
                KeyCode::Right => app.move_child()?,
                _ => {}
            }
        }
    }
}
