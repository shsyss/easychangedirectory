use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::env;
use std::path::PathBuf;
use tui::backend::Backend;
use tui::Terminal;

use crate::ui::ui;

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> crossterm::Result<()> {
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
