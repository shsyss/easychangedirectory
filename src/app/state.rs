use tui::widgets::ListState;

use super::Item;

pub trait State {
  fn next(&mut self) -> usize;
  fn previous(&mut self) -> usize;
  fn select(&mut self, index: usize);
}

#[derive(Debug)]
pub struct StatefulList {
  pub state: ListState,
  pub items: Vec<Item>,
}

impl StatefulList {
  pub fn selected(&self) -> usize {
    self.state.selected().unwrap()
  }
  pub fn unselect(&mut self) {
    self.state.select(None);
  }
  pub fn get_selected_item(&self) -> Item {
    self.items[self.selected()].clone()
  }
  pub fn with_items(items: Vec<Item>) -> StatefulList {
    let mut state = ListState::default();
    state.select(Some(0));
    StatefulList { state, items }
  }
  pub fn with_items_option(items: Vec<Item>, index: Option<usize>) -> StatefulList {
    let mut state = ListState::default();
    state.select(index);
    StatefulList { state, items }
  }
  pub fn with_items_select(items: Vec<Item>, index: usize) -> StatefulList {
    let mut state = ListState::default();
    state.select(Some(index));
    StatefulList { state, items }
  }
}

impl State for StatefulList {
  fn next(&mut self) -> usize {
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
    self.state.select(Some(i));
    i
  }
  fn previous(&mut self) -> usize {
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
    i
  }
  fn select(&mut self, index: usize) {
    self.state.select(Some(index));
  }
}
