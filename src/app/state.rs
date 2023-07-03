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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::app::{ItemType, Kind};

  impl Item {
    fn new_in_state_tests(s: &str) -> Self {
      Self { item: ItemType::Content(s.to_string()), kind: Kind::Content, index: None }
    }
  }

  #[test]
  fn test_with_items_option() {
    let state =
      StatefulList::with_items_option(vec![Item::new_in_state_tests("a"), Item::new_in_state_tests("b")], Some(1));
    assert_eq!(state.selected(), 1);
  }

  #[test]
  fn test_with_items_select() {
    let state = StatefulList::with_items_select(vec![Item::new_in_state_tests("a"), Item::new_in_state_tests("b")], 1);
    assert_eq!(state.selected(), 1);
  }

  #[test]
  fn test_next() {
    let mut state = StatefulList::with_items(vec![Item::new_in_state_tests("a"), Item::new_in_state_tests("b")]);
    assert_eq!(state.next(), 1);
    assert_eq!(state.next(), 0);
    state.unselect();
    assert_eq!(state.next(), 0);
  }

  #[test]
  fn test_previous() {
    let mut state = StatefulList::with_items(vec![Item::new_in_state_tests("a"), Item::new_in_state_tests("b")]);
    assert_eq!(state.previous(), 1);
    assert_eq!(state.previous(), 0);
    state.unselect();
    assert_eq!(state.previous(), 0);
  }

  #[test]
  fn test_select() {
    let mut state = StatefulList::with_items(vec![Item::new_in_state_tests("a"), Item::new_in_state_tests("b")]);
    state.select(1);
    assert_eq!(state.selected(), 1);
  }
}
