use easychangedirectory as ed;

use ed::error::PrintError;

fn main() {
  let matches = ed::build_cli().get_matches();

  if let Some(shell) = matches.get_one::<String>("init") {
    if let Err(e) = ed::init(shell) {
      e.eprintln();
    }
    return;
  } else if matches.contains_id("env") {
    match ed::Config::new() {
      Ok(c) => c.show_all(),
      Err(e) => e.eprintln(),
    };
    return;
  }

  let cd_path = match ed::app() {
    Ok(path) => path,
    Err(e) => {
      e.eprintln();
      return;
    }
  };

  if let Some(temp_path) = matches.get_one::<String>("temp_path") {
    if let Err(e) = ed::pipe_shell(&cd_path, temp_path) {
      e.eprintln();
      return;
    }
  }

  if let Ok(config) = ed::Config::new() {
    if config.is_wd() {
      println!("{}", cd_path.display());
    }
  }
}
