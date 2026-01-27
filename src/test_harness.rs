use bevy::{
    image::TextureAtlasPlugin, input::InputPlugin, input_focus::InputFocus, text::TextPlugin,
    ui::UiPlugin,
};
use bevy_test_harness::{TestRunnerPlugin, TestRunnerTimeout};

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        TestRunnerPlugin::default(),
        ConsolePlugin,
        DefaultPickingPlugins,
        WindowPlugin {
            primary_window: None,
            exit_condition: bevy::window::ExitCondition::DontExit,
            ..Default::default()
        },
        InputPlugin,
        UiPlugin,
        TextPlugin,
        TextureAtlasPlugin,
    ));
    app.add_systems(Startup, setup);
    app.insert_resource(TestRunnerTimeout(1.));
}

fn setup(mut commands: Commands, mut focus: ResMut<InputFocus>) {
    let id = commands.spawn(Console).id();
    focus.0 = Some(id);
}
