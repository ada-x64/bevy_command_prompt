use bevy::prelude::*;
use bevy_command_prompt::{ConsolePlugin, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::default());
    app.add_plugins(ConsolePlugin);
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Camera2d);
        commands.spawn(Console);
    });
    app.run();
}
