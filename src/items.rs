use std::fs;
use std::path::Path;

use crate::app::{Item, State, TypeItem};

pub fn read_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Item>> {
  let mut items = if let Ok(read_dir) = fs::read_dir(&path) {
    read_dir
      .filter_map(|entry| {
        let entry = entry.ok()?;
        let filepath = entry.path();
        let state = if filepath.is_dir() { State::Dir } else { State::File };
        Some(Item { item: TypeItem::Path(filepath), state })
      })
      .collect::<Vec<Item>>()
  } else {
    return Ok(vec![Item::default()]);
  };

  items.sort_by_key(|a| a.get_path());

  Ok(items)
}
