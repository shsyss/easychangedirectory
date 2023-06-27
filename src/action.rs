use std::path::PathBuf;

pub enum Action {
  Change(PathBuf),
  Keep,
  Print(PathBuf),
}

impl Action {
  pub fn execute(&self) -> PathBuf {
    let current = PathBuf::from(".");
    match self {
      Action::Change(cd_path) => cd_path.into(),
      Action::Keep => current,
      Action::Print(print_path) => {
        println!("{}", print_path.display());
        current
      }
    }
  }
}
