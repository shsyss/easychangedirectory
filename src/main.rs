use easychangedirectory::{app, app_info};

fn main() {
    app_info();

    match app() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("\x1b[31merror:\x1b[m  {}", e);
        }
    };
}
