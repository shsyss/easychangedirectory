use std::{
  env,
  fs::{self, File},
  io::Write,
  path::Path,
  process::{Command, Stdio},
  time::Duration,
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

static SHELL_COMMAND_TEMPLATE: &str = r#"cd -P {path}"#;

pub fn change_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
  // let cmd_name = "bash";
  let mut temp = TinyTemplate::new();
  // temp.add_template(cmd_name, SHELL_COMMAND_TEMPLATE)?;

  let context = Context::new(&path);

  // let shell_cmd = temp.render(cmd_name, &context)?;
  // Command::new(cmd_name).args(&["-c", &shell_cmd]).stdout(Stdio::inherit()).output()?;

  let cmd_name = ".";
  temp.add_template(cmd_name, include_str!("../templates/bash.txt"))?;
  let shell_script = temp.render(cmd_name, &context)?;
  let filepath = "./templates/result/ed.bash";
  let mut f = File::create(filepath)?;
  f.write_all(shell_script.as_bytes())?;

  Command::new(cmd_name).args(&[filepath]).stdout(Stdio::inherit()).output()?;

  // Command::new(cmd_name).args(&["-c", &shell_script]).stdout(Stdio::inherit()).output()?;

  Ok(())
}
