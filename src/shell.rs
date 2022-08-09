use std::{path::Path, process::Command};

pub fn change_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
  let cmd_name = "_easychangedirectory";
  let path = path.as_ref().to_string_lossy().to_string();

  Command::new(cmd_name).arg(path).output()?;

  Ok(())
}
