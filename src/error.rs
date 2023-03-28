pub trait PrintError {
  fn eprintln(&self);
}

impl PrintError for anyhow::Error {
  fn eprintln(&self) {
    eprintln!("\x1b[31mError:\x1b[m {}", self);
  }
}
