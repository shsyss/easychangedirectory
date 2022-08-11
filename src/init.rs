use std::{env::temp_dir, path::PathBuf};

use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
  temp_dir: PathBuf,
}

pub fn run(shell: &str) -> anyhow::Result<()> {
  let mut temp = TinyTemplate::new();
  let context = Context { temp_dir: temp_dir() };
  let shellscript = match shell {
    "bash" => {
      temp.add_template("init", include_str!("../templates/bash.txt"))?;
      println!("{}", temp.render("init", &context)?);
      temp.render("init", &context)?
    }
    _ => todo!(), // Shell::Fish => {}
                  // Shell::Powershell => {}
                  // Shell::Zsh => {}
  };

  println!("{}", shellscript);

  Ok(())
}
