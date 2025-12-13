pub mod data;
pub mod ui;

pub use bevy::prelude::*;
pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ui::plugin);
    }
}
