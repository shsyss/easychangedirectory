use std::{collections::HashMap, env::temp_dir};

use handlebars::Handlebars;
use serde_json::json;

use crate::shell;

pub fn run(shell: &str) -> anyhow::Result<()> {
  let init_map = HashMap::from([
    ("bash", shell::BASH),
    ("fish", shell::FISH),
    ("powershell", shell::POWERSHELL),
    ("zsh", shell::ZSH),
  ]);

  let shellscript = Handlebars::new().render_template(
    init_map.get(shell).unwrap(),
    &json!({ "temp_path": temp_dir().join("_easychangedirectory.txt") }),
  )?;

  println!("{}", shellscript);

  Ok(())
}
