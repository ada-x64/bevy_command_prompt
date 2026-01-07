use bevy::input::keyboard::Key;

use crate::prelude::*;

/// Sets the
pub fn set_from_history(
    input: In<ConsoleActionInput>,
    mut q_console: Query<&mut Console>,
    mut history_idx: Local<usize>,
    mut filtered_history: Local<Option<Vec<usize>>>,
    mut original_value: Local<Option<String>>,
) {
    let key = input.matched_keys.first().unwrap();
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
        let mut console = q_console.get_mut(input.console_id).unwrap();
        if filtered_history.is_none() {
            *original_value = Some(std::mem::take(&mut console.input));
            let f = console
                .history
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
            console.input = ov.clone();
        } else {
            let idx = fh[fh.len().saturating_sub(*history_idx + 1)];
            console.input = console.history[idx].clone();
        }
    }
}

pub fn plugin(app: &mut App) {
    app.register_console_action(
        ConsoleAction::new([Key::ArrowUp, Key::ArrowDown, Key::Enter]),
        set_from_history,
    );
}
