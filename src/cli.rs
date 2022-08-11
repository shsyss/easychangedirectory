use clap::{arg, command, Arg, Command};

pub fn build_cli() -> Command<'static> {
  command!()
    .arg(Arg::new("temp_path").hide(true))
    .arg(arg!(--init <SHELL> "Configure shell").required(false))
    .override_usage("ed[EXE]")
}
