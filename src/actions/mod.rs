//! _Actions_ are events that alter the command prompt in some way. For events that modify the world, see [commands.](super::commands)

use crate::prelude::*;

#[allow(clippy::module_inception)]
pub mod actions;
mod app_ext;
mod console_action;
pub mod prelude {
    pub use super::app_ext::*;
    pub use super::console_action::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(actions::plugin);
}
