use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_command_prompt::prelude::*;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
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
                    Console,
                    ConsolePrompt("<=================>\n=>".into()),
                    ConsoleUiSettings {
                        font_color: tailwind::AMBER_700.into(),
                        background_color: tailwind::SLATE_200.into(),
                    },
                    TextFont {
                        font,
                        font_size: 12.,
                        ..Default::default()
                    },
                ],
            ));
        },
    );
    app.run();
}
