use easychangedirectory as ed;

fn main() -> anyhow::Result<()> {
  let matches = ed::build_cli().get_matches();

  if let Some(shell) = matches.get_one::<String>("init") {
    ed::init(shell)?;
    return Ok(());
  } else if matches.contains_id("env") {
    ed::Config::new()?.show_all();
    return Ok(());
  }

  let cd_path = match ed::app() {
    Ok(path) => path,
    Err(e) => {
      eprintln!("\x1b[31mError:\x1b[m {}", e);
      return Ok(());
    }
  };

  if let Some(temp_path) = matches.get_one::<String>("temp_path") {
    if let Err(e) = ed::connect::pipe_shell(cd_path, temp_path) {
      eprintln!("\x1b[31mError:\x1b[m  {}", e);
    }
  }

  Ok(())
}
