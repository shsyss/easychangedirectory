use tui::widgets::ListState;

use super::{Item, State};

#[derive(Debug)]
pub struct Search {
  pub text: String,
  pub list: Vec<Item>,
  pub state: ListState,
}

impl Search {
  pub fn new() -> Self {
    let mut state = ListState::default();
    state.select(Some(0));
    Search { text: String::new(), list: vec![], state }
  }
}

impl Default for Search {
  fn default() -> Self {
    Self::new()
  }
}

impl State for Search {
  fn next(&mut self) -> usize {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.list.len() - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
    i
  }
  fn previous(&mut self) -> usize {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.list.len() - 1
        } else {
          i - 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
    i
  }
}
