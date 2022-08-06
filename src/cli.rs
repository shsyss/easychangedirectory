use clap::{command, Command};

pub fn build_cli() -> Command<'static> {
  command!().bin_name("ed")
}
