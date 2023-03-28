use std::fs::File;

use ::log::info;
use crossterm::event::KeyEvent;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

use crate::app::App;

pub fn init() {
  CombinedLogger::init(vec![WriteLogger::new(LevelFilter::Info, Config::default(), File::create("ed.log").unwrap())])
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
