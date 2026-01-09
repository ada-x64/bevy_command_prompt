use crate::prelude::*;

pub mod basic_input;
pub mod history;

pub fn plugin(app: &mut App) {
    app.add_plugins((history::plugin, basic_input::plugin));
}
