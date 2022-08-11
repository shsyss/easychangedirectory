use easychangedirectory::{app, build_cli, connect, init};

fn main() -> anyhow::Result<()> {
  let matches = build_cli().get_matches();

  if let Some(shell) = matches.get_one::<String>("init") {
    init::run(shell)?;
    return Ok(());
  }

  let cd_path = match app() {
    Ok(path) => path,
    Err(e) => {
      eprintln!("\x1b[31mError:\x1b[m  {}", e);
      return Ok(());
    }
  };

  if let Some(temp_path) = matches.get_one::<String>("temp_path") {
    if let Err(e) = connect::pipe_shell(cd_path, temp_path) {
      eprintln!("\x1b[31mError:\x1b[m  {}", e);
    }
  }

  Ok(())
}
