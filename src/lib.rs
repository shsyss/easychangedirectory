//! This crate is a command line tool, not a library

mod app;
mod cli;
mod connect;
mod env;
pub mod error;
mod init;
pub mod log;
mod shell;

pub use app::{app, Action};
pub use cli::cli;
pub use connect::pipe_shell;
pub use env::Config;
pub use init::init;
