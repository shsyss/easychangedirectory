use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::frame::ui;

pub fn app() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    self::run(&mut terminal)?;

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

fn run<B: Backend>(terminal: &mut Terminal<B>) -> crossterm::Result<()> {
    let current_dir = env::current_dir()?;
    let mut pwd = env::current_dir()?;
    loop {
        terminal.draw(|f| ui(f, &pwd, &current_dir))?;
        if let Event::Key(key) = event::read()? {
            // 終了key
            // TODO BackspaceとEscの時は開始前に戻して、ctrl+cとEnterは現在のディレクトリに移動
            // TODO 中央に現在のディレクトリのファイルのリスト
            // TODO 左に一つ上ののディレクトリのファイルのリスト
            // TODO 右に選択しているフォルダ直下のファイルのリスト
            // TODO ↑kw →ld ↓js ←ha
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
