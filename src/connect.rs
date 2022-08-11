use std::{env::temp_dir, fs::File, io::Write, path::PathBuf};

pub fn pipe_shell(path: PathBuf) -> anyhow::Result<()> {
  let temp_filepath = temp_dir().join("_easychangedirectory.txt");
  let mut f = File::create(temp_filepath)?;
  f.write_all(path.to_string_lossy().as_bytes())?;

  Ok(())
}
