use crate::prelude::*;
use bevy::{ecs::system::SystemId, platform::collections::HashMap};
use clap::Parser;

#[derive(Event, Clone, Debug)]
pub struct CallConsoleCommand(pub String);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct ConsoleCommands(HashMap<String, ConcreteConsoleCommand>);

#[derive(Debug)]
pub struct ConcreteConsoleCommand {
    pub cmd: clap::Command,
    pub dispatch: SystemId<In<String>>,
}

#[derive(Parser, Message, Reflect)]
#[command(name = "clear")]
pub struct ClearCmd;

/// List all the available commands.
#[derive(Parser, Message)]
#[command(name = "show-commands")]
pub struct ShowCommandsCmd {
    /// Print the command's short help message. By default, only print the name.
    #[arg(short, long)]
    pub long: bool,
}
