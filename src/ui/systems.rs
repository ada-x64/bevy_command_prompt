use crate::prelude::*;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{AccumulatedMouseScroll, MouseButtonInput},
    },
    input_focus::InputFocus,
    ui::ui_layout_system,
};

pub(crate) fn console_input_system(
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

pub fn plugin(app: &mut App) {
    app.add_message::<ConsolePrintln>();
    app.add_message::<ConsoleScrollMsg>();
    app.add_message::<ConsoleSubmitMsg>();
    app.add_systems(
        PostUpdate,
        console_input_system
            .run_if(resource_exists::<InputFocus>)
            .before(ui_layout_system),
    );
}
