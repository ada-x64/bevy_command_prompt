//! General systems related to console functionality. Mostly message queues.
use crate::prelude::*;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{MouseButtonInput, MouseWheel},
    },
    input_focus::InputFocus,
    text::LineHeight,
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsoleSystems;

pub fn handle_input(
    key_code_input: Res<ButtonInput<KeyCode>>,
    key_input: Res<ButtonInput<Key>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut mouse_events: MessageReader<MouseButtonInput>,
    mut wheel_events: MessageReader<MouseWheel>,
    actions: Res<ConsoleActionCache>,
    focus: Res<InputFocus>,
    mut q_console: Query<(&mut ComputedConsoleTextBlock, &LineHeight, &TextFont)>,
    mut commands: Commands,
) {
    if (!keyboard_events.is_empty() || !mouse_events.is_empty() || !wheel_events.is_empty())
        && let Some(console_id) = focus.0
        && let Ok((mut block, lineheight, font)) = q_console.get_mut(console_id)
    {
        block.trigger_rerender();
        // want to collect here so we can iterate multiple times.
        let keyboard_events = keyboard_events.read().collect::<Vec<_>>();
        let mouse_events = mouse_events.read().collect::<Vec<_>>();
        let wheel_events = wheel_events.read().collect::<Vec<_>>();
        let scroll = wheel_events
            .iter()
            .map(|e| match e.unit {
                bevy::input::mouse::MouseScrollUnit::Line => e.y as isize,
                bevy::input::mouse::MouseScrollUnit::Pixel => match lineheight {
                    LineHeight::Px(h) => (e.y * h) as isize,
                    LineHeight::RelativeToFont(h) => (h * font.font_size * e.y) as isize,
                },
            })
            .sum::<isize>();
        actions
            .iter()
            .filter_map(|(keybind, s)| {
                keybind
                    .to_system_input(
                        &keyboard_events,
                        &mouse_events,
                        scroll,
                        &key_input,
                        &key_code_input,
                        &mouse_input,
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
    mut buffer_q: Query<(&mut ConsoleBuffer, &mut ConsoleInputText)>,
) {
    for item in reader.read() {
        let (mut buffer, mut input) = c!(buffer_q.get_mut(item.console_id));
        c!(buffer.write(&item.message));
        input.anchor = buffer.reset_write_anchor();
    }
}

pub fn clear_view_queue(
    mut reader: MessageReader<ConsoleViewMsg>,
    mut query: Query<(&ConsoleBuffer, &ConsoleBufferView)>,
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
        let (buffer, view) = c!(query.get_mut(console_id));
        let new_view = reader.iter().fold(*view, |view, msg| match msg.action {
            ConsoleViewAction::Scroll(ydelta) => view.scroll(ydelta, buffer),
            ConsoleViewAction::JumpToBottom => view.jump_to_bottom(),
        });
        commands.entity(console_id).insert(new_view);
    }
}
pub fn on_resize(
    q: Query<
        (
            Entity,
            &ComputedNode,
            &TextFont,
            &mut ConsoleBufferFlags,
            &ConsoleBufferView,
            &LineHeight,
        ),
        Or<(Changed<ComputedNode>, Added<ConsoleBufferView>)>,
    >,
    mut commands: Commands,
) {
    for (entity, node, text_font, mut flags, view, line_height) in q {
        let new_view = view.resize(
            node.size().y,
            calc_line_height(line_height, text_font.font_size),
        );
        commands.entity(entity).insert(new_view);
        flags.needs_measure_fn = true;
    }
}
pub fn calc_line_height(line_height: &LineHeight, font_size: f32) -> f32 {
    match line_height {
        LineHeight::Px(px) => *px,
        LineHeight::RelativeToFont(scale) => *scale * font_size,
    }
}
