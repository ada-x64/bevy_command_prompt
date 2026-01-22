use clap::Parser;

use crate::prelude::*;

#[derive(Parser, Debug, Message, Clone)]
#[command(name = "echo")]
struct EchoCmd {
    #[arg(required = false)]
    text: Vec<String>,
    #[arg(short = 'e', long = "enable-escapes")]
    enable_escapes: bool,
    #[arg(short = 'E', long = "disable-escapes")]
    disable_escapes: bool,
}

fn process_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some(&'n') => {
                    chars.next();
                    result.push('\n');
                }
                Some(&'t') => {
                    chars.next();
                    result.push('\t');
                }
                Some(&'r') => {
                    chars.next();
                    result.push('\r');
                }
                Some(&'\\') => {
                    chars.next();
                    result.push('\\');
                }
                Some(&'a') => {
                    chars.next();
                    result.push('\x07');
                }
                Some(&'b') => {
                    chars.next();
                    result.push('\x08');
                }
                Some(&'f') => {
                    chars.next();
                    result.push('\x0c');
                }
                Some(&'v') => {
                    chars.next();
                    result.push('\x0b');
                }
                _ => result.push(c),
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn inner(input: In<CommandMsg<EchoCmd>>, world: &mut World) {
    let (cmd, console_id) = (input.0.command, input.0.console_id);
    let text = cmd.text.join(" ");
    let message = if cmd.enable_escapes && !cmd.disable_escapes {
        process_escapes(&text)
    } else {
        text
    };
    world.commands().write_message(ConsoleWriteMsg {
        message: message + "\n",
        console_id,
    });
}

fn on_find_msg(mut reader: MessageReader<CommandMsg<EchoCmd>>, mut commands: Commands) {
    for msg in reader.read() {
        commands.run_system_cached_with(inner, msg.clone());
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, on_find_msg);
    app.add_console_command::<EchoCmd>();
}
