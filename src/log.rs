use std::{fs::File, path::PathBuf};

use ::log::info;
use crossterm::event::KeyEvent;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

use crate::app::App;

pub struct LogOutput;

impl LogOutput {
  pub fn path() -> PathBuf {
    home::home_dir().unwrap().join(format!(".{}", env!("CARGO_PKG_NAME"))).join("ed.log")
  }
}

pub fn init() {
  let path = LogOutput::path();
  let parent = path.parent().unwrap();
  if !parent.exists() {
    std::fs::create_dir_all(parent).unwrap();
  }
  CombinedLogger::init(vec![WriteLogger::new(
    LevelFilter::Info,
    Config::default(),
    File::create(LogOutput::path()).unwrap(),
  )])
  .unwrap();
}

pub fn write(app: &App, key: &KeyEvent) {
  info!("--------------------------------");
  info!("path: {:?}", app.wd);
  info!("selected: {:?}", app.items.state.selected());
  info!("key: {:?}", key.code);
  info!("mode: {:?}", app.mode);
  info!("search: {:?}", app.search.text);
}
