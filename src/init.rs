use std::{env::temp_dir, path::PathBuf};

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
  let shellscript = match shell {
    "bash" => {
      temp.add_template("init", shell::BASH)?;
      temp.render("init", &context)?
    }
    "zsh" => {
      temp.add_template("init", shell::ZSH)?;
      temp.render("init", &context)?
    }
    _ => todo!(), // Shell::Fish => {}
                  // Shell::Powershell => {}
                  // Shell::Zsh => {}
  };

  println!("{}", shellscript);

  Ok(())
}
