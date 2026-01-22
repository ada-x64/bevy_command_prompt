<div align="center">
<img src="./.doc/logo.png" height=300 alt="Logo - 'bevy command prompt' over a sketch of a green magpie" />
</div>

This is an attempt at a `bevy_ui`-native dev console.

## Inspiration

This project would not exist without inspiration from the following sources:

- [makspll/bevy-console](https://github.com/makspll/bevy-console)
- [hymm/bevy_mod_debug_console](https://github.com/hymm/bevy_mod_debug_console)
- [mojang/brigadier](https://github.com/Mojang/brigadier)

## Features

- [x] Sane default UI built in native bevy.
- [x] Command parsing with [clap](https://crates.io/crates/clap)
- [x] Command history
- [ ] Basic built-in commands
  - [x] `clear` - clears the console
  - [x] `show` - list available commands, registered components, active entities, resources, assets, etc
- [ ] Command completion
  - [ ] Command names
  - [ ] Command parameters (when possible choices are enumerated)
- [x] Basic keyboard shortcuts (`^C`, `^L`)
- [x] Customizable UI
- [x] Custom actions
- [x] Virtual scrolling
- [ ] Input cursor

### Stretch goals

- [ ] Dynamic entity selection / query language a la brigadier
- [ ] Picker support
- [ ] Environment variable support
  - Simple key/value string store.
- [ ] Colorized commands with ANSI escapes
- [ ] Text selection, Copy/paste
  - Requires custom text rendering with comsic_text::edit

### Non-goals
- Multi-channel I/O
  - e.g. STDIN/STDOUT/STDERR
- Signal support
- Advanced ANSI
  - e.g. cursor commands, blinking characters

## Design principles

Built on modern bevy principles - Event-oriented architecture and a bevy-native UI. Simple. Lightweight. Customizable.
Examples and tests should emphasize these principles and demonstrate all core functionality.

CLI commands are simply Messages. They can be systems of any kind and should require no special mechanisms.
They should be reusable as common events in the world.

Devex comes first. This means that the console experience should be smooth and appealing to the game developer.
This may require sacrificing 'realism' e.g. the signal/async support above in favor of one-shot execution.

## About the bird

The bird on the logo is a [green
magpie](https://en.wikipedia.org/wiki/Common_green_magpie), a common species of
the crow family known for their intelligence and small size. The illustration is
from the [Big Book of Bird
Illustrations](https://www.overdrive.com/media/1405297/big-book-of-bird-illustrations)
by Maggie Kate.
