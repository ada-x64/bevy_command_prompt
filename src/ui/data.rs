use crate::prelude::*;

#[derive(Message, Event, Clone, Debug)]
pub struct ConsolePrintln {
    pub message: String,
    pub console_id: Entity,
}

#[derive(Message, Clone, Debug, Reflect)]
pub struct ConsoleScrollMsg {
    pub message: Pointer<Scroll>,
    pub console_id: Entity,
}

#[derive(Message, Clone, Debug, Reflect)]
pub struct ConsoleSubmitMsg {
    pub console_id: Entity,
}
