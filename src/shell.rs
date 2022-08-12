pub const BASH: &str = r#"\
# # easychangedirectory
# eval "$(easychangedirectory --init bash)"

function ed() \{
  if [[ "$#" -eq 0 ]]; then
    temp_path="{ temp_path }.$$"
    easychangedirectory "$\{temp_path}"
    path=`cat "$\{temp_path}"`
    cd "$\{path}"
  elif [[ "$#" -eq 1 ]] && [[ "$1" =~ ^-+ ]]; then
    easychangedirectory "$1"
  elif [[ "$#" -eq 1 ]]; then
    cd "$1"
  else
    :
  fi
}
"#;

pub const FISH: &str = r#"\

"#;

pub const POWERSHELL: &str = r#"\
"#;

pub const ZSH: &str = r#"\
# # easychangedirectory
# eval "$(easychangedirectory --init zsh)"

function ed() \{
  temp_path="{ temp_path }.$$"
  easychangedirectory "$\{temp_path}"
  path=`cat "$\{temp_path}"`
  cd "$\{path}"
}
"#;
