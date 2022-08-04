use std::fs;
use std::path::{Path, PathBuf};

use crate::app::{Item, State};

pub fn read_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Item>> {
    let items = if let Ok(read_dir) = fs::read_dir(&path) {
        read_dir
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let filepath = entry.path();
                let state = if filepath.is_dir() {
                    State::Dir
                } else {
                    State::File
                };
                Some(Item {
                    path: filepath,
                    state,
                })
            })
            .collect()
    } else {
        return Ok(vec![Item {
            path: PathBuf::new(),
            state: State::None,
        }]);
    };

    Ok(items)
}
