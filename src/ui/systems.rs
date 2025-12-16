use crate::prelude::*;
use bevy::{input_focus::InputFocus, ui::ui_layout_system};
use bevy_ui_text_input::SubmitText;

/// submission event reader
/// this must occur after [TextInputSystem](bevy_simple_text_input::TextInputSystem),
/// but that's in the Update schedule so no need to worry
fn on_submit_msg(
    mut reader: MessageReader<SubmitText>,
    filter: Query<Entity, With<ConsoleInputValue>>,
    mut commands: Commands,
    mut history: ResMut<CommandHistory>,
) {
    for msg in reader.read() {
        // note: txtinput value cleared on submit
        if filter.iter().any(|e| e == msg.entity) {
            commands.trigger(CallConsoleCommand(msg.text.clone()));
            history.push(msg.text.clone());
        }
    }
}

fn on_append_to_console(
    mut reader: MessageReader<ConsolePrint>,
    q: Query<(&mut ScrollPosition, &ComputedNode), With<ConsoleBodyTextWrapper>>,
) {
    // NOTE: The intention here is to allow for multiple console.
    // This code doesn't quite work for that.
    // Are we intended to allow multiples? I don't really see the use case,
    // but maximal flexibility in a library is probably best.
    let msgs = reader.read().collect::<Vec<_>>();
    for (mut pos, cnode) in q {
        for _msg in &msgs {
            let max_offset = (cnode.content_size() - cnode.size()) * cnode.inverse_scale_factor();
            pos.x = 0.;
            pos.y = max_offset.y;
        }
    }
}

// TODO: Custom input actions are going to require a custom input solution.
// None of the existing solutions are extensible.
// Fortunately, they all rely on cosmic_text, so I have plenty of examples to go off of.
fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    cmd_history: Res<CommandHistory>,
    mut console_input: Single<(Entity, &mut Text), With<ConsoleInputValue>>,
    mut history_idx: Local<usize>,
    mut filtered_history: Local<Option<Vec<usize>>>,
    mut original_value: Local<Option<String>>,
) {
    let mut pressed = keyboard.get_pressed();
    if pressed.any(|k| *k == KeyCode::ArrowUp) {
        if let Some(input) = focus.0
            && input == console_input.0
        {
            if filtered_history.is_none() {
                let f = cmd_history
                    .iter()
                    .enumerate()
                    .filter_map(|(i, s)| s.starts_with(console_input.1.as_str()).then_some(i))
                    .collect::<Vec<_>>();
                *filtered_history = Some(f);
                *original_value = Some(console_input.1.0.clone());
            }
            let fh = filtered_history.as_ref().unwrap();
            let ov = original_value.as_ref().unwrap();
            *history_idx = (*history_idx + 1).clamp(0, fh.len());
            info!("Setting from history");
            info!(?ov, ?history_idx);
            if *history_idx == 0 {
                console_input.1.0 = ov.clone();
            } else {
                let idx = fh[fh.len() - *history_idx - 1];
                console_input.1.0 = (*cmd_history)[idx].clone();
            }
        }
    } else if pressed.any(|k| *k == KeyCode::ArrowDown) && filtered_history.is_some() {
        let fh = filtered_history.as_ref().unwrap();
        let ov = original_value.as_ref().unwrap();
        *history_idx = (*history_idx - 1).clamp(0, fh.len());
        info!("Setting from history");
        info!(?ov, ?history_idx);
        if *history_idx == 0 {
            console_input.1.0 = ov.clone();
        } else {
            let idx = fh[fh.len() - *history_idx - 1];
            console_input.1.0 = (*cmd_history)[idx].clone();
        }
    } else {
        *filtered_history = None;
        *original_value = None;
    }
}

fn update_style() {
    // TODO
}

pub fn plugin(app: &mut App) {
    app.add_message::<ConsolePrint>();
    app.add_systems(
        PostUpdate,
        (
            on_submit_msg.before(ui_layout_system),
            on_append_to_console.after(ui_layout_system),
        ),
    );
    app.add_systems(
        PreUpdate,
        (
            keyboard_input.run_if(resource_changed::<ButtonInput<KeyCode>>),
            update_style.run_if(resource_changed::<ConsoleUiSettings>),
        ),
    );
}
