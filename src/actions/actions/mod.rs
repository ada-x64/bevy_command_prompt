use crate::prelude::*;

mod basic_input;
mod history;

// these actions should be overridable so they need to be made public
pub mod public {
    // pub use super::history::set_from_history;
    pub use super::basic_input::{delete_char, delete_word};
}

pub fn plugin(app: &mut App) {
    app.add_plugins((history::plugin, basic_input::plugin));
}
