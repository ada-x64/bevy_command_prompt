use crate::prelude::*;
use bevy::{ecs::system::SystemId, platform::collections::HashMap};

#[derive(Event, Clone, Debug)]
pub struct CallConsoleCommand(pub String);

#[derive(Resource, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct CommandHistory(Vec<String>);

#[derive(Resource, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct ConsoleCommands(HashMap<String, ConcreteConsoleCommand>);

#[derive(Debug, Clone, Reflect)]
#[reflect(opaque)]
pub struct ConcreteConsoleCommand {
    pub cmd: clap::Command,
    pub dispatch: SystemId<In<String>>,
}
