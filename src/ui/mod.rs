//! Implementation of the console UI.

use bevy::input_focus::InputFocus;

use crate::prelude::*;

mod buffer;
mod console;
mod data;
mod text;
mod view;

pub mod prelude {
    pub use super::buffer::*;
    pub use super::console::*;
    pub use super::data::*;
    pub use super::text::prelude::*;
    pub use super::view::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(text::plugin);
    app.init_resource::<InputFocus>();
    app.init_resource::<ConsoleTextPipeline>();
    app.add_message::<ConsoleScrollMsg>();
    app.add_message::<ConsoleViewMsg>();
    app.add_message::<ConsoleActionMsg>();
    app.add_message::<ConsoleWriteMsg>();
}
