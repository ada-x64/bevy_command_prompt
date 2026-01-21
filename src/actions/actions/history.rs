use bevy::input::keyboard::Key;

use crate::prelude::*;

// TODO: History gets a bit out of order.
pub fn set_from_history(
    input: In<ConsoleActionSystemInput>,
    mut q_console: Query<(&mut ConsoleInputText, &ConsoleHistory)>,
    mut history_idx: Local<usize>,
    mut filtered_history: Local<Option<Vec<usize>>>,
    mut original_value: Local<Option<String>>,
) {
    let key = input.matched_logical_keys().next();
    if key.is_none() {
        return;
    }
    let key = key.unwrap();
    let mut value = 0;
    match key {
        Key::ArrowUp => value = 1,
        Key::ArrowDown => value = -1,
        Key::Enter => {
            *history_idx = 0;
            *filtered_history = None;
            *original_value = None;
        }
        _ => {}
    }
    if matches!(key, Key::ArrowUp | Key::ArrowDown) {
        let (mut input_text, history) = q_console.get_mut(input.console_id).unwrap();
        if filtered_history.is_none() {
            *original_value = Some(std::mem::take(&mut input_text.text));
            let f = history
                .iter()
                .enumerate()
                .filter_map(|(i, s)| s.starts_with(original_value.as_ref().unwrap()).then_some(i))
                .collect::<Vec<_>>();
            *filtered_history = Some(f);
            debug!("Setting filtered_history {filtered_history:?}");
            debug!("Setting original_value {original_value:?}");
        }
        let fh = filtered_history.as_ref().unwrap();
        let ov = original_value.as_ref().unwrap();
        *history_idx = history_idx.saturating_add_signed(value).min(fh.len());
        if *history_idx == 0 {
            input_text.text = ov.clone();
        } else {
            let idx = fh[fh.len().saturating_sub(*history_idx + 1)];
            input_text.text = history[idx].clone();
        }
        input_text.cursor = input_text.text.len();
    }
}

pub fn plugin(app: &mut App) {
    app.register_console_action(
        ConsoleActionKeybind::new([Key::ArrowUp, Key::ArrowDown, Key::Enter])
            .without_modifiers([KeyCode::ShiftLeft, KeyCode::ShiftRight]),
        set_from_history,
    );
}
