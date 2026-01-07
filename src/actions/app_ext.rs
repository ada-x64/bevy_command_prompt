use crate::prelude::*;
use bevy::{ecs::system::SystemId, input::keyboard::Key, platform::collections::HashMap};

/// Stores all the registered console actions.
#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct ConsoleActionCache(HashMap<ConsoleAction, SystemId<In<ConsoleActionInput>>>);

pub struct ConsoleActionInput {
    pub action: ConsoleAction,
    pub console_id: Entity,
    pub matched_keys: Vec<Key>,
    pub matched_mods: Vec<KeyCode>,
}

pub trait ConsoleActionExt {
    /// Registers a new console action.
    /// This will push a key-value pair to the [ConsoleActionCache]
    /// and register the corresponding system.
    fn register_console_action<M>(
        &mut self,
        action: ConsoleAction,
        system: impl IntoSystem<In<ConsoleActionInput>, (), M> + 'static,
    ) -> &mut Self;
}
impl ConsoleActionExt for App {
    fn register_console_action<M>(
        &mut self,
        action: ConsoleAction,
        system: impl IntoSystem<In<ConsoleActionInput>, (), M> + 'static,
    ) -> &mut Self {
        self.world_mut().init_resource::<ConsoleActionCache>();
        let system = self.world_mut().register_system(system);
        self.world_mut()
            .resource_mut::<ConsoleActionCache>()
            .insert(action, system);
        self
    }
}
