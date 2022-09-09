pub const BASH: &str = r#"
# # easychangedirectory
# eval "$(easychangedirectory --init bash)"

function ed() {
  if [[ "$#" -eq 0 ]]; then
    temp_path="{{ temp_path }}.$$"
    easychangedirectory "${temp_path}"
    cd_path=`cat "${temp_path}"`
    cd "${cd_path}" || return
  elif [[ "$#" -eq 1 ]] && [[ "$1" =~ ^-+[a-zA-Z]+ ]]; then
    easychangedirectory "$1"
  elif [[ "$#" -eq 1 ]]; then
    cd "$1" || return
  else
    echo 'Too many arguments'
  fi
}
"#;

pub const FISH: &str = r#"
# # easychangedirectory
# easychangedirectory --init fish | source

function ed
  set arg_cnt (count $argv)
  if test "$arg_cnt" -eq 0
    set temp_path "{{ temp_path }}.$fish_pid"
    easychangedirectory "$temp_path"
    set cd_path (cat "$temp_path")
    cd "$cd_path"
  else if string match -r '^x\-+[a-zA-Z]+' "x$argv[1]" &> /dev/null
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
    $cd_path = (cat $temp_path)
    cd $cd_path
  } elseif ($args.Length -eq 1 -and $args[0] -match '^-+[a-zA-Z]+') {
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
    cd_path=`cat ${temp_path}`
    cd "${cd_path}" || return
  elif [[ "$#" -eq 1 ]] && [[ "$1" =~ ^-+[a-zA-Z]+ ]]; then
    easychangedirectory "$1"
  elif [[ "$#" -eq 1 ]]; then
    cd "$1" || return
  else
    echo 'Too many arguments'
  fi
}
"#;
