pub const BASH: &str = r#"\
# # easychangedirectory
# eval "$(easychangedirectory --init bash)"

function ed() \{
  temp_path="{ temp_path }.$$"
  easychangedirectory "$\{temp_path}"
  path=`cat "$\{temp_path}"`
  cd "$\{path}"
}
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
