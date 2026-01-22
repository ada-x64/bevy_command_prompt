use crate::prelude::*;
use bevy::{ecs::system::SystemId, platform::collections::HashMap};

#[derive(Event, Clone, Debug)]
pub struct SubmitEvent {
    pub console_id: Entity,
    input: String,
    args: Vec<String>,
}
impl SubmitEvent {
    pub fn new(console_id: Entity, input: String) -> Option<SubmitEvent> {
        shlex::split(&input).map(|args| Self {
            console_id,
            input,
            args,
        })
    }
    pub fn console_id(&self) -> Entity {
        self.console_id
    }
    pub fn input(&self) -> &str {
        &self.input
    }
    pub fn args(&self) -> &[String] {
        &self.args
    }
}

#[derive(Resource, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct ConsoleCommands(HashMap<String, ConcreteConsoleCommand>);

#[derive(Debug, Clone, strum::EnumIter, strum::Display)]
pub enum ConsoleShellCommands {
    #[strum(serialize = "clear")]
    Clear,
}

#[derive(Debug, Clone, Reflect)]
#[reflect(opaque)]
pub struct ConcreteConsoleCommand {
    pub cmd: clap::Command,
    pub dispatch: SystemId<In<SubmitEvent>>,
}

#[derive(Debug, Clone, Message)]
pub struct CommandMsg<T: ConsoleCommand> {
    pub console_id: Entity,
    pub command: T,
}
impl<T: ConsoleCommand> CommandMsg<T> {
    pub fn println(&self, commands: &mut Commands, message: String) {
        commands.write_message(ConsoleWriteMsg {
            message: message + "\n",
            console_id: self.console_id,
        });
    }
}
