use bevy::input_focus::InputFocus;

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
    app.add_plugins((events::plugin, systems::plugin, console::plugin));
    app.init_resource::<InputFocus>();
}
