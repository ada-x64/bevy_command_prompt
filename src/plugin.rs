use crate::prelude::*;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{AccumulatedMouseScroll, MouseButtonInput},
    },
    input_focus::InputFocus,
    ui::ui_layout_system,
};

pub fn handle_input(
    key_code_input: Res<ButtonInput<KeyCode>>,
    key_input: Res<ButtonInput<Key>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    scroll: Res<AccumulatedMouseScroll>,
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut mouse_events: MessageReader<MouseButtonInput>,
    actions: Res<ConsoleActionCache>,
    focus: Res<InputFocus>,
    mut q_console: Query<&mut ConsoleActionQueue>,
) {
    if !keyboard_events.is_empty()
        && let Some(console_id) = focus.0
        && let Ok(mut action_queue) = q_console.get_mut(console_id)
    {
        // want to collect here so we can iterate multiple times.
        let keyboard_events = keyboard_events.read().collect::<Vec<_>>();
        let mouse_events = mouse_events.read().collect::<Vec<_>>();
        actions
            .iter()
            .filter_map(|(keybind, s)| {
                keybind
                    .to_system_input(
                        &keyboard_events,
                        &mouse_events,
                        &key_input,
                        &key_code_input,
                        &mouse_input,
                        (scroll.delta != Vec2::ZERO).then_some(&*scroll),
                        console_id,
                    )
                    .map(|i| (i, *s))
            })
            .for_each(|(i, s)| {
                action_queue.push((i, s));
            });
    }
}

pub fn clear_action_queue(queues: Query<&mut ConsoleActionQueue>, mut commands: Commands) {
    for mut queue in queues {
        queue
            .drain(..)
            .for_each(|(i, s)| commands.run_system_with(s, i));
    }
}

pub fn clear_write_queue(queues: Query<(&mut ConsoleWriteQueue, &mut ConsoleBuffer)>) {
    for (mut queue, mut buffer) in queues {
        queue.drain(..).for_each(|string| {
            buffer.write(&string).expect("Failed to write to buffer!");
        });
    }
}

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
                handle_input.run_if(resource_exists::<InputFocus>),
                clear_action_queue,
                clear_write_queue,
            )
                .chain()
                .before(ui_layout_system),
        );
    }
}
