use std::{path::Path, process::Command};

pub fn change_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
  let cmd = "bash";
  let path = path.as_ref().to_string_lossy().to_string();

  Command::new(cmd).args(&[format!("source ~/.bashrc && _easychangedirectory {}", path)]).output()?;

  // let cmd = "_easychangedirectory";
  // let path = path.as_ref().to_string_lossy().to_string();

  // Command::new(cmd).arg(&path).output()?;

  Ok(())
}
