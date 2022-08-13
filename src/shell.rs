pub const BASH: &str = r#"\
# # easychangedirectory
# eval "$(easychangedirectory --init bash)"

function ed() {
  if [[ "$#" -eq 0 ]]; then
    temp_path="{{ temp_path }}.$$"
    easychangedirectory "${temp_path}"
    path=`cat "${temp_path}"`
    cd "${path}" || return
  elif [[ "$#" -eq 1 ]] && [[ "$1" = - ]]; then
    cd "$1" || return
  elif [[ "$#" -eq 1 ]] && [[ "$1" =~ ^-+ ]]; then
    easychangedirectory "$1"
  elif [[ "$#" -eq 1 ]]; then
    cd "$1" || return
  else
    :
  fi
}
"#;

pub const FISH: &str = r#"\
# # easychangedirectory
# easychangedirectory --init fish | source

function ed
  set arg_cnt (count $argv)
  if test $arg_cnt -eq 0
    set temp_path "{{ temp_path }}.$fish_pid"
    easychangedirectory "$temp_path"
    set path (cat "$temp_path")
    cd "$path"
  else if test $arg_cnt -eq 1 -a \( "x$argv[1]" = 'x-h' -o "x$argv[1]" = 'x--help' -o "x$argv[1]" = 'x-V' -o "x$argv[1]" = 'x--version' \)
    easychangedirectory "$argv[1]"
  else
    cd "$argv[1]"
  end
end
"#;

pub const POWERSHELL: &str = r#"\
# # easychangedirectory
# Invoke-Expression (& { (easychangedirectory --init powershell) -join "`n" } )

function ed() {

}
"#;

pub const ZSH: &str = r#"\
# # easychangedirectory
# eval "$(easychangedirectory --init zsh)"

function ed() {
  if [[ "$#" -eq 0 ]]; then
    temp_path="{{ temp_path }}.$$"
    easychangedirectory "${temp_path}"
    path=`cat "${temp_path}"`
    cd "${path}" || return
  elif [[ "$#" -eq 1 ]] && [[ "$1" = - ]]; then
    cd "$1" || return
  elif [[ "$#" -eq 1 ]] && [[ "$1" =~ ^-+ ]]; then
    easychangedirectory "$1"
  elif [[ "$#" -eq 1 ]]; then
    cd "$1" || return
  else
    :
  fi
}
"#;
