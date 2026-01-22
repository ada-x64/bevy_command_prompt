use bevy::prelude::*;
use bevy_command_prompt::prelude::*;
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (640, 480).into(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    // app.add_plugins(EguiPlugin::default());
    // app.add_plugins(WorldInspectorPlugin::default());
    app.add_plugins(ConsolePlugin);
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Camera2d);
        commands.spawn((
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                ..Default::default()
            },
            children![Console],
        ));
    });
    app.run();
}
