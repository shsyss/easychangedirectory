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
| **Bash**       | **&#128504;** | **&#128504;**  |
| **Fish**       | -             | **&#128504;**  |
| **Powershell** | **&#128504;** | **&#128504;**  |
| **Zsh**        | -             | **&#128504;**  |

## Installation

### Install


#### Register ***easychangedirectory*** in shell

<details>
<summary>Bash</summary>
Add to `~/.bashrc`

```
eval "$(easychangedirectory --init bash)"
```
Run `. ~/.bashrc` as needed
</details>

## After this

- Complete the above
- Image Preview
- Bug: Highlight shifted when moving left or right during search
- Bug: If the file content has a path, only the file name is displayed
