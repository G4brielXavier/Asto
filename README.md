# ASTO

![Build Status](https://github.com/G4brielXavier/Asto/actions/workflows/rust_ci.yml/badge.svg)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)
![Crates.io](https://img.shields.io/crates/v/Asto?color=orange)
![GitHub Repo stars](https://img.shields.io/github/stars/G4brielXavier/Asto?style=social)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/G4brielXavier/Asto)
![GitHub all releases](https://img.shields.io/github/downloads/G4brielXavier/Asto)
![Rust](https://img.shields.io/badge/Rust-3.11%2B-blue)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-lightgrey)
![Issues](https://img.shields.io/badge/Issues-welcome-brightgreen)

*A domain-specific language for organizing and documenting CLI commands - built in Rust.*

![ASTO](./assets/asto_banner.png)

## Summary

- [ASTO](#asto)
  - [Summary](#summary)
  - [🤔 What is **Asto**?](#-what-is-asto)
  - [🙂🫴 Why **Asto**?](#-why-asto)
  - [🤨 Who is **Asto** for?](#-who-is-asto-for)
  - [The **Asto** Syntax](#the-asto-syntax)
  - [Params with **Asto**](#params-with-asto)
  - [📥 installation](#-installation)
  - [🧩 VS Code Extension](#-vs-code-extension)
  - [Exporting `.asto` files](#exporting-asto-files)
    - [Export to JSON](#export-to-json)
    - [Export to MARKDOWN](#export-to-markdown)
- [Project Name](#project-name)
  - [Commands](#commands)
    - [`fnd hi`](#fnd-hi)
    - [`fnd bye`](#fnd-bye)
  - [Contribute](#contribute)
  - [License](#license)



## 🤔 What is **Asto**?

**Asto** is a small but powerfull DSL designed to help CLI engineers organize, describe, and document command-line tools - clearly and efficiently.

Whether you're designing a new CLI or keeping track of a large one, **Asto** gives you a clean structure to define:

- Inputs
- Descriptions
- Output behavior
- Status/version
- Params/Actions
- Exportable documentation


## 🙂🫴 Why **Asto**?

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

## 🤨 Who is **Asto** for?

- CLI engineer
- DevTools Creators
- Backend Developers    
- Documenting Writers
- Anyone building command-line tools


## The **Asto** Syntax

Using a fictional CLI called `friend-cli` (`fnd`).

`friend_cli_commands.asto`
   
```asto
> fnd hi --name
  / "Print 'Hi NAME'"
  : new
  {
    --name STRING "Username"
  }

> fnd bye --name
  / "Print 'Bye NAME'"
  : new
  {
    --name STRING "Username"
  }
```

| Symbol    | Meaning                                                                              |
| --------- | ------------------------------------------------------------------------------------ |
| `>`       | Input command                                                                        |
| `/ ""`    | Description                                                                          |
| `--param` | it represents a parameter                                                             |
| `:`       | Command's status (`new`, `depr`, `expm`, `stable`)  |
| `{}`      | Param info                                                                 |

## Params with **Asto**

To use params, use `--`:

```asto
> fn hi --name
```

To specific param's type or what the param represents, use `{}`

For example:

```asto
> fn hi --name --lastname
  / "Print 'Hi name lastname'"
  : new
  {
      --name string "First name"
      --lastname string "Last name"
  }
```

The syntax is:

```asto
--param_name type "Description"
```







## 📥 installation

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
asto vX.X.X
```

Without **Cargo**

Download the latest installer (`.exe`) from the releases page

## 🧩 VS Code Extension

Get syntax highlighting for your .asto files: *gabrielxavier.Asto-Plugin* (Search with this in vscode).


## Exporting `.asto` files

### Export to JSON

```bash
asto export friend_cli_command.asto --json
```

Output:

```bash
Asto CLI - "friend_cli_command.json" exported with successfully!
```

It generates:

`friend_cli_command.json`:

```json
[
  {
    "node": "Input",
    "command": "fnd hi --name",
    "description": "Print 'Hi name'",
    "prefix": "fnd",
    "function": "hi",
    "version": "new",
    "params": [
      "--name"
    ],
    "params_config": [
      {
        "name": "--name",
        "typeval": "STRING",
        "desc": "Username"
      }
    ]
  },
  {
    "node": "Input",
    "command": "fnd bye --name",
    "description": "Print 'Bye name'",
    "prefix": "fnd",
    "function": "bye",
    "version": "new",
    "params": [
      "--name"
    ],
    "params_config": [
      {
        "name": "--name",
        "typeval": "STRING",
        "desc": "Username"
      }
    ]
  }
]
```

---

### Export to MARKDOWN

```bash
asto export friend_cli_commands.asto --markdown
```

```bash
Asto CLI - "friend_cli_command.md" exported with successfully!
```

Output (`friend_cli_commands.md`):


# Project Name

By: YOUR_NAME_HERE

Informations about your CLI...


## Commands

### `fnd hi`

Print "Hi"

**Version:** New

Output is:

```bash
Hi Gabriel!
```

### `fnd bye`

Print "Bye"

**Version:** Experimental

Output is:

```bash
Bye Gabriel!
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
