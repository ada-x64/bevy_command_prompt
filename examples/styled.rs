use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_command_prompt::{ConsolePlugin, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::default());
    app.add_plugins(ConsolePlugin);
    app.add_systems(
        Startup,
        |mut commands: Commands, asset_server: ResMut<AssetServer>| {
            let font = asset_server.load::<Font>("FiraCode-Medium.ttf");
            commands.spawn(Camera2d);
            commands.spawn((
                Node {
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    ..Default::default()
                },
                children![
                    Console::default().with_prompt("<=================>\n=>".into()),
                    ConsoleUiSettings {
                        font: TextFont {
                            font,
                            font_size: 12.,
                            ..Default::default()
                        },
                        font_color: tailwind::AMBER_700.into(),
                        background_color: tailwind::SLATE_200.into(),
                    }
                ],
            ));
        },
    );
    app.run();
}
