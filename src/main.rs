use std::process;

use easychangedirectory::{app, app_info};

fn main() {
    app_info();

    let path = match app() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("\x1b[31merror:\x1b[m  {}", e);
            process::exit(1);
        }
    };
}
