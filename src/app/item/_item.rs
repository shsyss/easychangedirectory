use std::{fs, path::PathBuf};

use anyhow::Context;

use super::App;

#[derive(Debug, Clone)]
pub enum ItemType {
  Path(PathBuf),
  Content(String),
  SearchText(String),
}

impl ItemType {
  fn new_path() -> Self {
    Self::Path(PathBuf::new())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
  File,
  Dir,
  Content,
  Search,
  None,
}

#[derive(Debug, Clone)]
pub struct Item {
  pub item: ItemType,
  pub kind: Kind,
  pub index: Option<usize>,
}

impl Item {
  pub fn default() -> Self {
    Self { item: ItemType::new_path(), kind: Kind::None, index: None }
  }
  pub fn generate_child_items(&self) -> anyhow::Result<Vec<Item>> {
    if self.is_symlink() {
      if let ItemType::Path(path) = &self.item {
        return App::make_items(path.read_link()?);
      }
    }
    Ok(if self.is_dir() {
      App::make_items(self.get_path().unwrap())?
    } else if self.is_file() && self.can_read() {
      if let Ok(s) = fs::read_to_string(self.get_path().context("Non-string files are being read.")?) {
        s.lines()
          .enumerate()
          .map(|(i, s)| Item { item: ItemType::Content(s.to_string()), kind: Kind::Content, index: Some(i) })
          .collect()
      } else {
        vec![Item::default()]
      }
    } else {
      vec![Item::default()]
    })
  }
  pub fn generate_filename(&self) -> Option<String> {
    Some(self.get_path()?.file_name()?.to_string_lossy().into())
  }
  pub fn can_read(&self) -> bool {
    if let ItemType::Path(path) = &self.item {
      path.is_file()
    } else {
      false
    }
  }
  pub fn is_dir(&self) -> bool {
    matches!(self.kind, Kind::Dir)
  }
  pub fn is_file(&self) -> bool {
    matches!(self.kind, Kind::File)
  }
  fn is_symlink(&self) -> bool {
    if let Some(p) = self.get_path() {
      p.is_symlink()
    } else {
      false
    }
  }
  pub fn get_path(&self) -> Option<PathBuf> {
    if let ItemType::Path(path) = &self.item {
      Some(path.into())
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_item() {
    let item = Item { item: ItemType::Path(PathBuf::from("test")), kind: Kind::Dir, index: None };
    assert_eq!(item.is_dir(), true);
    assert_eq!(item.is_file(), false);
    assert_eq!(item.is_symlink(), false);
    assert_eq!(item.can_read(), false);
    assert_eq!(item.get_path(), Some(PathBuf::from("test")));
    assert_eq!(item.generate_filename(), Some("test".to_string()));
  }
}
