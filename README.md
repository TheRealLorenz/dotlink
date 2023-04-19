# dotlink

A simple program that can help you link all your dotfiles in place. 

Supports multiple presets, in order to avoid linking every file in every machine.

## Features!

- **Simple** TOML configuration file.
- Ability to have **multiple presets** for different machines.
- **Cross-Platform** (Unix and Windows).
- Doesn't overwrite links or files.
- If a link alreay exists, checks if it points to the right file.

## A real life use case (simplicity showcase)

I've setup my own [dotfiles](https://github.com/TheRealLorenz/dotfiles.git) repo with a [dotlink.toml](https://github.com/TheRealLorenz/dotfiles/blob/main/dotlink.toml) file.

In my case i simply clone the repo and run dotlink inside of it.

```bash
$ git clone https://github.com/TheRealLorenz/dotfiles.git
$ cd dotfiles
$ dotlink -p macOS
```

The program automatically picks up the config file inside the **current working directory** and links everything!

## How does it work?

dotlink relies on a config file, named `dotlink.toml`.

In the config file you can specify multiple presets, where every presets is a vector for entries.

Presets are top level keys of the config file.

There are two types of entries:

- Simple entries:
```toml
[[preset_name]]
name = 'foo'                  # File name
to = '/path/to/destination'   # Destination path
```

- Multiple entries:
```toml
[[preset_name]]
names = [ 'foo', 'bar', 'baz' ] # Multiple file names
to = '/path/to/destination/'    # Destination path
```

The program than simply symlinks every file specified by `name` or `names` to the corresponding `to`.

### Presets example

```toml
[[linux-wayland]]
names = [
  'sway',
  'sov',
  'zsh',
  'nushell',
  'waybar'
]
to = '~/.config'

[[linux-wayland]]
name = 'rc.zsh'
to = '~/.zshrc'

[[linux-wayland]]
name = 'tmux.conf'
to = '~/.tmux.conf'

[[linux-xorg]]
names = [
  'i3',
  'polybar',
  'zsh',
  'nushell'
]
to = '~/.config'

[[linux-xorg]]
name = 'rc.zsh'
to = '~/.zshrc' 

[[linux-xorg]]
name = 'tmux.conf'
to = '~/.tmux.conf' 

[[server]]
name = 'tmux.conf'
to = '~/.tmux.conf'

[[macOS]]
name = 'zsh'
to = '~/.config'

[[macOS]]
name = 'rc.zsh'
to = '~/.zshrc'

[[macOS]]
name = 'tmux.conf'
to = '~/.tmux.conf'

[[macOS]]
name = 'nushell'
to = '~/Application Support'

```

The example above defines 4 **presets**: **linux-wayland**, **linux-xorg**, **server**, **macOS**.
  
## Usage

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

## Planned features

- Manage linked files.
- Force linking.
- Publish to crates.io.
