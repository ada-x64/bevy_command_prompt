//! General systems related to console functionality. Mostly message queues.
use crate::prelude::*;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{AccumulatedMouseScroll, MouseButtonInput},
    },
    input_focus::InputFocus,
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
    mut q_console: Query<&mut ComputedConsoleTextBlock>,
    mut commands: Commands,
) {
    if !keyboard_events.is_empty()
        && let Some(console_id) = focus.0
        && let Ok(mut block) = q_console.get_mut(console_id)
    {
        block.trigger_rerender();
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
            .for_each(|(input, system)| {
                commands.write_message(ConsoleActionMsg {
                    console_id,
                    input,
                    system,
                });
            });
    }
}

pub fn clear_action_queue(mut reader: MessageReader<ConsoleActionMsg>, mut commands: Commands) {
    for item in reader.read() {
        commands.run_system_with(item.system, item.input.clone());
    }
}

pub fn clear_write_queue(
    mut reader: MessageReader<ConsoleWriteMsg>,
    mut buffer_q: Query<&mut ConsoleBuffer>,
) {
    for item in reader.read() {
        let mut buffer = c!(buffer_q.get_mut(item.console_id));
        c!(buffer.write(&item.message));
    }
}

pub fn clear_view_queue(
    mut reader: MessageReader<ConsoleViewMsg>,
    mut query: Query<(
        &ConsoleBuffer,
        &ConsoleBufferView,
        &ConsolePrompt,
        &mut ComputedConsoleTextBlock,
    )>,
    mut commands: Commands,
) {
    // collect for multiple iteration
    let reader = reader.read().collect::<Vec<_>>();
    // get all console_ids for bucketing
    let ids = reader.iter().fold(vec![], |mut accum, msg| {
        if !accum.contains(&msg.console_id) {
            accum.push(msg.console_id)
        }
        accum
    });
    for console_id in ids {
        let (buffer, view, prompt, mut block) = c!(query.get_mut(console_id));
        let new_view = reader.iter().fold(*view, |view, msg| match msg.action {
            ConsoleViewAction::Scroll(ydelta) => view.scroll(ydelta, buffer, prompt),
            ConsoleViewAction::JumpToBottom => view.jump_to_bottom(prompt, &mut block),
        });
        commands.entity(console_id).insert(new_view);
    }
}
