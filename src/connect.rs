use std::{fs::File, io::Write, path::PathBuf};

pub fn pipe_shell(path: &PathBuf, temp_path: &str) -> anyhow::Result<()> {
  let mut f = File::create(temp_path)?;
  f.write_all(path.to_string_lossy().as_bytes())?;

  Ok(())
}
