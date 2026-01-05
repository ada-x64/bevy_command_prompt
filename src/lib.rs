mod actions;
mod commands;
mod ui;

pub use actions::actions::public::*;

pub mod prelude {
    pub use super::actions::prelude::*;
    pub use super::commands::prelude::*;
    pub use super::ui::prelude::*;
    pub(crate) use bevy::prelude::*;
    pub(crate) use tiny_bail::prelude::*;
}
use prelude::*;

/// The main entrypoint for bevy_command_prompt.
pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ui::plugin, commands::plugin, actions::plugin));
    }
}
