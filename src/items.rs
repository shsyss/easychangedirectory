use std::fs;
use std::path::{Path, PathBuf};

pub fn read_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<PathBuf>> {
    let items = if let Ok(read_dir) = fs::read_dir(&path) {
        read_dir
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let filepath = entry.path();
                Some(filepath)
            })
            .collect()
    } else {
        return Ok(vec![PathBuf::new()]);
    };

    Ok(items)
}
