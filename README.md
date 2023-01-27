# rick

A cross platform, general productivity tool that speeds up your workflows straight from the command line.

## Usage

#### `docs`

Easily open up the documentation for popular resources like [Tailwind](https://tailwindcss.com), [MDN](https://developer.mozilla.org/en-US/docs), or more (PR's welcome :smile:). Uses `fzf` to easily search for the thing you're looking for. Automatically opens up the docs page when selected.


For more info:
```console
$ rick docs help

Open the documentation for a specific library, framework, or language

Usage: rick docs <COMMAND>

Commands:
  tw    Search tailwind docs
  mdn   Search mdn docs
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

#### Todo

A simple and pretty task tracker to easily manage your todos, right from your terminal.

```console
$ rick todo help

Manage your todos

Usage: rick todo <COMMAND>

Commands:
  add      Add a new todo
  edit     Edit a todo by id
  delete   Delete a todo by id
  list     List all of your current todos
  monitor  Monitor your todos, refreshing periodically (default is 5 seconds)
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
  ````

#### Installation

Before installing, you must have [fzf](https://github.com/junegunn/fzf) installed on your system and in your PATH.

The preferred method for installing `rick` is through cargo

```console
cargo install rick
```

However, you can find [prebuilt binaries on the release page if you don't have cargo installed](https://github.com/reaganmcf/rick/releases).
