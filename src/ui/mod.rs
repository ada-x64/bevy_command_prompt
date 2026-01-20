use bevy::{input_focus::InputFocus, text::LineHeight};

use crate::prelude::*;

mod console;
mod data;
mod text;

pub mod prelude {
    pub use super::console::*;
    pub use super::data::*;
    pub use super::text::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(console::plugin);
    app.init_resource::<InputFocus>();
    app.init_resource::<ConsoleTextPipeline>();
    app.add_message::<ConsoleScrollMsg>();
    app.add_message::<ConsoleSubmitMsg>();
}

pub fn calc_line_height(line_height: &LineHeight, font_size: f32) -> f32 {
    match line_height {
        LineHeight::Px(px) => *px,
        LineHeight::RelativeToFont(scale) => *scale * font_size,
    }
}
