//! Console commands are events that modify the world. These are not to be
//! confused with [bevy::prelude::Command], though the concepts are related.

use crate::prelude::*;

mod app_ext;
#[allow(clippy::module_inception)]
mod commands;
mod data;
mod events;

pub mod prelude {
    pub use super::app_ext::*;
    pub use super::commands::prelude::*;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ConsoleCommands>();
    app.add_plugins((events::plugin, commands::plugin));
}
