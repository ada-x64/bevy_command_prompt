use bevy::input::keyboard::Key;

use crate::prelude::*;

pub fn delete_char(input: In<ConsoleActionInput>, mut console_q: Query<&mut Console>) {
    if let Ok(mut console) = console_q.get_mut(input.console_id) {
        console.input.pop();
    }
}
pub fn delete_word(input: In<ConsoleActionInput>, mut console_q: Query<&mut Console>) {
    if let Ok(mut console) = console_q.get_mut(input.console_id) {
        let last_ws = console.input.rfind(char::is_whitespace).unwrap_or_default();
        console.input.truncate(last_ws);
    }
}

pub fn plugin(app: &mut App) {
    app.register_console_action(
        ConsoleAction::new(Key::Backspace)
            .without_modifiers([KeyCode::ControlLeft, KeyCode::ControlRight]),
        delete_char,
    );
    app.register_console_action(
        ConsoleAction::new(Key::Backspace)
            .with_modifiers([KeyCode::ControlLeft, KeyCode::ControlRight]),
        delete_word,
    );
}
