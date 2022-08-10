# easychangedirectory (unfinished)

> **cli tool to comfortably change directories.**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Rust](https://github.com/shsyss/easychangedirectory/actions/workflows/rust.yml/badge.svg)](https://github.com/shsyss/easychangedirectory/actions/workflows/rust.yml)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

[Features](#features) / [Support](#support) / [Usage](#usage) / [Installation](#installation) / [After this](#after-this)

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

I can't say for sure because I don't have it, but I think the program will probably work on any OS. However, I don't know anything about the outside of the program, such as installation.

## Installation

## After this

- Complete the above
- Image Preview
- Bug: Highlight shifted when moving left or right during search