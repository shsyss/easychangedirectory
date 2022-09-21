# easychangedirectory

> **Tools for easy cd**

[![Latest version](https://img.shields.io/crates/v/easychangedirectory)](https://crates.io/crates/easychangedirectory)
[![crates.io downloads](https://img.shields.io/crates/d/easychangedirectory?label=downloads&style=flat-square)](https://crates.io/crates/easychangedirectory)
[![Github All Releases](https://img.shields.io/github/downloads/shsyss/easychangedirectory/total.svg)](https://github.com/shsyss/easychangedirectory/releases)

[![Apache](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Build Status](https://github.com/shsyss/easychangedirectory/actions/workflows/rust.yml/badge.svg)](https://github.com/shsyss/easychangedirectory/actions/workflows/rust.yml)
[![release](https://github.com/shsyss/easychangedirectory/actions/workflows/release.yml/badge.svg)](https://github.com/shsyss/easychangedirectory/actions/workflows/release.yml)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

[Features](#features) / [Usage](#usage) / [Support shell](#support-shell) / [Installation](#installation) / [Environment variable](#environment-variable) / [After this](#after-this)

## Features

- Can change paths visually
- The `cd` functionality can also be used as-is

![demo](./assets/demo.gif)

## Usage

Command `ed`

| Key                | Description                                 |
| ------------------ | ------------------------------------------- |
| `↑` `k`            | Move up                                     |
| `↓` `j`            | Move down                                   |
| `←` `h`            | Move parent directory                       |
| `→` `l`            | Move Child directory                        |
| `Home`             | Move to top                                 |
| `End`              | Move to bottom                              |
| `PageUp`           | Skip a little and move up                   |
| `PageDown`         | Skip a little and move down                 |
| `Enter` `c` `;`    | Change directory to current directory       |
| `Esc` `Ctrl+c` `q` | Exit and return to original directory       |
| `Insert` `Ctrl+s`  | Search mode switch (Char key will not work) |
| `Backspace`        | Delete one character from the search string |
| `Delete`           | Delete all search strings                   |
| `V`                | Open VSCode in the current directory        |

<!-- | `L`                | Open Lapce in the current directory         | -->

Please let us know if you have any key map requests. If it is traditional, we will add it immediately.

## Support shell

- Bash
- Fish
- Powershell
- Zsh

## Installation

[Install](#install) and [Register in the shell](#register-easychangedirectory-in-shell)

### Install

- If you can use Cargo

```
cargo install --locked easychangedirectory
```

- Download from Release page

  - Download the data appropriate for your environment and place it in a directory with a path

We will add more as needed.

### Register **_easychangedirectory_** in shell

<details>
<summary>Bash</summary>

Add to `~/.bashrc` (Change as necessary)

```bash
eval "$(easychangedirectory --init bash)"
```

Run `. ~/.bashrc` as needed

</details>

<details>
<summary>Fish</summary>

Add to `~/.config/fish/config.fish` (Change as necessary)

```fish
easychangedirectory --init fish | source
```

Run `. ~/.config/fish/config.fish` as needed

</details>

<details>
<summary>Powershell</summary>

Add to the file found by `echo $profile`

```powershell
Invoke-Expression (& { (easychangedirectory --init powershell | Out-String) } )
```

Run `. $profile` as needed

</details>

<details>
<summary>Zsh</summary>

Add to `~/.zshrc` (Change as necessary)

```zsh
eval "$(easychangedirectory --init zsh)"
```

Run `. ~/.zshrc` as needed

</details>

## Environment variable

You can check all environment variable values with `ed --env`.

- `_ED_PWD`
  - If the value is `1`, print current directory after execution
- `_ED_SET_BG`
  - If the value is `1`, set black background
- `_ED_SHOW_INDEX`
  - If the value is `1`, the index is displayed on the left side of the list
- `_ED_VIEW_FILE_CONTENTS`
  - If the value is `1`, the file contents can be viewed
  - Preview is possible without setting

## After this

- Add Support
- Add tests