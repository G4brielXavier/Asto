# ASTO

![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)
![Crates.io](https://img.shields.io/crates/v/asto?color=orange)
![GitHub Repo stars](https://img.shields.io/github/stars/G4brielXavier/kio?style=social)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/G4brielXavier/Asto)
![GitHub all releases](https://img.shields.io/github/downloads/G4brielXavier/HovaForge/total)
![Rust](https://img.shields.io/badge/Rust-3.11%2B-blue)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-lightgrey)
![Issues](https://img.shields.io/badge/Issues-welcome-brightgreen)

*A domain-specific language for organizing and documenting CLI commands - built in Rust.*

![ASTO](./assets/asto_banner.png)

## Summary

- [ASTO](#asto)
  - [Summary](#summary)
  - [What is **Asto**?](#what-is-asto)
  - [Why **Asto**?](#why-asto)
  - [Who is **Asto** for?](#who-is-asto-for)
  - [The **Asto** Syntax](#the-asto-syntax)
  - [Params with **Asto**](#params-with-asto)
  - [Instalation](#instalation)
  - [Exporting **Asto** files](#exporting-asto-files)
    - [Export to JSON](#export-to-json)
    - [Export to MARKDOWN](#export-to-markdown)
- [Friend CLI](#friend-cli)
  - [Commands](#commands)
    - [`fnd hi`](#fnd-hi)
    - [`fnd bye`](#fnd-bye)
  - [Contribute](#contribute)
  - [License](#license)



## What is **Asto**?

**Asto** is a small but powerfull DSL designed to help CLI engineers organize, describe, and document command-line tools - clearly and efficiently.

Whether you're designing a new CLI or keeping track of a large one, **Asto** gives you a clean structure to define:

- Inputs
- Descriptions
- Output behaviour
- Status/version
- Params/Actions
- Exportable documentation


## Why **Asto**?

- Minimal syntax
- Easy to read
- Fast to write
- Exportable to `.json` or `.md`
- Perfect for documenting CLI tools
- Designed for DevTools engineers
- Fully Offline
- Runs on Windows, Linux and macOS
- Built in **Rust** for performance

**Asto** is not a replacement for your CLI. <br>
**Asto** is the language you use to structure it.

## Who is **Asto** for?

- CLI engineer
- DevTools Creators
- Backend Developers    
- Documenting Writers
- Anyone building command-line tools


## The **Asto** Syntax

Using a fictional CLI called `friend-cli` (`fnd`).

`friend_cli_commands.asto`
   
```asto
> fnd hi
/ Print "Hi"
: 0
$ Hi!

> fnd bye
/ Print "Bye"
: 0
$ Bye!
```

| Symbol | Meaning                                                                       |
| ------ | ----------------------------------------------------------------------------- |
| `>`    | Input command                                                                 |
| `/`    | Description                                                                   |
| `:`    | Status / version of that command (0 = new, 1 = finished, 0.5 = experimental…) |
| `$`    | Output example                                                                |

## Params with **Asto**

To specify a param use: `{}`

For example:

```asto
> fn hi --name --lastname

{
    --name STRING First name
    --lastname STRING Last name
}
```

The syntax is:

```asto
--param_name TYPE_VALUE : Description
```







## Instalation

With **Cargo**

```bash
cargo install asto
```

Verify installation:

```
asto --version
```

Output:

```bash
Asto vX.X.X
```

Without **Cargo**

Download the latest installer (`.exe`) from the releases page.



## Exporting **Asto** files

### Export to JSON

```bash
asto export friend_cli_command.asto --json
```

Output:

```bash
...Asto is exporting to JSON -.-

JSON generated with successfully!
```

It generates:

`outasto/friend_cli_command.json`:

```json
[
    {
        "input": "fnd hi",
        "description": "Print \"Hi\"",
        "status": 0,
        "out": "Hi!"
    },
    {
        "input": "fnd bye",
        "description": "Print \"Bye\"",
        "status": 0.5,
        "out": "Bye!"
    }
]
```

---

### Export to MARKDOWN

```bash
asto export friend_cli_commands.asto --md
```

**Asto** will prompt for metadata:

```bash
Title: Friend CLI
Description: A CLI as your friend.
Author: Gabriel Xavier

...Asto is reading -.-
...Asto is exporting to Markdown -.-

Markdown generated with successfully!
```

Output (`outasto/friend_cli_commands.md`):


# Friend CLI

By: Gabriel Xavier

A CLI as your friend.


## Commands

### `fnd hi`

Print "Hi"

**Status:** No Implemented

Output is:

```bash
Hi!
```

### `fnd bye`

Print "Bye"

**Status:** Deprecated

Output is:

```bash
Bye!
```

---


## Contribute

1. Fork
2. Create Branch `git branch -b my-improvement`
3. Commit `git commit -m "improve:: description of commands"`
4. Push `git push origin my-improvement`
5. Open a PR
   
Ideas, docs, features and fixes are welcome.


## License

**MIT License** - free to use, modify and integrate.
