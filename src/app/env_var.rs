use serde::Deserialize;

use super::{Item, Kind};

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Config {
  pub _ed_show_index: Option<u8>,
  pub _ed_view_file_contents: Option<u8>,
  pub _ed_set_bg: Option<u8>,
}

impl Config {
  pub fn new() -> anyhow::Result<Self> {
    Ok(envy::from_env::<Self>()?)
  }

  pub fn is_show_index(&self, items: &[Item]) -> bool {
    self._ed_show_index.eq(&Some(1)) && !items.is_empty() && !items[0].kind.eq(&Kind::Search)
  }
  pub fn is_view_file_contents(&self) -> bool {
    self._ed_view_file_contents.eq(&Some(1))
  }
  pub fn is_set_bg(&self) -> bool {
    self._ed_set_bg.eq(&Some(1))
  }
}
