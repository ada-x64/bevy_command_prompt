use crate::prelude::*;
use bevy::{
    input::mouse::MouseScrollUnit, input_focus::InputFocus, platform::collections::HashMap,
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

// TODO: Custom input actions are going to require a custom input solution.
// None of the existing solutions are extensible.
// Fortunately, they all rely on cosmic_text, so I have plenty of examples to go off of.
fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    mut q_console: Query<&mut Console>,
    mut history_idx: Local<usize>,
    mut filtered_history: Local<Option<Vec<usize>>>,
    mut original_value: Local<Option<String>>,
) {
    let mut pressed = keyboard.get_pressed();
    if let Some(input) = focus.0
        && let Ok(mut console) = q_console.get_mut(input)
    {
        if pressed.any(|k| *k == KeyCode::ArrowUp) {
            if filtered_history.is_none() {
                *original_value = Some(std::mem::take(&mut console.input));
                let f = console
                    .history
                    .iter()
                    .enumerate()
                    .filter_map(|(i, s)| {
                        s.starts_with(original_value.as_ref().unwrap()).then_some(i)
                    })
                    .collect::<Vec<_>>();
                *filtered_history = Some(f);
            }
            let fh = filtered_history.as_ref().unwrap();
            let ov = original_value.as_ref().unwrap();
            *history_idx = (*history_idx + 1).clamp(0, fh.len());
            info!("Setting from history");
            info!(?ov, ?history_idx);
            if *history_idx == 0 {
                console.input = ov.clone();
            } else {
                let idx = fh[fh.len() - *history_idx - 1];
                console.input = (console.history)[idx].clone();
            }
        } else if pressed.any(|k| *k == KeyCode::ArrowDown) && filtered_history.is_some() {
            let fh = filtered_history.as_ref().unwrap();
            let ov = original_value.as_ref().unwrap();
            *history_idx = (*history_idx - 1).clamp(0, fh.len());
            info!("Setting from history");
            info!(?ov, ?history_idx);
            if *history_idx == 0 {
                console.input = ov.clone();
            } else {
                let idx = fh[fh.len() - *history_idx - 1];
                console.input = console.history[idx].clone();
            }
        } else {
            *filtered_history = None;
            *original_value = None;
        }
    } else {
        error!("Could not access console history")
    }
}

fn on_scroll(
    mut reader: MessageReader<ConsoleScrollMsg>,
    mut commands: Commands,
    settings_q: Query<&ConsoleUiSettings>,
    view_q: Query<&ConsoleBufferView>,
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
            .and_modify(|d| *d += delta)
            .or_insert(delta);
    }
    for (console_id, delta) in map.into_iter() {
        let view = view_q.get(console_id).unwrap();
        let start = view.start.saturating_add_signed(delta.y as isize);
        let new_view = ConsoleBufferView { start, ..*view };
        commands.entity(console_id).insert(new_view);
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<ConsolePrintln>();
    app.add_systems(
        PostUpdate,
        (
            on_submit_msg.before(ui_layout_system),
            on_scroll.before(ui_layout_system),
        ),
    );
    app.add_systems(
        PreUpdate,
        keyboard_input.run_if(resource_changed::<ButtonInput<KeyCode>>),
    );
}
