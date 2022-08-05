use clap_complete::shells::Bash;
use clap_complete::{generate_to, shells::PowerShell};
use std::env;
use std::io::Error;

include!("./src/cli.rs");
fn main() -> Result<(), Error> {
  let mut cmd = build_cli();
  let bin_name = "ed";
  let out_dir = "./completion";
  generate_to(PowerShell, &mut cmd, bin_name, out_dir)?;
  generate_to(Bash, &mut cmd, bin_name, out_dir)?;

  Ok(())
}
