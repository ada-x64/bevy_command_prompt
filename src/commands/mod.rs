use crate::prelude::*;

mod app_ext;
mod data;
mod events;

pub mod prelude {
    pub use super::app_ext::*;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(events::plugin);
    app.init_resource::<ConsoleCommands>();
    app.add_console_command::<ClearCmd>();
    app.add_console_command::<ShowCommandsCmd>();
}
