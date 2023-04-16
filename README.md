# dotlink

A simple program that can help you link all your dotfiles in place. 

Supports multiple presets, in order to avoid linking every file in every machine.

> **Warning**
> UNIX only (for now)

## Features!

- Ability to have multiple presets for different machines.
- Simple TOML configuration file.
- Doesn't overwrite links or files.
- If a link alreay exists, checks if it points to the right file.

## Planned features

- Manage linked files.
- Force linking.
- Windows support.
- Publish to crates.io.

## A real life use case

I've setup my own [dotfiles](https://github.com/TheRealLorenz/dotfiles.git) repo with a [dotlink.toml](https://github.com/TheRealLorenz/dotfiles/blob/main/dotlink.toml) file.

In my case i simply clone the repo and run dotlink inside of it.

```bash
$ git clone https://github.com/TheRealLorenz/dotfiles.git
$ cd dotfiles
$ dotlink -p macOS
```

The program automatically picks up the config file inside the **current working directory** and links everything!

## Config file

dotlink relies on a config file, named `dotlink.toml`.

```toml
[presets.linux-wayland]
to = '~/.config'
links = [
  'sway',
  'sov',
  'zsh',
  'nushell',
  'waybar',
  { name = 'rc.zsh', to = '~/.zshrc' },
  { name = 'tmux.conf', to = '~/.tmux.conf' }
]

[presets.linux-xorg]
to = '~/.config'
links = [
  'i3',
  'polybar',
  'zsh',
  'nushell',
  { name = 'rc.zsh', to = '~/.zshrc' },
  { name = 'tmux.conf', to = '~/.tmux.conf' }
]

[presets.server]
to = '~/.config'
links = [ { name = 'tmux.conf', to = '~/.tmux.conf' } ]

[presets.macOS]
to = '~/.config'
links = [
  'zsh',
  { name = 'rc.zsh', to = '~/.zshrc' },
  { name = 'tmux.conf', to = '~/.tmux.conf' },
  { name = 'nushell', to = '~/Application Support' }
]
```

The example above defines 4 **presets**: **linux-wayland**, **linux-xorg**, **server**, **macOS**.

Each **preset** has 2 main fields: 
  - `to`: a string which represents the dir that simple entries will be linked to.
  - `links`: an array of entries.
  
  > #### Types of entries
  > - Simple entry: a string which represents the name of the entry.
  > - Custom entry: an object which contains the name of the entry and the custom link location.
  
## Using

Running `dotlink -h` will show the help message
```
Usage: dotlink [OPTIONS] [PATH]

Arguments:
  [PATH]  

Options:
  -p, --preset <PRESET>  Which preset to use [default: default]
  -l, --list-presets     
  -F, --file <FILE>      Custom config file location
      --dry-run          
  -h, --help             Print help

```

> `PATH` represents the **path to the dotfiles**. Defaults to the **current working directory**.

## Installing

Clone the repository

```bash
$ git clone https://github.com/TheRealLorenz/dotlink
```

Install with [Cargo](https://docs.rs/cargo/latest/cargo/)

```bash
$ cargo install --path dotlink
```
