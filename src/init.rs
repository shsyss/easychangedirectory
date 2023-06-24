use std::env::temp_dir;

use handlebars::Handlebars;
use serde_json::json;

use crate::shell::Shell;

pub fn init(shell: &Shell) -> anyhow::Result<()> {
  let shellscript = Handlebars::new()
    .render_template(shell.get_template(), &json!({ "temp_path": temp_dir().join("_easychangedirectory.txt") }))?;

  println!("{}", shellscript);

  Ok(())
}
