use std::{
  env,
  io::{self, Write},
  path::Path,
  process::{Command, Stdio},
};

use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
  path: String,
}

impl Context {
  fn new<P: AsRef<Path>>(path: P) -> Self {
    Context { path: path.as_ref().to_string_lossy().to_string() }
  }
}

// static SHELL_COMMAND_TEMPLATE: &str = "cd -- {path}";

pub fn change_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
  // let mut temp = TinyTemplate::new();
  // temp.add_template("bash", SHELL_COMMAND_TEMPLATE)?;

  // let context = Context::new(&path);

  // let shell_command = temp.render("bash", &context)?;
  let path = path.as_ref().to_string_lossy().to_string();

  // writeln!(io::stdout(), "cd {}", path).pipe;

  Ok(())
}
