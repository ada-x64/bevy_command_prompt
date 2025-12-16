use clap::Parser;

use crate::prelude::*;

#[derive(Parser, Message, Reflect)]
#[command(name = "clear")]
pub struct ClearCmd;

fn on_clear(mut reader: MessageReader<ClearCmd>, mut commands: Commands) {
    if !reader.is_empty() {
        reader.clear();
        commands.trigger(ClearConsole);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, on_clear);
    app.add_console_command::<ClearCmd>();
}
