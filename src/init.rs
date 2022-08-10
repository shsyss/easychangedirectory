pub fn run(shell: &str) -> anyhow::Result<()> {
  let shellscript = match shell {
    "bash" => {
      include_str!("../templates/bash.txt")
    }
    _ => todo!(), // Shell::Fish => {}
                  // Shell::Powershell => {}
                  // Shell::Zsh => {}
  };

  println!("{}", shellscript);

  Ok(())
}
