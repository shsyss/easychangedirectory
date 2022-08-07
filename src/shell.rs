use std::{
  io::Write,
  path::Path,
  process::{Command, Stdio},
};

pub fn change_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
  Ok(())
}
