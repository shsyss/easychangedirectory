use clap::{arg, command, Arg, Command};

pub fn build_cli() -> Command {
  command!()
    .arg(Arg::new("temp_path").hide(true).required(false))
    .arg(arg!(--init <SHELL> "Configure shell").required(false))
    .arg(arg!(--env "Show all environment variables").required(false))
    .about("Tools for easy cd\nThe `cd` functionality can also be used as-is")
    .override_usage(
      "ed\n
 -------------------------------------------------------------
|   Key         |    Description                              |
|---------------|---------------------------------------------|
| ↑ k           | Move up                                     |
| ↓ j           | Move down                                   |
| ← h           | Move parent directory                       |
| → l           | Move Child directory                        |
| Home	        | Move to top                                 |
| End           | Move to bottom                              |
| PageUp        | Skip a little and move up                   |
| PageDown      | Skip a little and move down                 |
| Enter c ;     | Change directory to current directory       |
| Esc Ctrl+c q	| Exit and return to original directory       |
| Insert Ctrl+s	| Search mode switch (Char key will not work) |
| Backspace     | Delete one character from the search string |
| Delete        | Delete all search strings                   |
| V             | Open vscode                                 |
 -------------------------------------------------------------",
    )
}
