use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io, path::PathBuf};
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
    pub items: StatefulList<PathBuf>,
    pub parent_items: Vec<PathBuf>,
    pub grandparent_items: Vec<PathBuf>,
}

impl App {
    fn new() -> anyhow::Result<App> {
        let pwd = env::current_dir()?;
        let items = items::read_dir(&pwd)?;
        let parent_items = items::read_dir(pwd.parent().unwrap())?;
        let grandparent_items = items::read_dir(pwd.parent().unwrap().parent().unwrap())?;
        Ok(App {
            items: StatefulList::with_items(items),
            parent_items,
            grandparent_items,
        })
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

fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> crossterm::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            // 終了key
            // TODO BackspaceとEscの時は開始前に戻して、ctrl+cとEnterは現在のディレクトリに移動
            // TODO 右に選択しているフォルダ直下のファイルのリスト
            // TODO →l ←h
            match key.code {
                // finish
                KeyCode::Backspace => return Ok(()),
                KeyCode::Esc => return Ok(()),
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(()),
                // TODO: change directory
                KeyCode::Enter => return Ok(()),
                // up and down move
                KeyCode::Char('k') => app.items.previous(),
                KeyCode::Char('j') => app.items.next(),
                KeyCode::Up => app.items.previous(),
                KeyCode::Down => app.items.next(),
                // TODO: left and right move
                KeyCode::Char('h') => {}
                KeyCode::Char('l') => {}
                KeyCode::Left => {}
                KeyCode::Right => {}
                _ => {}
            }
        }
    }
}
