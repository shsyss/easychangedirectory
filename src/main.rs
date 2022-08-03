use easychangedirectory::app;

fn main() {
    match app() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("\x1b[31merror:\x1b[m  {}", e);
        }
    };
}
