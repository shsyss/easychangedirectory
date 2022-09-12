use std::{fs::File, io::Write, path::Path};

pub fn pipe_shell(path: &Path, temp_path: &str) -> anyhow::Result<()> {
  let mut f = File::create(temp_path)?;
  f.write_all(path.to_string_lossy().as_bytes())?;

  Ok(())
}
