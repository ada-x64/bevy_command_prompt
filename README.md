This is an attempt at a bevy_ui-native dev console inspired by [makspll/bevy-console](https://github.com/makspll/bevy-console).

## Features

- [ ] Sane default UI built in native bevy.
- [ ] Command parsing (clap?)
- [ ] Command history
    - Perhaps serialized.
- [ ] Command completion
    - [ ] Command names
    - [ ] Command parameters (when possible choices are enumerated)
- [ ] Basic keyboard shortcuts (`^C`, `^L`, `tab`, ` \` `)

### Stretch goals

- [ ] Dynamic entity selection / query language a la Minecraft commands
    - See [brigadier](https://github.com/Mojang/brigadier)
- [ ] Picker support
- [ ] Customizable UI
- [ ] Signal support (e.g. `^C` sends a signal to the currently executing command)
    - Would require the ability for a running command to capture input and output, which means asynchronous execution.
- [ ] Virtual scrolling for large command history

## Design principles

Built on modern bevy principles - Event-oriented architecture and a bevy-native UI. Simple. Lightweight. Customizable.
Examples and tests should emphasize these principles and demonstrate all core functionality.

CLI commands are simply bevy events. They can be systems of any kind and should require no special mechanisms.
They should be reusable as common events in the world.

Devex comes first. This means that the console experience should be smooth and appealing to the game developer.
This may require sacrificing 'realism' e.g. the signal/async support above in favor of one-shot execution.
