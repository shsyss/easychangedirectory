use std::{
  io::Write,
  path::Path,
  process::{Command, Stdio},
};

pub fn change_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
  let child = Command::new("ls").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
  dbg!(child.stdout.unwrap());

  Ok(())
}
