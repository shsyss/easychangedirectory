pub const BASH: &str = r#"
# # easychangedirectory
# eval "$(easychangedirectory --init bash)"

function ed() {
  if [[ "$#" -eq 0 ]]; then
    temp_path="{{ temp_path }}.$$"
    easychangedirectory "${temp_path}"
    path=`cat "${temp_path}"`
    cd "${path}" || return
  elif [[ "$#" -eq 1 ]] && [[ "$1" = '-' ]]; then
    cd "$1" || return
  elif [[ "$#" -eq 1 ]] && [[ "$1" =~ ^-+ ]]; then
    easychangedirectory "$1"
  elif [[ "$#" -eq 1 ]]; then
    cd "$1" || return
  else
    echo 'Too many arguments'
  fi
}
"#;

// TODO: I can't do regular expressions in Fish.
pub const FISH: &str = r#"
# # easychangedirectory
# easychangedirectory --init fish | source

function ed
  set arg_cnt (count $argv)
  if test "$arg_cnt" -eq 0
    set temp_path "{{ temp_path }}.$fish_pid"
    easychangedirectory "$temp_path"
    set path (cat "$temp_path")
    cd "$path"
  else if test "$arg_cnt" -eq 1 -a \( "x$argv[1]" = 'x-h' -o "x$argv[1]" = 'x--help' -o "x$argv[1]" = 'x-V' -o "x$argv[1]" = 'x--version' \)
    easychangedirectory "$argv[1]"
  else if test "$arg_cnt" -eq 1
    cd "$argv[1]"
  else
    echo 'Too many arguments'
  end
end
"#;

pub const POWERSHELL: &str = r#"
# # easychangedirectory
# Invoke-Expression (& { (easychangedirectory --init powershell | Out-String) } )

function ed {
  if ($args.Length -eq 0) {
    $temp_path = "{{ temp_path }}.$pid"
    easychangedirectory $temp_path
    $path = (cat $temp_path)
    cd $path
  } elseif ($args.Length -eq 1 -and $args[0] -eq '-') {
    cd $args[0]
  } elseif ($args.Length -eq 1 -and $args[0][0] -eq '-') {
    easychangedirectory $args[0]
  } elseif ($args.Length -eq 1) {
    cd $args[0]
  } else {
    echo 'Too many arguments'
  }
}
"#;

pub const ZSH: &str = r#"
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
    echo 'Too many arguments'
  fi
}
"#;
