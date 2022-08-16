use std::{env, path::PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tui::{backend::Backend, Terminal};

use super::{App, Mode};

pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> anyhow::Result<PathBuf> {
  let current = env::current_dir()?;
  loop {
    terminal.draw(|f| super::ui(f, &mut app))?;
    if let Event::Key(key) = event::read()? {
      match app.mode {
        Mode::Normal => {
          match key.code {
            // finish
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(current),
            KeyCode::Esc => return Ok(current),
            KeyCode::Backspace => return Ok(current),

            // change directory
            KeyCode::Enter => return Ok(app.pwd),

            // home
            KeyCode::Home => app.move_home()?,
            // end
            KeyCode::End => app.move_end()?,
            // pageUp
            KeyCode::PageUp => app.move_page_up()?,
            // pageDown
            KeyCode::PageDown => app.move_page_down()?,
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

            KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => app.mode = Mode::Search,

            _ => {}
          }
        }
        Mode::Search => {
          match key.code {
            // finish
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(current),
            KeyCode::Esc => return Ok(current),

            // change directory
            KeyCode::Enter => return Ok(app.pwd),

            KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => app.mode = Mode::Normal,

            // input
            KeyCode::Char(c) => {
              app.search.text.push(c);
              app.update_search_list();
            }
            KeyCode::Backspace => {
              app.search.text.pop();
              app.update_search_list();
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
