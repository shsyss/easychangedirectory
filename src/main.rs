use easychangedirectory::{app, build_cli};

fn main() -> anyhow::Result<()> {
  build_cli().get_matches();

  let path = match app() {
    Ok(path) => path,
    Err(e) => {
      eprintln!("\x1b[31mError:\x1b[m  {}", e);
      return Ok(());
    }
  };

  Ok(())
}
