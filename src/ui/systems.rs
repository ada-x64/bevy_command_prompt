use crate::prelude::*;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::MouseScrollUnit,
    },
    input_focus::InputFocus,
    platform::collections::HashMap,
    ui::ui_layout_system,
};

/// submission event reader
/// this must occur after [TextInputSystem](bevy_simple_text_input::TextInputSystem),
/// but that's in the Update schedule so no need to worry
fn on_submit_msg(
    mut reader: MessageReader<ConsoleSubmitMsg>,
    mut query: Query<&mut Console>,
    mut commands: Commands,
) {
    for msg in reader.read() {
        if let Ok(mut console) = query.get_mut(msg.console_id) {
            commands.trigger(CallCommandEvent {
                command_name: console.input.clone(),
                console_id: msg.console_id,
            });
            let history_value = std::mem::take(&mut console.input);
            console.history.push(history_value);
        } else {
            error!("Could not submit from console with id {}", msg.console_id);
        }
    }
}

fn set_from_history(
    input: In<(Key, Entity)>,
    mut q_console: Query<&mut Console>,
    mut history_idx: Local<usize>,
    mut filtered_history: Local<Option<Vec<usize>>>,
    mut original_value: Local<Option<String>>,
) {
    let In((key, console_id)) = input;
    let mut value = 0;
    match key {
        Key::ArrowUp => value = 1,
        Key::ArrowDown => value = -1,
        Key::Enter => {
            *history_idx = 0;
            *filtered_history = None;
            *original_value = None;
            value = 0;
        }
        _ => {}
    }
    let mut console = q_console.get_mut(console_id).unwrap();
    if filtered_history.is_none() {
        *original_value = Some(std::mem::take(&mut console.input));
        let f = console
            .history
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.starts_with(original_value.as_ref().unwrap()).then_some(i))
            .collect::<Vec<_>>();
        *filtered_history = Some(f);
    }
    if matches!(key, Key::ArrowUp | Key::ArrowDown) {
        let fh = filtered_history.as_ref().unwrap();
        let ov = original_value.as_ref().unwrap();
        *history_idx = history_idx.saturating_add_signed(value).min(fh.len());
        if *history_idx == 0 {
            console.input = ov.clone();
        } else {
            let idx = fh[fh.len().saturating_sub(*history_idx + 1)];
            console.input = console.history[idx].clone();
        }
    }
}

fn keyboard_input(
    button_inputs: Res<ButtonInput<KeyCode>>, // for modifiers
    mut input_events: MessageReader<KeyboardInput>,
    focus: Res<InputFocus>,
    mut q_console: Query<(&mut Console, &ConsoleBufferView)>,
    mut commands: Commands,
) {
    if !input_events.is_empty()
        && let Some(console_id) = focus.0
        && let Ok((mut console, buffer_view)) = q_console.get_mut(console_id)
    {
        let mut needs_refresh = false;
        // assumes console buffer can be written to right now
        // this is always true in real terminals
        for event in input_events.read() {
            if button_inputs.just_pressed(event.key_code) {
                commands.run_system_cached_with(
                    set_from_history,
                    (event.logical_key.clone(), console_id),
                );
                match event.logical_key {
                    Key::ArrowUp | Key::ArrowDown => {
                        needs_refresh = true;
                    }
                    Key::Enter => {
                        commands.write_message(ConsoleSubmitMsg { console_id });
                        needs_refresh = true;
                    }
                    Key::Character(ref c) => {
                        console.input = format!("{}{}", console.input, c);
                        needs_refresh = true;
                    }
                    Key::Space => {
                        console.input.push(' ');
                        needs_refresh = true;
                    }
                    Key::Backspace => {
                        if button_inputs.pressed(KeyCode::ControlLeft)
                            || button_inputs.pressed(KeyCode::ControlRight)
                        {
                            let last_ws =
                                console.input.rfind(char::is_whitespace).unwrap_or_default();
                            console.input.truncate(last_ws);
                        } else {
                            console.input.pop();
                        }
                        needs_refresh = true;
                    }
                    Key::Tab => {
                        // completion
                        needs_refresh = true;
                    }
                    _ => {}
                }
            }
        }
        if needs_refresh {
            commands
                .entity(console_id)
                .insert(buffer_view.jump_to_bottom(&console));
        }
    }
}

fn on_scroll(
    mut reader: MessageReader<ConsoleScrollMsg>,
    mut commands: Commands,
    settings_q: Query<&ConsoleUiSettings>,
    console_q: Query<(&Console, &ConsoleBufferView)>,
) {
    let mut map = HashMap::<Entity, Vec2>::new();
    for msg in reader.read() {
        let settings = settings_q.get(msg.console_id).unwrap();
        let delta = match msg.message.unit {
            MouseScrollUnit::Line => Vec2::new(msg.message.x, msg.message.y),
            MouseScrollUnit::Pixel => {
                Vec2::new(msg.message.x, msg.message.y / settings.line_height())
            }
        };
        map.entry(msg.console_id)
            .and_modify(|d| *d -= delta)
            .or_insert(delta);
    }
    for (console_id, delta) in map.into_iter() {
        let (console, view) = console_q.get(console_id).unwrap();
        let range = view.range;
        let buffer_size = console.buffer.lines().count();
        let prompt_size = console.prompt.lines().count();
        if buffer_size <= range {
            continue; // disable scroll. this should be handled elsewhere.
        }
        let start = view
            .start
            .saturating_add_signed(delta.y as isize)
            .min(buffer_size - range + prompt_size);
        let new_view = ConsoleBufferView { start, ..*view };
        commands.entity(console_id).insert(new_view);
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<ConsolePrintln>();
    app.add_message::<ConsoleScrollMsg>();
    app.add_message::<ConsoleSubmitMsg>();
    app.add_systems(
        PostUpdate,
        (
            keyboard_input.run_if(
                resource_changed::<ButtonInput<KeyCode>>.and(resource_exists::<InputFocus>),
            ),
            on_submit_msg.before(ui_layout_system),
            on_scroll.before(ui_layout_system),
        ),
    );
}
