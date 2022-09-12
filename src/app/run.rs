use std::{path::PathBuf, process::Command};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tui::{backend::Backend, Terminal};

use super::{App, Mode};

pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> anyhow::Result<PathBuf> {
  let current = PathBuf::from(".");
  loop {
    terminal.draw(|f| super::ui(f, &mut app))?;
    if let Event::Key(key) = event::read()? {
      match app.mode {
        Mode::Normal => {
          match key.code {
            // finish
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(current),
            KeyCode::Char('q') => return Ok(current),
            KeyCode::Esc => return Ok(current),

            // change directory
            KeyCode::Char('c') => return Ok(app.wd),
            KeyCode::Char(';') => return Ok(app.wd),
            KeyCode::Enter => return Ok(app.wd),

            // move
            KeyCode::Home => app.move_home()?,
            KeyCode::End => app.move_end()?,
            KeyCode::PageUp => app.move_page_up()?,
            KeyCode::PageDown => app.move_page_down()?,
            KeyCode::Char('j') => app.move_next()?,
            KeyCode::Down => app.move_next()?,
            KeyCode::Char('k') => app.move_previous()?,
            KeyCode::Up => app.move_previous()?,
            KeyCode::Char('h') => app.move_parent()?,
            KeyCode::Left => app.move_parent()?,
            KeyCode::Char('l') => app.move_child()?,
            KeyCode::Right => app.move_child()?,

            // search
            KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => app.mode = Mode::Search,
            KeyCode::Insert => app.mode = Mode::Search,
            KeyCode::Backspace => {
              app.search.text.pop();
              app.update_search_effect()?;
            }
            KeyCode::Delete => {
              app.search.text.clear();
              app.update_search_effect()?;
            }

            // Execute command
            KeyCode::Char('V') => {
              Command::new("code").arg(&app.wd).output()?;
            }
            // KeyCode::Char('L') => {
            //   Command::new("lapce").arg(&app.wd).output()?;
            // }
            _ => {}
          }
        }
        Mode::Search => {
          match key.code {
            // finish
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(current),
            KeyCode::Esc => return Ok(current),

            // change directory
            KeyCode::Enter => return Ok(app.wd),

            // search
            KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => app.mode = Mode::Normal,
            KeyCode::Insert => app.mode = Mode::Normal,

            // input
            KeyCode::Char(c) => {
              app.search.text.push(c);
              app.update_search_effect()?;
            }
            KeyCode::Backspace => {
              app.search.text.pop();
              app.update_search_effect()?;
            }
            KeyCode::Delete => {
              app.search.text.clear();
              app.update_search_effect()?;
            }

            // move
            KeyCode::Home => app.move_home()?,
            KeyCode::End => app.move_end()?,
            KeyCode::PageUp => app.move_page_up()?,
            KeyCode::PageDown => app.move_page_down()?,
            KeyCode::Down => app.move_next()?,
            KeyCode::Up => app.move_previous()?,
            KeyCode::Left => app.move_parent()?,
            KeyCode::Right => app.move_child()?,

            _ => {}
          }
        }
      }
    }
  }
}
