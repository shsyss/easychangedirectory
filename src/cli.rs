use clap::{arg, command, Command};

pub fn build_cli() -> Command<'static> {
  command!().arg(arg!(--init <SHELL> "Configure shell").required(false)).override_usage("ed[EXE]")
}
