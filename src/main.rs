use std::process;

use easychangedirectory::{app, build_cli};

fn main() {
  build_cli().get_matches();

  let path = match app() {
    Ok(path) => path,
    Err(e) => {
      eprintln!("\x1b[31merror:\x1b[m  {}", e);
      process::exit(1);
    }
  };
}
