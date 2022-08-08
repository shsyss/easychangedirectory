use std::{
  any, env,
  io::{self, Write},
  path::Path,
  process::{Command, Stdio},
  time::Duration,
};

use anyhow::anyhow;
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

// static SHELL_COMMAND_TEMPLATE: &str = "cd {path}";
static SHELL_COMMAND_TEMPLATE: &str = r#"echo {path}"#;

pub fn change_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
  let cmd_name = "bash";
  let mut temp = TinyTemplate::new();
  temp.add_template(cmd_name, SHELL_COMMAND_TEMPLATE)?;

  let context = Context::new(&path);

  let shell_cmd = temp.render(cmd_name, &context)?;
  Command::new(cmd_name).args(&["--noprofile", "--norc", "-c", &shell_cmd]).stdout(Stdio::inherit()).output()?;

  std::thread::sleep(Duration::new(1, 0));

  dbg!(env::current_dir()?);

  // writeln!(io::stdout(), "cd {}", path).pipe;

  Ok(())
}
