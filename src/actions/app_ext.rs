use crate::prelude::*;
use bevy::{ecs::system::IntoObserverSystem, platform::collections::HashMap};

#[derive(Clone, Debug, Event, PartialEq, Eq, Hash)]
pub struct ConsoleActionEvent {
    pub action: ConsoleAction,
    pub console_id: Entity,
}

/// Marker component for console action observer entities.
#[derive(Component, Debug)]
pub struct ConsoleActionObserver;

pub trait IntoConsoleActionObserverSystem<M>:
    IntoObserverSystem<ConsoleActionEvent, (), M>
{
}
impl<T, M> IntoConsoleActionObserverSystem<M> for T where
    T: IntoObserverSystem<ConsoleActionEvent, (), M>
{
}

/// Stores all the registered console actions.
#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct ConsoleActionCache(HashMap<ConsoleAction, Entity>);

pub trait ConsoleActionExt {
    /// Registers a new console action.
    /// This will push a key-value pair to the [ConsoleActionCache]
    /// and spawn a global observer to watch for the action.
    fn register_console_action<M>(
        &mut self,
        action: ConsoleAction,
        system: impl IntoConsoleActionObserverSystem<M>,
    ) -> &mut Self;
}
impl ConsoleActionExt for App {
    fn register_console_action<M>(
        &mut self,
        action: ConsoleAction,
        system: impl IntoConsoleActionObserverSystem<M>,
    ) -> &mut Self {
        self.world_mut().init_resource::<ConsoleActionCache>();
        let system = self
            .world_mut()
            .add_observer(system)
            .insert(ConsoleActionObserver)
            .id();
        self.world_mut()
            .resource_mut::<ConsoleActionCache>()
            .insert(action, system);
        self
    }
}
