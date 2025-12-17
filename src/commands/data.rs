use crate::prelude::*;
use bevy::{ecs::system::SystemId, platform::collections::HashMap};

#[derive(Event, Clone, Debug)]
pub struct CallCommandEvent {
    pub command_name: String,
    pub console_id: Entity,
}

#[derive(Resource, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct ConsoleCommands(HashMap<String, ConcreteConsoleCommand>);

#[derive(Debug, Clone, strum::EnumIter, strum::Display)]
pub enum ConsoleBuiltin {
    #[strum(serialize = "clear")]
    Clear,
}

#[derive(Debug, Clone, Reflect)]
#[reflect(opaque)]
pub struct ConcreteConsoleCommand {
    pub cmd: clap::Command,
    pub dispatch: SystemId<In<CallCommandEvent>>,
}

#[derive(Debug, Clone, Message)]
pub struct CommandMsg<T: ConsoleCommand> {
    pub console_id: Entity,
    pub command: T,
}
impl<T: ConsoleCommand> CommandMsg<T> {
    pub fn println(&self, commands: &mut Commands, message: String) {
        commands.trigger(ConsolePrintln {
            message,
            console_id: self.console_id,
        })
    }
}
