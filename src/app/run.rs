use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::Terminal;

use crate::ui::ui;

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> crossterm::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;
        if let Event::Key(key) = event::read()? {
            // 終了key
            if (key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c'))
                || key.code == KeyCode::Enter
                || key.code == KeyCode::Esc
            {
                return Ok(());
            }
        }
    }
}
