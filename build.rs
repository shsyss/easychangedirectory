use clap_complete::{
  generate_to,
  shells::{Bash, Fish, PowerShell, Zsh},
};
use std::env;
use std::io::Error;

include!("./src/cli.rs");
fn main() -> Result<(), Error> {
  let mut cmd = build_cli();
  let bin_name = "ed";
  let out_dir = "./completion";
  generate_to(Bash, &mut cmd, bin_name, out_dir)?;
  generate_to(Zsh, &mut cmd, bin_name, out_dir)?;
  generate_to(Fish, &mut cmd, bin_name, out_dir)?;
  generate_to(PowerShell, &mut cmd, bin_name, out_dir)?;

  Ok(())
}
