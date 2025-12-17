use crate::prelude::*;

mod app_ext;
#[allow(clippy::module_inception)]
mod commands;
mod data;
mod events;

pub mod prelude {
    pub use super::app_ext::*;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ConsoleCommands>();
    app.add_plugins((events::plugin, commands::plugin));
}
