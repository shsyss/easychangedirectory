use std::fs;
use std::path::Path;

use crate::app::{Item, ItemType, Kind};

pub fn read_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Item>> {
  let mut items = if let Ok(read_dir) = fs::read_dir(&path) {
    read_dir
      .filter_map(|entry| {
        let entry = entry.ok()?;
        let filepath = entry.path();
        let kind = if filepath.is_dir() { Kind::Dir } else { Kind::File };
        Some(Item { item: ItemType::Path(filepath), kind, index: 0 })
      })
      .collect::<Vec<Item>>()
  } else {
    return Ok(vec![Item::default()]);
  };

  items.sort_by_key(|item| item.get_path());
  Ok(
    items
      .iter_mut()
      .enumerate()
      .map(|(i, item)| {
        item.index = i;
        item.clone()
      })
      .collect(),
  )
}
