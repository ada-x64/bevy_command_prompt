use bevy::input_focus::InputFocus;

use crate::prelude::*;

mod console;
mod data;
mod events;

pub mod prelude {
    pub use super::console::*;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((events::plugin, console::plugin));
    app.init_resource::<InputFocus>();
    app.add_message::<ConsolePrintln>();
    app.add_message::<ConsoleScrollMsg>();
    app.add_message::<ConsoleSubmitMsg>();
}
