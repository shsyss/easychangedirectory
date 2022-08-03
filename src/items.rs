use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    Ok(fs::read_dir(&path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let filepath = entry.path();
            Some(filepath)
        })
        .collect())
}
