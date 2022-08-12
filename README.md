# easychangedirectory (unfinished)

> **Tools for easy cd**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Rust](https://github.com/shsyss/easychangedirectory/actions/workflows/rust.yml/badge.svg)](https://github.com/shsyss/easychangedirectory/actions/workflows/rust.yml)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

[Features](#features) / [Usage](#usage) / [Support](#support) / [Installation](#installation) / [After this](#after-this)

## Features

- Can change paths visually

![demo](./assets/demo.gif)

## Usage

| Key                          | Description                           |
| ---------------------------- | ------------------------------------- |
| `k` `↑`                      | Move up                               |
| `j` `↓`                      | Move down                             |
| `h` `←`                      | Move parent directory                 |
| `l` `→`                      | Move Child directory                  |
| `Home`                       | Move to top                           |
| `End`                        | Move to bottom                        |
| `PageUp` `Alt + k`           | Skip a little and move up             |
| `PageDown` `Alt + j`         | Skip a little and move down           |
| `Enter`                      | Change directory to current directory |
| `Backspace` `Esc` `Ctrl + c` | Exit and return to original directory |

Please let us know if you have any key map requests. If it is traditional, we will add it immediately.

## Support

| Shell          | Windows       | Linux (Ubuntu) |
| ---------------|:-------------:|:--------------:|
| **Bash**       | -             | **&#128504;**  |
| **Fish**       | -             | **&#128504;**  |
| **Powershell** | -             | -              |
| **Zsh**        | -             | -              |

## Installation

### Install
not yet

#### Register ***easychangedirectory*** in shell

<details>
<summary>Bash</summary>

Add to `~/.bashrc`. (Change as necessary)
```
eval "$(easychangedirectory --init bash)"
```
Run `. ~/.bashrc` as needed
</details>

<details>
<summary>Fish</summary>

Add to `~/.config/fish/config.fish`. (Change as necessary)
```
easychangedirectory --init fish | source
```
Run `. ~/.config/fish/config.fish` as needed
</details>

<details>
<summary>Zsh unimplemented</summary>

Add to `~/.zshrc`. (Change as necessary)
```
eval "$(easychangedirectory --init zsh)"
```
Run `. ~/.zshrc` as needed
</details>

## After this

- Complete the above
- To be able to do the same with cd
- Image Preview
- Zsh: Error `__vsc_command_output_start:3` is displayed at the second and subsequent `ed` executions.
- Bug: Highlight shifted when moving left or right during search
- Bug: Search suggestions are displayed from the index prior to the search
- Bug: Skip move does not work properly
- Bug: If the file content has a path, only the file name is displayed
