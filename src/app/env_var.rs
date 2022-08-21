use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Config {
  pub _ed_show_index: Option<usize>,
  pub _ed_view_file_contents: Option<usize>,
}

impl Config {
  pub fn new() -> anyhow::Result<Self> {
    Ok(envy::from_env::<Self>()?)
  }
}
