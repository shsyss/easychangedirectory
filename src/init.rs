use std::{collections::HashMap, env::temp_dir, path::PathBuf};

use serde::Serialize;
use tinytemplate::TinyTemplate;

use crate::shell;

#[derive(Serialize)]
struct Context {
  temp_path: PathBuf,
}

pub fn run(shell: &str) -> anyhow::Result<()> {
  let mut temp = TinyTemplate::new();
  let context = Context { temp_path: temp_dir().join("_easychangedirectory.txt") };

  let init_map = HashMap::from([
    ("bash", shell::BASH),
    ("fish", shell::FISH),
    ("powershell", shell::POWERSHELL),
    ("zsh", shell::ZSH),
  ]);

  temp.add_template("init", init_map.get(shell).unwrap_or(&"bash"))?;
  let shellscript = temp.render("init", &context)?;

  println!("{}", shellscript);

  Ok(())
}
