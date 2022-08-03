use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io, path::PathBuf};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, List, Row, Table, Widget},
    widgets::{ListItem, ListState},
    Frame, Terminal,
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }
}

pub struct App {
    items: StatefulList<PathBuf>,
}

impl App {
    fn new() -> App {
        App {
            items: StatefulList::with_items(vec![
                env::current_dir().unwrap(),
                env::current_dir().unwrap(),
                env::current_dir().unwrap(),
                env::current_dir().unwrap(),
            ]),
        }
    }
}

pub fn app() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
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
            // TODO 中央に現在のディレクトリのファイルのリスト
            // TODO 左に一つ上ののディレクトリのファイルのリスト
            // TODO 右に選択しているフォルダ直下のファイルのリスト
            // TODO ↑k →l ↓j ←h
            if (key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c'))
                || key.code == KeyCode::Enter
                || key.code == KeyCode::Esc
                || key.code == KeyCode::Backspace
            {
                return Ok(());
            }
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC);

    let items = app
        .items
        .items
        .iter()
        .map(|p| {
            let mut lines = vec![Spans::from(p.to_string_lossy().to_string())];
            ListItem::new(lines)
        })
        .collect::<Vec<_>>();

    let items = List::new(items).block(
        Block::default()
            .title("easychangedirectory")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .border_type(BorderType::Rounded),
    );

    f.render_stateful_widget(items, size, &mut app.items.state);
}
