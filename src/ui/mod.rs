use bevy_ui_text_input::TextInputPlugin;

use crate::prelude::*;

mod console;
mod data;
mod events;
mod systems;

pub mod prelude {
    pub use super::console::*;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        TextInputPlugin,
        events::plugin,
        systems::plugin,
        console::plugin,
    ));
}
