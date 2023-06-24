use easychangedirectory as ed;

use ed::error::PrintError;

fn main() {
  let cli = ed::cli();

  cli.match_options();

  let cd_path = match ed::app() {
    Ok(path) => path,
    Err(e) => {
      e.eprintln();
      return;
    }
  };

  cli.match_temp_path(&cd_path);

  if let Ok(config) = ed::Config::new() {
    if config.is_wd() {
      println!("Now: {}", cd_path.display());
    }
    if config.is_log() {
      println!("Log output location: {}", ed::log::LogOutput::path().display());
    }
  }
}
