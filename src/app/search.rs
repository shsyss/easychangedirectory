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
  fn select(&mut self, index: usize) {
    self.state.select(Some(index));
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::app::{ItemType, Kind};

  impl Item {
    fn new_in_search_tests(s: &str) -> Self {
      Self { item: ItemType::Content(s.to_string()), kind: Kind::Content, index: None }
    }
  }

  #[test]
  fn test_next() {
    let mut search = Search::new();
    search.list = vec![Item::new_in_search_tests("a"), Item::new_in_search_tests("b"), Item::new_in_search_tests("c")];
    assert_eq!(search.next(), 1);
    assert_eq!(search.next(), 2);
    assert_eq!(search.next(), 0);
  }

  #[test]
  fn test_previous() {
    let mut search = Search::new();
    search.list = vec![Item::new_in_search_tests("a"), Item::new_in_search_tests("b"), Item::new_in_search_tests("c")];
    assert_eq!(search.previous(), 2);
    assert_eq!(search.previous(), 1);
    assert_eq!(search.previous(), 0);
  }

  #[test]
  fn test_select() {
    let mut search = Search::new();
    search.list = vec![Item::new_in_search_tests("a"), Item::new_in_search_tests("b"), Item::new_in_search_tests("c")];
    search.select(1);
    assert_eq!(search.state.selected(), Some(1));
  }
}
