use bevy::input::keyboard::Key;

use crate::prelude::*;

pub fn delete_char(trigger: On<ConsoleActionEvent>, mut console_q: Query<&mut Console>) {
    if let Ok(mut console) = console_q.get_mut(trigger.console_id) {
        console.buffer.pop();
    }
}
pub fn delete_word(trigger: On<ConsoleActionEvent>, mut console_q: Query<&mut Console>) {
    if let Ok(mut console) = console_q.get_mut(trigger.console_id) {
        let last_ws = console.input.rfind(char::is_whitespace).unwrap_or_default();
        console.input.truncate(last_ws);
    }
}

pub fn plugin(app: &mut App) {
    app.register_console_action(ConsoleAction::new(Key::Backspace), delete_char);
    app.register_console_action(
        ConsoleAction::new(Key::Backspace)
            .with_modifiers([KeyCode::ControlLeft, KeyCode::ControlRight]),
        delete_word,
    );
}
