use crate::prelude::*;
use bevy::{
    ecs::system::SystemId,
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::MouseButtonInput,
    },
    platform::collections::HashMap,
};

pub trait IntoConsoleActionSystem<M>: IntoSystem<In<ConsoleActionSystemInput>, (), M> {}
impl<T, M> IntoConsoleActionSystem<M> for T where T: IntoSystem<In<ConsoleActionSystemInput>, (), M> {}

pub type ConsoleActionSystem = SystemId<In<ConsoleActionSystemInput>>;

/// Stores all the registered console actions.
// NOTE: SystemIds are lightweight,so there's no need to worry about redundancy here.
#[derive(Resource, Debug, Deref, DerefMut, Default, Clone, Reflect)]
pub struct ConsoleActionCache(
    #[reflect(ignore)] HashMap<ConsoleActionKeybind, ConsoleActionSystem>,
);

#[derive(Debug, Clone)]
pub struct ConsoleActionSystemInput {
    pub console_id: Entity,
    pub matched_input: Vec<MatchedInput>,
    pub matched_mods: Vec<KeyCode>,
}
impl ConsoleActionSystemInput {
    pub fn matched_key_input(&self) -> impl Iterator<Item = &KeyboardInput> {
        self.matched_input.iter().filter_map(|input| {
            if let MatchedInput::Key(k) = input {
                Some(k)
            } else {
                None
            }
        })
    }
    pub fn matched_logical_keys(&self) -> impl Iterator<Item = &Key> {
        self.matched_input.iter().filter_map(|input| {
            if let MatchedInput::Key(k) = input {
                Some(&k.logical_key)
            } else {
                None
            }
        })
    }
    pub fn matched_mouse_input(&self) -> impl Iterator<Item = &MouseButtonInput> {
        self.matched_input.iter().filter_map(|input| {
            if let MatchedInput::Mouse(m) = input {
                Some(m)
            } else {
                None
            }
        })
    }

    pub fn matched_scroll(&self) -> Option<isize> {
        self.matched_input.iter().find_map(|input| {
            if let MatchedInput::Scroll(s) = input {
                Some(*s)
            } else {
                None
            }
        })
    }
}

pub trait ConsoleActionExt {
    /// Registers a new console action.
    /// This will push a key-value pair to the [ConsoleActionCache]
    /// and register the corresponding system.
    fn register_console_action<M>(
        &mut self,
        keybind: ConsoleActionKeybind,
        system: impl IntoConsoleActionSystem<M> + 'static,
    ) -> &mut Self;
}
impl ConsoleActionExt for App {
    fn register_console_action<M>(
        &mut self,
        action: ConsoleActionKeybind,
        system: impl IntoConsoleActionSystem<M> + 'static,
    ) -> &mut Self {
        self.world_mut().init_resource::<ConsoleActionCache>();
        let system = self.world_mut().register_system(system);
        self.world_mut()
            .resource_mut::<ConsoleActionCache>()
            .insert(action, system);
        self
    }
}
