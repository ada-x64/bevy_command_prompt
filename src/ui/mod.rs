use crate::prelude::*;
use bevy_simple_text_input::TextInputPlugin;

mod bundle;
mod data;
mod events;
mod systems;

pub mod prelude {
    pub use super::bundle::*;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((TextInputPlugin, events::plugin, systems::plugin));
    app.init_resource::<ConsoleInputPrompt>();
}
