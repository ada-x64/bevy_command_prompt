mod actions;
mod commands;
mod systems;
#[cfg(test)]
mod test_harness;
mod ui;

pub mod prelude {
    pub use super::ConsolePlugin;
    pub use super::actions::prelude::*;
    pub use super::commands::prelude::*;
    pub use super::systems::*;
    pub use super::ui::prelude::*;
    pub(crate) use bevy::prelude::*;
    pub(crate) use tiny_bail::prelude::*;
}

use bevy::{input_focus::InputFocus, ui::ui_layout_system};
use prelude::*;

/// The main entrypoint for bevy_command_prompt.
pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            crate::ui::plugin,
            crate::commands::plugin,
            crate::actions::plugin,
        ));
        app.add_systems(
            PostUpdate,
            (
                (
                    handle_input.run_if(resource_exists::<InputFocus>),
                    update_console_input_text,
                    clear_action_queue,
                    clear_write_queue,
                    clear_view_queue,
                )
                    .chain()
                    .before(ui_layout_system)
                    .in_set(ConsoleSystems),
                on_resize.after(ui_layout_system).in_set(ConsoleSystems),
            ),
        );
    }
}
