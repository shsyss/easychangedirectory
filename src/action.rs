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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_action_execute() {
    let current = PathBuf::from(".");
    let cd_path = PathBuf::from("/tmp");
    let action = Action::Change(cd_path.clone());
    assert_eq!(action.execute(), cd_path);
    let action = Action::Keep;
    assert_eq!(action.execute(), current);
    let action = Action::Print(cd_path);
    assert_eq!(action.execute(), current);
  }
}
